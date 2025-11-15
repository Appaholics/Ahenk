//! Integration tests for ahenk database synchronization infrastructure.
//!
//! These tests verify the core CRUD operations for:
//! - Users (authentication)
//! - Devices (device management)
//! - OplogEntry (CRDT operation log)
//! - Peer (P2P peer tracking)

use ahenk::db::operations;
use ahenk::models::{Device, OplogEntry, Peer, User};
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_user_crud_operations() {
    // Create a temporary database
    let conn =
        operations::initialize_database(":memory:").expect("Failed to create in-memory database");

    // Create test user
    let user_id = Uuid::new_v4();
    let user = User {
        user_id,
        user_name: "testuser".to_string(),
        user_password_hash: "hashed_password_123".to_string(),
        user_mail: "test@example.com".to_string(),
        created_at: Utc::now(),
    };

    // Test CREATE
    operations::create_user(&conn, &user).expect("Failed to create user");

    // Test READ by user_id
    let retrieved_user = operations::get_user(&conn, user_id)
        .expect("Failed to get user")
        .expect("User not found");

    assert_eq!(retrieved_user.user_id, user_id);
    assert_eq!(retrieved_user.user_name, "testuser");
    assert_eq!(retrieved_user.user_mail, "test@example.com");
    assert_eq!(retrieved_user.user_password_hash, "hashed_password_123");

    // Test READ by username
    let retrieved_by_name = operations::get_user_by_name(&conn, "testuser")
        .expect("Failed to get user by name")
        .expect("User not found by name");
    assert_eq!(retrieved_by_name.user_id, user_id);

    // Test READ by email
    let retrieved_by_mail = operations::get_user_by_mail(&conn, "test@example.com")
        .expect("Failed to get user by email")
        .expect("User not found by email");
    assert_eq!(retrieved_by_mail.user_id, user_id);
}

#[test]
fn test_device_crud_operations() {
    // Create a temporary database
    let conn =
        operations::initialize_database(":memory:").expect("Failed to create in-memory database");

    // Create test user first (devices belong to users)
    let user_id = Uuid::new_v4();
    let user = User {
        user_id,
        user_name: "deviceuser".to_string(),
        user_password_hash: "hashed_password".to_string(),
        user_mail: "device@example.com".to_string(),
        created_at: Utc::now(),
    };
    operations::create_user(&conn, &user).expect("Failed to create user");

    // Create test device
    let device_id = Uuid::new_v4();
    let device = Device {
        device_id,
        user_id,
        device_type: "mobile".to_string(),
        push_token: Some("test_token_123".to_string()),
        last_seen: None,
    };

    // Test CREATE
    operations::create_device(&conn, &device).expect("Failed to create device");

    // Test READ by device_id
    let retrieved_device = operations::get_device(&conn, device_id)
        .expect("Failed to get device")
        .expect("Device not found");

    assert_eq!(retrieved_device.device_id, device_id);
    assert_eq!(retrieved_device.user_id, user_id);
    assert_eq!(retrieved_device.device_type, "mobile");
    assert_eq!(
        retrieved_device.push_token,
        Some("test_token_123".to_string())
    );

    // Test READ by user_id
    let devices =
        operations::get_devices_by_user_id(&conn, user_id).expect("Failed to get devices by user");

    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].device_id, device_id);

    // Test UPDATE last_seen
    let now = Utc::now();
    operations::update_device_last_seen(&conn, device_id, now)
        .expect("Failed to update device last_seen");

    let updated_device = operations::get_device(&conn, device_id)
        .expect("Failed to get updated device")
        .expect("Updated device not found");

    assert!(updated_device.last_seen.is_some());
}

#[test]
fn test_oplog_and_peer_crud_operations() {
    let conn =
        operations::initialize_database(":memory:").expect("Failed to create in-memory database");

    // Create test user
    let user_id = Uuid::new_v4();
    let user = User {
        user_id,
        user_name: "sync_user".to_string(),
        user_password_hash: "sync_password".to_string(),
        user_mail: "sync@example.com".to_string(),
        created_at: Utc::now(),
    };
    operations::create_user(&conn, &user).expect("Failed to create user");

    // Create test device
    let device_id = Uuid::new_v4();
    let device = Device {
        device_id,
        user_id,
        device_type: "laptop".to_string(),
        push_token: None,
        last_seen: None,
    };
    operations::create_device(&conn, &device).expect("Failed to create device");

    // Test OplogEntry CREATE
    let oplog_entry = OplogEntry {
        id: Uuid::new_v4(),
        device_id,
        timestamp: chrono::Utc::now().timestamp_millis(),
        table: "app_data".to_string(),
        op_type: "create".to_string(),
        data: serde_json::json!({
            "content": "Sample app data",
            "record_id": Uuid::new_v4().to_string(),
        }),
    };
    operations::create_oplog_entry(&conn, &oplog_entry).expect("Failed to create oplog entry");

    // Test OplogEntry READ
    let entries =
        operations::get_oplog_entries_since(&conn, chrono::Utc::now().timestamp_millis() - 60000)
            .expect("Failed to get oplog entries");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].table, "app_data");
    assert_eq!(entries[0].op_type, "create");

    // Test Peer CREATE
    let peer = Peer {
        peer_id: Uuid::new_v4(),
        user_id,
        device_id,
        last_known_ip: Some("192.168.1.100".to_string()),
        last_sync_time: Some(Utc::now().timestamp_millis()),
    };
    operations::create_peer(&conn, &peer).expect("Failed to create peer");

    // Test Peer READ by user_id
    let peers =
        operations::get_peers_by_user_id(&conn, user_id).expect("Failed to get peers by user");
    assert_eq!(peers.len(), 1);
    assert_eq!(peers[0].last_known_ip, Some("192.168.1.100".to_string()));

    // Test Peer READ all
    let all_peers = operations::get_all_peers(&conn).expect("Failed to get all peers");
    assert_eq!(all_peers.len(), 1);
}

#[test]
fn test_multiple_devices_per_user() {
    let conn =
        operations::initialize_database(":memory:").expect("Failed to create in-memory database");

    // Create user
    let user_id = Uuid::new_v4();
    let user = User {
        user_id,
        user_name: "multidevice_user".to_string(),
        user_password_hash: "password".to_string(),
        user_mail: "multi@example.com".to_string(),
        created_at: Utc::now(),
    };
    operations::create_user(&conn, &user).expect("Failed to create user");

    // Create multiple devices
    let device1 = Device {
        device_id: Uuid::new_v4(),
        user_id,
        device_type: "ios".to_string(),
        push_token: Some("ios_token".to_string()),
        last_seen: Some(Utc::now()),
    };
    let device2 = Device {
        device_id: Uuid::new_v4(),
        user_id,
        device_type: "android".to_string(),
        push_token: Some("android_token".to_string()),
        last_seen: Some(Utc::now()),
    };
    let device3 = Device {
        device_id: Uuid::new_v4(),
        user_id,
        device_type: "desktop".to_string(),
        push_token: None,
        last_seen: Some(Utc::now()),
    };

    operations::create_device(&conn, &device1).expect("Failed to create device 1");
    operations::create_device(&conn, &device2).expect("Failed to create device 2");
    operations::create_device(&conn, &device3).expect("Failed to create device 3");

    // Verify all devices are retrieved
    let devices =
        operations::get_devices_by_user_id(&conn, user_id).expect("Failed to get devices");
    assert_eq!(devices.len(), 3);

    // Verify device types
    let device_types: Vec<String> = devices.iter().map(|d| d.device_type.clone()).collect();
    assert!(device_types.contains(&"ios".to_string()));
    assert!(device_types.contains(&"android".to_string()));
    assert!(device_types.contains(&"desktop".to_string()));
}

#[test]
fn test_oplog_ordering_by_timestamp() {
    let conn =
        operations::initialize_database(":memory:").expect("Failed to create in-memory database");

    // Create user and device
    let user_id = Uuid::new_v4();
    let user = User {
        user_id,
        user_name: "order_user".to_string(),
        user_password_hash: "password".to_string(),
        user_mail: "order@example.com".to_string(),
        created_at: Utc::now(),
    };
    operations::create_user(&conn, &user).expect("Failed to create user");

    let device_id = Uuid::new_v4();
    let device = Device {
        device_id,
        user_id,
        device_type: "test".to_string(),
        push_token: None,
        last_seen: None,
    };
    operations::create_device(&conn, &device).expect("Failed to create device");

    // Create multiple oplog entries with different timestamps
    let base_timestamp = chrono::Utc::now().timestamp_millis();

    for i in 0..5 {
        let entry = OplogEntry {
            id: Uuid::new_v4(),
            device_id,
            timestamp: base_timestamp + (i * 1000),
            table: "test_table".to_string(),
            op_type: "create".to_string(),
            data: serde_json::json!({"index": i}),
        };
        operations::create_oplog_entry(&conn, &entry).expect("Failed to create oplog entry");
    }

    // Retrieve entries since beginning
    let entries = operations::get_oplog_entries_since(&conn, base_timestamp - 1000)
        .expect("Failed to get entries");

    assert_eq!(entries.len(), 5);

    // Verify entries are ordered by timestamp (ascending)
    for i in 0..4 {
        assert!(entries[i].timestamp < entries[i + 1].timestamp);
    }
}
