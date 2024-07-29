
use log::info;
use log4rs::encode::json;
use quinn::{ClientConfig, Endpoint, VarInt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};
use quinn::Connection;
use url::Url;
use std::collections::HashMap;
use std::net::{Ipv6Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{anyhow, Result};


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
    id_own: String,
    send_stream: quinn::SendStream,
    response_rx: tokio::sync::mpsc::Receiver<Value>
}

struct SkipServerVerification;

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        Arc::new(Self)
    }
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

impl CoordinatorClient {

    //This is done to get the temporary Ipv6
    async fn get_public_ipv6() -> Result<Ipv6Addr> {
        let response = reqwest::get("https://api64.ipify.org")
            .await?
            .text()
            .await?;
        let ip: Ipv6Addr = response.parse()?;
        Ok(ip)
    }

    pub async fn connect(coordinator_url: Url, id_own: String, mut endpoint: Endpoint) -> Self {

        let sock_list = coordinator_url
            .socket_addrs(|| Some(2222))
            .map_err(|_| "Couldn't resolve to any address").unwrap();
        
        let connection = endpoint.connect(sock_list[0], "asd").unwrap().await.unwrap();
        info!(
            "[Coordinator client] Connected to: {}",
            connection.remote_address(),
        );
        
        let (mut send_stream, mut recv_stream) = connection
            .open_bi()
            .await
            .map_err(|e| format!("failed to open stream: {}", e)).unwrap();


        let (response_tx, mut response_rx) = mpsc::channel::<serde_json::Value>(100);

        let conn = connection.clone();

        tokio::spawn(async move {
            loop {
                let mut buffer = vec![0; 4]; // Buffer to read the message length
                // Read the length of the next JSON message
                match recv_stream.read_exact(&mut buffer).await {
                    Ok(_) => {
                        let message_length = u32::from_be_bytes(buffer.try_into().unwrap()) as usize;
                        let mut json_buffer = vec![0; message_length];

                        // Read the actual JSON message
                        match recv_stream.read_exact(&mut json_buffer).await {
                            Ok(_) => {
                                let json_string = String::from_utf8_lossy(&json_buffer);
                                match serde_json::from_str::<serde_json::Value>(&json_string) {
                                    Ok(json_resp) => {
                                        if response_tx.send(json_resp).await.is_err() {
                                            break;
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("failed to parse JSON: {}:{}", json_string, e);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("failed to read JSON message: {}", e);
                                //connection.close(0u32.into(), b"err");
                                //send_stream.finish().await;
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("failed to read message length: {}", e);
                        //send_stream.finish().await;
                        //connection.close(0u32.into(), b"err");
                        break;
                    }
                }
            }
        });

        CoordinatorClient {
            conn,
            id_own,
            send_stream,
            response_rx
        }
    }

    pub fn configure_client( endpoint: &mut Endpoint) {
        let crypto = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth();

        let mut client_config = ClientConfig::new(Arc::new(crypto));
        let mut transport_config = enable_mtud_if_supported();
        transport_config.max_idle_timeout(Some(VarInt::from_u32(60_000).into()));
        transport_config.keep_alive_interval(Some(std::time::Duration::from_secs(1)));
        client_config.transport_config(Arc::new(transport_config));

        endpoint.set_default_client_config(client_config);
    }


    pub async fn close_connection(&mut self){
        self.conn.close(0u32.into(), b"done");
    }

    
    pub async fn read_response<T>(&mut self) -> Result<T, anyhow::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        while let Some(response) = self.response_rx.recv().await {
            info!("Received {}", response.to_string());

            let result = serde_json::from_value::<T>(response.clone())
                .map_err(|e| anyhow!("failed to parse JSON: {}:{}", response.to_string(), e))?;
            return Ok(result);
        }

        Err(anyhow!("No matching response received"))
    }

    pub async fn send_packet<T>(&mut self, packet: &T) -> Result<(), anyhow::Error>
    where
        T: serde::Serialize,
    {
        let serialized_packet = serde_json::to_string(packet)
            .map_err(|e| anyhow!("failed to serialize packet: {}", e))?;

        self.send_stream.write_all(serialized_packet.as_bytes())
            .await
            .map_err(|e| anyhow!("failed to send request: {}", e))?;

        info!("Sent {}", serialized_packet);

        Ok(())
    }
    
    pub async fn register_endpoint(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        let ipv6 = match CoordinatorClient::get_public_ipv6().await {
            Ok(ip) => ip.to_string(),
            Err(_) => String::from("None")
        };

        let register_msg = serde_json::json!({"id": "REGISTER", "own_id": self.id_own, "ipv6": ipv6});

        self.send_packet(&register_msg).await;
        let res = self.read_response::<HashMap<String, String>>().await;
        info!("{}", res.unwrap().get("status").unwrap());

        Ok(())
    }

    pub async fn connect_to(&mut self, target: String) -> Result<(), Box<dyn std::error::Error>> {

        let register_msg = serde_json::json!({"id": "CLIENT_CONNECTED", "target_client_id": target});

        self.send_packet(&register_msg).await;
        let res = self.read_response::<HashMap<String, String>>().await;
        info!("{}", res.unwrap().get("status").unwrap());

        Ok(())
    }

    pub async fn new_session(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let register_msg = serde_json::json!({"id": "NEW_SESSION", "own_id": self.id_own});

        self.send_packet(&register_msg).await;
        let res = self.read_response::<HashMap<String, String>>().await;
        info!("{}", res.unwrap().get("status").unwrap());

        Ok(())
    }


}
