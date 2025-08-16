use core::str;
use std::net::SocketAddr;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message, MaybeTlsStream};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

pub struct ClientStream {
    pub receiver: mpsc::Receiver<String>,
    pub sender: mpsc::Sender<String>,
}

///Base for server-bound packet
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PacketBase {
    pub own_id: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Auth {
    ///The ID that this client wants to identify as to other peers
    pub id: String,
    pub ipv4: Option<SocketAddr>,
    pub ipv6: Option<SocketAddr>,
    /// JWT token for authentication (contains account and device info)
    pub jwt_token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
///Client-bound
pub struct AuthResponse {
    pub token: Option<String>,
    pub success: bool,
    pub status_msg: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct HeartbeatRequest {
    pub device_id: String,
    pub jwt_token: String,
    pub version: Option<String>,  // Client/server version
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct HeartbeatResponse {
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct UpdateIp {
    pub ipv6: Option<SocketAddr>,
    pub ipv4: Option<SocketAddr>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct NewSession {
    pub target_id: String,
    pub session_id: String,
    pub public_key_base64: String,
    pub signed_data: String,
    pub signature: String,
}

///Client-bound
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct NewChannelResponse {
    pub channel_id: String,
}

///Client-bound
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct NewChannelRequest {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ServerConnectionRequest {
    pub session_id: String,
    pub public_key_base64: String,
    pub signed_data: String,
    pub signature: String,
}

///Client-bound
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PeerProtocolChanged {
    pub peer_id: String,
    pub new_ip: SocketAddr,
    pub old_ip: SocketAddr,
}

///Client-bound
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PeerIpChanged {
    pub peer_id: String,
    pub new_ip: SocketAddr,
    pub old_ip: SocketAddr,
}

///Client-bound
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ConnectTo {
    pub target: SocketAddr,
    pub session_id: String,
    pub target_public_key: String, // Ed25519 public key for verification
    pub signed_data: String,
    pub signature: String,
}

///Client-bound
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SessionData {
    pub session_id: String,
}

///Client-bound
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Status {
    pub code: i32,
    pub session_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Packet {
    Auth(Auth),
    AuthResponse(AuthResponse),
    UpdateIp(UpdateIp),
    NewSession(NewSession),
    ServerConnectionRequest(ServerConnectionRequest),
    PeerProtocolChanged(PeerProtocolChanged),
    PeerIpChanged(PeerIpChanged),
    ConnectTo(ConnectTo),
    Status(Status),
    NewChannelResponse(NewChannelResponse),
    NewChannelRequest(NewChannelRequest),
    Heartbeat(HeartbeatRequest),
    HeartbeatResponse(HeartbeatResponse),
}

///Server-bound packet with extra data for authentication
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ServerPacket {
    pub base: Option<PacketBase>,
    pub packet: Packet,
}

impl ClientStream {
    pub async fn send_packet<T>(&mut self, packet: &T) -> Result<(), anyhow::Error>
    where
        T: serde::Serialize,
    {
        let serialized_packet = serde_json::to_string(packet)
            .map_err(|e| anyhow!("failed to serialize packet: {}", e))?;

        self.sender.send(serialized_packet).await
            .map_err(|e| anyhow!("failed to send WebSocket message: {}", e))
    }

    pub async fn read_response<T>(&mut self) -> Result<T>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let message = self.receiver.recv().await
            .ok_or_else(|| anyhow!("WebSocket receiver closed"))?;

        serde_json::from_str::<T>(&message)
            .map_err(|e| anyhow!("Failed to parse JSON: {}", e))
    }
}

/// Create a WebSocket ClientStream from a WebSocket connection
pub async fn create_websocket_stream(ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Result<ClientStream> {
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let (tx, mut rx) = mpsc::channel::<String>(100);
    let (response_tx, response_rx) = mpsc::channel::<String>(100);

    // Spawn task to handle WebSocket sending
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if let Err(e) = ws_sender.send(Message::Text(message)).await {
                log::error!("WebSocket send error: {}", e);
                break;
            }
        }
    });

    // Spawn task to handle WebSocket receiving
    tokio::spawn(async move {
        while let Some(message) = ws_receiver.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    if let Err(e) = response_tx.send(text).await {
                        log::error!("WebSocket response channel error: {}", e);
                        break;
                    }
                }
                Ok(Message::Close(_)) => {
                    log::info!("WebSocket connection closed");
                    break;
                }
                Err(e) => {
                    log::error!("WebSocket receive error: {}", e);
                    break;
                }
                _ => {} // Ignore other message types
            }
        }
    });

    Ok(ClientStream {
        receiver: response_rx,
        sender: tx,
    })
}