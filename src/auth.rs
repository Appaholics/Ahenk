/// Device Authorization Handshake Module
///
/// Implements secure device pairing using QR codes and cryptographic challenges.
/// This allows users to add new devices to their account by scanning a QR code
/// from an already authorized device.
use crate::models::Device;
use chrono::{DateTime, Duration, Utc};
use libp2p::identity::Keypair;
use libp2p::PeerId;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Authorization challenge that gets encoded in a QR code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthChallenge {
    /// Unique challenge ID
    pub challenge_id: Uuid,
    /// User ID that created this challenge
    pub user_id: Uuid,
    /// Device ID that created this challenge (the "authorizer")
    pub authorizer_device_id: Uuid,
    /// Peer ID of the authorizer for P2P connection
    pub authorizer_peer_id: String,
    /// IP address/multiaddr of authorizer
    pub authorizer_address: String,
    /// Cryptographic nonce for challenge-response
    pub nonce: String,
    /// Public key of the authorizer
    pub public_key: Vec<u8>,
    /// Timestamp when challenge was created
    pub created_at: DateTime<Utc>,
    /// Challenge expires after this time
    pub expires_at: DateTime<Utc>,
}

/// Response from a device attempting to pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    /// The challenge this is responding to
    pub challenge_id: Uuid,
    /// Device requesting authorization
    pub requesting_device_id: Uuid,
    /// Peer ID of requesting device
    pub requesting_peer_id: String,
    /// Device type (phone, tablet, desktop, watch)
    pub device_type: String,
    /// Device name chosen by user
    pub device_name: String,
    /// Signed nonce as proof of receipt
    pub signed_nonce: Vec<u8>,
    /// Public key of requesting device
    pub public_key: Vec<u8>,
}

/// Result of authorization attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthResult {
    /// Authorization successful
    Success {
        device_id: Uuid,
        user_id: Uuid,
        sync_key: Vec<u8>,
    },
    /// Authorization failed
    Failed { reason: String },
    /// Challenge expired
    Expired,
    /// Invalid signature
    InvalidSignature,
}

/// Pairing session state
#[derive(Debug, Clone)]
pub struct PairingSession {
    /// The challenge for this session
    pub challenge: AuthChallenge,
    /// Whether challenge has been consumed
    pub consumed: bool,
    /// When the session was created
    pub created_at: DateTime<Utc>,
}

/// Device authorization manager
pub struct DeviceAuthManager {
    /// Active pairing sessions (challenge_id -> session)
    sessions: std::collections::HashMap<Uuid, PairingSession>,
}

impl DeviceAuthManager {
    /// Create a new device authorization manager
    pub fn new() -> Self {
        Self {
            sessions: std::collections::HashMap::new(),
        }
    }

    /// Generate a new authorization challenge
    ///
    /// This creates a QR code payload that can be scanned by a new device
    pub fn create_challenge(
        &mut self,
        user_id: Uuid,
        authorizer_device_id: Uuid,
        authorizer_keypair: &Keypair,
        authorizer_address: String,
        validity_minutes: i64,
    ) -> Result<AuthChallenge, String> {
        let challenge_id = Uuid::new_v4();
        let nonce = generate_nonce();
        let created_at = Utc::now();
        let expires_at = created_at + Duration::minutes(validity_minutes);

        let peer_id = PeerId::from(authorizer_keypair.public());
        let public_key = authorizer_keypair.public().encode_protobuf();

        let challenge = AuthChallenge {
            challenge_id,
            user_id,
            authorizer_device_id,
            authorizer_peer_id: peer_id.to_string(),
            authorizer_address,
            nonce,
            public_key,
            created_at,
            expires_at,
        };

        // Store the session
        let session = PairingSession {
            challenge: challenge.clone(),
            consumed: false,
            created_at,
        };

        self.sessions.insert(challenge_id, session);

        Ok(challenge)
    }

    /// Encode challenge to QR code string (JSON)
    pub fn encode_challenge_to_qr(challenge: &AuthChallenge) -> Result<String, String> {
        serde_json::to_string(challenge).map_err(|e| format!("Failed to encode: {}", e))
    }

    /// Decode challenge from QR code string
    pub fn decode_challenge_from_qr(qr_data: &str) -> Result<AuthChallenge, String> {
        serde_json::from_str(qr_data).map_err(|e| format!("Failed to decode: {}", e))
    }

    /// Validate and consume an authorization response
    pub fn validate_response(
        &mut self,
        conn: &Connection,
        response: &AuthResponse,
        _requesting_keypair: &Keypair,
    ) -> Result<AuthResult, String> {
        // Get the session
        let session = self
            .sessions
            .get_mut(&response.challenge_id)
            .ok_or("Challenge not found")?;

        // Check if already consumed
        if session.consumed {
            return Ok(AuthResult::Failed {
                reason: "Challenge already used".to_string(),
            });
        }

        // Check expiration
        if Utc::now() > session.challenge.expires_at {
            return Ok(AuthResult::Expired);
        }

        // Verify signature
        if !verify_signature(
            &session.challenge.nonce,
            &response.signed_nonce,
            &response.public_key,
        ) {
            return Ok(AuthResult::InvalidSignature);
        }

        // Mark as consumed
        session.consumed = true;

        // Create device in database
        let device = Device {
            device_id: response.requesting_device_id,
            user_id: session.challenge.user_id,
            device_type: response.device_type.clone(),
            push_token: None,
            last_seen: Some(Utc::now()),
        };

        crate::db::operations::create_device(conn, &device)
            .map_err(|e| format!("Failed to create device: {}", e))?;

        // Generate sync key (shared secret for this device)
        let sync_key = generate_sync_key(&session.challenge.public_key, &response.public_key);

        Ok(AuthResult::Success {
            device_id: response.requesting_device_id,
            user_id: session.challenge.user_id,
            sync_key,
        })
    }

    /// Clean up expired sessions
    pub fn cleanup_expired(&mut self) {
        let now = Utc::now();
        self.sessions
            .retain(|_, session| now < session.challenge.expires_at);
    }

    /// Get active session count
    pub fn active_session_count(&self) -> usize {
        self.sessions.len()
    }

    /// Cancel a challenge
    pub fn cancel_challenge(&mut self, challenge_id: Uuid) -> bool {
        self.sessions.remove(&challenge_id).is_some()
    }
}

impl Default for DeviceAuthManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate a cryptographic nonce
fn generate_nonce() -> String {
    use argon2::password_hash::rand_core::OsRng;
    use argon2::password_hash::rand_core::RngCore;

    let mut nonce = [0u8; 32];
    OsRng.fill_bytes(&mut nonce);
    hex::encode(nonce)
}

/// Verify a signature using Ed25519 via libp2p
fn verify_signature(nonce: &str, signature: &[u8], public_key: &[u8]) -> bool {
    use libp2p::identity::PublicKey;

    match PublicKey::try_decode_protobuf(public_key) {
        Ok(pk) => {
            // Verify the signature using the libp2p keypair
            pk.verify(nonce.as_bytes(), signature)
        }
        Err(_) => false,
    }
}

/// Generate a shared sync key from two public keys
fn generate_sync_key(public_key1: &[u8], public_key2: &[u8]) -> Vec<u8> {
    use argon2::password_hash::rand_core::OsRng;
    use argon2::password_hash::rand_core::RngCore;

    // In production, use proper key exchange (e.g., ECDH)
    // For now, generate a random key
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    // Mix in the public keys for determinism
    for (i, byte) in public_key1.iter().take(16).enumerate() {
        key[i] ^= byte;
    }
    for (i, byte) in public_key2.iter().take(16).enumerate() {
        key[i + 16] ^= byte;
    }

    key.to_vec()
}

/// Create an authorization response from a scanned challenge
pub fn create_auth_response(
    challenge: &AuthChallenge,
    device_type: String,
    device_name: String,
    keypair: &Keypair,
) -> Result<AuthResponse, String> {
    let requesting_device_id = Uuid::new_v4();
    let peer_id = PeerId::from(keypair.public());
    let public_key = keypair.public().encode_protobuf();

    // Sign the nonce
    let signed_nonce = keypair
        .sign(challenge.nonce.as_bytes())
        .map_err(|e| format!("Failed to sign nonce: {}", e))?;

    Ok(AuthResponse {
        challenge_id: challenge.challenge_id,
        requesting_device_id,
        requesting_peer_id: peer_id.to_string(),
        device_type,
        device_name,
        signed_nonce,
        public_key,
    })
}

/// Complete workflow: Authorizer side
pub struct AuthorizerWorkflow {
    manager: DeviceAuthManager,
}

impl AuthorizerWorkflow {
    pub fn new() -> Self {
        Self {
            manager: DeviceAuthManager::new(),
        }
    }

    /// Step 1: Generate QR code data
    pub fn generate_qr_code(
        &mut self,
        user_id: Uuid,
        device_id: Uuid,
        keypair: &Keypair,
        address: String,
    ) -> Result<String, String> {
        let challenge = self.manager.create_challenge(
            user_id, device_id, keypair, address, 5, // 5 minute validity
        )?;

        DeviceAuthManager::encode_challenge_to_qr(&challenge)
    }

    /// Step 2: Process response from new device
    pub fn authorize_device(
        &mut self,
        conn: &Connection,
        response: &AuthResponse,
        requesting_keypair: &Keypair,
    ) -> Result<AuthResult, String> {
        self.manager
            .validate_response(conn, response, requesting_keypair)
    }

    /// Clean up old challenges
    pub fn cleanup(&mut self) {
        self.manager.cleanup_expired();
    }

    /// Get the number of active pairing sessions
    pub fn active_session_count(&self) -> usize {
        self.manager.active_session_count()
    }
}

impl Default for AuthorizerWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete workflow: New device side
pub struct NewDeviceWorkflow;

impl NewDeviceWorkflow {
    /// Step 1: Scan QR code and parse challenge
    pub fn scan_qr_code(qr_data: &str) -> Result<AuthChallenge, String> {
        DeviceAuthManager::decode_challenge_from_qr(qr_data)
    }

    /// Step 2: Create response to send to authorizer
    pub fn create_pairing_request(
        challenge: &AuthChallenge,
        device_type: String,
        device_name: String,
        keypair: &Keypair,
    ) -> Result<AuthResponse, String> {
        create_auth_response(challenge, device_type, device_name, keypair)
    }

    /// Step 3: Connect to authorizer via P2P
    pub fn connect_to_authorizer(challenge: &AuthChallenge) -> Result<String, String> {
        // Return the multiaddr to connect to
        Ok(format!(
            "{}/p2p/{}",
            challenge.authorizer_address, challenge.authorizer_peer_id
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity;

    #[test]
    fn test_generate_nonce() {
        let nonce = generate_nonce();
        assert_eq!(nonce.len(), 64); // 32 bytes = 64 hex chars
    }

    #[test]
    fn test_create_and_encode_challenge() {
        let mut manager = DeviceAuthManager::new();
        let keypair = identity::Keypair::generate_ed25519();
        let user_id = Uuid::new_v4();
        let device_id = Uuid::new_v4();

        let challenge = manager
            .create_challenge(
                user_id,
                device_id,
                &keypair,
                "/ip4/192.168.1.100/tcp/4001".to_string(),
                5,
            )
            .unwrap();

        let encoded = DeviceAuthManager::encode_challenge_to_qr(&challenge).unwrap();
        let decoded = DeviceAuthManager::decode_challenge_from_qr(&encoded).unwrap();

        assert_eq!(challenge.challenge_id, decoded.challenge_id);
        assert_eq!(challenge.user_id, decoded.user_id);
    }

    #[test]
    fn test_auth_workflow() {
        let authorizer_keypair = identity::Keypair::generate_ed25519();
        let requester_keypair = identity::Keypair::generate_ed25519();

        let mut workflow = AuthorizerWorkflow::new();
        let user_id = Uuid::new_v4();
        let device_id = Uuid::new_v4();

        // Generate QR
        let qr_data = workflow
            .generate_qr_code(
                user_id,
                device_id,
                &authorizer_keypair,
                "/ip4/127.0.0.1/tcp/4001".to_string(),
            )
            .unwrap();

        // Scan QR on new device
        let challenge = NewDeviceWorkflow::scan_qr_code(&qr_data).unwrap();

        // Create response
        let response = NewDeviceWorkflow::create_pairing_request(
            &challenge,
            "phone".to_string(),
            "My Phone".to_string(),
            &requester_keypair,
        )
        .unwrap();

        assert_eq!(response.challenge_id, challenge.challenge_id);
        assert_eq!(response.device_type, "phone");
    }

    #[test]
    fn test_session_cleanup() {
        let mut manager = DeviceAuthManager::new();
        let keypair = identity::Keypair::generate_ed25519();

        // Create challenge with -1 minute validity (already expired)
        let mut challenge = manager
            .create_challenge(
                Uuid::new_v4(),
                Uuid::new_v4(),
                &keypair,
                "/ip4/127.0.0.1/tcp/4001".to_string(),
                -1,
            )
            .unwrap();

        assert_eq!(manager.active_session_count(), 1);

        manager.cleanup_expired();

        assert_eq!(manager.active_session_count(), 0);
    }
}
