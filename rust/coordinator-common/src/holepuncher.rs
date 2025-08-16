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
        private_key_path: Option<String>,
        target_public_key: Option<String>,
    ) -> Result<()> {
        let c_client = &self.c_client;

        let mut receiver = c_client.subscribe_to_packets().await;
        let sender = c_client.new_packet_sender();

        let base = PacketBase {
            token: self.c_client.token.clone(),
            own_id: self.c_client.id_own.clone(),
        };

        // Generate crypto fields for authentication
        let (public_key_base64, signed_data, signature) = if let (Some(key_path), Some(target_key)) = (&private_key_path, &target_public_key) {
            use russh::keys::load_secret_key;
            use russh::keys::PublicKeyBase64;
            use uuid::Uuid;
            
            // Load private key
            let private_key = load_secret_key(key_path, None)
                .map_err(|e| anyhow::anyhow!("Failed to load private key: {}", e))?;
            
            // Get our public key in base64 format
            let public_key_base64 = private_key.public_key().public_key_base64();
            
            // Create challenge: CONNECTION:<target_public_key>
            let challenge = format!("CONNECTION:{}", target_key);
            
            // Sign the challenge
            use russh::keys::ssh_key::HashAlg;
            let signature = private_key.sign("sessio", HashAlg::default(), challenge.as_bytes())
                .map_err(|e| anyhow::anyhow!("Failed to sign challenge: {}", e))?;
            
            // Convert signature to base64
            use base64::{Engine, engine::general_purpose};
            use russh::keys::ssh_key::LineEnding;
            let signature_pem = signature.to_pem(LineEnding::LF)
                .map_err(|e| anyhow::anyhow!("Failed to serialize signature: {}", e))?;
            let signature_b64 = general_purpose::STANDARD.encode(&signature_pem);
            
            (public_key_base64, challenge, signature_b64)
        } else {
            // Fallback to empty values if crypto parameters not provided
            (String::new(), String::new(), String::new())
        };

        sender
            .send(ServerPacket {
                base: Some(base),
                packet: Packet::NewSession(NewSession {
                    session_id: token.clone(),
                    target_id: target,
                    public_key_base64,
                    signed_data,
                    signature,
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

                                // Verify cryptographic signature if present
                                if !data.target_public_key.is_empty() && !data.signed_data.is_empty() && !data.signature.is_empty() {
                                    use russh::keys::parse_public_key_base64;
                                    use base64;
                                    
                                    // Parse the sender's public key
                                    let sender_public_key = match parse_public_key_base64(&data.target_public_key) {
                                        Ok(key) => key,
                                        Err(e) => {
                                            error!("Failed to parse sender public key: {}", e);
                                            break;
                                        }
                                    };
                                    
                                    // Decode the signature
                                    use base64::{Engine, engine::general_purpose};
                                    let signature_pem = match general_purpose::STANDARD.decode(&data.signature) {
                                        Ok(bytes) => bytes,
                                        Err(e) => {
                                            error!("Failed to decode signature: {}", e);
                                            break;
                                        }
                                    };
                                    
                                    // Parse signature back to SshSig
                                    use russh::keys::ssh_key::SshSig;
                                    let signature = match SshSig::from_pem(&signature_pem) {
                                        Ok(sig) => sig,
                                        Err(e) => {
                                            error!("Failed to parse signature: {}", e);
                                            break;
                                        }
                                    };
                                    
                                    // Load our own public key to verify the challenge contains our key
                                    let our_public_key = if let Some(ref key_path) = private_key_path {
                                        use russh::keys::load_secret_key;
                                        use russh::keys::PublicKeyBase64;
                                        
                                        match load_secret_key(key_path, None) {
                                            Ok(private_key) => private_key.public_key().public_key_base64(),
                                            Err(e) => {
                                                error!("Failed to load our private key for verification: {}", e);
                                                break;
                                            }
                                        }
                                    } else {
                                        error!("No private key path provided for verification");
                                        break;
                                    };
                                    
                                    // Verify the challenge format
                                    let expected_challenge = format!("CONNECTION:{}", our_public_key);
                                    if data.signed_data != expected_challenge {
                                        error!("Invalid challenge format. Expected: {}, Got: {}", expected_challenge, data.signed_data);
                                        break;
                                    }
                                    
                                    // Verify the signature
                                    let signature_valid = sender_public_key.verify("sessio", data.signed_data.as_bytes(), &signature).is_ok();
                                    
                                    if !signature_valid {
                                        error!("Signature verification failed - connection denied");
                                        break;
                                    }

                                    let is_known_host = match dirs::home_dir()
                                                .map(|h| h.join(".sessio/keys/known_hosts"))
                                                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found")) {
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

                                    info!("Cryptographic signature verified successfully");
                                } else {
                                     error!("Holepunch failed: signature missing in connect to request. Could not verify authenticity of connecting device.");
                                }

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
