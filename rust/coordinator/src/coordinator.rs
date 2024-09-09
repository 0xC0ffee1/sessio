use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use common::utils::events::EventBus;

use log::{error, info, warn};

use russh_keys::key::{PublicKey, Signature};

use rustls_pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use tokio::fs;
use tokio::sync::broadcast::{Receiver, Sender};

use std::net::SocketAddr;

use quinn::{crypto, Connection, Endpoint, ServerConfig, VarInt};

use quinn_proto::crypto::rustls::QuicServerConfig;

use anyhow::{bail, Context, Result};

use crate::common::*;

use uuid::Uuid;

use crate::Opt;

/// Returns default server configuration along with its certificate.
async fn configure_server(key_path: &PathBuf, cert_path: &PathBuf) -> Result<ServerConfig> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let (certs, key) = get_certs(key_path, cert_path).await?;

    let server_crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    let mut server_config =
        quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(server_crypto)?));
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());
    transport_config.max_idle_timeout(Some(VarInt::from_u32(60_000).into()));
    transport_config.keep_alive_interval(Some(std::time::Duration::from_secs(1)));

    #[cfg(windows)]
    transport_config.mtu_discovery_config(Some(quinn::MtuDiscoveryConfig::default()));

    Ok(server_config)
}

async fn get_certs<'a>(
    key_path: &PathBuf,
    cert_path: &PathBuf,
) -> Result<(Vec<CertificateDer<'a>>, PrivateKeyDer<'a>)> {
    let key = fs::read(key_path)
        .await
        .with_context(|| "failed to read private key")?;
    let key = if key_path.extension().map_or(false, |x| x == "der") {
        PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key))
    } else {
        rustls_pemfile::private_key(&mut &*key)
            .context("malformed PKCS #1 private key")?
            .ok_or_else(|| anyhow::Error::msg("no private keys found"))?
    };
    let cert_chain = fs::read(cert_path)
        .await
        .context("failed to read certificate chain")?;
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
    sh.run(&endpoint).await.unwrap();
}

#[derive(Default)]
struct Server {
    sessions: HashMap<String, Session>,
    clients: HashMap<String, Client>,
    event_bus: EventBus<ServerPacket>,
}

struct Session {
    //Server initiates a new session
    server_id: String,
    client_id: String,
    using_ipv6: bool,
}

struct Client {
    conn: Connection,
    auth_token: String,
    stream: Sender<Packet>,
    id: String,
    session_ids: Vec<String>,
    ///Current ipv6
    ipv6: Option<SocketAddr>,
    ///Current ipv4
    ipv4: SocketAddr,
}

impl Server {
    //receiver = receive packets to be sent
    //sender = forwards the received packets to the other parts of the server
    //stream = bidirectional communication stream to transport packets
    fn client_communication_task(
        mut stream: ClientStream,
        mut receiver: Receiver<Packet>,
        sender: Sender<ServerPacket>,
    ) {
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Ok(packet) = receiver.recv() => {
                        if let Err(e) = stream.send_packet::<Packet>(&packet).await {
                            log::error!("Can't send packet to stream! {}", e);
                            break;
                        };
                    },
                    //Server-bound packet
                    Ok(packet) = stream.read_response::<ServerPacket>() => {
                        if let Err(e) = sender.send(packet) {
                            log::error!("Can't broadcast packet! {}", e);
                            break;
                        }
                    }
                    else => {
                        log::debug!("Client communication task closed");
                        break;
                    }
                }
            }
        });
    }

    async fn accept_connections(endpoint: &Endpoint) -> Result<Connection> {
        info!("Waiting for connections..");
        let incoming_conn = match endpoint.accept().await {
            Some(conn) => conn,
            None => {
                bail!("None received");
            }
        };
        let conn = match incoming_conn.await {
            Ok(conn) => conn,
            Err(e) => {
                bail!("Failed to accept conn: {}", e);
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
            "[Coordination Server] Connection accepted: ({}, {})",
            conn.remote_address(),
            sni
        );

        Ok(conn)
    }

    async fn authorize_take_client(&mut self, packet: &PacketBase) -> Result<Client> {
        //Todo: implement token check
        let client = self
            .clients
            .remove(&packet.own_id)
            .context("Client not found")?;

        if packet.token != client.auth_token {
            bail!("Token mismatch!");
        }

        Ok(client)
    }

    //QUIC by default prevents replay attacks so this should be fine
    async fn check_publickey(&mut self, packet: &Auth) -> Result<bool> {
        let Ok(public_key) = russh_keys::parse_public_key_base64(&packet.public_key_base64) else {
            bail!("Could not parse public key (base64)");
        };

        let keys: Vec<PublicKey> = common::utils::keygen::read_authorized_keys(None).await?;

        if !keys.contains(&public_key) {
            warn!(
                "Public key is not an authorized key! {}",
                packet.public_key_base64
            );
            return Ok(false);
        }

        let sig = Signature::from_base64(packet.signature.as_bytes())?;

        if public_key.verify_detached(packet.signed_data.as_bytes(), sig.as_ref()) {
            info!("Signature is valid.");
            Ok(true)
        } else {
            warn!("Signature verification failed.");
            Ok(false)
        }
    }

    async fn handle_packet(
        &mut self,
        mut client_self: Client,
        packet: ServerPacket,
    ) -> Result<(Packet, Client)> {
        let packet = packet.packet;

        let response = match packet {
            Packet::UpdateIp(data) => {
                let addr_ipv4_current = client_self.conn.remote_address();

                let addr_ipv4_old = client_self.ipv4;
                let addr_ipv6_old = client_self.ipv6;

                let sessions = &mut self.sessions;
                for session_id in client_self.session_ids.iter() {
                    if let Some(session) = sessions.get_mut(session_id) {
                        let is_client = client_self.id.clone() == session.client_id;

                        let other_peer = if is_client {
                            let server_id = session.server_id.clone();
                            self.clients
                                .get_mut(&server_id)
                                .context("UPDATE_IP: Server not present")?
                        } else {
                            let client_id = session.client_id.clone();
                            self.clients
                                .get_mut(&client_id)
                                .context("UPDATE_IP: Client not present")?
                        };
                        let other_ipv6_old = other_peer.ipv6;
                        let other_ipv4_old = other_peer.ipv4;

                        if session.using_ipv6 && data.ipv6.is_none() {
                            //Protocol change: ipv6 -> ipv4
                            _ = client_self
                                .stream
                                .send(Packet::PeerIpChanged(PeerIpChanged {
                                    peer_id: client_self.id.clone(),
                                    new_ip: other_ipv4_old.clone(),
                                    old_ip: other_ipv6_old.context("Other ipv6 is empty")?,
                                }));

                            _ = other_peer.stream.send(Packet::PeerIpChanged(PeerIpChanged {
                                peer_id: client_self.id.clone(),
                                new_ip: addr_ipv4_current,
                                old_ip: addr_ipv6_old.context("Own ipv6 is empty")?,
                            }));

                            session.using_ipv6 = false;
                            client_self.ipv6 = None;
                        } else if !session.using_ipv6 && data.ipv6.is_some() {
                            //Protocol change: ipv4 -> ipv6
                            if let Some(other_ipv6) = other_ipv6_old.as_ref() {
                                _ = client_self
                                    .stream
                                    .send(Packet::PeerIpChanged(PeerIpChanged {
                                        peer_id: client_self.id.clone(),
                                        new_ip: other_ipv6.clone(),
                                        old_ip: other_ipv4_old.clone(),
                                    }));

                                _ = other_peer.stream.send(Packet::PeerIpChanged(PeerIpChanged {
                                    peer_id: client_self.id.clone(),
                                    new_ip: data.ipv6.context("Own ipv6 is empty")?,
                                    old_ip: addr_ipv4_old.clone(),
                                }));

                                session.using_ipv6 = true;
                            }
                        } else if session.using_ipv6
                            && addr_ipv6_old.context("Own old ipv6 is empty")?
                                != data.ipv6.context("Own ipv6 is empty")?
                        {
                            //This will just make the peer ping this new destination, creating the mappings

                            _ = other_peer.stream.send(Packet::PeerIpChanged(PeerIpChanged {
                                peer_id: client_self.id.clone(),
                                new_ip: data.ipv6.unwrap(),
                                old_ip: addr_ipv6_old.unwrap().clone(),
                            }));

                            session.using_ipv6 = true;
                        } else if !session.using_ipv6 && addr_ipv4_old != addr_ipv4_current {
                            _ = other_peer.stream.send(Packet::PeerIpChanged(PeerIpChanged {
                                peer_id: client_self.id.clone(),
                                new_ip: addr_ipv4_current,
                                old_ip: addr_ipv4_old,
                            }));
                        } else {
                            continue;
                        }
                    }
                }
                {
                    //Updating the ips
                    client_self.ipv4 = addr_ipv4_current;

                    client_self.ipv6 = data.ipv6;
                }

                Packet::Status(Status {
                    code: 200,
                    msg: "Success".into(),
                })
            }
            Packet::NewSession(data) => {
                //Sent by client

                let target_id = &data.target_id;

                let sessions = &mut self.sessions;
                let mut client_addr = client_self.conn.remote_address();
                if let Some(server) = self.clients.get_mut(target_id) {
                    let mut using_ipv6 = false;
                    if server.ipv6.is_some() {
                        // Update client_addr if the client has an IPv6 address, otherwise keep the original client_addr
                        if let Some(client_ipv6) = client_self.ipv6.as_ref() {
                            client_addr = client_ipv6.clone();
                            using_ipv6 = true;
                        }
                    }

                    let session_id = Uuid::new_v4().to_string();

                    sessions.insert(
                        session_id.clone(),
                        Session {
                            server_id: server.id.clone(),
                            client_id: client_self.id.clone(),
                            using_ipv6: using_ipv6,
                        },
                    );

                    client_self.session_ids.push(session_id.clone());
                    server.session_ids.push(session_id.clone());

                    info!("Sending connect packet to server!");

                    let _ = server.stream.send(Packet::ConnectTo(ConnectTo {
                        target: client_addr,
                        session_id: session_id.clone(),
                    }));

                    Packet::Status(Status {
                        code: 200,
                        msg: session_id,
                    })
                } else {
                    Packet::Status(Status {
                        code: 404,
                        msg: "Not found".into(),
                    })
                }
            }
            Packet::ServerConnectionRequest(data) => {
                //Sent as a response from the server
                let session_id = &data.session_id;

                let sessions = &mut self.sessions;
                if let Some(session) = sessions.get_mut(session_id) {
                    //Telling the client to connect to the server as a "response" to complete the UDP hole punch
                    let client = self
                        .clients
                        .get_mut(&session.client_id)
                        .context("SERVER_SENT_CONNECTION_REQUEST: Session client not present")?;

                    let mut server_addr = client_self.ipv4;

                    //Check if both support ipv6
                    if client.ipv6.is_some() {
                        // Update client_addr if the client has an IPv6 address, otherwise keep the original client_addr
                        server_addr = client_self.ipv6.clone().unwrap_or(server_addr);
                    }

                    let _ = client.stream.send(Packet::ConnectTo(ConnectTo {
                        target: server_addr,
                        session_id: client.id.clone(),
                    }));

                    Packet::Status(Status {
                        code: 200,
                        msg: "Success".into(),
                    })
                } else {
                    Packet::Status(Status {
                        code: 404,
                        msg: "Session not found".into(),
                    })
                }
            }
            _ => Packet::Status(Status {
                code: 400,
                msg: "Bad packet".into(),
            }),
        };
        Ok((response, client_self))
    }

    async fn handle_auth(&mut self, mut stream: ClientStream, conn: &Connection) -> Result<Client> {
        let auth_packet = stream.read_response::<ServerPacket>().await?;
        let packet_type = auth_packet.packet;

        let mut response = AuthResponse {
            success: false,
            token: None,
            status_msg: None,
        };

        let Packet::Auth(auth_data) = packet_type else {
            response.status_msg = Some("Packet is not of type auth".into());
            let _ = stream
                .send_packet::<Packet>(&Packet::AuthResponse(response))
                .await;
            bail!("Packet is not of type auth!");
        };

        let found = self.check_publickey(&auth_data).await?;

        if !found {
            response.status_msg = Some("Publickey check failed".into());
            let _ = stream
                .send_packet::<Packet>(&Packet::AuthResponse(response))
                .await;
            bail!("Publickey check failed for {}", auth_data.id);
        }
        response.success = true;

        let token = Uuid::new_v4().to_string();
        response.token = Some(token.clone());

        if let Err(e) = stream
            .send_packet::<Packet>(&Packet::AuthResponse(response))
            .await
        {
            bail!(
                "Failed to send auth response back to {}: {}",
                auth_data.id,
                e
            );
        }

        let client_events = EventBus::<Packet>::default();

        let msg_sender = client_events.new_sender().await;
        let receiver = client_events.subscribe().await;

        let client = Client {
            id: auth_data.id,
            conn: conn.clone(),
            auth_token: token,
            stream: msg_sender,
            session_ids: Vec::new(),
            ipv6: auth_data.ipv6,
            ipv4: conn.remote_address(),
        };

        let global_sender = self.event_bus.new_sender().await;

        Server::client_communication_task(stream, receiver, global_sender.clone());

        let conn_cloned = conn.clone();

        //Creating a new task for handling bi-stream packets
        tokio::spawn(async move {
            loop {
                let Ok(stream) = get_stream_from_conn(&conn_cloned).await else {
                    info!(
                        "Connection closed. Terminating bi-stream handler for {}",
                        conn_cloned.remote_address()
                    );
                    break;
                };
                Server::client_communication_task(
                    stream,
                    client_events.subscribe().await,
                    global_sender.clone(),
                );
            }
        });

        Ok(client)
    }

    async fn run(&mut self, endpoint: &Endpoint) -> Result<()> {
        let mut global_receiver = self.event_bus.subscribe().await;
        loop {
            tokio::select! {
                Ok(conn) = Server::accept_connections(endpoint) => {
                    //Wait for client to initiate a bi-directional stream
                    let comm_stream = match get_stream_from_conn(&conn).await {
                        Ok(conn) => conn,
                        Err(e) => {
                            log::error!("Failed to accept bi-stream for {}: {}", conn.remote_address(), e);
                            continue;
                        }
                    };

                    let client = match self.handle_auth(comm_stream, &conn).await {
                        Ok(client) => client,
                        Err(e) => {
                            error!("{} Failed auth: {}", conn.remote_address(), e);
                            continue;
                        }
                    };

                    self.clients.insert(client.id.clone(), client);
                },
                Ok(server_packet) = global_receiver.recv() => {
                    let Some(base) = &server_packet.base.as_ref() else {
                        warn!("Received malformed packet without base!");
                        continue;
                    };

                    let client = match self.authorize_take_client(base).await {
                        Ok(client) => client,
                        Err(e) => {
                            error!("Client failed auth: {}", e);
                            continue;
                        }
                    };

                    let (packet, client) = match self.handle_packet(client, server_packet).await {
                        Ok(packet) => packet,
                        Err(e) => {
                            error!("Error processing packet {}", e);
                            continue;
                        }
                    };

                    if let Err(e) = client.stream.send(packet) {
                        continue;
                    }
                    //Reinserting it
                    self.clients.insert(client.id.clone(), client);
                }
            }
        }
    }
}
