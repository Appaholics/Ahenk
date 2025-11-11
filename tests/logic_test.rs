//! Logic layer tests for nexus-core synchronization infrastructure.
//!
//! These tests verify the business logic for:
//! - User authentication (registration, login)
//! - Device management
//! - P2P sync messages
//! - Peer management

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;
use nexus_core::db::operations;
use nexus_core::logic;
use nexus_core::models::{Device, User};
use rusqlite::Connection;
use uuid::Uuid;

// ============================================================================
// Test Helper Functions
// ============================================================================

fn setup_empty_db() -> Connection {
    operations::initialize_database(":memory:").expect("Failed to create in-memory database")
}

fn setup_db_with_user_and_device() -> (Connection, Uuid, Uuid) {
    let conn =
        operations::initialize_database(":memory:").expect("Failed to create in-memory database");
    let user_id = Uuid::new_v4();
    let user = User {
        user_id,
        user_name: "testuser".to_string(),
        user_password_hash: "password".to_string(),
        user_mail: "test@example.com".to_string(),
        created_at: Utc::now(),
    };
    operations::create_user(&conn, &user).expect("Failed to create user");

    let device_id = Uuid::new_v4();
    let device = Device {
        device_id,
        user_id,
        device_type: "test_device".to_string(),
        push_token: None,
        last_seen: None,
    };
    operations::create_device(&conn, &device).expect("Failed to create device");

    (conn, user_id, device_id)
}

// ============================================================================
// User Management Tests
// ============================================================================

#[test]
fn test_register_user_hashes_password_and_normalizes_fields() {
    let conn = setup_empty_db();
    let password = "SecretPass123!".to_string();

    let user = logic::register_user(
        &conn,
        "  TestUser  ".to_string(),
        "USER@Example.COM".to_string(),
        password.clone(),
    )
    .expect("User registration failed");

    // Verify field normalization
    assert_eq!(user.user_name, "TestUser");
    assert_eq!(user.user_mail, "user@example.com");

    // Verify password was hashed (not stored in plaintext)
    assert_ne!(user.user_password_hash, password);

    // Verify user was persisted to database
    let stored = operations::get_user_by_name(&conn, "TestUser")
        .expect("Query failed")
        .expect("User not found after registration");
    assert_eq!(stored.user_id, user.user_id);

    // Verify password hash is valid Argon2
    let hash = PasswordHash::new(&stored.user_password_hash).expect("Invalid stored hash");
    Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .expect("Password verification failed");
}

#[test]
fn test_register_user_rejects_duplicate_username_and_email() {
    let conn = setup_empty_db();

    // Register first user
    logic::register_user(
        &conn,
        "testuser".to_string(),
        "user@example.com".to_string(),
        "SecretPass123!".to_string(),
    )
    .expect("Initial registration failed");

    // Attempt to register with duplicate username
    let username_err = logic::register_user(
        &conn,
        "testuser".to_string(),
        "other@example.com".to_string(),
        "AnotherPass!".to_string(),
    )
    .expect_err("Expected duplicate username to fail");
    assert!(username_err.contains("Username"));

    // Attempt to register with duplicate email
    let email_err = logic::register_user(
        &conn,
        "otheruser".to_string(),
        "user@example.com".to_string(),
        "AnotherPass!".to_string(),
    )
    .expect_err("Expected duplicate email to fail");
    assert!(email_err.contains("Email"));
}

#[test]
fn test_login_user_by_username_and_email() {
    let conn = setup_empty_db();
    let password = "SecretPass123!";

    // Register user
    logic::register_user(
        &conn,
        "testuser".to_string(),
        "test@example.com".to_string(),
        password.to_string(),
    )
    .expect("Registration failed");

    // Test login by username
    let by_username =
        logic::login_user(&conn, "testuser", password).expect("Login by username failed");
    assert_eq!(by_username.user_name, "testuser");

    // Test login by email
    let by_email =
        logic::login_user(&conn, "test@example.com", password).expect("Login by email failed");
    assert_eq!(by_email.user_id, by_username.user_id);

    // Test login with wrong password
    let wrong_password_err =
        logic::login_user(&conn, "testuser", "wrong").expect_err("Expected invalid password");
    assert!(wrong_password_err.contains("Invalid credentials"));
}

#[test]
fn test_login_user_timing_safe_verification() {
    let conn = setup_empty_db();

    // Register user
    logic::register_user(
        &conn,
        "secureuser".to_string(),
        "secure@example.com".to_string(),
        "CorrectPassword123!".to_string(),
    )
    .expect("Registration failed");

    // Test with non-existent user (should not leak existence)
    let nonexistent_err = logic::login_user(&conn, "nonexistent", "password")
        .expect_err("Expected nonexistent user to fail");
    assert!(nonexistent_err.contains("Invalid credentials"));

    // Test with existing user but wrong password (same error message)
    let wrong_pass_err = logic::login_user(&conn, "secureuser", "WrongPassword")
        .expect_err("Expected wrong password to fail");
    assert!(wrong_pass_err.contains("Invalid credentials"));
}

// ============================================================================
// Device Management Tests
// ============================================================================

#[test]
fn test_add_device_to_user_and_get_user_devices() {
    let conn = setup_empty_db();

    // Register user
    let user = logic::register_user(
        &conn,
        "devicetester".to_string(),
        "devicetester@example.com".to_string(),
        "SecretPass123!".to_string(),
    )
    .expect("Registration failed");

    // Add device with field normalization
    let device = logic::add_device_to_user(
        &conn,
        user.user_id,
        "  mobile  ".to_string(),
        Some("  token123  ".to_string()),
    )
    .expect("Adding device failed");

    assert_eq!(device.user_id, user.user_id);
    assert_eq!(device.device_type, "mobile");
    assert_eq!(device.push_token, Some("token123".to_string()));
    assert!(device.last_seen.is_some());

    // Verify device can be retrieved
    let devices =
        logic::get_user_devices(&conn, user.user_id).expect("Fetching devices failed");
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].device_id, device.device_id);

    // Test validation: empty device type should fail
    let empty_type_err =
        logic::add_device_to_user(&conn, user.user_id, "   ".to_string(), None)
            .expect_err("Expected empty device type to fail");
    assert!(empty_type_err.contains("Device type"));
}

#[test]
fn test_add_device_to_nonexistent_user() {
    let conn = setup_empty_db();
    let nonexistent_user_id = Uuid::new_v4();

    let err = logic::add_device_to_user(
        &conn,
        nonexistent_user_id,
        "mobile".to_string(),
        None,
    )
    .expect_err("Expected adding device to nonexistent user to fail");

    assert!(err.contains("User not found"));
}

// ============================================================================
// P2P Sync Tests
// ============================================================================

#[test]
fn test_generate_device_id() {
    let (peer_id, _keypair) = logic::sync::generate_device_id();
    // Verify it's a valid libp2p peer ID (starts with "12D3")
    assert!(peer_id.to_string().starts_with("12D"));
}

#[test]
fn test_sync_message_encode_decode() {
    use nexus_core::logic::sync::{SyncMessage, decode_sync_message, encode_sync_message};

    let user_id = Uuid::new_v4();
    let device_id = Uuid::new_v4();

    // Test Announce message
    let announce_msg = SyncMessage::Announce {
        user_id,
        device_id,
        peer_id: "test_peer_id".to_string(),
    };

    let encoded = encode_sync_message(&announce_msg).unwrap();
    let decoded = decode_sync_message(&encoded).unwrap();

    match decoded {
        SyncMessage::Announce {
            user_id: uid,
            device_id: did,
            peer_id,
        } => {
            assert_eq!(uid, user_id);
            assert_eq!(did, device_id);
            assert_eq!(peer_id, "test_peer_id");
        }
        _ => panic!("Expected Announce message"),
    }

    // Test RequestSync message
    let timestamp = Utc::now().timestamp();
    let request_msg = SyncMessage::RequestSync {
        user_id,
        since_timestamp: timestamp,
    };

    let encoded = encode_sync_message(&request_msg).unwrap();
    let decoded = decode_sync_message(&encoded).unwrap();

    match decoded {
        SyncMessage::RequestSync {
            user_id: uid,
            since_timestamp: ts,
        } => {
            assert_eq!(uid, user_id);
            // Allow for small timestamp differences due to serialization
            assert!((ts - timestamp).abs() < 2);
        }
        _ => panic!("Expected RequestSync message"),
    }
}

#[test]
fn test_update_peer_info() {
    use nexus_core::logic::sync::update_peer_info;

    let (conn, user_id, _device_id) = setup_db_with_user_and_device();

    // Create a peer device
    let peer_device_id = Uuid::new_v4();
    let peer_peer_id = "peer_123".to_string();
    let peer_ip = Some("192.168.1.100".to_string());

    // Update peer info (should create a new peer)
    update_peer_info(
        &conn,
        user_id,
        peer_device_id,
        peer_peer_id.clone(),
        peer_ip.clone(),
    )
    .unwrap();

    // Verify peer was created
    let peers = operations::get_peers_by_user_id(&conn, user_id).unwrap();
    assert_eq!(peers.len(), 1);
    assert_eq!(peers[0].device_id, peer_device_id);
    assert_eq!(peers[0].last_known_ip, peer_ip);
    assert!(peers[0].last_sync_time.is_some());
}

// ============================================================================
// P2P CRDT Sync Tests (Ignored - Need Refactoring for Generic Data)
//
// These tests were written for app-specific data (tasks) and need to be
// refactored to work with generic app data. They test important CRDT
// functionality but require app-level integration.
//
// TODO: Refactor these tests to use generic app data instead of tasks
// ============================================================================

/*
#[test]
#[ignore]
fn test_sync_with_peer() {
    // NOTE: This test needs to be refactored for generic app data.
    // Original test used task lists and tasks which are no longer in core.
    // Apps should implement their own sync tests using their data types.

    use nexus_core::crdt;

    // Setup two "devices" with separate databases but same user
    let (mut conn1, user_id, device_id1) = setup_db_with_user_and_device();

    // Create a second device database for the same user
    let mut conn2 =
        operations::initialize_database(":memory:").expect("Failed to create in-memory database");
    // ... rest of test omitted (see git history for original)
}

#[test]
#[ignore]
fn test_bidirectional_sync() {
    // NOTE: This test needs to be refactored for generic app data.
    // Apps should implement bidirectional sync tests with their own data types.
    // ... test omitted (see git history for original)
}

#[test]
#[ignore]
fn test_sync_prevents_duplicates() {
    // NOTE: This test needs to be refactored for generic app data.
    // The CRDT merge functionality should prevent duplicates,
    // but this test needs to use app-specific data to verify.
    // ... test omitted (see git history for original)
}
*/
