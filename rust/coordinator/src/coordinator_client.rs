use common::utils::events::EventBus;
use log::{error, info};
use quinn::{ConnectionClose, ConnectionError, Endpoint, VarInt};

use russh_keys::key::KeyPair;
use russh_keys::PublicKeyBase64;
use rustls::RootCertStore;
use serde::{Deserialize, Serialize};

use stunclient::StunClient;
use tokio::net::UdpSocket;

use tokio::sync::broadcast::Receiver;
use tokio::sync::mpsc;

use quinn::Connection;
use url::Url;
use uuid::Uuid;

use std::net::SocketAddr;

use std::sync::Arc;
use stun_client::nat_behavior_discovery::*;
use stun_client::*;

use quinn_proto::crypto::rustls::QuicClientConfig;

use anyhow::{anyhow, Context, Error, Result};

use crate::common::{Auth, ClientStream, Packet, PacketBase, ServerPacket};

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub ip: String,
    pub port: u16,
}

pub enum ClientType {
    SshClient,
    SshServer,
}

pub struct CoordinatorClient {
    conn: Connection,
    pub id_own: String,
    pub token: String,
    //Server-bound tx
    server_packet_sender: mpsc::Sender<ServerPacket>,
    //Client-bound rx
    client_packet_bus: EventBus<Packet>,
    endpoint: Endpoint,
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

impl CoordinatorClient {
    pub fn is_closed(&self) -> Option<ConnectionError> {
        return self.conn.close_reason();
    }

    pub fn clone_conn(&self) -> Connection {
        self.conn.clone()
    }

    pub async fn get_nat_type() -> Result<NATFilteringType> {
        let mut client = Client::new("[::]:0", None).await?;

        let mapping_result =
            check_nat_filtering_behavior(&mut client, "stun.l.google.com:19302").await?;

        Ok(mapping_result.filtering_type)
    }

    pub async fn get_external_ips(
        sock_v4: &UdpSocket,
        sock_v6: &UdpSocket,
    ) -> (Option<SocketAddr>, Option<SocketAddr>) {
        //@TODO maybe make a ip discovery service on coord server

        //stun.l.google.com in ipv4, if ipv6 is enabled, that would resolve to ipv6
        let client_v4 = StunClient::new("74.125.250.129:19302".parse().unwrap());
        let external_v4 = client_v4.query_external_address_async(sock_v4).await.ok();

        //Just making sure it is ipv6
        let client_v6 = StunClient::new("[2001:4860:4864:5:8000::1]:19302".parse().unwrap());
        let external_v6 = client_v6.query_external_address_async(sock_v6).await.ok();

        (external_v4, external_v6)
    }

    pub async fn get_external_ipv6(sock_v6: &UdpSocket) -> Option<SocketAddr> {
        let client_v6 = StunClient::new("[2001:4860:4864:5:8000::1]:19302".parse().unwrap());
        let external_v6 = client_v6.query_external_address_async(sock_v6).await.ok();

        external_v6
    }

    //The ipv6 might be different if using a vpn
    /// Gets the current external ipv6 when the port is known.
    /// This is done because we can't reuse the same socket that quinn::Endpoint has taken ownership of (in a cross-platform way).
    /// The weakness of this approach is that this assumes the port stays the same, so systems using NAT64 for example won't work with this.
    pub async fn get_new_external_ipv6(port: u16) -> Option<SocketAddr> {
        let sock = UdpSocket::bind("[::]:0").await.unwrap();

        //Just making sure it is ipv6
        let client_v6 = StunClient::new("[2001:4860:4864:5:8000::1]:19302".parse().unwrap());
        let external_v6 = client_v6.query_external_address_async(&sock).await.ok();

        if let Some(v6) = external_v6 {
            Some(SocketAddr::new(v6.ip(), port))
        } else {
            None
        }
    }

    async fn handle_auth(
        id_own: String,
        stream: &mut ClientStream,
        key_pair: KeyPair,
        ipv6: Option<SocketAddr>,
    ) -> Result<String> {
        let random_data = Uuid::new_v4().to_string();

        let signature = key_pair.sign_detached(&random_data.as_bytes())?;
        let signature_b64 = signature.to_base64();

        let auth_packet = Auth {
            id: id_own,
            ipv6,
            public_key_base64: key_pair.public_key_base64(),
            signed_data: random_data.to_string(),
            signature: signature_b64,
        };

        let packet = ServerPacket {
            base: None,
            packet: Packet::Auth(auth_packet),
        };

        stream.send_packet::<ServerPacket>(&packet).await?;

        let auth_res = stream.read_response::<Packet>().await?;
        info!("Read packet {:?}", auth_res);

        let Packet::AuthResponse(data) = auth_res else {
            anyhow::bail!("Got wrong packet type in auth");
        };

        if !data.success {
            anyhow::bail!(data.status_msg.unwrap());
        }

        Ok(data.token.unwrap())
    }

    pub async fn connect(
        coordinator_url: Url,
        id_own: String,
        endpoint: Endpoint,
        key_pair: KeyPair,
        ipv6: Option<SocketAddr>,
    ) -> Result<Self> {
        let sock_list = coordinator_url
            .socket_addrs(|| Some(2222))
            .map_err(|_| "Couldn't resolve to any address")
            .map_err(|e| anyhow!(e.to_string()))?;

        let connection = endpoint
            .connect(
                sock_list[0],
                &coordinator_url
                    .host()
                    .context("Host parse error")?
                    .to_string(),
            )
            .unwrap()
            .await?;
        info!(
            "[Coordinator client] Connected to: {}",
            connection.remote_address(),
        );

        let (send_stream, recv_stream) = connection.open_bi().await?;

        let client_packet_bus = EventBus::<Packet>::default();

        let conn = connection.clone();

        let mut stream = ClientStream {
            recv_stream,
            send_stream: Some(send_stream),
        };

        let token =
            CoordinatorClient::handle_auth(id_own.clone(), &mut stream, key_pair, ipv6).await?;

        let (server_packet_sender, mut server_packet_receiver) = mpsc::channel::<ServerPacket>(16);

        let client_packet_sender = client_packet_bus.new_sender().await;

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    //Server-bound
                    Some(send_packet) = server_packet_receiver.recv() => {
                        if let Err(e) = stream.send_packet::<ServerPacket>(&send_packet).await {
                            error!("Failed to send packet to stream {}", e);
                            break;
                        }
                    },
                    //Client-bound
                    Ok(recv_packet) = stream.read_response::<Packet>() => {
                        if let Err(e) = client_packet_sender.send(recv_packet) {
                            error!("Failed to broadcast received packet {}", e);
                            break;
                        }
                    }
                }
            }
        });

        Ok(CoordinatorClient {
            conn,
            id_own,
            client_packet_bus,
            endpoint,
            token,
            server_packet_sender,
        })
    }

    pub async fn subscribe_to_packets(&self) -> Receiver<Packet> {
        self.client_packet_bus.subscribe().await
    }

    pub fn borrow_endpoint(&mut self) -> &mut Endpoint {
        return &mut self.endpoint;
    }

    pub fn configure_crypto(endpoint: &mut Endpoint) {
        let root_store = RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.into(),
        };

        let client_crypto = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let mut client_config =
            quinn::ClientConfig::new(Arc::new(QuicClientConfig::try_from(client_crypto).unwrap()));

        let mut transport_config = enable_mtud_if_supported();
        transport_config.max_idle_timeout(Some(VarInt::from_u32(60_000).into()));
        transport_config.keep_alive_interval(Some(std::time::Duration::from_secs(1)));
        client_config.transport_config(Arc::new(transport_config));

        endpoint.set_default_client_config(client_config);
    }

    pub async fn close_connection(&mut self) {
        self.conn.close(0u32.into(), b"done");
    }

    pub fn new_packet_sender(&self) -> mpsc::Sender<ServerPacket> {
        self.server_packet_sender.clone()
    }

    ///Wraps the input Packet into a ServerPacket by using information from this struct
    pub async fn send_server_packet(&self, packet: Packet) -> Result<(), anyhow::Error> {
        self.server_packet_sender
            .send(ServerPacket {
                base: Some(PacketBase {
                    own_id: self.id_own.clone(),
                    token: self.token.clone(),
                }),
                packet,
            })
            .await?;

        Ok(())
    }
}
