// #![cfg(feature = "rustls")]

use chrono::Utc;
use clap::Parser;
use client::Msg;
use russh::client::Handle;
use key::KeyPair;
use quinn::{ClientConfig, Connection, Endpoint, EndpointConfig, VarInt};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use serde_json::{json, Value};
use ssh_key::known_hosts;
use tokio::fs::File;
use tokio::sync::oneshot::channel;
use uuid::Uuid;
use std::collections::HashMap;
use std::f64::consts::E;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV6};
use std::sync::atomic::{AtomicU32, Ordering};
use std::{error::Error, net::SocketAddr, sync::Arc};
use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufWriter, Stdin, Stdout};

use quinn_proto::crypto::rustls::QuicClientConfig;

use crate::ipc::clientipc::session_data::{Kind as SessionKind, PtySession};
use crate::ipc::clientipc::SessionData;
#[cfg(not(windows))]
use tokio::signal::unix::{signal, SignalKind};
#[cfg(windows)]
use tokio::signal::windows::ctrl_c;
use crate::ipc::clientipc::msg::Type;

use url::Url;

use std::pin::Pin;
use std::task::Poll;

use russh_keys::*;

use async_trait::async_trait;


use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;
use bytes::Bytes;
use anyhow::{bail, Context, Result};
use russh::*;
use russh_keys::*;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs, UdpSocket};
use tokio::sync::{mpsc, Mutex};
use tokio::{task, time};
use russh_sftp::{client::SftpSession, protocol::OpenFlags};
use futures::{select, stream};
use crossterm::{
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode, size as terminal_size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    event::{read, Event, KeyCode},
};



use coordinator::coordinator_client::CoordinatorClient;
use common::utils::streams::BiStream;


#[allow(unused_imports)]
use log::{debug, error, info, trace, warn, Level};

#[derive(Parser, Debug)]
#[clap(name = "client")]
pub struct Opt {
    //the url to the coordination server. E.g. quic://127.0.0.1:2223
    #[clap(long, short = 'c')]
    coordinator: Url,

    //User to authenticate as
    #[clap(long, short = 'u')]
    username: Option<String>,

    //The path to your private key
    #[clap(long, short = 'k')]
    private_key: PathBuf,

    #[clap(long, short = 'k', default_value = "known_hosts")]
    known_hosts_path: PathBuf,

    //The identifier of the target machine
    target_id: String
}

#[tokio::main]
pub async fn run(cli: Opt) -> anyhow::Result<()>{
    env_logger::builder()
    .filter_level(log::LevelFilter::Info)
    .init();

    /* info!("Key path: {:?}", cli.private_key);

    let mut client = Client::default();

    let connection_id = client.new_connection(cli.target_id.clone(), cli.coordinator, None).await?;

    let session_id = client.new_session(
        cli.target_id,
        SessionDataSessionKind::Pty(PtySession{}),
        "asd".to_string(),
        cli.private_key.clone(),
        cli.known_hosts_path.clone()
    )
    .await?;

    info!("Connected");

    let session_guard = client.sessions.get_mut("asd").unwrap();
    let mut session = session_guard.lock().await;

    let stdout: Stdout = tokio::io::stdout();   
    let stdin: Stdin = tokio::io::stdin();

    let code = {
        let mut stdout_std = std::io::stdout();
        enable_raw_mode().unwrap();
        execute!(stdout_std, EnterAlternateScreen, Clear(ClearType::All)).unwrap();
        let (w, h) = terminal_size()?;

        //let channel_id = session.request_pty(w as u32, h as u32).await?;
        //let result = session.request_shell(&channel_id, stdin, stdout).await?;

        disable_raw_mode().unwrap();
        execute!(stdout_std, LeaveAlternateScreen).unwrap();

        1
    };

    println!("Exitcode: {:?}", code);
    let _ = session.close().await; */
    Ok(())
}


/// Enables MTUD if supported by the operating system
#[cfg(not(any(windows, os = "linux")))]
pub fn enable_mtud_if_supported() -> quinn::TransportConfig {
    quinn::TransportConfig::default()
}

/// Enables MTUD if supported by the operating system
#[cfg(any(windows, os = "linux"))]
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
fn configure_client() -> Result<ClientConfig, Box<dyn Error>> {
    let _ = rustls::crypto::ring::default_provider().install_default();
    let mut client_config = ClientConfig::new(Arc::new(QuicClientConfig::try_from(
        rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth(),
    )?));

    let mut transport_config = enable_mtud_if_supported();
    transport_config.max_idle_timeout(Some(VarInt::from_u32(60_000).into()));
    transport_config.keep_alive_interval(Some(std::time::Duration::from_secs(5)));
    client_config.transport_config(Arc::new(transport_config));

    Ok(client_config)
}

/// Constructs a QUIC endpoint configured for use a client only.
///
/// ## Args
///
/// - server_certs: list of trusted certificates.
#[allow(unused)]
pub fn make_client_endpoint(socket: UdpSocket) -> Result<Endpoint, Box<dyn Error>> {
    let client_cfg = configure_client()?;

    let runtime = quinn::default_runtime()
    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "no async runtime found"))?;

    let mut endpoint = Endpoint::new_with_abstract_socket(
    EndpointConfig::default(), 
    None,
    runtime.wrap_udp_socket(socket.into_std()?)?,
    runtime)?;
    
    endpoint.set_default_client_config(client_cfg);
    Ok(endpoint)
}

#[derive(Default)]
pub struct Client {
    //Map of active connections
    pub connections: HashMap<String, Connection>,
    pub sessions: HashMap<String, Arc<Mutex<Session>>>,
    pub data_folder_path: Option<PathBuf>
}


//The name "Session" is confusing, it's actually a SSH connection
pub struct Session {
    handle: Handle<ClientHandler>,
    pub id: String,
    pub server_id: String,
    pub username: String,
    pub data: SessionData,
    pub channels: HashMap<ChannelId, Arc<Mutex<Channel<Msg>>>>,
    pub sftp_session: Option<SftpSession>
}

pub struct ClientHandler {
    connection: Connection,
    remote_addr: SocketAddr,
    server_id: String,
    known_hosts_path: PathBuf
}

impl Client {

    pub fn set_data_folder(&mut self, path: PathBuf) {
        self.data_folder_path = Option::from(path);
    }

    pub async fn get_settings_file(&self) -> Result<File> {
        let mut f = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(self.data_folder_path.as_ref()
            .context("Data folder not set")?
            .join("settings.json")).await?;
        Ok(f)
    }

    pub async fn get_save_file(&self) -> Result<File> {
        let mut f = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(self.data_folder_path.as_ref()
            .context("Data folder not set")?
            .join("save.json")).await?;
        
        Ok(f)
    }

    pub async fn get_json_as<T>(file: File) -> Result<T> 
    where
    T: serde::de::DeserializeOwned + Default
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
    T: serde::Serialize
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
    T: serde::de::DeserializeOwned
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
    T: serde::Serialize
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

    async fn connection_update_task(mut c_client: CoordinatorClient){
        let mut update_interval = time::interval(Duration::from_secs(2));
        loop {
            tokio::select! {
                _ = update_interval.tick() => {
                    let ext_ipv6 = CoordinatorClient::get_new_external_ipv6(c_client.borrow_endpoint().
                local_addr().unwrap().port()).await;
                    let _ = c_client.update_external_ip(ext_ipv6.clone()).await;
                    match ext_ipv6 {
                        Some(ip) => {
                            info!("Updated external ipv6 to {}", ip);
                        }
                        None =>{
                            info!("Updated external ipv6 to None");
                        }
                    }
                   
                }
                response = c_client.read_response::<HashMap<String, String>>() => {
                    let response = response.unwrap();
                    match response.get("id").map(String::as_str) {
                        Some("PEER_IP_CHANGED") => {
                            //Server ip has changed
                            //Just sending a packet to the client for the mappings

                            //todo: Create a periodic timer for client to send update packet, and listen for ip changes from server
                            let new_ip: SocketAddr = response.get("new_ip").unwrap().parse().unwrap();
                            let _ = c_client.borrow_endpoint().connect(new_ip, "client");
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    //Create a new connection and on success return the its ID
    pub async fn new_connection(&mut self, target_id: String, coordinator: Url, ipv6: Option<Ipv6Addr>) -> anyhow::Result<()> {
        if let Some(conn) = self.connections.get_mut(&target_id) {
            if let None = conn.close_reason() {
                //Connection is still open, reusing the old one
                info!("Reusing connection for {}", target_id);
                return Ok(());
            }
        }

        //let sock_v4 = UdpSocket::bind::<SocketAddr>("0.0.0.0:0".parse().unwrap()).await.unwrap();
        let sock_v6 = UdpSocket::bind::<SocketAddr>("[::]:0".parse().unwrap()).await.unwrap();

        let endpoint_v6 = make_client_endpoint(sock_v6).unwrap();

        let start_time = Utc::now();

        let mut c_client = loop {
            //Add id verification
            match CoordinatorClient::connect(coordinator.clone(), Uuid::new_v4().to_string(), endpoint_v6.clone()).await {
                Ok(client) => break client,
                Err(e) => {
                    info!("Failed to connect to coordination server {}\nRetrying in 5 seconds..", e);
                    time::sleep(Duration::from_secs(5)).await;
                }
            }
        };

        let conn = attempt_holepunch(&mut c_client, target_id.clone(), coordinator, endpoint_v6).await?;

        tokio::spawn(async move {
            Client::connection_update_task(c_client).await;
        });

        let end_time = Utc::now();
        let elapsed_time = end_time - start_time;
        println!("Took to holepunch: {} ms", elapsed_time.num_milliseconds());

        self.connections.insert(target_id.clone(), conn);

        Ok(())
    }

    pub async fn new_session<T>(
        &mut self,
        target_id: String,
        data: SessionData,
        username: String,
        session_id: Option<String>,
        private_key_path: T,
        known_hosts_path: T
    )  -> anyhow::Result<(String)> where T: AsRef<Path> {

        let data_folder = self.data_folder_path.as_ref().context("Data folder not set")?;
        let private_key_path = data_folder.join("keys/id_ed25519");
        let known_hosts_path = data_folder.join("keys/known_hosts");

        let res = load_secret_key(private_key_path, None);
        //Supports RSA since russh 0.44-beta.1
        let Ok(key_pair) = res else {
             bail!("Failed to load key at! {}", res.err().unwrap().to_string());
        };

        //Reusing the same session
/*         if(self.sessions.contains_key(&username)) {
            log::info!("Reusing session {}", username);
            return Ok(());
        }
 */
        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(60 * 60)),
            ..<_>::default()
        };

        let config = Arc::new(config);

        let Some(connection) = self.connections.get(&target_id) else {
            bail!("No connection made for {}", target_id);
        };
        
        info!(
            "[client] Connected to: {}",
            connection.remote_address(),
        );
        
        let (mut send, mut recv) = connection
            .open_bi()
            .await
            .map_err(|e| format!("failed to open stream: {}", e)).unwrap();

        
        let bi_stream = BiStream {recv_stream: recv, send_stream: send};

        let session_handler = ClientHandler {
            remote_addr: connection.remote_address(),
            server_id: target_id.clone(),
            connection: connection.clone(),
            known_hosts_path: known_hosts_path.to_path_buf()
        };

        let mut handle = russh::client::connect_stream(config, bi_stream, session_handler).await?;

        //let signal_thread = create_signal_thread();

        info!("Authenticating!");

        // use publickey authentication, with or without certificate
        let auth_res = handle
            .authenticate_publickey(username.clone(), Arc::new(key_pair))
            .await?;

        if !auth_res {
            anyhow::bail!("Authentication (with publickey) failed");
        }
        let id = if session_id.is_none() {
            Uuid::new_v4().to_string()
        } else {
            session_id.unwrap()
        };
        let session = Session {
            id: id.clone(),
            username: username,
            server_id: target_id.to_string(),
            data: data.clone(),
            handle,
            channels: HashMap::new(),
            sftp_session: None
        };
      
        self.sessions.insert(id.clone(), Arc::new(Mutex::new(session)));

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

        let is_known_res = russh_keys::check_known_hosts_path(host, port, _server_public_key, &self.known_hosts_path);

        if let Ok(known) = is_known_res {
            if(!known){
                info!("Learned new host {}:{}", host, port);
                russh_keys::learn_known_hosts_path(host, port, _server_public_key, &self.known_hosts_path)?;
            }
        } 
        else if let Err(e) = is_known_res {
            match e {
                russh_keys::Error::KeyChanged {line} => {
                    error!("Key changed at line: {}", line);
                }
                _ => {
                    error!("Unknown error: {}", e.to_string());
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


async fn attempt_holepunch(client: &mut CoordinatorClient, target: String, coordinator: Url, 
    mut endpoint: Endpoint,
) -> io::Result<Connection> {

    let ipv6 = CoordinatorClient::get_new_external_ipv6(endpoint.local_addr()?.port()).await;
    let _ = client.register_endpoint(ipv6).await;
    let _ = client.connect_to(target).await;

    loop {
        info!("Waiting for server to connect.");
        let response = client.read_response::<HashMap<String, String>>().await.unwrap();
        match response.get("id").map(String::as_str) {
            Some("CONNECT_TO") => {
                let target: SocketAddr = response.get("target").unwrap().parse().unwrap();
                let target_id = response.get("target_id").unwrap();

                match endpoint.connect(target, "server").unwrap().await {
                    Ok(conn) => {
                        let _ = client.send_packet(&json!({"id":"CONNECT_OK", "target_id":target_id})).await;
                        return Ok(conn);
                    }
                    Err(e) => {
                        info!("Connection failed: {}", e);
                    }
                }
            }
            _ => {

            }
        }

        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}


impl Session {


    //This is not optimal as it blocks the whole session
    pub async fn direct_tcpip_forward(&mut self, local_host: &str, local_port: u32, remote_host: &str, remote_port: u32) -> Result<()>{
        let listener = TcpListener::bind((local_host, local_port as u16)).await?;

        let remote_host = remote_host.to_string();

        loop {
            let (mut stream, addr) = listener.accept().await?;

            let mut channel = self.handle.channel_open_direct_tcpip(remote_host.clone(), remote_port, 
            addr.ip().to_string(), addr.port() as u32).await?;

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
    }

    pub async fn request_sftp(&mut self) -> Result<ChannelId> {

        let mut channel = self.handle.channel_open_session().await?;
        info!("Channel opened!");

        let channel_id = channel.id();
        channel.request_subsystem(true, "sftp").await?;
        info!("Subsystem requested!");

        let sftp = SftpSession::new(channel.into_stream()).await?;
        info!("session created!");

        self.sftp_session = Option::from(sftp);

        Ok(channel_id)
    }

    pub async fn request_shell(
        channel_guard: Arc<Mutex<Channel<Msg>>>,
        mut input_rx: Arc<Mutex<mpsc::Receiver<crate::ipc::clientipc::Msg>>>,
        output_tx: Arc<Mutex<mpsc::Sender<Vec<u8>>>>,
    ) -> Result<u32> {

        //Todo make this accept proto messages
        let mut channel = channel_guard.lock().await;

        let _ = channel.request_shell(false).await;

        let code;

        let mut stdin_closed = false;

        let mut input = input_rx.lock().await;

        loop {
            tokio::select! {
                Some(input_msg) = input.recv(), if !stdin_closed => {
                    match input_msg.r#type {
                        Some(Type::Data(data)) => {
                            //Sends the data to the shell stream
                            info!("IPC: Forwarding data!");
                            let payload: &[u8] = &data.payload;
                            channel.data(payload).await;
                        }
                        Some(Type::PtyResize(req)) => {
                            info!("IPC: Resizing PTY!");
                            channel.window_change(req.col_width, req.row_height, 0, 0).await;
                        }
                        _ => {}
                    }
                },
                Some(msg) = channel.wait() => {
                    match msg {
                        ChannelMsg::Data { ref data } => {
                            output_tx.lock().await.send(data.to_vec()).await?;
                        }
                        ChannelMsg::ExitStatus { exit_status } => { 
                            code = exit_status;
                            if !stdin_closed {
                                channel.eof().await?;
                            }
                            break;
                        }
                        _ => {}
                    }
                },
            }
        }
        Ok(code)
    }
    
    pub async fn new_channel(&mut self) -> Result<ChannelId> {
        let channel = self.handle.channel_open_session().await?;
        let channel_id = channel.id();

        info!("OPENING SESSION {}", channel.id());

        self.channels.insert(channel.id(), Arc::new(Mutex::new(channel)));

        Ok(channel_id)
    }

    pub async fn resize_pty(&mut self, channel_id: &ChannelId, col_width: u32, row_height: u32) -> Result<()>{
        let channel_guard = self.channels.get(&channel_id).unwrap();
        let mut channel = channel_guard.lock().await;

        channel
            .window_change(
                col_width ,
                row_height,
                0,
                0,
            )
            .await?;
        
        info!("PTY Requested!");
        Ok(())
    }

    pub async fn request_pty(&mut self, channel_id: &ChannelId, col_width: u32, row_height: u32) -> Result<()>{
        let channel_guard = self.channels.get(&channel_id).unwrap();
        let mut channel = channel_guard.lock().await;

        let session = Arc::new(&self);

        channel
            .request_pty(
                false,
                &env::var("TERM").unwrap_or("xterm".into()),
                col_width ,
                row_height,
                0,
                0,
                &[], 
            )
            .await?;
        
        info!("PTY Requested!");
        Ok(())
    }

    async fn close(&mut self) -> Result<()> {
        info!("Disconnecting!");
        self.handle
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        info!("Disconnected!");
        Ok(())
    }
}


#[cfg(windows)]
fn create_signal_thread() -> impl core::future::Future<Output = ()> {
    async move {
        let mut stream = match ctrl_c() {
            Ok(s) => s,
            Err(e) => {
                error!("[client] create signal stream error: {}", e);
                return;
            }
        };
        stream.recv().await;
        info!("[client] got signal Ctrl-C");
    }
}
#[cfg(not(windows))]
fn create_signal_thread() -> impl core::future::Future<Output = ()> {
    async move {
        let mut stream = match signal(SignalKind::hangup()) {
            Ok(s) => s,
            Err(e) => {
                error!("[client] create signal stream error: {}", e);
                return;
            }
        };
        stream.recv().await;
        info!("[client] got signal HUP");
    }
}
