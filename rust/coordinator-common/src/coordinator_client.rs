use std::error::Error;
// EventBus not used in Docker build - commenting out to fix compilation
// use common::utils::events::EventBus;
use log::{debug, error, info, warn};
use reqwest::Client as HttpClient;
use tokio_tungstenite::connect_async;

use serde::{Deserialize, Serialize};

use stunclient::StunClient;
use tokio::net::UdpSocket;

use tokio::sync::broadcast::Receiver;
use tokio::sync::mpsc;

use url::Url;

use std::net::SocketAddr;

use stun_client::nat_behavior_discovery::*;
use stun_client::*;

use anyhow::{anyhow, Result};
use common::utils::events::EventBus;
use crate::common::{Auth, ClientStream, Packet, PacketBase, ServerPacket, create_websocket_stream, HeartbeatRequest};
use base64::{Engine as _, engine::general_purpose};

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
    pub http_client: HttpClient,
    pub id_own: String,
    pub token: String,
    pub coordinator_url: Url,
    //Server-bound tx
    server_packet_sender: mpsc::Sender<ServerPacket>,

    //Client-bound rx - EventBus commented out to fix Docker build
    client_packet_bus: EventBus<Packet>,
}

// HTTP/WebSocket doesn't need MTUD configuration - removed legacy functions

impl CoordinatorClient {
    pub fn is_closed(&self) -> bool {
        // WebSocket client is considered closed if the HTTP client is dropped
        false // For now, always return false - could be enhanced with connection state tracking
    }

    pub async fn get_nat_type() -> Result<NATFilteringType> {
        let mut client = Client::new("[::]:0", None).await?;

        let mapping_result =
            check_nat_filtering_behavior(&mut client, "stun.l.google.com:19302").await?;

        Ok(mapping_result.filtering_type)
    }

    pub async fn get_external_ips_dual_sock(
        sock: &UdpSocket,
    ) -> (Option<SocketAddr>, Option<SocketAddr>) {
        //stun.l.google.com in ipv6 mapped ipv4 address
        let client_v4 = StunClient::new("[::ffff:74.125.250.129]:19302".parse().unwrap());
        let external_v4 = client_v4.query_external_address_async(sock).await.ok();

        //Just making sure it is ipv6
        let client_v6 = StunClient::new("[2001:4860:4864:5:8000::1]:19302".parse().unwrap());
        let external_v6 = client_v6.query_external_address_async(sock).await.ok();

        (external_v4, external_v6)
    }

    pub async fn get_external_ips(
        sock_v4: &UdpSocket,
        sock_v6: &UdpSocket,
    ) -> (Option<SocketAddr>, Option<SocketAddr>) {
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

    pub async fn get_new_external_ipv4() -> (Option<SocketAddr>, UdpSocket) {
        let sock = UdpSocket::bind("[::]:0").await.unwrap();

        let client_v4 = StunClient::new("74.125.250.129:19302".parse().unwrap());
        let external_v4 = client_v4.query_external_address_async(&sock).await.ok();

        return (external_v4, sock)
    }

    fn construct_api_url(base_url: &Url, endpoint: &str) -> Result<Url> {
        let mut api_url = base_url.clone();
        let api_path = format!("/api{}", endpoint);
        api_url.set_path(&api_path);
        Ok(api_url)
    }


    async fn handle_auth(
        http_client: &HttpClient,
        coordinator_url: &Url,
        id_own: String,
        jwt_token: String,
        ipv4: Option<SocketAddr>,
        ipv6: Option<SocketAddr>,
    ) -> Result<()> {

        let auth_packet = Auth {
            id: id_own,
            ipv4,
            ipv6,
            jwt_token,
        };

        let auth_url = Self::construct_api_url(coordinator_url, "/auth")?;

        let response = http_client
            .post(auth_url)
            .json(&auth_packet)
            .send()
            .await?
            .json::<crate::common::AuthResponse>()
            .await?;

        info!("Auth response: {:?}", response);

        if !response.success {
            anyhow::bail!(response.status_msg.unwrap_or_else(|| "Authentication failed".to_string()));
        }

        Ok(())
    }

    pub async fn new_stream(&self) -> Result<ClientStream> {
        // For HTTP/WebSocket, we create a new WebSocket connection
        let mut ws_url = Self::construct_api_url(&self.coordinator_url, "/ws")?;
        ws_url.set_scheme("ws").map_err(|_| anyhow!("Invalid URL scheme"))?;

        let (ws_stream, _) = connect_async(ws_url).await?;
        create_websocket_stream(ws_stream).await
    }

    pub async fn connect(
        coordinator_url: Url,
        id_own: String,
        jwt_token: String,
        ipv4: Option<SocketAddr>,
        ipv6: Option<SocketAddr>,
    ) -> Result<Self> {
        let http_client = HttpClient::new();

        info!(
            "[Coordinator client] Connecting to: {}",
            coordinator_url,
        );

        // Then establish WebSocket connection
        let mut ws_url = Self::construct_api_url(&coordinator_url, "/ws")?;
        ws_url.set_scheme("ws").map_err(|_| anyhow!("Invalid URL scheme"))?;

        let (ws_stream, _) = connect_async(ws_url).await?;
        let mut stream = create_websocket_stream(ws_stream).await?;

        CoordinatorClient::handle_auth(
            &http_client,
            &coordinator_url,
            id_own.clone(),
            jwt_token.clone(),
            ipv4,
            ipv6,
        ).await?;

        // EventBus commented out to fix Docker build
        let client_packet_bus = EventBus::<Packet>::default();
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

        let client = CoordinatorClient {
            http_client,
            id_own: id_own.clone(),
            coordinator_url: coordinator_url.clone(),
            token: jwt_token.clone(),
            server_packet_sender,
            client_packet_bus,
        };

        // Start heartbeat task
        client.start_heartbeat_task(id_own, jwt_token).await;

        Ok(client)
    }

    pub async fn subscribe_to_packets(&self) -> Receiver<Packet> {
        self.client_packet_bus.subscribe().await
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

    /// Starts a background task that sends heartbeat packets to update device last seen
    /// Note: The jwt_token parameter is passed as the second parameter for historical reasons
    pub async fn start_heartbeat_task(&self, device_id: String, jwt_token: String) {
        let server_packet_sender = self.server_packet_sender.clone();
        let id_own = self.id_own.clone();
        let token = self.token.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));

            loop {
                interval.tick().await;

                let heartbeat_request = HeartbeatRequest {
                    device_id: device_id.clone(),
                    jwt_token: jwt_token.clone(),
                    version: Some(env!("CARGO_PKG_VERSION").to_string()),
                };

                let heartbeat_packet = ServerPacket {
                    base: Some(PacketBase {
                        own_id: id_own.clone(),
                        token: token.clone(),
                    }),
                    packet: Packet::Heartbeat(heartbeat_request),
                };

                match server_packet_sender.send(heartbeat_packet).await {
                    Ok(_) => {
                        debug!("Heartbeat sent successfully over WebSocket");
                    }
                    Err(e) => {
                        error!("Failed to send heartbeat over WebSocket: {}", e);
                    }
                }
            }
        });
    }

    /// Starts a background task that syncs authorized keys with cryptographic verification
    /// This replaces the less secure implementations in client and server
    /// passkey_json: Full JSON-serialized Passkey for signature verification (from webauthn_credentials table)
    pub async fn start_authorized_keys_sync_task(
        &self,
        jwt_token: String,
        passkey_json: Option<String>,
        sync_interval_secs: u64,
        authorized_keys_path: std::path::PathBuf,
        include_unverified: bool,
    ) {
        let http_client = self.http_client.clone();
        let coordinator_url = self.coordinator_url.clone();

        info!("passkey {:?}", passkey_json);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(sync_interval_secs));

            loop {
                interval.tick().await;

                // Construct authorized keys URL
                let authorized_keys_url = match coordinator_url.join("/api/authorized-keys") {
                    Ok(url) => url,
                    Err(e) => {
                        error!("Failed to construct authorized keys URL: {}", e);
                        continue;
                    }
                };

                // Prepare request - account_id is extracted from JWT token, not from request body
                let request_body = serde_json::json!({});

                // Make API request
                let response = match http_client
                    .post(authorized_keys_url)
                    .header("Authorization", format!("Bearer {}", jwt_token))
                    .json(&request_body)
                    .send()
                    .await
                {
                    Ok(response) => response,
                    Err(e) => {
                        error!("Failed to fetch authorized keys: {}", e);
                        continue;
                    }
                };

                // Parse response
                let authorized_keys_response: serde_json::Value = match response.json().await {
                    Ok(data) => data,
                    Err(e) => {
                        error!("Failed to parse authorized keys response: {}", e);
                        continue;
                    }
                };

                let keys = match authorized_keys_response.get("keys").and_then(|k| k.as_array()) {
                    Some(keys) => keys,
                    None => {
                        error!("Invalid authorized keys response format");
                        continue;
                    }
                };

                // Process and verify keys
                let mut authorized_entries = Vec::new();
                let mut verified_count = 0;
                let mut unverified_count = 0;

                for key_data in keys {
                    let device_id = key_data.get("device_id").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let public_key = key_data.get("public_key").and_then(|v| v.as_str()).unwrap_or("");
                    let os_name = key_data.get("os_name").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let signature = key_data.get("signature").and_then(|v| v.as_str());

                    if public_key.is_empty() {
                        continue;
                    }

                    // Verify cryptographic signature if available and passkey JSON provided
                    let is_verified = if let (Some(signature_str), Some(passkey_json_str)) = (signature, &passkey_json) {
                        // Parse signature JSON to get full payload
                        match serde_json::from_str::<serde_json::Value>(signature_str) {
                            Ok(signature_payload) => {
                                // Extract credential ID from signature payload
                                let credential_id = signature_payload.get("credential_id")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("unknown");

                                // Use WebAuthn signature verification with JSON Passkey
                                match Self::verify_device_signature_with_json_passkey(&signature_payload, device_id, public_key, passkey_json_str, credential_id).await {
                                    Ok(true) => {
                                        verified_count += 1;
                                        true
                                    }
                                    Ok(false) => {
                                        error!("Signature verification failed for device: {}", device_id);
                                        false
                                    }
                                    Err(e) => {
                                        error!("Error verifying signature for device {}: {}", device_id, e);
                                        false
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to parse signature JSON for device {}: {}", device_id, e);
                                false
                            }
                        }
                    } else {
                        // No signature or no passkey JSON - treat as unverified
                        unverified_count += 1;
                        false
                    };

                    // Include entry based on verification status
                    if is_verified {
                        authorized_entries.push(format!("ssh-ed25519 {} {}@{}", public_key, device_id, os_name));
                    }
                }

                // Write authorized_keys file
                let content = authorized_entries.join("\n") + "\n";
                match tokio::fs::write(&authorized_keys_path, content).await {
                    Ok(_) => {
                        info!(
                            "Authorized keys updated: {} verified, {} unverified, {} total entries written to {:?}",
                            verified_count, unverified_count, authorized_entries.len(), authorized_keys_path
                        );
                    }
                    Err(e) => {
                        error!("Failed to write authorized keys file {:?}: {}", authorized_keys_path, e);
                    }
                }
            }
        });
    }

    /// Verify device signature using JSON Passkey (same as coordinator)
    /// This provides cryptographic verification that the device was authorized by the user
    async fn verify_device_signature_with_json_passkey(
        signature_payload: &serde_json::Value,
        device_id: &str,
        device_public_key: &str,
        passkey_json: &str,
        credential_id: &str
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // Extract WebAuthn verification data from the signature payload
        // This data comes from the coordinator and could be forged
        // We will validate it cryptographically below
        let webauthn_data = match signature_payload.get("webauthn_verification_data") {
            Some(data) => data,
            None => return Err("Missing WebAuthn verification data".into()),
        };

        let client_data_json_b64 = webauthn_data["client_data_json"].as_str()
            .ok_or("Missing client_data_json")?;
        let authenticator_data_b64 = webauthn_data["authenticator_data"].as_str()
            .ok_or("Missing authenticator_data")?;
        let signature_b64 = webauthn_data["signature"].as_str()
            .ok_or("Missing signature")?;

        // Robust base64 decoding function to handle padding issues
        let robust_decode = |input: &str| -> Result<Vec<u8>, Box<dyn std::error::Error>> {
            match general_purpose::STANDARD.decode(input) {
                Ok(bytes) => Ok(bytes),
                Err(_) => {
                    // Try with padding added if initial decode fails
                    let mut padded = input.to_string();
                    while padded.len() % 4 != 0 {
                        padded.push('=');
                    }
                    Ok(general_purpose::STANDARD.decode(&padded)?)
                }
            }
        };

        // Decode the data with robust base64 handling
        let client_data_json = robust_decode(client_data_json_b64)?;
        let authenticator_data = robust_decode(authenticator_data_b64)?;
        let signature = robust_decode(signature_b64)?;

        // SECURITY CRITICAL: Verify what the user actually signed
        // We cannot trust the coordinator's claim about the challenge content

        // 1. Parse the actual client data that was signed
        let client_data_str = String::from_utf8(client_data_json.clone())?;
        let client_data: serde_json::Value = serde_json::from_str(&client_data_str)?;

        // 2. Extract the actual challenge that was signed by the user
        let actual_challenge_b64 = client_data["challenge"].as_str()
            .ok_or("Missing challenge in client data")?;
        // Decode challenge
        let actual_challenge_bytes = robust_decode(actual_challenge_b64)?;
        let actual_challenge_str = String::from_utf8(actual_challenge_bytes)?;

        // 3. Reconstruct what the challenge SHOULD be for this device authorization
        let expected_device_data = format!("SIGN_DEVICE:{}:{}", device_id, device_public_key);
        let expected_device_data_b64 = general_purpose::STANDARD.encode(expected_device_data.as_bytes());

        // 4. Verify the actual challenge contains our expected device authorization
        // Challenge format: <base64_device_data>:<timestamp>
        let challenge_parts: Vec<&str> = actual_challenge_str.split(':').collect();
        if challenge_parts.len() < 2 {
            info!("SECURITY: Invalid challenge format in signature: {}", actual_challenge_str);
            return Ok(false);
        }

        let actual_device_data_b64 = challenge_parts[0];
        if actual_device_data_b64 != expected_device_data_b64 {
            warn!("SECURITY: Challenge device data mismatch!");
            warn!("SECURITY: Expected device data: {}", expected_device_data);

            warn!("SECURITY: This indicates a compromised coordinator or signature reuse attack!");
            return Ok(false);
        }

        // Note: Timestamp is present in challenge but we don't enforce expiration
        // Device authorizations remain valid until explicitly revoked by the user

        // Now perform the cryptographic verification using JSON Passkey
        // 1. Hash the client data JSON
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&client_data_json);
        let client_data_hash = hasher.finalize();

        // 2. Combine authenticator data + client data hash
        let mut signed_data = authenticator_data.clone();
        signed_data.extend_from_slice(&client_data_hash);

        // 3. Convert signature to raw format and verify with p256 crate
        let raw_signature = if signature.len() == 64 {
            signature.to_vec()
        } else if signature.len() > 64 && signature[0] == 0x30 {
            match Self::der_to_raw_ecdsa_signature(&signature) {
                Ok(raw) => raw,
                Err(e) => {
                    info!("Failed to convert DER signature for device {}: {}", device_id, e);
                    return Ok(false);
                }
            }
        } else {
            info!("Unsupported signature format for device {}", device_id);
            return Ok(false);
        };

        // 4. Verify signature using p256 crate directly
        // This step ensures that even if the coordinator provided fake client data,
        // the signature must still be cryptographically valid against the actual signed data
        match Self::verify_webauthn_signature_with_passkey(&raw_signature, &signed_data, passkey_json, credential_id).await {
            Ok(true) => {
                Ok(true)
            }
            Ok(false) => {
                info!("SECURITY: Device {} signature verification FAILED", device_id);
                info!("SECURITY: Signature is cryptographically invalid - possible coordinator forgery");
                Ok(false)
            }
            Err(e) => {
                info!("SECURITY: Device {} signature verification ERROR: {}", device_id, e);
                Ok(false)
            }
        }
    }

    /// Verify WebAuthn signature using the same method as the coordinator
    /// Uses the WebAuthn data to perform proper authentication verification
    async fn verify_webauthn_signature_with_passkey(
        signature: &[u8],
        signed_data: &[u8],
        passkey_json: &str,
        credential_id: &str
    ) -> Result<bool, String> {
        use webauthn_rs::prelude::*;


        // Deserialize the JSON Passkey
        let stored_passkey: Passkey = serde_json::from_str(passkey_json)
            .map_err(|e| format!("Failed to deserialize stored passkey: {:?}", e))?;

        // Verify credential ID matches - handle both URL_SAFE and URL_SAFE_NO_PAD encodings
        let decode_credential_id = |id: &str| -> Result<Vec<u8>, String> {
            // Try URL_SAFE_NO_PAD first (most common for WebAuthn)
            if let Ok(bytes) = general_purpose::URL_SAFE_NO_PAD.decode(id) {
                return Ok(bytes);
            }

            // Try URL_SAFE with padding
            if let Ok(bytes) = general_purpose::URL_SAFE.decode(id) {
                return Ok(bytes);
            }

            // Try standard base64 encodings as fallback
            if let Ok(bytes) = general_purpose::STANDARD_NO_PAD.decode(id) {
                return Ok(bytes);
            }

            if let Ok(bytes) = general_purpose::STANDARD.decode(id) {
                return Ok(bytes);
            }

            Err("Invalid credential ID encoding - tried all base64 variants".to_string())
        };

        let credential_id_bytes = decode_credential_id(credential_id)?;

        if stored_passkey.cred_id().as_ref() != &credential_id_bytes {
            return Err("Credential ID mismatch".to_string());
        }

        // Get the COSE key and perform direct ECDSA verification using p256 crate
        let cose_key = stored_passkey.get_public_key();

        // For ES256 (ECDSA with P-256), extract the key coordinates
        if let COSEKeyType::EC_EC2(ref ec_key) = cose_key.key {
            if ec_key.curve == ECDSACurve::SECP256R1 {
                return Self::verify_ecdsa_p256_signature(&ec_key.x, &ec_key.y, signed_data, signature);
            }
        }

        Err("Unsupported key type or curve".to_string())
    }


    /// Verify ECDSA signature using p256 crate (more reliable than webauthn-rs COSE verification)
    fn verify_ecdsa_p256_signature(
        x_bytes: &[u8],
        y_bytes: &[u8],
        signed_data: &[u8],
        signature: &[u8]
    ) -> Result<bool, String> {
        use p256::ecdsa::{Signature, VerifyingKey, signature::Verifier};
        use p256::{EncodedPoint, PublicKey};
        use p256::elliptic_curve::sec1::FromEncodedPoint;


        // Ensure coordinates are exactly 32 bytes
        if x_bytes.len() != 32 || y_bytes.len() != 32 {
            return Err(format!("Invalid coordinate length: x={}, y={}", x_bytes.len(), y_bytes.len()));
        }

        // Create uncompressed point (0x04 + x + y)
        let mut point_bytes = Vec::with_capacity(65);
        point_bytes.push(0x04); // Uncompressed point indicator
        point_bytes.extend_from_slice(x_bytes);
        point_bytes.extend_from_slice(y_bytes);

        // Create encoded point
        let encoded_point = EncodedPoint::from_bytes(&point_bytes)
            .map_err(|e| format!("Failed to create encoded point: {:?}", e))?;

        // Create public key
        let public_key = PublicKey::from_encoded_point(&encoded_point)
            .into_option()
            .ok_or("Failed to create public key from encoded point")?;

        // Create verifying key
        let verifying_key = VerifyingKey::from(&public_key);

        // Convert signature to raw format if needed and verify
        let raw_signature = if signature.len() == 64 {
            // Already in raw format
            signature.to_vec()
        } else if signature.len() > 64 && signature[0] == 0x30 {
            // Convert from ASN.1 DER to raw format
            match Self::der_to_raw_ecdsa_signature(signature) {
                Ok(raw) => raw,
                Err(e) => {
                    info!("Failed to convert DER signature to raw format: {}", e);
                    return Ok(false);
                }
            }
        } else {
            info!("Unsupported signature format: length={}, first_byte={:02x}", signature.len(), signature.get(0).unwrap_or(&0));
            return Ok(false);
        };

        match Signature::from_bytes(raw_signature.as_slice().into()) {
            Ok(sig) => {
                match verifying_key.verify(signed_data, &sig) {
                    Ok(()) => {
                        return Ok(true);
                    }
                    Err(e) => {
                        warn!("ECDSA signature verification failed: {:?}", e);
                        return Ok(false);
                    }
                }
            }
            Err(e) => {
                info!("Failed to parse signature as raw format: {:?}", e);
                return Ok(false);
            }
        }
    }


    /// Convert ASN.1 DER encoded ECDSA signature to raw format (r || s)
    fn der_to_raw_ecdsa_signature(der_sig: &[u8]) -> Result<Vec<u8>, String> {
        if der_sig.len() < 6 || der_sig[0] != 0x30 {
            return Err("Invalid ASN.1 DER signature format".to_string());
        }

        let mut pos = 2; // Skip SEQUENCE tag and length
        let mut raw_sig = Vec::with_capacity(64);

        // Parse first INTEGER (r value)
        if pos >= der_sig.len() || der_sig[pos] != 0x02 {
            return Err("Expected INTEGER tag for r value".to_string());
        }
        pos += 1;

        let r_len = der_sig[pos] as usize;
        pos += 1;

        if pos + r_len > der_sig.len() {
            return Err("Invalid r value length".to_string());
        }

        // Copy r value, removing leading zero if present (for 33-byte integers)
        let r_start = if r_len == 33 && der_sig[pos] == 0x00 { pos + 1 } else { pos };
        let r_actual_len = if r_len == 33 && der_sig[pos] == 0x00 { 32 } else { r_len };

        // Pad with leading zeros if r is less than 32 bytes
        if r_actual_len < 32 {
            raw_sig.extend(vec![0u8; 32 - r_actual_len]);
        }
        raw_sig.extend_from_slice(&der_sig[r_start..r_start + r_actual_len]);

        pos += r_len;

        // Parse second INTEGER (s value)
        if pos >= der_sig.len() || der_sig[pos] != 0x02 {
            return Err("Expected INTEGER tag for s value".to_string());
        }
        pos += 1;

        let s_len = der_sig[pos] as usize;
        pos += 1;

        if pos + s_len > der_sig.len() {
            return Err("Invalid s value length".to_string());
        }

        // Copy s value, removing leading zero if present (for 33-byte integers)
        let s_start = if s_len == 33 && der_sig[pos] == 0x00 { pos + 1 } else { pos };
        let s_actual_len = if s_len == 33 && der_sig[pos] == 0x00 { 32 } else { s_len };

        // Pad with leading zeros if s is less than 32 bytes
        if s_actual_len < 32 {
            raw_sig.extend(vec![0u8; 32 - s_actual_len]);
        }
        raw_sig.extend_from_slice(&der_sig[s_start..s_start + s_actual_len]);

        if raw_sig.len() != 64 {
            return Err(format!("Invalid raw signature length: {}", raw_sig.len()));
        }

        Ok(raw_sig)
    }

    /// Starts a background task that syncs known hosts (device public keys) with cryptographic verification
    /// This is the client-side equivalent of start_authorized_keys_sync_task
    /// passkey_json: Full JSON-serialized Passkey for signature verification that is stored locally.
    pub async fn start_known_hosts_sync_task(
        &self,
        jwt_token: String,
        passkey_json: Option<String>,
        sync_interval_secs: u64,
        known_hosts_path: std::path::PathBuf,
    ) {
        let http_client = self.http_client.clone();
        let coordinator_url = self.coordinator_url.clone();


        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(sync_interval_secs));

            loop {
                interval.tick().await;

                // Construct authorized keys URL (servers share same endpoint)
                let authorized_keys_url = match coordinator_url.join("/api/authorized-keys") {
                    Ok(url) => url,
                    Err(e) => {
                        error!("Failed to construct authorized keys URL: {}", e);
                        continue;
                    }
                };

                // Prepare request - account_id is extracted from JWT token
                let request_body = serde_json::json!({});

                // Make API request
                let response = match http_client
                    .post(authorized_keys_url)
                    .header("Authorization", format!("Bearer {}", jwt_token))
                    .json(&request_body)
                    .send()
                    .await
                {
                    Ok(response) => response,
                    Err(e) => {
                        error!("Failed to fetch server keys for known hosts: {}", e);
                        continue;
                    }
                };

                // Parse response
                let authorized_keys_response: serde_json::Value = match response.json().await {
                    Ok(data) => data,
                    Err(e) => {
                        error!("Failed to parse server keys response: {}", e);
                        continue;
                    }
                };

                let keys = match authorized_keys_response.get("keys").and_then(|k| k.as_array()) {
                    Some(keys) => keys,
                    None => {
                        error!("Invalid server keys response format");
                        continue;
                    }
                };

                // Process and verify server keys for known_hosts
                let mut known_hosts_entries = Vec::new();
                let mut verified_count = 0;
                let mut unverified_count = 0;

                for key_data in keys {
                    let device_id = key_data.get("device_id").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let public_key = key_data.get("public_key").and_then(|v| v.as_str()).unwrap_or("");
                    let os_name = key_data.get("os_name").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let signature = key_data.get("signature").and_then(|v| v.as_str());

                    if public_key.is_empty() {
                        continue;
                    }

                    // Verify cryptographic signature if available and passkey JSON provided
                    let is_verified = if let (Some(signature_str), Some(passkey_json_str)) = (signature, &passkey_json) {
                        // Parse signature JSON to get full payload
                        match serde_json::from_str::<serde_json::Value>(signature_str) {
                            Ok(signature_payload) => {
                                // Extract credential ID from signature payload
                                let credential_id = signature_payload.get("credential_id")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("unknown");

                                // Use WebAuthn signature verification with JSON Passkey
                                match Self::verify_device_signature_with_json_passkey(&signature_payload, device_id, public_key, passkey_json_str, credential_id).await {
                                    Ok(true) => {
                                        verified_count += 1;
                                        true
                                    }
                                    Ok(false) => {
                                        error!("Signature verification failed for server: {}", device_id);
                                        false
                                    }
                                    Err(e) => {
                                        error!("Error verifying signature for server {}: {}", device_id, e);
                                        false
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to parse signature JSON for server {}: {}", device_id, e);
                                false
                            }
                        }
                    } else {
                        // No signature or no passkey JSON - treat as unverified
                        unverified_count += 1;
                        false
                    };

                    // Only include verified server entries in known_hosts
                    if is_verified {
                        // Using device_id as hostname for consistent identification
                        known_hosts_entries.push(format!("ssh-ed25519 {} {}@{}", public_key, device_id, os_name));
                    }
                }

                // Write known_hosts file
                let content = known_hosts_entries.join("\n") + "\n";
                match tokio::fs::write(&known_hosts_path, content).await {
                    Ok(_) => {
                        info!(
                            "Known hosts updated: {} verified devices, {} unverified, {} total entries written to {:?}",
                            verified_count, unverified_count, known_hosts_entries.len(), known_hosts_path
                        );
                    }
                    Err(e) => {
                        error!("Failed to write known hosts file {:?}: {}", known_hosts_path, e);
                    }
                }
            }
        });
    }

}
