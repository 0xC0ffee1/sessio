use core::str;
use std::net::SocketAddr;

use anyhow::{anyhow, Result, bail};
use log::info;
use serde_json::Value;
use serde::{Serialize, Deserialize};

pub(crate) struct ClientStream {
    pub(crate)  recv_stream: quinn::RecvStream,
    pub(crate) send_stream: Option<quinn::SendStream>,
}

///Base for server-bound packet
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PacketBase{
    pub own_id: String,
    pub token: String
 }

 #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
 pub struct Auth {
    ///The ID that this client wants to identify as to other peers
    pub id: String,
    pub ipv6: Option<SocketAddr>,
    pub public_key_base64: String,
    pub signed_data: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
///Client-bound
pub struct AuthResponse {
    pub token: Option<String>,
    pub success: bool,
    pub status_msg: Option<String>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct UpdateIp {
    pub ipv6: Option<SocketAddr>,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct NewSession {
    pub target_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ServerConnectionRequest {
    pub client_id: String,
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
    pub target_id: String,
}

///Client-bound
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Status {
    pub code: i32,
    pub msg: String
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
}

///Server-bound packet with extra data for authentication
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ServerPacket{
    pub base: Option<PacketBase>,
    pub packet: Packet
}


impl ClientStream {

    pub async fn send_packet<T>(&mut self, packet: &T) -> Result<(), anyhow::Error>
    where
        T: serde::Serialize,
    {
        let serialized_packet = serde_json::to_string(packet)
            .map_err(|e| anyhow!("failed to serialize packet: {}", e))?;

        info!("Sending packet {}", serialized_packet);

        if let Some(send_stream) = &mut self.send_stream {

            let message_length = serialized_packet.len() as u32;
            let mut buffer = Vec::new();
            buffer.extend(&message_length.to_be_bytes());
            buffer.extend(serialized_packet.as_bytes());
    
            send_stream.write_all(&buffer)
                .await
                .map_err(|e| anyhow!("failed to send request: {}", e))?;
        }
        else {
            bail!("Can't send in a unistream!")
        }

        Ok(())
    }

    pub async fn read_response<T>(&mut self) -> Result<T>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let mut length_buf = [0u8; 4];
        self.recv_stream.read_exact(&mut length_buf).await?;
        let message_length = u32::from_be_bytes(length_buf) as usize;
    
        let mut buf = vec![0u8; message_length];
        self.recv_stream.read_exact(&mut buf).await?;
    
        // Now buf contains the full packet data
        match serde_json::from_slice::<T>(&buf) {
            Ok(value) => {
                info!("Received {:?}", value);
                Ok(value)
            }
            Err(e) => {
                Err(anyhow!("Failed to parse JSON: {}", e))
            }
        }
    }
}


pub(crate) async fn get_stream_from_conn(connection: &quinn::Connection) -> Result<ClientStream> {
    let (quinn_send, quinn_recv) = match connection.accept_bi().await {
        Ok(stream) => stream,
        Err(e) => {
            return Err(e.into());
        }
    };
    let conn = ClientStream {
        send_stream: Some(quinn_send),
        recv_stream: quinn_recv,
    };

    Ok(conn)
}