use std::path::PathBuf;
use std::time::Duration;
use std::{collections::HashMap, default};
use std::sync::Arc;
use std::str;
use log::{error, info};
use rustls::crypto::CryptoProvider;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{io, sync::Mutex};
use rustls_pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};

use std::net::{Ipv4Addr, SocketAddr};
use clap::Parser;
use quinn::{crypto, Endpoint, ServerConfig, VarInt};

use quinn_proto::crypto::rustls::QuicServerConfig;

use anyhow::{Context, Error};
use serde::Deserialize;

use serde_json::{json, Value};
use anyhow::{anyhow, Result};
use common::utils::streams::BiStream;


use uuid::{uuid, Uuid};

use crate::Opt;

/// Returns default server configuration along with its certificate.
async fn configure_server(key_path: &PathBuf, cert_path: &PathBuf) -> Result<ServerConfig> {
    rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");

    let (certs, key) = get_certs(key_path, cert_path).await?;

    let mut server_crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    let mut server_config =
        quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(server_crypto)?));
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());
    transport_config.max_idle_timeout(Some(VarInt::from_u32(60_000).into()));
    transport_config.keep_alive_interval(Some(std::time::Duration::from_secs(1)));
    #[cfg(any(windows, os = "linux"))]
    transport_config.mtu_discovery_config(Some(quinn::MtuDiscoveryConfig::default()));

    Ok(server_config)
}

async fn get_certs<'a>(key_path: &PathBuf, cert_path: &PathBuf) -> Result<(Vec<CertificateDer<'a>>, PrivateKeyDer<'a>)> {
    let key = fs::read(key_path).await.with_context(|| "failed to read private key")?;
    let key = if key_path.extension().map_or(false, |x| x == "der") {
        PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key))
    } else {
        rustls_pemfile::private_key(&mut &*key)
            .context("malformed PKCS #1 private key")?
            .ok_or_else(|| anyhow::Error::msg("no private keys found"))?
    };
    let cert_chain = fs::read(cert_path).await.context("failed to read certificate chain")?;
    let cert_chain = if cert_path.extension().map_or(false, |x| x == "der") {
        vec![CertificateDer::from(cert_chain)]
    } else {
        rustls_pemfile::certs(&mut &*cert_chain)
            .collect::<Result<_, _>>()
            .context("invalid PEM-encoded certificate")?
    };

    Ok((cert_chain, key))
}



#[tokio::main]
pub async fn run(options: Opt) {
    let mut builder = env_logger::Builder::from_default_env();
    if cfg!(debug_assertions) {
        // Debug mode
        builder.filter_level(log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();
    
    let server_config = configure_server(&options.key, &options.cert).await.unwrap();
    let endpoint = Endpoint::server(server_config, options.listen).unwrap();

    let mut sh = Server::default();

    println!("Started!");
    sh.run_quic(&endpoint).await.unwrap();
}


struct Session {
    //Server initiates a new session
    server: Arc<Client>,
    client: Option<Arc<Client>>,
}

struct Client {
    stream: Arc<ClientStream>,

    //Set once client has registered
    id: Mutex<Option<String>>,

    //Set once clients initiates or joins a session
    session_id: Mutex<Option<String>>,

    ipv6: Mutex<Option<SocketAddr>>
}

struct ClientStream {
    addr: SocketAddr,
    recv_stream: Mutex<quinn::RecvStream>,
    send_stream: Mutex<quinn::SendStream>,
}

impl ClientStream {

    pub async fn send_packet<T>(&self, packet: &T) -> Result<(), anyhow::Error>
    where
        T: serde::Serialize,
    {
        let serialized_packet = serde_json::to_string(packet)
            .map_err(|e| anyhow!("failed to serialize packet: {}", e))?;

        let mut send_stream = self.send_stream.lock().await;

        let message_length = serialized_packet.len() as u32;
        let mut buffer = Vec::new();
        buffer.extend(&message_length.to_be_bytes());
        buffer.extend(serialized_packet.as_bytes());

        send_stream.write_all(&buffer)
            .await
            .map_err(|e| anyhow!("failed to send request: {}", e))?;

        Ok(())
    }

    pub async fn read_response<T>(&self) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut buf = Vec::new();
        let mut read_buf = [0; 1024];

        let mut recv_stream = self.recv_stream.lock().await;

        loop {
            let n = match recv_stream.read(&mut read_buf).await? {
                Some(n) if n > 0 => n,
                Some(_) | None => break, // EOF or stream closed
            };
            buf.extend_from_slice(&read_buf[..n]);

            match serde_json::from_slice::<Value>(&buf) {
                Ok(value) => {
                    info!("Received {}", value.to_string());
                    let result = serde_json::from_value::<T>(value)
                        .map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;
                    return Ok(result);
                }
                Err(e) if e.is_eof() => {
                    // Incomplete data, continue reading
                    continue;
                }
                Err(e) => {
                    return Err(anyhow!("Failed to parse JSON: {}", e));
                }
            }
        }

        Err(anyhow!("No matching response received"))
    }

}

#[derive(Clone)]

#[derive(Default)]
struct Server {
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    clients: Arc<Mutex<HashMap<String, Arc<Client>>>>,
}

fn read_packet_field<'a>(field: &str, packet: &'a HashMap<String, String>) -> Option<&'a String> {
    let value = packet.get(field);
    if value.is_none() {
        error!("Field '{}' not found in packet", field);
    }
    value
}


async fn close_client_connection(connection: quinn::Connection, client: Arc<Client>, sessions: Arc<Mutex<HashMap<String, Session>>>){
    if let Err(f_e) = client.stream.send_stream.lock().await.finish() {
        error!("Failed to close stream {}! Closing connection.", f_e);
        connection.close(0u32.into(), b"Closing connection");
    }
}


async fn handle_connection(connection: quinn::Connection, 
    clients: Arc<Mutex<HashMap<String, Arc<Client>>>>, 
    sessions: Arc<Mutex<HashMap<String, Session>>>
) {
    let (mut quinn_send, mut quinn_recv) = match connection.accept_bi().await {
        Ok(stream) => stream,
        Err(e) => {
            error!("[server] open quic stream error: {}", e);
            return;
        }
    };

    let mut conn = Arc::new(ClientStream {
        addr: connection.remote_address(),
        send_stream: Mutex::new(quinn_send),
        recv_stream: Mutex::new(quinn_recv),
    });

    let mut client = Arc::new(Client {
        stream: conn.clone(),
        id: Mutex::new(None),
        session_id: Mutex::new(None),
        ipv6: Mutex::new(None)
    });

    loop {
        let packet = match conn.read_response::<HashMap<String, String>>().await {
            Ok(packet) => packet,
            Err(e) => {
                error!("Failed to read packet. Closing connection to {}. {}", client.id.lock().await.clone().unwrap_or("Unconnected".to_string()), e);
                close_client_connection(connection, client, sessions).await;
                break;
            }
        };

        let response = match packet.get("id").map(String::as_str) {
            Some("REGISTER") => {
                let client_addr = connection.remote_address();
                
                let Some(own_id) = read_packet_field("own_id", &packet) else {
                    continue;
                };

                let mut client_id_guard = client.session_id.lock().await;

                *client_id_guard = Some(own_id.clone());
                
                //Checking if the client supports ipv6
                if let Some(ipv6) = read_packet_field("ipv6", &packet) {
                    if ipv6 != "None" {
                        let mut client_ipv6_guard = client.ipv6.lock().await;
       
                        info!("Parsing {}", ipv6);
                        *client_ipv6_guard = Some(ipv6.parse().unwrap());
                    }
                };

                //let mut clients = clients.lock().await;

                //clients.insert(own_id.to_string(), client.clone());
                json!({"status": "200"})                                                                                                            
            }
            Some("NEW_SESSION") => {
                //Sent by server
                let Some(own_id) = read_packet_field("own_id", &packet) else {
                    continue;
                };

                let mut sessions = sessions.lock().await;

                //Initially there's no client connected
                sessions.insert(own_id.to_string(), Session {
                    server: client.clone(), //self
                    client: None,
                });
                json!({"status": "200"})
            }
            Some("CLIENT_CONNECTED") => {
                //Sent by ssh client
                let mut client_addr = connection.remote_address();
                let Some(target_id) = read_packet_field("target_client_id", &packet) else {
                    continue;
                };
                
                let mut sessions = sessions.lock().await;
                if let Some(session) = sessions.get_mut(target_id) {
                    session.client = Some(client.clone());

                    let mut session_id_guard = client.session_id.lock().await;

                    *session_id_guard = Some(target_id.clone());
                    
                    //Check if both support ipv6
                    if session.server.ipv6.lock().await.is_some() {
                        // Update client_addr if the client has an IPv6 address, otherwise keep the original client_addr
                        client_addr = client.ipv6.lock().await.clone().unwrap_or(client_addr);
                    }

                    //Telling the server to send the first packet in the UDP hole punch process
                    info!("Sending connect packet to server!");
                    let _ = session.server.stream.send_packet::<Value>(&json!({"id" : "CONNECT_TO", "target" : client_addr})).await;

                    json!({"status": "200", "server" : session.server.stream.addr})
                }
                else {
                    json!({"status": "404"})
                }
            }
            Some("SERVER_SENT_CONNECTION_REQUEST") => {
                //Sent as a response from the server
                let Some(own_id) = read_packet_field("own_id", &packet) else {
                    continue;
                };

                let mut sessions = sessions.lock().await;
                if let Some(session) = sessions.get_mut(own_id) {
                    //Telling the client to connect to the server as a "response" to complete the UDP hole punch
                    let Some(client) = session.client.as_mut() else {
                        error!("At SERVER_SENT_CONNECTION_REQUEST: Session has no associated client!");
                        continue;
                    };

                    let mut server_addr = connection.remote_address();

                    //Check if both support ipv6
                    if client.ipv6.lock().await.is_some() {
                        // Update client_addr if the client has an IPv6 address, otherwise keep the original client_addr
                        server_addr = session.server.ipv6.lock().await.clone().unwrap_or(server_addr);
                    }
                    
                    let _ = client.stream.send_packet::<Value>(&json!({"id" : "CONNECT_TO", "target" : server_addr, "target_id": own_id})).await;
                    json!({"status": "200"})
                }
                else {
                    json!({"status": "404"})
                }
            }
            Some("CONNECT_OK") => {
                let Some(target_id) = read_packet_field("target_id", &packet) else {
                    continue;
                };

                let mut sessions = sessions.lock().await;
                if let Some(session) = sessions.get_mut(target_id) {
                    let _ = session.server.stream.send_packet::<Value>(&json!({"id" : "SESSION_FINISHED"})).await;

                    let _ = session.server.stream.send_stream.lock().await.finish();
                    sessions.remove(target_id);
                    

                    json!({"status": "200"})
                }
                else {
                    json!({"status": "404"})
                }
            }
            _ => {
                json!({"error": "Unknown command"})
            }
        };
        info!("Writing {}", response.to_string());
        {
            if let Err(e) = client.stream.send_packet(&response).await{
                error!("Failed to send packet to client {}! Closing stream.", e);
                close_client_connection(connection, client, sessions).await;
                break;
            }
        }
    }
    info!("[server] exit client");
}

trait QuicServer{
    async fn run_quic(
        &mut self,
        connection: &Endpoint
    ) -> Result<()>;
}

pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];

impl QuicServer for Server {
    
    async fn run_quic(
        &mut self,
        endpoint: &Endpoint
        ) -> Result<()> {
       
        loop{
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

            let clients = self.clients.clone();

            let sessions = self.sessions.clone();
            tokio::spawn(async move {
                handle_connection(conn, clients, sessions).await;
            });
        }
    }
}

