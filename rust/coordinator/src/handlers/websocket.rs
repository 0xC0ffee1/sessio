use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{mpsc, Mutex};
use axum::{extract::{ws::WebSocketUpgrade, State}, response::Response};
use axum::extract::ws::{WebSocket, Message};
use log::{info, error, warn, debug};
use futures::{SinkExt, StreamExt};
use uuid::Uuid;
use anyhow::{Result, Context, bail};
use crate::models::{Server, Session, Client};
use sessio_coordinator_common::common::{ServerPacket, Packet, PacketBase, PeerIpChanged, Status, ConnectTo, UpdateIp, NewSession, ServerConnectionRequest, HeartbeatRequest, HeartbeatResponse};
use crate::auth::jwt::validate_device_jwt_token;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<Mutex<Server>>>,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

/// Handle WebSocket connections with improved protocol
pub async fn handle_websocket(socket: WebSocket, state: Arc<Mutex<Server>>) {
    let (ws_sender, mut ws_receiver) = mpsc::channel::<String>(100);
    let (mut ws_sink, mut ws_stream) = socket.split();
    
    // Track device info for this connection
    let mut device_id: Option<String> = None;

    // WebSocket sender task
    let sender_task = tokio::spawn(async move {
        while let Some(message) = ws_receiver.recv().await {
            if let Err(e) = ws_sink.send(Message::Text(message)).await {
                error!("WebSocket send error: {}", e);
                break;
            }
        }
    });
    
    // Handle incoming WebSocket messages
    let receiver_task = tokio::spawn(async move {
        while let Some(msg) = ws_stream.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str::<ServerPacket>(&text) {
                        Ok(server_packet) => {
                            debug!("Received packet from {}: {:?}", device_id.as_deref().unwrap_or("unknown"), server_packet);
                            
                            let mut server = state.lock().await;
                            match handle_websocket_packet(&mut server, server_packet, &ws_sender, &mut device_id).await {
                                Ok(_) => {},
                                Err(e) => {
                                    error!("Error handling WebSocket packet: {}", e);
                                    // Send error response
                                    let error_response = Packet::Status(Status {
                                        code: 500,
                                        session_id: "".to_string(),
                                    });
                                    if let Ok(response_json) = serde_json::to_string(&error_response) {
                                        let _ = ws_sender.send(response_json).await;
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            error!("Failed to parse WebSocket message: {}", e);
                        }
                    }
                },
                Ok(Message::Close(_)) => {
                    info!("WebSocket connection closed for device: {:?}", device_id);
                    break;
                },
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                },
                _ => {} // Ignore other message types
            }
        }
        
        // Cleanup on disconnect
        if let Some(device_id) = device_id {
            let mut server = state.lock().await;
            cleanup_device_connection(&mut server, &device_id).await;
        }
    });
    
    // Wait for either task to complete
    tokio::select! {
        _ = sender_task => {},
        _ = receiver_task => {},
    }
}

/// Handle WebSocket packets with improved logic
async fn handle_websocket_packet(
    server: &mut Server,
    server_packet: ServerPacket,
    ws_sender: &mpsc::Sender<String>,
    device_id: &mut Option<String>,
) -> Result<()> {
    let Some(base) = &server_packet.base else {
        warn!("Received malformed packet without base");
        return Ok(());
    };

    // Validate JWT token
    let (validated_device_id, account_id) = validate_device_jwt_token(&base.token)
        .ok_or_else(|| anyhow::anyhow!("Invalid JWT token"))?;
    
    // Verify device_id matches the packet's own_id
    if validated_device_id != base.own_id {
        bail!("Device ID mismatch in JWT token: {} != {}", validated_device_id, base.own_id);
    }
    
    // Set device_id for this connection if not already set
    if device_id.is_none() {
        *device_id = Some(validated_device_id.clone());
        info!("WebSocket connection established for device: {}", validated_device_id);
    }
    
    // Register or update device in server state
    let mut client = ensure_device_registered(server, &validated_device_id, account_id.clone(), ws_sender.clone()).await?;
    
    // Process the actual packet
    let response_packet = match &server_packet.packet {
        Packet::UpdateIp(data) => {
            handle_update_ip(server, &mut client, data).await?
        },
        Packet::NewSession(data) => {
            handle_new_session(server, &mut client, data).await?
        },
        Packet::ServerConnectionRequest(data) => {
            handle_server_connection_request(server, &mut client, data).await?
        },
        Packet::Heartbeat(data) => {
            handle_heartbeat(server, &mut client, data, &account_id).await?
        },
        _ => {
            warn!("Unhandled packet type");
            Packet::Status(Status {
                code: 400,
                session_id: "".to_string(),
            })
        }
    };

    // Update client in server state
    server.clients.insert(client.id.clone(), client);

    // Send response
    let response_json = serde_json::to_string(&response_packet)?;
    debug!("Sending response to {}: {}", validated_device_id, response_json);
    
    if let Err(e) = ws_sender.send(response_json).await {
        error!("Error sending response to device {}: {}", validated_device_id, e);
    }

    Ok(())
}

/// Ensure device is properly registered and return client
async fn ensure_device_registered(
    server: &mut Server,
    device_id: &str,
    account_id: String,
    ws_sender: mpsc::Sender<String>
) -> Result<Client> {
    // Check if client already exists (from auth_handler)
    if let Some(mut existing_client) = server.clients.remove(device_id) {
        // Update WebSocket sender to the real connection
        existing_client.ws_sender = Some(ws_sender);
        // Update last seen
        server.db.update_device_last_seen(device_id, uuid::Uuid::parse_str(&account_id)?).await
            .context("Failed to update device last seen")?;
        
        return Ok(existing_client);
    }
    
    // Client must be created via auth_handler first
    bail!("Client not found. Device must authenticate via /auth endpoint first: {}", device_id);
}

/// Handle IP update packets
async fn handle_update_ip(
    server: &mut Server,
    client: &mut Client,
    data: &UpdateIp,
) -> Result<Packet> {
    let new_ipv4 = data.ipv4.unwrap_or(client.ipv4);
    let old_ipv4 = client.ipv4;
    let old_ipv6 = client.ipv6;
    
    // Update client IP addresses
    client.ipv4 = new_ipv4;
    client.ipv6 = data.ipv6;
    
    // Notify peers in active sessions about IP changes
    for session_id in &client.session_ids {
        if let Some(session) = server.sessions.get_mut(session_id) {
            let is_client = client.id == session.client_id;
            
            // Get the other peer
            let other_peer_id = if is_client {
                &session.server_id
            } else {
                &session.client_id
            };
            
            if let Some(other_peer) = server.clients.get(other_peer_id) {
                notify_peer_ip_change(other_peer, client, old_ipv4, old_ipv6, session).await?;
            }
        }
    }
    
    debug!("Updated IP for device {}: IPv4: {:?} -> {:?}, IPv6: {:?} -> {:?}", 
           client.id, old_ipv4, new_ipv4, old_ipv6, data.ipv6);
    
    Ok(Packet::Status(Status {
        code: 200,
        session_id: "".to_string(),
    }))
}

/// Handle new session creation
async fn handle_new_session(
    server: &mut Server,
    client: &mut Client,
    data: &NewSession,
) -> Result<Packet> {
    let target_id = &data.target_id;
    let session_id = Uuid::new_v4().to_string();
    
    // Check if target device exists
    let target_device = server.clients.get_mut(target_id)
        .ok_or_else(|| anyhow::anyhow!("Target device not found: {}", target_id))?;
    
    // Get target device info from database to fetch public key
    let target_device_info = server.db.get_device_by_id(target_id, target_device.account_id).await?
        .ok_or_else(|| anyhow::anyhow!("Target device not found in database: {}", target_id))?;
    
    // Get client's public key for verification
    let client_device_info = server.db.get_device_by_id(&client.id, client.account_id).await?
        .ok_or_else(|| anyhow::anyhow!("Client device not found in database: {}", client.id))?;
    
    // Determine IP protocol preference
    let using_ipv6 = client.ipv6.is_some() && target_device.ipv6.is_some();
    let client_addr = if using_ipv6 {
        client.ipv6.unwrap_or(client.ipv4)
    } else {
        client.ipv4
    };
    
    let session = Session {
        server_id: target_device.id.clone(),
        client_id: client.id.clone(),
        using_ipv6,
    };
    
    server.sessions.insert(session_id.clone(), session);
    
    // Add session to both devices
    client.session_ids.push(session_id.clone());
    target_device.session_ids.push(session_id.clone());
    
    // Send connect packet to target device with client's public key and crypto fields
    let connect_packet = Packet::ConnectTo(ConnectTo {
        target: client_addr,
        session_id: session_id.clone(),
        target_public_key: data.public_key_base64.clone(), // Client's public key
        signed_data: data.signed_data.clone(),
        signature: data.signature.clone(),
    });
    
    let packet_json = serde_json::to_string(&connect_packet)?;
    if let Some(ws_sender) = &target_device.ws_sender {
        if let Err(e) = ws_sender.send(packet_json).await {
            error!("Failed to send connect packet to target device {}: {}", target_id, e);
            return Ok(Packet::Status(Status {
                code: 500,
                session_id: session_id.clone(),
            }));
        }
    } else {
        error!("Target device {} has no WebSocket connection", target_id);
        return Ok(Packet::Status(Status {
            code: 503, // Service Unavailable
            session_id: session_id.clone(),
        }));
    }
    
    info!("Created session {} between {} and {}", session_id, client.id, target_id);
    
    Ok(Packet::Status(Status {
        code: 200,
        session_id,
    }))
}

/// Handle server connection request (response to new session)
async fn handle_server_connection_request(
    server: &mut Server,
    client: &mut Client,
    data: &ServerConnectionRequest,
) -> Result<Packet> {
    let session_id = &data.session_id;
    
    // Find the session
    let session = server.sessions.get(session_id)
        .ok_or_else(|| anyhow::anyhow!("Session not found: {}", session_id))?;
    
    // Get the client that initiated the session
    let client_peer = server.clients.get(&session.client_id)
        .ok_or_else(|| anyhow::anyhow!("Client not found: {}", session.client_id))?;
    
    // Get server's public key for verification  
    let server_device_info = server.db.get_device_by_id(&client.id, client.account_id).await?
        .ok_or_else(|| anyhow::anyhow!("Server device not found in database: {}", client.id))?;
    
    // Determine server address to send to client
    let server_addr = if session.using_ipv6 {
        client.ipv6.unwrap_or(client.ipv4)
    } else {
        client.ipv4
    };
    
    // Send connect-to packet directly to client via WebSocket with server's public key and crypto fields
    let connect_packet = Packet::ConnectTo(ConnectTo {
        target: server_addr,
        session_id: session_id.clone(),
        target_public_key: data.public_key_base64.clone(), // Server's public key
        signed_data: data.signed_data.clone(),
        signature: data.signature.clone(),
    });
    
    let packet_json = serde_json::to_string(&connect_packet)?;
    if let Some(ws_sender) = &client_peer.ws_sender {
        if let Err(e) = ws_sender.send(packet_json).await {
            error!("Failed to send connect packet to client: {}", e);
            return Ok(Packet::Status(Status {
                code: 500,
                session_id: session_id.clone(),
            }));
        }
    } else {
        error!("Client {} has no WebSocket connection", client_peer.id);
        return Ok(Packet::Status(Status {
            code: 503, // Service Unavailable
            session_id: session_id.clone(),
        }));
    }
    
    info!("Completed holepunch for session {}", session_id);
    
    Ok(Packet::Status(Status {
        code: 200,
        session_id: session_id.clone(),
    }))
}

/// Handle heartbeat packets
async fn handle_heartbeat(
    server: &mut Server,
    client: &mut Client,
    data: &HeartbeatRequest,
    account_id: &str,
) -> Result<Packet> {
    // Parse account_id to UUID
    let account_uuid = Uuid::parse_str(account_id)
        .context("Failed to parse account_id as UUID")?;
    
    // Update last seen and version for the device
    server.db.update_device_heartbeat(&client.id, account_uuid, data.version.clone()).await
        .context("Failed to update device heartbeat")?;
    
    // Log version if provided
    if let Some(version) = &data.version {
        debug!("Heartbeat received from device: {} (v{})", client.id, version);
    } else {
        debug!("Heartbeat received from device: {}", client.id);
    }
    
    Ok(Packet::HeartbeatResponse(HeartbeatResponse {
        success: true,
        message: Some("Heartbeat acknowledged".to_string()),
    }))
}

/// Notify peer about IP address changes
async fn notify_peer_ip_change(
    peer: &Client,
    changed_client: &Client,
    old_ipv4: std::net::SocketAddr,
    old_ipv6: Option<std::net::SocketAddr>,
    session: &Session,
) -> Result<()> {
    let (new_ip, old_ip) = if session.using_ipv6 {
        if let (Some(new_ipv6), Some(old_ipv6)) = (changed_client.ipv6, old_ipv6) {
            (new_ipv6, old_ipv6)
        } else {
            // Protocol change from IPv6 to IPv4 or vice versa
            (changed_client.ipv4, old_ipv4)
        }
    } else {
        (changed_client.ipv4, old_ipv4)
    };
    
    if new_ip != old_ip {
        let packet = Packet::PeerIpChanged(PeerIpChanged {
            peer_id: changed_client.id.clone(),
            new_ip,
            old_ip,
        });
        
        let packet_json = serde_json::to_string(&packet)?;
        if let Some(ws_sender) = &peer.ws_sender {
            if let Err(e) = ws_sender.send(packet_json).await {
                error!("Failed to notify peer {} about IP change: {}", peer.id, e);
            } else {
                debug!("Notified peer {} about IP change for {}", peer.id, changed_client.id);
            }
        } else {
            warn!("Peer {} has no WebSocket connection to notify about IP change", peer.id);
        }
    }
    
    Ok(())
}

/// Clean up device connection when WebSocket disconnects
async fn cleanup_device_connection(server: &mut Server, device_id: &str) {
    info!("Cleaning up connection for device: {}", device_id);
    
    // Remove device from server state
    if let Some(client) = server.clients.remove(device_id) {
        // Clean up sessions
        for session_id in &client.session_ids {
            if let Some(session) = server.sessions.remove(session_id) {
                // Notify the other peer about session termination
                let other_peer_id = if session.client_id == device_id {
                    &session.server_id
                } else {
                    &session.client_id
                };
                
                if let Some(other_peer) = server.clients.get(other_peer_id) {
                    let termination_packet = Packet::Status(Status {
                        code: 410, // Gone
                        session_id: session_id.clone(),
                    });
                    
                    if let Ok(packet_json) = serde_json::to_string(&termination_packet) {
                        if let Some(ws_sender) = &other_peer.ws_sender {
                            let _ = ws_sender.send(packet_json).await;
                        }
                    }
                }
            }
        }
        
        info!("Cleaned up {} sessions for device {}", client.session_ids.len(), device_id);
    }
}