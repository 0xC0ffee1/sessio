// #![cfg(feature = "rustls")]

use chrono::Utc;
use clap::Parser;
use client::Msg;
use common::utils::events::EventBus;
use common::utils::map_ipv4_to_ipv6;
use key::KeyPair;
use quinn::{ClientConfig, Connection, Endpoint, EndpointConfig, VarInt};
use russh::client::Handle;
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use serde_json::{json, Value};
use sessio_coordinator::holepuncher::HolepunchService;
use ssh_key::known_hosts;
use tokio::fs::File;

use ring::digest::{Context as DigestContext, Digest, SHA256};

use tokio::sync::broadcast::Sender;

use std::collections::HashMap;
use std::f64::consts::E;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV6};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::{error::Error, net::SocketAddr, sync::Arc};
use tokio::io::{
    self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufWriter, Stdin, Stdout,
};
use tokio::sync::oneshot::channel;
use uuid::Uuid;

use quinn_proto::crypto::rustls::QuicClientConfig;

use crate::ipc::clientipc::client_event::{self, CloseEvent, StreamType};
use crate::ipc::clientipc::msg::{Data, PtyRequest, Type};
use crate::ipc::clientipc::session_data::{Kind as SessionKind, PtySession};
use crate::ipc::clientipc::Settings;
use crate::ipc::clientipc::{client_event::ServerMigrateEvent, ClientEvent, SessionData};
use crate::ipc::{self, clientipc};
#[cfg(not(windows))]
use tokio::signal::unix::{signal, SignalKind};
#[cfg(windows)]
use tokio::signal::windows::ctrl_c;

use url::Url;

use std::pin::Pin;
use std::task::Poll;

use russh_keys::*;

use async_trait::async_trait;

use anyhow::{bail, Context, Result};
use bytes::Bytes;
use crossterm::{
    event::{read, Event, KeyCode},
    execute, queue,
    terminal::{
        disable_raw_mode, enable_raw_mode, size as terminal_size, Clear, ClearType,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use futures::{select, stream};
use russh::*;
use russh_keys::*;
use russh_sftp::{client::SftpSession, protocol::OpenFlags};
use std::env;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs, UdpSocket};
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio::{task, time};

use common::utils::streams::BiStream;
use sessio_coordinator::coordinator_client::CoordinatorClient;

//Reusable channel where the listening end always takes the receiver

pub struct ChannelBiStream {
    //The client-bound message listener
    pub client_messages: EventBus<clientipc::Msg>,
    pub server_messages: EventBus<clientipc::Msg>,
}

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn, Level};

#[derive(Clone)]
struct PeerChangeMsg {
    pub new_ip: SocketAddr,
    pub old_ip: SocketAddr,
}

/// Enables MTUD if supported by the operating system
#[cfg(unix)]
pub fn enable_mtud_if_supported() -> quinn::TransportConfig {
    quinn::TransportConfig::default()
}

/// Enables MTUD if supported by the operating system
#[cfg(windows)]
pub fn enable_mtud_if_supported() -> quinn::TransportConfig {
    let mut transport_config = quinn::TransportConfig::default();
    transport_config.mtu_discovery_config(Some(quinn::MtuDiscoveryConfig::default()));
    transport_config
}

#[derive(Debug)]
//The actual authenticity of the server is verified by the SSH protocol
struct SkipServerVerification(Arc<rustls::crypto::CryptoProvider>);

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self(Arc::new(rustls::crypto::ring::default_provider())))
    }
}

impl rustls::client::danger::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp: &[u8],
        _now: UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        let der_bytes = _end_entity.as_ref();
        let mut hasher = DigestContext::new(&SHA256);

        hasher.update(der_bytes);

        let fingerprint: Digest = hasher.finish();

        // Convert the fingerprint to a hexadecimal string if needed
        let fingerprint_hex = hex::encode(fingerprint.as_ref());

        info!("Certificate fingerprint (SHA-256): {}", fingerprint_hex);

        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        self.0.signature_verification_algorithms.supported_schemes()
    }
}

fn configure_client() -> Result<ClientConfig> {
    let _ = rustls::crypto::ring::default_provider().install_default();
    let mut client_config = ClientConfig::new(Arc::new(QuicClientConfig::try_from(
        rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth(),
    )?));

    let mut transport_config = enable_mtud_if_supported();
    transport_config.max_idle_timeout(Some(VarInt::from_u32(10_000).into()));
    transport_config.keep_alive_interval(Some(std::time::Duration::from_secs(5)));
    client_config.transport_config(Arc::new(transport_config));

    Ok(client_config)
}

#[allow(unused)]
pub fn make_client_endpoint(socket: UdpSocket) -> Result<Endpoint> {
    let client_cfg = configure_client()?;

    let runtime = quinn::default_runtime()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "no async runtime found"))?;

    let mut endpoint = Endpoint::new_with_abstract_socket(
        EndpointConfig::default(),
        None,
        runtime.wrap_udp_socket(socket.into_std()?)?,
        runtime,
    )?;

    endpoint.set_default_client_config(client_cfg);
    Ok(endpoint)
}

pub struct Client {
    //Map of active connections
    pub connections: HashMap<String, Connection>,
    pub sessions: HashMap<String, Arc<Mutex<Session>>>,
    pub data_folder_path: PathBuf,
    pub event_bus: EventBus<ClientEvent>,
    //This is optional for the initial setting configuration
    pub coordinator: Option<HolepunchService>,
    pub endpoint: Endpoint,
}

//The name "Session" is confusing, it's actually a SSH connection
//Because russh does not support for creating a new stream for each channel,
//we're using one channel per ssh connection here.
//The ssh connections are still multiplexed through the same QUIC connection.
pub struct Session {
    handle: Handle<ClientHandler>,
    pub id: String,
    pub server_id: String,
    pub username: String,
    pub data: SessionData,

    pub closed: Arc<AtomicBool>,

    //If a channel is opened for this session
    pub active: Arc<AtomicBool>,

    pub channel_stream: ChannelBiStream,
    pub sftp_session: Option<SftpSession>,
    pub event_sender: Sender<ClientEvent>,
}

pub struct ClientHandler {
    connection: Connection,
    remote_addr: SocketAddr,
    server_id: String,
    session_id: String,
    event_tx: Sender<ClientEvent>,
    known_hosts_path: PathBuf,
}

impl Client {
    pub fn check_coordinator_enabled(&self) -> bool {
        self.coordinator.is_some()
    }

    pub async fn init_coordinator(&mut self) -> Result<()> {
        let data_folder_path = self.data_folder_path.clone();
        let settings =
            Client::get_json_as::<Settings>(Client::get_settings_file(&data_folder_path).await?)
                .await?;

        let coord_url = Url::parse(&settings.coordinator_url)?;

        let ipv6 =
            CoordinatorClient::get_new_external_ipv6(self.endpoint.local_addr().unwrap().port())
                .await;

        let key_pair = Client::get_keypair(&data_folder_path)?;

        self.coordinator = HolepunchService::new(
            coord_url,
            self.endpoint.clone(),
            key_pair,
            ipv6,
            settings.device_id,
        )
        .await
        .ok();

        Ok(())
    }

    pub async fn handle_event(&mut self, event: &ClientEvent) -> Result<()> {
        match &event.kind {
            Some(client_event::Kind::Close(close_event)) => {
                match close_event.stream_type() {
                    StreamType::Channel | StreamType::Session => {
                        if let Some(session) = self.sessions.remove(&close_event.id) {
                            let session = session.lock().await;
                            session.closed.store(true, Ordering::SeqCst);
                        }
                    }
                    StreamType::Transport => {
                        //QUIC connection closed
                        self.sessions.clear();
                        bail!("Quic connection closed!");
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn new(data_folder_path: String) -> Result<Self> {
        log::info!("Loading settings from {}", data_folder_path);
        let data_folder_path = PathBuf::from(data_folder_path);
        let udp_socket = UdpSocket::bind("[::]:0").await?;
        let endpoint = make_client_endpoint(udp_socket)?;

        let mut client = Client {
            data_folder_path: data_folder_path,
            connections: HashMap::default(),
            endpoint: endpoint,
            sessions: HashMap::default(),
            event_bus: EventBus::default(),
            coordinator: None,
        };

        client.init_coordinator();

        Ok(client)
    }

    pub async fn get_settings_file(path: &Path) -> Result<File> {
        let mut f = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(path.join("settings.json"))
            .await?;
        Ok(f)
    }

    pub async fn get_save_file(path: &Path) -> Result<File> {
        let mut f = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(path.join("save.json"))
            .await?;

        Ok(f)
    }

    pub async fn get_json_as<T>(file: File) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents).await?;
        let data: T = if contents.is_empty() {
            T::default()
        } else {
            serde_json::from_str(&contents)?
        };
        Ok(data)
    }

    pub async fn save_json_as<T>(file: File, data: T) -> Result<T>
    where
        T: serde::Serialize,
    {
        //Truncate
        file.set_len(0).await?;
        let str: String = serde_json::to_string_pretty(&data)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(str.as_bytes()).await?;
        writer.flush().await?;
        Ok(data)
    }

    pub async fn get_json_value<T>(key: &str, file: File) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents).await?;
        let mut data: HashMap<String, T> = if contents.is_empty() {
            HashMap::new()
        } else {
            serde_json::from_str(&contents)?
        };
        let value = data.remove(key);
        Ok(value)
    }

    ///Returns the old value
    pub async fn set_json_value<T>(key: &str, value: &T, file: File) -> Result<Option<Value>>
    where
        T: serde::Serialize,
    {
        let (mut reader, mut writer) = tokio::io::split(file);
        let mut reader = BufReader::new(reader);
        let mut contents = String::new();
        reader.read_to_string(&mut contents).await?;
        let mut data: HashMap<String, Value> = if contents.is_empty() {
            HashMap::new()
        } else {
            serde_json::from_str(&contents)?
        };

        let new_value = serde_json::to_value(value)?;

        let old_value = data.insert(key.to_string(), new_value);

        let mut writer = BufWriter::new(writer);

        let updated_contents = serde_json::to_string(&data)?;

        // Write the updated JSON string back to the file
        writer.write_all(updated_contents.as_bytes()).await?;
        writer.flush().await?;

        Ok(old_value)
    }

    pub fn init_connection(&mut self, target_id: String, conn: Connection) {
        self.connections.insert(target_id.clone(), conn);
    }

    //Create a new connection and on success return the its ID
    pub async fn new_connection(
        &mut self,
        target_id: String,
        conn_tx: mpsc::Sender<Connection>,
    ) -> anyhow::Result<bool> {
        if let Some(conn) = self.connections.get_mut(&target_id) {
            if conn.close_reason().is_none() {
                //Connection is still open, reusing the old one
                info!("Reusing connection for {}", target_id);
                return Ok(false);
            }
        }

        let Some(coordinator) = self.coordinator.as_ref() else {
            bail!("Coordinator not initialized!");
        };

        coordinator
            .attempt_holepunch(target_id.clone(), conn_tx)
            .await?;

        Ok(true)
    }

    pub fn get_keypair(path: &Path) -> Result<KeyPair> {
        let private_key_path = path.join("keys/id_ed25519");
        let res = load_secret_key(private_key_path, None)?;
        Ok(res)
    }

    pub fn session_exists(&self, session_id: &str) -> bool {
        self.sessions.contains_key(session_id)
    }

    pub async fn new_session<T>(
        &mut self,
        target_id: String,
        data: SessionData,
        username: String,
        session_id: Option<String>,
        private_key_path: T,
        known_hosts_path: T,
    ) -> anyhow::Result<String>
    where
        T: AsRef<Path>,
    {
        let known_hosts_path = self.data_folder_path.join("keys/known_hosts");

        let key_pair = Client::get_keypair(&self.data_folder_path)?;

        if let Some(session_id) = &session_id {
            if let Some(session) = self.sessions.get(session_id) {
                let session = session.lock().await;
                if session.is_active() && !session.is_closed() {
                    log::info!("Reusing session {}", session_id);
                    return Ok(session_id.clone());
                }
            }
        }

        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(60 * 60)),
            ..<_>::default()
        };

        let config = Arc::new(config);

        let Some(connection) = self.connections.get(&target_id) else {
            bail!("No connection made for {}", target_id);
        };

        info!("[client] Connected to: {}", connection.remote_address(),);

        let (mut send, mut recv) = connection
            .open_bi()
            .await
            .map_err(|e| format!("failed to open stream: {}", e))
            .unwrap();

        let bi_stream = BiStream {
            recv_stream: recv,
            send_stream: send,
        };

        let id = if session_id.is_none() {
            Uuid::new_v4().to_string()
        } else {
            session_id.unwrap()
        };

        let session_handler = ClientHandler {
            remote_addr: connection.remote_address(),
            server_id: target_id.clone(),
            connection: connection.clone(),
            known_hosts_path: known_hosts_path.to_path_buf(),
            event_tx: self.event_bus.new_sender().await,
            session_id: id.clone(),
        };

        let mut handle =
            russh::client::connect_stream(config, Box::new(bi_stream), session_handler).await?;

        //let signal_thread = create_signal_thread();

        info!("Authenticating!");

        // use publickey authentication, with or without certificate
        let auth_res = handle
            .authenticate_publickey(username.clone(), Arc::new(key_pair))
            .await?;

        if !auth_res {
            anyhow::bail!("Authentication (with publickey) failed");
        }

        let session = Session {
            id: id.clone(),
            username: username,
            server_id: target_id.to_string(),
            data: data.clone(),
            handle,
            channel_stream: ChannelBiStream {
                client_messages: EventBus::default(),
                server_messages: EventBus::default(),
            },
            sftp_session: None,
            closed: Arc::new(AtomicBool::new(false)),
            active: Arc::new(AtomicBool::new(false)),
            event_sender: self.event_bus.new_sender().await,
        };

        self.sessions
            .insert(id.clone(), Arc::new(Mutex::new(session)));

        Ok((id))
    }
}

// More SSH event handlers
// can be defined in this trait
#[async_trait]
impl russh::client::Handler for ClientHandler {
    type Error = russh::Error;

    //The default path is /home/ssh for some reason
    async fn check_server_key(
        &mut self,
        _server_public_key: &key::PublicKey,
    ) -> Result<bool, Self::Error> {
        let host = &self.server_id;
        let port = 0;

        let is_known_res = russh_keys::check_known_hosts_path(
            host,
            port,
            _server_public_key,
            &self.known_hosts_path,
        );

        if let Ok(known) = is_known_res {
            if !known {
                info!("Learned new host {}:{}", host, port);
                russh_keys::learn_known_hosts_path(
                    host,
                    port,
                    _server_public_key,
                    &self.known_hosts_path,
                )?;
            }
        } else if let Err(e) = is_known_res {
            match e {
                russh_keys::Error::KeyChanged { line } => {
                    error!("Key changed at line: {}", line);
                    return Ok(false);
                }
                _ => {
                    error!("Unknown error: {}", e.to_string());
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    async fn channel_close(
        &mut self,
        channel: ChannelId,
        session: &mut russh::client::Session,
    ) -> Result<(), Self::Error> {
        info!("Channel closed!");
        Ok(())
    }

    /*     async fn channel_accept_stream(&mut self,
        id: ChannelId) -> Result<Option<Box<dyn SubStream>>, Self::Error> {

        info!("Waiting on new channel stream!");
        let res = self.connection.accept_bi().await.unwrap();

        let option = Option::from(Box::new(BiStream {send_stream: res.0, recv_stream: res.1}));

        info!("Accepted new channel substream!");

        return Ok(option.map(|b| b as Box<dyn russh::SubStream>))
    } */
}

impl Session {
    pub fn is_closed(&self) -> bool {
        return self.closed.load(Ordering::SeqCst);
    }

    pub fn is_active(&self) -> bool {
        return self.active.load(Ordering::SeqCst);
    }

    pub fn set_active(&self) {
        return self.active.store(true, Ordering::SeqCst);
    }

    pub async fn direct_tcpip_forward(
        session: Arc<Mutex<Session>>,
        local_host: &str,
        local_port: u32,
        remote_host: &str,
        remote_port: u32,
    ) -> Result<()> {
        tokio::spawn(async move {});
        let listener = TcpListener::bind((local_host, local_port as u16)).await?;

        let remote_host = remote_host.to_string();

        let (closed, active) = {
            let session = session.lock().await;
            (session.closed.clone(), session.active.clone())
        };

        tokio::spawn(async move {
            while !closed.load(Ordering::SeqCst) {
                let Ok((mut stream, addr)) = listener.accept().await else {
                    continue;
                };

                let mut channel = {
                    let session = session.lock().await;
                    session
                        .handle
                        .channel_open_direct_tcpip(
                            remote_host.clone(),
                            remote_port,
                            addr.ip().to_string(),
                            addr.port() as u32,
                        )
                        .await
                        .unwrap()
                };
                active.store(true, Ordering::SeqCst);

                tokio::spawn(async move {
                    let mut cin = channel.make_writer();
                    let mut cout = channel.make_reader();

                    let (mut s_read, mut s_write) = stream.split();
                    tokio::try_join! {
                        tokio::io::copy(&mut s_read, &mut cin),
                        tokio::io::copy(&mut cout, &mut s_write)
                    }
                });
            }
        });
        Ok(())
    }

    //We will have to do this separetely here because Channel::into_stream() consumes the channel
    pub async fn request_sftp(&mut self) -> Result<ChannelId> {
        let mut channel = self.handle.channel_open_session().await?;
        info!("Channel opened!");

        let channel_id = channel.id();
        channel.request_subsystem(true, "sftp").await?;
        info!("Subsystem requested!");

        let sftp = SftpSession::new(channel.into_stream()).await?;
        info!("session created!");
        self.set_active();

        self.sftp_session = Option::from(sftp);

        Ok(channel_id)
    }

    pub async fn new_session_channel(&mut self) -> Result<()> {
        let mut channel = self.handle.channel_open_session().await?;
        self.set_active();

        let mut server_receiver = self.channel_stream.server_messages.subscribe().await;

        let client_sender = self.channel_stream.client_messages.new_sender().await;

        let event_sender = self.event_sender.clone();
        let channel_id = self.id.clone();

        let closed = { self.closed.clone() };

        tokio::spawn(async move {
            while !closed.load(Ordering::SeqCst) {
                tokio::select! {
                    Ok(msg) = server_receiver.recv() => {
                        match msg.r#type {
                            Some(Type::Data(data)) => {
                                let payload: &[u8] = &data.payload;
                                if let Err(e) = channel.data(payload).await {
                                    let _ = event_sender.send(ClientEvent {
                                        kind: Some(client_event::Kind::Close(CloseEvent {
                                            stream_type: client_event::StreamType::Session.into(),
                                            close_reason: format!{"{}", e},
                                            id: channel_id
                                        }))
                                    });
                                    break;
                                }
                            }
                            Some(Type::ShellRequest(_)) => {
                                //This will start the PTY data stream from server to client
                                let _ = channel.request_shell(false).await;
                            }
                            Some(Type::PtyRequest(req)) => {
                                let _ = channel
                                .request_pty(
                                    false,
                                    &env::var("TERM").unwrap_or("xterm".into()),
                                    req.col_width ,
                                    req.row_height,
                                    0,
                                    0,
                                    &[],
                                )
                                .await;
                            }
                            Some(Type::PtyResize(req)) => {
                                let _ = channel.window_change(req.col_width, req.row_height, 0, 0).await;
                            }
                            Some(_) => {}
                            None => {}
                        }
                    },
                    msg = channel.wait() => {
                        match msg {
                            Some(ChannelMsg::Data { ref data }) => {
                                client_sender.send(clientipc::Msg { r#type: Some(Type::Data(Data {
                                    payload: data.to_vec()
                                })) });
                            }
                            Some(ChannelMsg::ExitStatus { exit_status }) => {
                                info!("Channel received exit! {:?}", exit_status);
                                let _ = event_sender.send(ClientEvent {
                                    kind: Some(client_event::Kind::Close(CloseEvent {
                                        stream_type: client_event::StreamType::Channel.into(),
                                        close_reason: format!{"{}", exit_status},
                                        id: channel_id
                                    }))
                                });
                                channel.eof().await;
                                break;
                            }
                            None => {
                                //This is usually called when timeout happens
                                let _ = event_sender.send(ClientEvent {
                                    kind: Some(client_event::Kind::Close(CloseEvent {
                                        stream_type: client_event::StreamType::Channel.into(),
                                        close_reason: "Channel closed".into(),
                                        id: channel_id
                                    }))
                                });
                                channel.eof().await;
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
        Ok(())
    }

    pub async fn close(&mut self) -> Result<()> {
        info!("Disconnecting!");
        self.closed.store(false, Ordering::SeqCst);
        self.handle
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        info!("Disconnected!");
        Ok(())
    }
}
