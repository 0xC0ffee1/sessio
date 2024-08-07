use std::collections::HashMap;
use std::f32::consts::E;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use rustls::internal::msgs::base;
use serde_json::json;
use tokio::process::Command as TokioCommand;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

use homedir::home;
use tokio::net::{TcpStream, UdpSocket};
use tokio::time;
use std::process::{Command, Stdio};
use std::str;
use async_trait::async_trait;
use russh::server::{Msg, Server as _, Session};
use russh::*;
use russh_keys::*;
use tokio::sync::Mutex;
use rand::rngs::OsRng;
use ssh_key::{Algorithm, HashAlg, LineEnding, PrivateKey, PublicKey};
use russh_keys::key::KeyPair;
use rand::CryptoRng;
use log::{debug, error, info};
use tokio::fs::read_to_string;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV6, TcpListener};
use clap::Parser;
use quinn::{crypto, Connection, Endpoint, ServerConfig, VarInt};

use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtyPair, PtySize, PtySystem, SlavePty};
use anyhow::{bail, Context, Error};
use std::pin::Pin;
use std::task::Poll;
use serde::Deserialize;
use russh::Channel;
use std::sync::mpsc::channel;
use std::time::Duration;

use common::utils::keygen::generate_keypair;
use coordinator::coordinator_client::*;
use url::Url;
use crate::sftp::*;

use common::utils::streams::BiStream;


#[derive(Parser, Debug)]
#[clap(name = "client")]
pub struct Opt {
    #[clap(long, short = 'c')]
    coordinator: Url,

    //The identifier of this machine
    id: String,
    
    //The path to your private key
    #[clap(long, short = 'p', default_value = "keys/ssh_host_ed25519_key")]
    private_key: PathBuf
}

/// Returns default server configuration along with its certificate.
fn configure_server() -> Result<(ServerConfig, Vec<u8>), Box<dyn std::error::Error>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = cert.serialize_der().unwrap();
    let priv_key = cert.serialize_private_key_der();
    let priv_key = rustls::PrivateKey(priv_key);
    let cert_chain = vec![rustls::Certificate(cert_der.clone())];

    let mut server_config = ServerConfig::with_single_cert(cert_chain, priv_key)?;
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());
    transport_config.max_idle_timeout(Some(VarInt::from_u32(60_000).into()));
    transport_config.keep_alive_interval(Some(std::time::Duration::from_secs(5)));
    #[cfg(any(windows, os = "linux"))]
    transport_config.mtu_discovery_config(Some(quinn::MtuDiscoveryConfig::default()));

    Ok((server_config, cert_der))
}

#[allow(unused)]
pub fn make_server_endpoint(bind_addr: SocketAddr) -> Result<(Endpoint, Vec<u8>), Box<dyn std::error::Error>> {
    let (server_config, server_cert) = configure_server()?;
    let endpoint = Endpoint::server(server_config, bind_addr)?;
    Ok((endpoint, server_cert))
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


async fn attempt_holepunch(id: String, coordinator: Url, 
    mut endpoint_v4: Endpoint, 
    mut endpoint_v6: Endpoint) 
    -> anyhow::Result<(), anyhow::Error> {

    let mut registration_interval = time::interval(Duration::from_secs(60));
    loop {

        CoordinatorClient::configure_client(&mut endpoint_v4);
        CoordinatorClient::configure_client(&mut endpoint_v6);

        let mut client = loop {
            match CoordinatorClient::connect(coordinator.clone(), id.clone(), endpoint_v4.clone()).await {
                Ok(client) => break client,
                Err(e) => {
                    info!("Failed to connect to coordination server {}\nRetrying in 5 seconds..", e);
                    time::sleep(Duration::from_secs(5)).await;
                }
            }
        };
        _ = client.register_endpoint(endpoint_v6.local_addr().unwrap()).await;

        _ = client.new_session().await;
        info!("Created new session!");

        loop {
            info!("Waiting for client to connect.");
            tokio::select! {
                _ = registration_interval.tick() => {
                    info!("Registering with the server");
                    client.register_endpoint(endpoint_v6.local_addr().unwrap()).await.unwrap();
                }
                response = client.read_response::<HashMap<String, String>>() => {
                    let response = response?;
                    match response.get("id").map(String::as_str) {
                        Some("CONNECT_TO") => {
                            info!("Attempting connection");
                            
                            let target: SocketAddr = response.get("target").unwrap().parse().unwrap();

                            let endpoint: &Endpoint = if target.is_ipv4() {
                                &endpoint_v4
                            } else {
                                &endpoint_v6
                            };

                            match endpoint.connect(target, "client") {
                                Ok(_) => {
                                    info!("Connection attempt made!");
                                }
                                Err(e) => {
                                    info!("Connection failed: {}", e);
                                }
                            }
                            client.send_packet(&json!({"id":"SERVER_SENT_CONNECTION_REQUEST", "own_id":id.clone()})).await?;
                            
                        }
                        Some("SESSION_FINISHED") => {
                            break;
                        }
                        _ => {
                            // Handle other messages if needed
                        }
                    }
                }
            }
        }
    }
}



#[tokio::main]
pub async fn run(opt: Opt) {
    let mut builder = env_logger::Builder::from_default_env();
    if cfg!(debug_assertions) {
        // Debug mode
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();

    let host_key = load_host_key(opt.private_key).unwrap();

    let config = russh::server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_secs(3),
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
        keys: vec![host_key],
        ..Default::default()
    };

    
    let conf: ServerConf = ServerConf::new();

    let addr_v4 = "0.0.0.0:2222";
    let v6_ip = common::utils::ipv6::get_first_global_ipv6().unwrap_or(Ipv6Addr::UNSPECIFIED);

    let (endpoint_v4, _) = make_server_endpoint(addr_v4.parse().unwrap()).unwrap();
    let (endpoint_v6, _) = make_server_endpoint(SocketAddr::V6(SocketAddrV6::new(v6_ip, 0, 0, 0))).unwrap();

    let endpoint_v4_clone = endpoint_v4.clone();
    let endpoint_v6_clone = endpoint_v6.clone();

    tokio::spawn(async move {
        attempt_holepunch(opt.id, opt.coordinator, endpoint_v4_clone, endpoint_v6_clone).await;
    });

    let config = Arc::new(config);
    
    println!("Started!");
    let config_v4 = config.clone();
    let v4_handle = tokio::spawn(async move {
        let mut sh = Server {};
        sh.run_quic(config_v4, &endpoint_v4).await.unwrap();
    });
    let config_v6 = config.clone();
    let v6_handle = tokio::spawn(async move {
        let mut sh = Server {};
        sh.run_quic(config_v6, &endpoint_v6).await.unwrap();
    });
    let (v4,v6) = tokio::join!(v4_handle, v6_handle);
    // Handle potential errors
    v4.unwrap();
    v6.unwrap();
}

fn load_host_key<P: AsRef<Path>>(path: P) -> Result<KeyPair, Box<dyn std::error::Error>> {
    let path = path.as_ref();
    if !path.exists() {
        if path != Path::new("keys/ssh_host_ed25519_key") {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Specified private key file doesn't exist.")));
        }
        generate_keypair("keys/", Algorithm::Ed25519, "ssh_host_ed25519_key");
    }
    let private_key = russh_keys::load_secret_key(path.to_str().unwrap(), None)?;
    Ok(private_key)
}

//A session
#[derive(Clone, Default)]
struct ServerSession {
    clients: Arc<Mutex<HashMap<ChannelId, Channel<Msg>>>>,
    ptys: Arc<Mutex<HashMap<ChannelId, Arc<PtyStream>>>>,
    id: Arc<AtomicUsize>,
    user: Option<String>
}

struct Server {}

struct PtyStream{
    reader: Mutex<Box<dyn Read + Send>>,
    writer: Mutex<Box<dyn Write + Send>>,
    slave: Mutex<Box<dyn SlavePty + Send>>,
    master: Mutex<Box<dyn MasterPty + Send>>
}

trait QuicServer{
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
        loop{
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
                sni);

            

            //A single connection can spawn multiple streams
            tokio::spawn(async move {
                loop {
                    let conf = conf.clone();
                    let (mut quinn_send, mut quinn_recv) = match conn.accept_bi().await {
                        Ok(stream) => stream,
                        Err(e) => {
                            error!("[server] open quic stream error: {}", e);
                            break;
                        }
                    };
                    
                    let mut bi_stream = BiStream {recv_stream: quinn_recv, send_stream: quinn_send};

                    let handler = ServerSession {
                        ..Default::default()
                    };

                    info!("New client connected!");

                    tokio::spawn(async move {
                        let session = match russh::server::run_stream(conf, bi_stream, handler).await {
                            Ok(s) => s,
                            Err(e) => {
                                error!("Connection setup failed");
                                return
                            }
                        };
                        match session.await {
                            Ok(_) => debug!("Connection closed"),
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


async fn read_authorized_keys(user: &str) -> anyhow::Result<Vec<key::PublicKey>> {
    
    let path = home(user)?
    .with_context(|| format!("Home directory not found for user {}", user))?
    .join(".sessio/authorized_keys");

    if !path.exists() {
        // Create the file and its parent directories if they don't exist
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        tokio::fs::File::create(&path).await?;
    }

    let mut file = tokio::fs::File::open(&path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    let mut keys = Vec::new();

    for line in contents.lines() {
        let mut split = line.split_whitespace();
        
        split.next();

        if let Ok(public_key) = russh_keys::parse_public_key_base64(split.next().unwrap()) {
            keys.push(public_key);
        }
        else {
            anyhow::bail!("Failed to read authorized public key {}", line)
        }
    }

    Ok(keys)
}


impl ServerSession {
    pub async fn take_channel(&mut self, channel_id: ChannelId) -> Channel<Msg> {
        let mut clients = self.clients.lock().await;
        clients.remove(&channel_id).unwrap()
    }
}

#[async_trait]
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
        
        info!("Forwarding {}:{} for {}:{}", host_to_connect, port_to_connect, originator_address, originator_port);
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
            info!("Channel session opened! Client ID: {}, Channel ID: {:?}", new_id, channel.id());
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
            vec![OsString::from("cmd.exe"), OsString::from("/C"), OsString::from(format!("runas /user:{}", user))]
        } else {
            vec![OsString::from("/usr/bin/sudo"), OsString::from("-u"), OsString::from(user), OsString::from("/bin/bash")]
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

                    }).await {
                        Ok(Ok((n, buffer))) if n == 0 => {
                            debug!("PTY: No more data to read.");
                            break;
                        }
                        Ok(Ok((n,buffer))) => {
                            debug!("PTY read {} bytes", n);
                            //info!("Sending {}", String::from_utf8_lossy(&buffer[0..n]));
                            if let Err(e) = handle_reader.data(channel_id, CryptoVec::from_slice(&buffer[0..n])).await {
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

                let mut child = stream.slave.blocking_lock().spawn_command(command_builder).expect("Failed to spawn child process");
                child.wait().expect("Failed to wait on child process")
            }).await;

            match child_status {
                Ok(status) => {
                    if status.success() {
                        info!("Child process exited successfully.");
                        //reader_handle.abort();
                        let _ = handle_waiter.exit_status_request(channel_id, status.exit_code()).await;
                        let _ = handle_waiter.close(channel_id).await;
                    } else {
                        error!("Child process exited with status: {:?}", status);
                        //reader_handle.abort();
                        let _ = handle_waiter.exit_status_request(channel_id, status.exit_code()).await;
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

        let _ = ptys_guard.get(&channel_id).unwrap().master.lock().await.resize(PtySize {
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

        info!("PTY request received: term={}, col_width={}, row_height={}", term, col_width, row_height);

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
        

        self.ptys
        .lock()
        .await
            .insert(channel_id, Arc::new(PtyStream {
                reader: master_reader,
                writer: master_writer,
                master: master_lock,
                slave: Mutex::new(slave)
            }));
        
        session.request_success();
        Ok(())
    }


    async fn auth_password(&mut self, user: &str, password: &str) -> Result<server::Auth, Self::Error> {
        Ok(server::Auth::Reject { proceed_with_methods: (Some(MethodSet::PUBLICKEY)) })
    }

    async fn auth_publickey_offered(
        &mut self,
        user: &str,
        public_key: &key::PublicKey,
    ) -> Result<server::Auth, Self::Error> {
        //User based auth isn't implemented yet

        log::debug!("Attempting to authenticate user: {}", user);
        log::debug!("Public key: {:?}", public_key);

        let authorized_keys = read_authorized_keys(user).await.map_err(|e| {
            error!("{}", e);
            russh::Error::CouldNotReadKey 
        })?;
        let res = if authorized_keys.contains(&public_key) {server::Auth::Accept} else {server::Auth::Reject { proceed_with_methods: (None) }};

        Ok(res)
    }

    async fn auth_publickey(
        &mut self,
        user: &str,
        public_key: &key::PublicKey,
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

            pty_writer
                .write_all(data)
                .map_err(anyhow::Error::new)?;

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