use std::collections::HashMap;
use std::f32::consts::E;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use common::utils::map_ipv4_to_ipv6;
use quinn::rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use rustls::internal::msgs::base;
use serde_json::json;
use sessio_coordinator_common::common::{Packet, PacketBase, ServerConnectionRequest, ServerPacket};
use sessio_coordinator_common::holepuncher::HolepunchService;

use tokio::process::Command as TokioCommand;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

use dirs;
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::broadcast::Receiver;

use clap::Parser;
use log::{debug, error, info};
use quinn::{crypto, Connection, Endpoint, EndpointConfig, ServerConfig, VarInt};
use rand::rngs::OsRng;
use rand::{seq, CryptoRng};
use russh::server::{Msg, Server as _, Session};
use russh::*;
use russh::MethodKind;
use russh::keys::{load_secret_key, PublicKeyBase64};
use russh::keys::ssh_key::{Algorithm, HashAlg, LineEnding, PrivateKey, PublicKey};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV6, TcpListener};
use std::process::{Command, Stdio};
use std::str;
use tokio::fs::read_to_string;
use tokio::sync::{mpsc, mpsc::Sender, Mutex};
use tokio::{select, time};
use toml::ser;

use anyhow::{bail, Context, Error};
use portable_pty::{
    native_pty_system, CommandBuilder, MasterPty, PtyPair, PtySize, PtySystem, SlavePty,
};
use russh::Channel;
use serde::Deserialize;
use std::pin::Pin;
use std::task::Poll;
use std::time::{Duration, Instant};

use crate::{sftp::*, Opt};
use crate::config_manager::ServerConfigManager;
use common::utils::keygen::generate_keypair;
use sessio_coordinator_common::coordinator_client::*;
use url::Url;
use common::utils::quinn_utils::configure_client;
use common::utils::streams::BiStream;

/// Returns default server configuration along with its certificate.
fn configure_server() -> anyhow::Result<ServerConfig> {
    let _ = rustls::crypto::ring::default_provider().install_default();
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();

    let cert_der = CertificateDer::from(cert.cert);
    let priv_key = PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der());

    let mut server_config =
        ServerConfig::with_single_cert(vec![cert_der.clone()], priv_key.into())?;

    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());
    transport_config.max_idle_timeout(Some(VarInt::from_u32(60_000).into()));
    transport_config.keep_alive_interval(Some(std::time::Duration::from_secs(5)));
    #[cfg(windows)]
    transport_config.mtu_discovery_config(Some(quinn::MtuDiscoveryConfig::default()));

    Ok(server_config)
}

#[allow(unused)]
pub fn make_server_endpoint(socket: UdpSocket) -> anyhow::Result<Endpoint> {
    let server_config = configure_server()?;

    //todo set IPV6_V6ONLY false on windows

    let runtime = quinn::default_runtime()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "no async runtime found"))?;

    let mut endpoint = Endpoint::new_with_abstract_socket(
        EndpointConfig::default(),
        Some(server_config),
        runtime.wrap_udp_socket(socket.into_std()?)?,
        runtime,
    )?;

    // Needed if this endpoint is the one initiating connections (in hole punching)
    let client_cfg = configure_client();
    endpoint.set_default_client_config(client_cfg?);

    Ok(endpoint)
}

#[derive(Deserialize, Debug)]
struct ServerConf {
    proxy: HashMap<String, SocketAddr>,
}

impl ServerConf {
    fn new() -> Self {
        ServerConf {
            proxy: HashMap::<String, SocketAddr>::new(),
        }
    }
}

#[derive(Clone)]
struct PeerChangeMsg {
    pub id: String,
    pub new_ip: SocketAddr,
    pub old_ip: SocketAddr,
}

async fn listen_to_coordinator(endpoint: Endpoint, mut holepuncher: HolepunchService) {
    let mut receiver: Receiver<Packet> = holepuncher.c_client.subscribe_to_packets().await;
    let mut sender = holepuncher.c_client.new_packet_sender();

    let id = holepuncher.c_client.id_own.clone();

    tokio::spawn(async move {
        info!("Listening to coordinator");
        loop {
            tokio::select! {
                // Check if the holepuncher connection is closed
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                    if holepuncher.c_client.is_closed() {
                        log::warn!(
                            "Coordinator connection closed. Reconnecting.."
                        );
                        if let Err(e) = holepuncher.reconnect().await {
                            log::error!("{}", e);
                            continue;
                        }
                        receiver = holepuncher.c_client.subscribe_to_packets().await;
                        sender = holepuncher.c_client.new_packet_sender();
                        //Restart ip updater
                        holepuncher.start_connection_update_task();
                    }
                },

                // Await the next packet from the receiver
                result = receiver.recv() => {
                    let packet = match result {
                        Ok(packet) => packet,
                        Err(e) => {
                            log::error!("Failed to recv packet from coordinator. {}", e);
                            break;
                        }
                    };
                    info!("Server received from coordinator {:?}", packet);
                    match packet {
                        Packet::ConnectTo(data) => {
                            log::info!("connect to received");
                            match endpoint.connect(data.target, "client") {
                                Ok(_) => {
                                    info!("Connection attempt made!");
                                }
                                Err(e) => {
                                    info!("Connection failed: {}", e);
                                }
                            }
                            let _ = sender
                                .send(ServerPacket {
                                    base: Some(PacketBase {
                                        own_id: id.clone(),
                                        token: holepuncher.c_client.token.clone(),
                                    }),
                                    packet: Packet::ServerConnectionRequest(ServerConnectionRequest {
                                        session_id: data.session_id,
                                    }),
                                })
                                .await;
                        }
                        Packet::PeerIpChanged(data) => {
                            //Creating NAT mappings
                            _ = endpoint.connect(data.new_ip, "client");
                        }
                        _ => {}
                    }
                }
            }
        }
    });
}

pub async fn run(opt: crate::RunConfig) {
    let mut builder = env_logger::Builder::from_default_env();
    if cfg!(debug_assertions) {
        // Debug mode
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();

    // Initialize configuration manager
    let mut config_manager = ServerConfigManager::new()
        .expect("Failed to initialize configuration manager");
    
    // Load or create default settings
    let settings = config_manager.load_settings().await
        .expect("Failed to load server settings");
    
    // Check if server is registered
    if !config_manager.is_registered().await.unwrap_or(false) {
        eprintln!("Server is not registered. Please run 'sessio-server install' first.");
        std::process::exit(1);
    }
    
    let (jwt_token, device_id) = config_manager.get_account_info().await
        .expect("Failed to get account information");
    
    info!("Using JWT token for authentication");
    info!("Using device ID: {}", device_id);
    
    // Load host key from settings
    let host_key = load_host_key(&settings.private_key_path).unwrap();
    
    // Get coordinator URL from settings
    let coordinator_url = config_manager.get_coordinator_url().await
        .expect("Invalid coordinator URL in settings");
    
    // Check if using HTTP coordinator is allowed
    if coordinator_url.scheme() == "http" && !config_manager.is_http_coordinator_allowed().await.unwrap_or(false) {
        panic!("HTTP coordinator connections are not allowed. Enable 'dangerously_use_http_coordinator' setting in config file or use HTTPS.");
    }

    // Get SSH configuration from settings
    let ssh_config = config_manager.get_ssh_config().await
        .expect("Failed to get SSH configuration");
    
    let config = server::Config {
        inactivity_timeout: Some(Duration::from_secs(ssh_config.inactivity_timeout)),
        auth_rejection_time: Duration::from_secs(ssh_config.auth_rejection_time),
        auth_rejection_time_initial: Some(Duration::from_secs(0)),
        keys: vec![host_key.clone()],
        ..Default::default()
    };
    let sock_v6 = UdpSocket::bind::<SocketAddr>("[::]:0".parse().unwrap())
        .await
        .unwrap();


    // Discover external IPs before creating endpoint
    let (external_ipv4, external_ipv6) = CoordinatorClient::get_external_ips_dual_sock(&sock_v6).await;
    
    info!("Server discovered external IPs - IPv4: {:?}, IPv6: {:?}", external_ipv4, external_ipv6);
    
    // Use the IPv6 socket for the endpoint (dual-stack)
    let mut endpoint_v6 = make_server_endpoint(sock_v6).unwrap();

    // CoordinatorClient::configure_crypto removed for WebSocket-only implementation
    let holepuncher =
        HolepunchService::new(coordinator_url.clone(), jwt_token.clone(), external_ipv4, external_ipv6, device_id.clone(), endpoint_v6.clone())
            .await
            .unwrap();
    
    // Start consolidated authorized keys synchronization using coordinator client
    let sync_interval = config_manager.get_authorized_keys_sync_interval().await
        .expect("Failed to get authorized keys sync interval");
    
    // Load passkey JSON for signature verification
    let account_data = config_manager.load_account_data().await
        .expect("Failed to load account data");
    let passkey_json = account_data.passkey_public_key; // Now contains full JSON Passkey instead of base64 CBOR
    
    // For server, we need to write to each user's home directory
    // The coordinator client's sync task will handle writing to the appropriate path
    // We'll use a placeholder path here - the actual implementation should iterate through users
    let authorized_keys_path = dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/root"))
        .join(".sessio/authorized_keys");
    
    holepuncher.c_client.start_authorized_keys_sync_task(
        jwt_token.clone(),
        passkey_json,
        sync_interval,
        authorized_keys_path,
        false, // Only include verified keys
    ).await;

    // Start heartbeat task
    holepuncher.c_client.start_heartbeat_task(
        device_id.clone(),
        jwt_token.clone(),
    ).await;
    
    listen_to_coordinator(endpoint_v6.clone(), holepuncher).await;

    let config = Arc::new(config);

    let config_v6 = config.clone();
    let v6_handle = tokio::spawn(async move {
        let mut sh = Server {};
        sh.run_quic(config_v6, &endpoint_v6).await.unwrap();
    });
    let v6 = tokio::join!(v6_handle);
}

fn load_host_key<P: AsRef<Path>>(path: P) -> Result<russh::keys::ssh_key::PrivateKey, Box<dyn std::error::Error>> {
    let path = path.as_ref();
    if !path.exists() {
        generate_keypair(
            path.parent().unwrap_or_else(|| Path::new("keys/")),
            Algorithm::Ed25519,
            path.file_name().unwrap().to_str().unwrap(),
        )?;
    }
    let private_key = load_secret_key(path.to_str().unwrap(), None)?;
    Ok(private_key)
}

//A session
#[derive(Clone, Default)]
struct ServerSession {
    clients: Arc<Mutex<HashMap<ChannelId, Channel<Msg>>>>,
    ptys: Arc<Mutex<HashMap<ChannelId, Arc<PtyStream>>>>,
    id: Arc<AtomicUsize>,
    user: Option<String>,
}

struct Server {}

struct PtyStream {
    reader: Mutex<Box<dyn Read + Send>>,
    writer: Mutex<Box<dyn Write + Send>>,
    slave: Mutex<Box<dyn SlavePty + Send>>,
    master: Mutex<Box<dyn MasterPty + Send>>,
}

trait QuicServer {
    async fn run_quic(
        &mut self,
        config: Arc<russh::server::Config>,
        connection: &Endpoint,
    ) -> Result<(), std::io::Error>;
}

#[async_trait::async_trait]
impl server::Server for Server {
    type Handler = ServerSession;
    fn new_client(&mut self, _: Option<std::net::SocketAddr>) -> ServerSession {
        ServerSession::default()
    }
}

impl QuicServer for Server {
    async fn run_quic(
        &mut self,
        config: Arc<server::Config>,
        endpoint: &Endpoint,
    ) -> Result<(), io::Error> {
        let config_cloned = config.clone();

        loop {
            let conf = config_cloned.clone();
            info!("Waiting for connections..");
            let incoming_conn = match endpoint.accept().await {
                Some(conn) => conn,
                None => {
                    continue;
                }
            };
            let conn = match incoming_conn.await {
                Ok(conn) => conn,
                Err(e) => {
                    error!("[server] accept connection error: {}", e);
                    continue;
                }
            };

            let sni = conn
                .handshake_data()
                .unwrap()
                .downcast::<crypto::rustls::HandshakeData>()
                .unwrap()
                .server_name
                .unwrap_or(conn.remote_address().ip().to_string());

            info!(
                "[server] connection accepted: ({}, {})",
                conn.remote_address(),
                sni
            );

            //A single connection can spawn multiple streams

            tokio::spawn(async move {
                loop {
                    let conf = conf.clone();
                    let remote = conn.remote_address();
                    let (mut quinn_send, mut quinn_recv) = match conn.accept_bi().await {
                        Ok(stream) => stream,
                        Err(e) => {
                            error!("[server] open quic stream error: {}", e);
                            break;
                        }
                    };

                    let mut bi_stream = BiStream {
                        recv_stream: quinn_recv,
                        send_stream: quinn_send,
                    };

                    let handler = ServerSession {
                        ..Default::default()
                    };

                    info!("New client connected!");

                    tokio::spawn(async move {
                        let session =
                            match russh::server::run_stream(conf, Box::new(bi_stream), handler)
                                .await
                            {
                                Ok(s) => s,
                                Err(e) => {
                                    error!("Connection setup failed");
                                    return;
                                }
                            };

                        match session.await {
                            Ok(_) => {
                                debug!("Connection closed")
                            }
                            Err(e) => {
                                error!("Connection closed with error {}", e);
                                //TODO handle errors
                            }
                        }
                    });
                }
            });
        }
    }
}

impl ServerSession {
    pub async fn take_channel(&mut self, channel_id: ChannelId) -> Channel<Msg> {
        let mut clients = self.clients.lock().await;
        clients.remove(&channel_id).unwrap()
    }
}


impl server::Handler for ServerSession {
    type Error = anyhow::Error;

    /// Basic local forwarding
    #[allow(unused_variables)]
    async fn channel_open_direct_tcpip(
        &mut self,
        mut channel: Channel<Msg>,
        host_to_connect: &str,
        port_to_connect: u32,
        originator_address: &str,
        originator_port: u32,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        info!(
            "Forwarding {}:{} for {}:{}",
            host_to_connect, port_to_connect, originator_address, originator_port
        );
        let host = host_to_connect.to_string();
        let mut stream = TcpStream::connect((host, port_to_connect as u16)).await?;

        tokio::spawn(async move {
            let mut cin = channel.make_writer();
            let mut cout = channel.make_reader();

            let (mut s_read, mut s_write) = stream.split();
            tokio::try_join! {
                tokio::io::copy(&mut s_read, &mut cin),
                tokio::io::copy(&mut cout, &mut s_write)
            }
        });

        Ok(true)
    }

    async fn subsystem_request(
        &mut self,
        channel_id: ChannelId,
        name: &str,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        info!("subsystem: {}", name);

        if name == "sftp" {
            let channel = self.take_channel(channel_id).await;
            let user = self.user.as_ref().unwrap().clone();
            let sftp = SftpSession::new(user);
            session.channel_success(channel_id);
            russh_sftp::server::run(channel.into_stream(), sftp).await;
        } else {
            session.channel_failure(channel_id);
        }

        Ok(())
    }

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        {
            let new_id = self.id.fetch_add(1, Ordering::SeqCst); // Atomic increment
            let mut clients = self.clients.lock().await;
            info!(
                "Channel session opened! Client ID: {}, Channel ID: {:?}",
                new_id,
                channel.id()
            );
            clients.insert(channel.id(), channel);
        }
        Ok(true)
    }

    /*     async fn open_channel_stream(&mut self,
        channel: ChannelId)
        -> Result<Option<Box<dyn SubStream>>, Self::Error> {

        if let Some(conn) = self.connection.as_ref() {
            let res = conn.open_bi().await?;
            let option = Option::from(Box::new(BiStream {send_stream: res.0, recv_stream: res.1}));

            info!("Opened a new channel stream!");
            return Ok(option.map(|b| b as Box<dyn russh::SubStream>))
        }

        Ok(None)
    } */

    async fn shell_request(
        &mut self,
        channel_id: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let handle_reader = session.handle();
        let handle_waiter = session.handle();

        let ptys = self.ptys.clone();

        let Some(user) = &self.user else {
            bail!("Authentication has not finished yet(?)");
        };
        let shell = if cfg!(windows) {
            vec![
                OsString::from("cmd.exe"),
                OsString::from("/C"),
                OsString::from(format!("runas /user:{}", user)),
            ]
        } else {
            vec![
                OsString::from("/usr/bin/sudo"),
                OsString::from("-u"),
                OsString::from(user),
                OsString::from("/bin/bash"),
            ]
        };

        tokio::spawn(async move {
            let pty_cloned = ptys.clone();
            let reader_handle = tokio::spawn(async move {
                loop {
                    let mut buffer = vec![0; 1024];
                    let pty_cloned = ptys.clone();
                    match tokio::task::spawn_blocking(move || {
                        let stream = pty_cloned.blocking_lock().get(&channel_id).unwrap().clone();
                        let mut reader = stream.reader.blocking_lock();
                        reader.read(&mut buffer).map(|n| (n, buffer))
                    })
                    .await
                    {
                        Ok(Ok((n, _))) if n == 0 => {
                            debug!("PTY: No more data to read.");
                            break;
                        }
                        Ok(Ok((n, buffer))) => {
                            if let Err(e) = handle_reader
                                .data(channel_id, CryptoVec::from_slice(&buffer[0..n]))
                                .await
                            {
                                error!("Error sending PTY data to client: {:?}", e);
                                break;
                            }
                        }
                        Ok(Err(e)) => {
                            error!("PTY read error: {:?}", e);
                            break;
                        }
                        Err(e) => {
                            error!("Join error: {:?}", e);
                            break;
                        }
                    }
                }
            });

            let child_status = tokio::task::spawn_blocking(move || {
                let stream = pty_cloned.blocking_lock().get(&channel_id).unwrap().clone();

                let command_builder = CommandBuilder::from_argv(shell);

                let mut child = stream
                    .slave
                    .blocking_lock()
                    .spawn_command(command_builder)
                    .expect("Failed to spawn child process");
                child.wait().expect("Failed to wait on child process")
            })
            .await;

            match child_status {
                Ok(status) => {
                    if status.success() {
                        info!("Child process exited successfully.");
                        //reader_handle.abort();
                        let _ = handle_waiter
                            .exit_status_request(channel_id, status.exit_code())
                            .await;
                        let _ = handle_waiter.close(channel_id).await;
                    } else {
                        error!("Child process exited with status: {:?}", status);
                        //reader_handle.abort();
                        let _ = handle_waiter
                            .exit_status_request(channel_id, status.exit_code())
                            .await;
                        let _ = handle_waiter.close(channel_id).await;
                    }
                }
                Err(e) => {
                    error!("Failed to wait on child process: {:?}", e);
                }
            }
        });
        Ok(())
    }

    async fn window_change_request(
        &mut self,
        channel_id: ChannelId,
        col_width: u32,
        row_height: u32,
        pix_width: u32,
        pix_height: u32,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let clone = self.ptys.clone();
        let ptys_guard = clone.lock().await;
        let pty = ptys_guard.get(&channel_id).unwrap();

        let _ = pty.master.lock().await.resize(PtySize {
            rows: row_height as u16,
            cols: col_width as u16,
            pixel_width: pix_width as u16,
            pixel_height: pix_height as u16,
        });

        Ok(())
    }

    async fn pty_request(
        &mut self,
        channel_id: ChannelId,
        term: &str,
        col_width: u32,
        row_height: u32,
        pix_width: u32,
        pix_height: u32,
        modes: &[(Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        info!("Requesting PTY!");

        info!(
            "PTY request received: term={}, col_width={}, row_height={}",
            term, col_width, row_height
        );

        let pty_system = native_pty_system();
        let pty_pair = pty_system.openpty(PtySize {
            rows: row_height as u16,
            cols: col_width as u16,
            pixel_width: pix_width as u16,
            pixel_height: pix_height as u16,
        })?;

        let pair = pty_pair;
        let slave = pair.slave;
        let mut master = pair.master;

        let master_reader = Mutex::new(master.try_clone_reader().unwrap());
        let mut master_writer = Mutex::new(master.take_writer().unwrap());

        let master_lock = Mutex::new(master);

        self.ptys.lock().await.insert(
            channel_id,
            Arc::new(PtyStream {
                reader: master_reader,
                writer: master_writer,
                master: master_lock,
                slave: Mutex::new(slave),
            }),
        );

        session.request_success();
        Ok(())
    }

    async fn auth_password(
        &mut self,
        user: &str,
        password: &str,
    ) -> Result<server::Auth, Self::Error> {
        Ok(server::Auth::Reject {
            proceed_with_methods: Some({
                let mut methods = MethodSet::empty();
                methods.push(MethodKind::PublicKey);
                methods
            }),
            partial_success: false,
        })
    }

    async fn auth_publickey_offered(
        &mut self,
        user: &str,
        public_key: &PublicKey,
    ) -> Result<server::Auth, Self::Error> {
        //User based auth isn't implemented yet

        log::debug!("Attempting to authenticate user: {}", user);
        log::debug!("Public key: {:?}", public_key);

        let authorized_keys = common::utils::keygen::read_authorized_keys(Some(user))
            .await
            .map_err(|e| {
                error!("{}", e);
                russh::Error::CouldNotReadKey
            })?;
        let res = if authorized_keys.contains(&public_key) {
            server::Auth::Accept
        } else {
            server::Auth::Reject {
                proceed_with_methods: None,
                partial_success: false,
            }
        };

        Ok(res)
    }

    async fn auth_publickey(
        &mut self,
        user: &str,
        public_key: &PublicKey,
    ) -> Result<server::Auth, Self::Error> {
        self.user = Some(user.into());
        //Accept after auth_publickey_offered has succeeded
        Ok(server::Auth::Accept)
    }

    async fn data(
        &mut self,
        channel_id: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        if let Some(pty_stream) = self.ptys.lock().await.get_mut(&channel_id) {
            log::info!("pty_writer: data = {data:02x?}");

            let mut pty_writer = pty_stream.writer.lock().await;

            pty_writer.write_all(data).map_err(anyhow::Error::new)?;

            pty_writer.flush().map_err(anyhow::Error::new)?;
        }
        Ok(())
    }

    async fn extended_data(
        &mut self,
        channel: ChannelId,
        code: u32,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        info!("Receiving extended data!");
        Ok(())
    }

    async fn channel_eof(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        // After a client has sent an EOF, indicating that they don't want
        // to send more data in this session, the channel can be closed.
        info!("Receiving channel eof!");
        session.close(channel);

        Ok(())
    }

    #[allow(unused_variables)]
    async fn exec_request(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        info!("Receiving exec req!");
        Ok(())
    }

    #[allow(unused_variables)]
    async fn signal(
        &mut self,
        channel: ChannelId,
        signal: Sig,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        info!("Receiving signal!");
        Ok(())
    }

    #[allow(unused_variables)]
    async fn channel_close(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        info!("Receiving channel close!");
        session.close(channel);
        Ok(())
    }

    ///According to RFC4254, the client must first request the reverse port forwarding
    async fn tcpip_forward(
        &mut self,
        address: &str,
        port: &mut u32,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        info!("Received tcpip_forward");
        let handle = session.handle();
        let address = address.to_string();
        let port = *port;
        tokio::spawn(async move {
            let channel = handle
                .channel_open_forwarded_tcpip(address, port, "1.2.3.4", 1234)
                .await
                .unwrap();
            let _ = channel.data(&b"Hello from a forwarded port"[..]).await;
            let _ = channel.eof().await;
        });
        Ok(true)
    }
}




