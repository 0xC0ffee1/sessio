use std::{net::SocketAddr, time::Duration};

use crate::{
    common::{
        NewSession, Packet, PacketBase, ServerPacket,
        UpdateIp,
    },
    coordinator_client::CoordinatorClient,
};

use log::{error, info, warn};

use anyhow::Result;
use quinn::{Connection, Endpoint};
use tokio::{sync::mpsc::Sender, time};
use url::Url;
// use uuid::Uuid; // Currently unused

pub struct HolepunchService {
    pub c_client: CoordinatorClient,
    coordinator_url: Url,
    jwt_token: String,

    // The public ips
    ipv4: Option<SocketAddr>,
    ipv6: Option<SocketAddr>,

    // Quinn endpoint for P2P connections
    pub endpoint: Endpoint,
}

impl HolepunchService {
    async fn connect(
        coordinator_url: &Url,
        jwt_token: String,
        ipv4: Option<SocketAddr>,
        ipv6: Option<SocketAddr>,
        id_own: String,
    ) -> Result<CoordinatorClient> {
        let c_client = loop {
            match CoordinatorClient::connect(
                coordinator_url.clone(),
                id_own.clone(),
                jwt_token.clone(),
                ipv4,
                ipv6,
            )
                .await
            {
                Ok(client) => break client,
                Err(e) => {
                    // HTTP/WebSocket specific error handling
                    if e.to_string().contains("Connection refused") || e.to_string().contains("timeout") {
                        error!(
                            "Failed to connect to coordination server {}\nRetrying in 5 seconds..",
                            e
                        );
                        time::sleep(Duration::from_secs(5)).await;
                    } else {
                        error!("Failed to connect to coordination server: {}", e);
                        return Err(e.context("Possibly failed auth: check coord server logs"));
                    }
                }
            }
        };
        Ok(c_client)
    }

    ///Account ID is used to authenticate with the coordinator
    pub async fn new(
        coordinator_url: Url,
        jwt_token: String,
        ipv4: Option<SocketAddr>,
        ipv6: Option<SocketAddr>,
        id_own: String,
        endpoint: Endpoint,
    ) -> Result<Self> {
        let c_client =
            HolepunchService::connect(&coordinator_url, jwt_token.clone(), ipv4.clone(), ipv6.clone(), id_own.clone()).await?;
        let mut service = HolepunchService {
            c_client,
            coordinator_url,
            jwt_token,
            ipv4,
            ipv6,
            endpoint,
        };
        service.start_connection_update_task();
        Ok(service)
    }

    pub async fn reconnect(&mut self) -> Result<()> {
        // For WebSocket, we don't need to track specific ports like QUIC
        // Just reconnect with the same parameters
        self.c_client = HolepunchService::connect(
            &self.coordinator_url,
            self.jwt_token.clone(),
            self.ipv4,
            self.ipv6,
            self.c_client.id_own.clone(),
        )
            .await?;
        Ok(())
    }

    pub async fn attempt_holepunch(
        &self,
        target: String,
        token: String,
        connection_sender: Sender<Connection>,
    ) -> Result<()> {
        let c_client = &self.c_client;

        let mut receiver = c_client.subscribe_to_packets().await;
        let sender = c_client.new_packet_sender();

        let base = PacketBase {
            token: self.c_client.token.clone(),
            own_id: self.c_client.id_own.clone(),
        };

        sender
            .send(ServerPacket {
                base: Some(base),
                packet: Packet::NewSession(NewSession {
                    session_id: token.clone(),
                    target_id: target,
                }),
            })
            .await?;

        info!("new session packet sent!");


        let response = receiver.recv().await?;

        info!("status received!");

        if let Packet::Status(status) = response {
            // Check for a 404 error
            if status.code == 404 {
                anyhow::bail!("Target device not found!");
            }
        } else {
            anyhow::bail!("Protocol error: wrong packet received!");
        }

        let timeout_duration = Duration::from_secs(10);


        let endpoint_clone = self.endpoint.clone();
        tokio::spawn(async move {
            let timeout_future = tokio::time::sleep(timeout_duration);
            tokio::pin!(timeout_future);
            loop {
                tokio::select! {
                    packet = receiver.recv() => {
                        let packet = match packet {
                            Ok(packet) => packet,
                            Err(e) => {
                                error!("Failed to receive packet: {}", e);
                                break;
                            }
                        };
                        match packet {
                            Packet::ConnectTo(data) => {
                                info!("trying to connect to {:?}", data.target);
                                
                                // Verify target public key against known_hosts before connecting
                                // Use string comparison since russh is not available in this crate
                                let is_known_host = match std::env::var("KNOWN_HOSTS_PATH")
                                    .map(std::path::PathBuf::from)
                                    .or_else(|_| {
                                        dirs::home_dir()
                                            .map(|h| h.join(".sessio/known_hosts"))
                                            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found"))
                                    }) {
                                    Ok(known_hosts_path) => {
                                        match tokio::fs::read_to_string(&known_hosts_path).await {
                                            Ok(content) => {
                                                // Check if the target public key is in known_hosts
                                                content.lines().any(|line| {
                                                    // Parse SSH public key format: "ssh-ed25519 <key> <comment>"
                                                    let parts: Vec<&str> = line.trim().split_whitespace().collect();
                                                    if parts.len() >= 2 && parts[0] == "ssh-ed25519" {
                                                        parts[1] == data.target_public_key
                                                    } else {
                                                        false
                                                    }
                                                })
                                            }
                                            Err(e) => {
                                                error!("Failed to read known_hosts: {}", e);
                                                false
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to get known_hosts path: {}", e);
                                        false
                                    }
                                };
                                
                                if !is_known_host {
                                    warn!("Connection denied: target public key {} not found in known_hosts", data.target_public_key);
                                    error!("Holepunch failed: target not in known_hosts. Please sign the target device from the coordinator web ui.");
                                    break;
                                }
                                
                                info!("Target public key verified in known_hosts, attempting connection");
                                match endpoint_clone.connect(data.target, "server").unwrap().await {
                                    Ok(conn) => {
                                        let _ = connection_sender.send(conn).await;
                                        info!("Holepunch succeeded!");
                                        break;
                                    }
                                    Err(e) => {
                                        info!("Connection failed: {}", e);
                                        break;
                                    }
                                }
                            }
                            _ => {}
                        }
                    },
                    _ = &mut timeout_future => {
                        error!("Attempt to holepunch timed out after {:?}", timeout_duration);
                        break;
                    }
                }
            }
            drop(connection_sender);
        });
        Ok(())
    }

    pub fn start_connection_update_task(&mut self) {
        let mut update_interval = time::interval(Duration::from_secs(2));

        let sender = self.c_client.new_packet_sender();

        let id = self.c_client.id_own.clone();
        let jwt = self.jwt_token.clone();

        let mut ipv4 = self.ipv4.clone();
        let mut ipv6 = self.ipv6.clone();
        let endpoint = self.endpoint.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = update_interval.tick() => {

                        // Use endpoint for IP discovery
                        let (ipv4_now, ipv6_now, sock) = CoordinatorClient::get_external_ips_dual_sock_new().await;

                        // Update IPv4 if it has changed
                        if let Some(ipv4_now) = ipv4_now {
                            if ipv4.is_none() || ipv4_now.ip() != ipv4.unwrap().ip() {
                                ipv4 = Some(ipv4_now);
                                ipv6 = ipv6_now;

                                endpoint.rebind(sock.into_std().unwrap()).expect("Failed to rebind socket");
                            }
                        }

                        let packet = ServerPacket {
                            base: Some(PacketBase {
                                own_id: id.clone(),
                                token: jwt.clone(),
                            }),
                            packet: Packet::UpdateIp(UpdateIp {
                                ipv6,
                                ipv4
                            })
                        };

                        if let Err(e) = sender.send(packet).await {
                            log::warn!("Could not send update packet {}", e);
                            break;
                        }
                    }
                }
            }
        });
    }
}
