// #![cfg(feature = "rustls")]

use chrono::Utc;
use clap::Parser;
use client::Msg;
use russh::client::Handle;
use key::KeyPair;
use quinn::{ClientConfig, Connection, Endpoint, VarInt};
use serde_json::json;
use ssh_key::known_hosts;
use tokio::fs::File;
use tokio::sync::oneshot::channel;
use uuid::Uuid;
use std::collections::HashMap;
use std::f64::consts::E;
use std::net::Ipv4Addr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::{error::Error, net::SocketAddr, sync::Arc};
use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, Stdin, Stdout};

#[cfg(not(windows))]
use tokio::signal::unix::{signal, SignalKind};
#[cfg(windows)]
use tokio::signal::windows::ctrl_c;

use url::Url;

use std::pin::Pin;
use std::task::Poll;
use std::task::Context;

use russh_keys::*;

use async_trait::async_trait;


use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;
use bytes::Bytes;
use anyhow::{bail, Result};
use russh::*;
use russh_keys::*;
use tokio::net::ToSocketAddrs;
use tokio::sync::{mpsc, Mutex};
use tokio::task;
use russh_sftp::{client::SftpSession, protocol::OpenFlags};
use futures::stream;
use crossterm::{
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode, size as terminal_size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    event::{read, Event, KeyCode},
};

use socket2::{Socket, Domain, Type};
use std::net::UdpSocket;

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

    info!("Key path: {:?}", cli.private_key);

    let mut client = Client::default();

    let connection_id = client.new_connection(cli.target_id.clone(), cli.coordinator).await?;

    let session_id = client.new_session(
        cli.target_id,
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
    let _ = session.close().await;
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

struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

fn configure_client() -> Result<ClientConfig, Box<dyn Error>> {
    let crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(SkipServerVerification::new())
        .with_no_client_auth();

    let mut client_config = ClientConfig::new(Arc::new(crypto));
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
pub fn make_client_endpoint(addr: &str) -> Result<Endpoint, Box<dyn Error>> {
    let client_cfg = configure_client()?;
    let mut endpoint = Endpoint::client(addr.parse()?)?;
    endpoint.set_default_client_config(client_cfg);
    Ok(endpoint)
}

#[derive(Default)]
pub struct Client {
    //Map of active connections
    pub connections: HashMap<String, Connection>,

    pub sessions: HashMap<String, Arc<Mutex<Session>>>
}

//The name "Session" is confusing, it's actually a SSH connection
pub struct Session {
    handle: Handle<ClientHandler>,
    id: String,
    pub channels: HashMap<ChannelId, Arc<Mutex<Channel<Msg>>>>,
    pub sftp_session: Option<SftpSession>
}

pub struct ClientHandler {
    connection: Connection,
    remote_addr: SocketAddr,
    known_hosts_path: PathBuf
}


impl Client {
    //Create a new connection and on success return the its ID
    pub async fn new_connection(&mut self, target_id: String, coordinator: Url) -> anyhow::Result<()> {
        if let Some(conn) = self.connections.get_mut(&target_id) {
            if let None = conn.close_reason() {
                //Connection is still open, reusing the old one
                info!("Reusing connection for {}", target_id);
                return Ok(());
            }
        }
        
        let endpoint_v4 = make_client_endpoint("0.0.0.0:0").unwrap();
        let endpoint_v6 = make_client_endpoint("[::]:0").unwrap();

        let start_time = Utc::now();
        let conn = attempt_holepunch(target_id.clone(), coordinator, endpoint_v4.clone(), endpoint_v6.clone()).await?;
        let end_time = Utc::now();
        let elapsed_time = end_time - start_time;
        println!("Took to holepunch: {} ms", elapsed_time.num_milliseconds());

        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(60 * 60)),
            ..<_>::default()
        };

        self.connections.insert(target_id.clone(), conn);

        Ok(())
    }

    pub async fn new_session<T>(
        &mut self,
        target_id: String,
        username: String,
        private_key_path: T,
        known_hosts_path: T
    )  -> anyhow::Result<(String)> where T: AsRef<Path> {
        let start_time = Utc::now();
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
            connection: connection.clone(),
            known_hosts_path: known_hosts_path.as_ref().to_path_buf()
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

        let session = Session {
            id: username.clone(),
            handle,
            channels: HashMap::new(),
            sftp_session: None
        };

        let id = Uuid::new_v4().to_string();
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
        let host = &self.remote_addr.ip().to_string();
        let port = self.remote_addr.port();


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


async fn attempt_holepunch(target: String, coordinator: Url, 
    mut endpoint_v4: Endpoint,
    mut endpoint_v6: Endpoint) -> io::Result<Connection> {

    let mut client = CoordinatorClient::connect(coordinator, Uuid::new_v4().to_string(), endpoint_v4.clone()).await;
    let _ = client.register_endpoint(endpoint_v6.local_addr().unwrap()).await;

    let _ = client.connect_to(target).await;

    loop {
        info!("Waiting for server to connect.");
        let response = client.read_response::<HashMap<String, String>>().await.unwrap();
        match response.get("id").map(String::as_str) {
            Some("CONNECT_TO") => {
                info!("Attempting connection");
                
                let target: SocketAddr = response.get("target").unwrap().parse().unwrap();
                let target_id = response.get("target_id").unwrap();

                let endpoint: &Endpoint = if target.is_ipv4() {
                    &endpoint_v4
                } else {
                    &endpoint_v6
                };

                match endpoint.connect(target, "server").unwrap().await {
                    Ok(conn) => {
                        info!("Connection successful!");
                        let _ = client.send_packet(&json!({"id":"CONNECT_OK", "target_id":target_id})).await;
                        let _ = client.close_connection().await;
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
        mut input_rx: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>,
        output_tx: Arc<Mutex<mpsc::Sender<Vec<u8>>>>,
    ) -> Result<u32> {
        let mut channel = channel_guard.lock().await;

        let _ = channel.request_shell(false).await;

        let code;

        let mut stdin_closed = false;

        let mut input = input_rx.lock().await;

        loop {
            tokio::select! {
                Some(input_data) = input.recv(), if !stdin_closed => {
                    if input_data.is_empty() {
                        stdin_closed = true;
                        channel.eof().await?;
                    } else {
                        let c: &[u8] = &input_data;
                        channel.data(c).await.unwrap();
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
