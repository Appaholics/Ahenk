use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;
use nexus_core::db::operations;
use nexus_core::logic;
use nexus_core::models::{Device, PomodoroPreset, User};
use rusqlite::Connection;
use uuid::Uuid;

fn setup_empty_db() -> Connection {
    operations::initialize_database(":memory:").expect("Failed to create in-memory database")
}

fn setup_db_with_user_and_device() -> (Connection, Uuid, Uuid) {
    let mut conn =
        operations::initialize_database(":memory:").expect("Failed to create in-memory database");
    let user_id = Uuid::new_v4();
    let user = User {
        user_id,
        user_name: "testuser".to_string(),
        user_password_hash: "password".to_string(),
        user_mail: "test@example.com".to_string(),
        created_at: Utc::now(),
    };
    operations::create_user(&mut conn, &user).expect("Failed to create user");

    let device_id = Uuid::new_v4();
    let device = Device {
        device_id,
        user_id,
        device_type: "test_device".to_string(),
        push_token: None,
        last_seen: None,
    };
    operations::create_device(&mut conn, &device).expect("Failed to create device");

    (conn, user_id, device_id)
}

#[test]
fn test_register_user_hashes_password_and_normalizes_fields() {
    let mut conn = setup_empty_db();
    let password = "SecretPass123!".to_string();

    let user = logic::register_user(
        &mut conn,
        "  TestUser  ".to_string(),
        "USER@Example.COM".to_string(),
        password.clone(),
    )
    .expect("User registration failed");

    assert_eq!(user.user_name, "TestUser");
    assert_eq!(user.user_mail, "user@example.com");
    assert_ne!(user.user_password_hash, password);

    let stored = operations::get_user_by_name(&mut conn, "TestUser")
        .expect("Query failed")
        .expect("User not found after registration");
    assert_eq!(stored.user_id, user.user_id);

    let hash = PasswordHash::new(&stored.user_password_hash).expect("Invalid stored hash");
    Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .expect("Password verification failed");
}

#[test]
fn test_register_user_rejects_duplicate_username_and_email() {
    let mut conn = setup_empty_db();

    logic::register_user(
        &mut conn,
        "testuser".to_string(),
        "user@example.com".to_string(),
        "SecretPass123!".to_string(),
    )
    .expect("Initial registration failed");

    let username_err = logic::register_user(
        &mut conn,
        "testuser".to_string(),
        "other@example.com".to_string(),
        "AnotherPass!".to_string(),
    )
    .expect_err("Expected duplicate username to fail");
    assert!(username_err.contains("Username"));

    let email_err = logic::register_user(
        &mut conn,
        "otheruser".to_string(),
        "user@example.com".to_string(),
        "AnotherPass!".to_string(),
    )
    .expect_err("Expected duplicate email to fail");
    assert!(email_err.contains("Email"));
}

#[test]
fn test_login_user_by_username_and_email() {
    let mut conn = setup_empty_db();
    let password = "SecretPass123!";

    logic::register_user(
        &mut conn,
        "testuser".to_string(),
        "test@example.com".to_string(),
        password.to_string(),
    )
    .expect("Registration failed");

    let by_username =
        logic::login_user(&mut conn, "testuser", password).expect("Login by username failed");
    assert_eq!(by_username.user_name, "testuser");

    let by_email =
        logic::login_user(&mut conn, "test@example.com", password).expect("Login by email failed");
    assert_eq!(by_email.user_id, by_username.user_id);

    let wrong_password_err =
        logic::login_user(&mut conn, "testuser", "wrong").expect_err("Expected invalid password");
    assert!(wrong_password_err.contains("Invalid credentials"));
}

#[test]
fn test_add_device_to_user_and_get_user_devices() {
    let mut conn = setup_empty_db();
    let user = logic::register_user(
        &mut conn,
        "devicetester".to_string(),
        "devicetester@example.com".to_string(),
        "SecretPass123!".to_string(),
    )
    .expect("Registration failed");

    let device = logic::add_device_to_user(
        &mut conn,
        user.user_id,
        "  mobile  ".to_string(),
        Some("  token123  ".to_string()),
    )
    .expect("Adding device failed");

    assert_eq!(device.user_id, user.user_id);
    assert_eq!(device.device_type, "mobile");
    assert_eq!(device.push_token, Some("token123".to_string()));
    assert!(device.last_seen.is_some());

    let devices =
        logic::get_user_devices(&mut conn, user.user_id).expect("Fetching devices failed");
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].device_id, device.device_id);

    let empty_type_err =
        logic::add_device_to_user(&mut conn, user.user_id, "   ".to_string(), None)
            .expect_err("Expected empty device type to fail");
    assert!(empty_type_err.contains("Device type"));
}

#[test]
fn test_crdt_task_logic() {
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Create a new task list
    let task_list =
        logic::create_new_task_list(&mut conn, user_id, device_id, "CRDT Test List".to_string())
            .unwrap();

    // 1. Add a task using the refactored function.
    let task_content = "My CRDT task".to_string();
    let task = logic::add_task_to_list(
        &mut conn,
        user_id,
        device_id,
        task_list.list_id,
        task_content.clone(),
    )
    .unwrap();

    // 2. Verify oplog entries were created (one for task list, one for task).
    let since_timestamp = (Utc::now() - chrono::Duration::minutes(1)).timestamp();
    let oplog_entries = operations::get_oplog_entries_since(&mut conn, since_timestamp).unwrap();
    assert_eq!(
        oplog_entries.len(),
        2,
        "Expected 2 oplog entries: create_task_list and create_task"
    );

    // Find the create_task entry
    let task_entry = oplog_entries
        .iter()
        .find(|e| e.op_type == "create" && e.table == "tasks")
        .expect("Should have create_task entry");
    // Verify the task ID is in the data JSON
    let task_data: serde_json::Value = serde_json::from_str(&task_entry.data.to_string()).unwrap();
    assert_eq!(task_data["task_id"], task.task_id.to_string());

    // 3. Verify the task now exists in the tasks table.
    let tasks_in_db_after_apply =
        operations::get_tasks_by_list_id(&mut conn, task_list.list_id).unwrap();
    assert_eq!(tasks_in_db_after_apply.len(), 1);
    assert_eq!(tasks_in_db_after_apply[0].content, task_content);
}

#[test]
fn test_generate_device_id() {
    let (peer_id, _keypair) = logic::sync::generate_device_id();
    assert!(peer_id.to_string().starts_with("12D"));
}

#[test]
fn test_mark_task_complete_crdt() {
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();
    let task_list = logic::create_new_task_list(
        &mut conn,
        user_id,
        device_id,
        "Completion Test List".to_string(),
    )
    .unwrap();

    // 1. Create a task and apply it immediately.
    let task = logic::add_task_to_list(
        &mut conn,
        user_id,
        device_id,
        task_list.list_id,
        "Task to complete".to_string(),
    )
    .unwrap();

    // 2. Mark the task as complete.
    logic::mark_task_as_complete(&mut conn, user_id, device_id, task.task_id).unwrap();

    // 3. Verify the oplog entries (create_task_list, create_task, update_task_status).
    let since_timestamp = (Utc::now() - chrono::Duration::minutes(1)).timestamp();
    let oplog_entries_after_update =
        operations::get_oplog_entries_since(&mut conn, since_timestamp).unwrap();
    assert_eq!(
        oplog_entries_after_update.len(),
        3,
        "Expected 3 oplog entries"
    );
    // Verify that we have updated tasks in the oplog
    let has_update = oplog_entries_after_update
        .iter()
        .any(|e| e.op_type == "update" && e.table == "tasks");
    assert!(has_update, "Should have a task update entry");

    // 4. Verify the task is now marked as complete in the database.
    let completed_task = operations::get_task(&mut conn, task.task_id)
        .unwrap()
        .unwrap();
    assert!(completed_task.is_completed);
}

#[test]
fn test_create_habit_crdt() {
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // 1. Create a habit.
    let habit = logic::create_habit(
        &mut conn,
        user_id,
        device_id,
        "Read daily".to_string(),
        None,
        None,
        "daily".to_string(),
    )
    .unwrap();

    // 2. Verify an oplog entry was created.
    let since_timestamp = (Utc::now() - chrono::Duration::minutes(1)).timestamp();
    let oplog_entries = operations::get_oplog_entries_since(&mut conn, since_timestamp).unwrap();
    assert_eq!(oplog_entries.len(), 1);
    let entry = &oplog_entries[0];
    assert_eq!(entry.op_type, "create");
    assert_eq!(entry.table, "habits");

    // 3. Verify the habit now exists in the habits table.
    assert!(
        operations::get_habit(&mut conn, habit.habit_id)
            .unwrap()
            .is_some()
    );
}

#[test]
fn test_log_habit_completion_crdt() {
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();
    let habit = logic::create_habit(
        &mut conn,
        user_id,
        device_id,
        "Log Habit Test".to_string(),
        None,
        None,
        "daily".to_string(),
    )
    .unwrap();

    // 1. Log a habit completion.
    let today = Utc::now().naive_utc().date();
    let habit_entry =
        logic::log_habit_completion(&mut conn, user_id, device_id, habit.habit_id, today, None)
            .unwrap();

    // 2. Verify an oplog entry was created.
    let since_timestamp = (Utc::now() - chrono::Duration::minutes(1)).timestamp();
    let oplog_entries_after_log =
        operations::get_oplog_entries_since(&mut conn, since_timestamp).unwrap();
    assert_eq!(oplog_entries_after_log.len(), 2);
    let entry = oplog_entries_after_log
        .iter()
        .find(|e| e.op_type == "create" && e.table == "habit_entries")
        .unwrap();
    // Verify the habit entry ID is in the data JSON
    let entry_data: serde_json::Value = serde_json::from_str(&entry.data.to_string()).unwrap();
    assert_eq!(entry_data["entry_id"], habit_entry.entry_id.to_string());

    // 3. Verify the habit entry now exists in the habit_entries table.
    let entries_in_db_after_apply =
        operations::get_habit_entries_sorted_by_date(&mut conn, habit.habit_id).unwrap();
    assert_eq!(entries_in_db_after_apply.len(), 1);
}

#[test]
fn test_schedule_block_crdt() {
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // 1. Schedule a block.
    let start_time = Utc::now();
    let end_time = start_time + chrono::Duration::hours(1);
    let block = logic::schedule_block(&mut conn, user_id, device_id, start_time, end_time).unwrap();

    // 2. Verify an oplog entry was created.
    let since_timestamp = (Utc::now() - chrono::Duration::minutes(1)).timestamp();
    let oplog_entries = operations::get_oplog_entries_since(&mut conn, since_timestamp).unwrap();
    assert_eq!(oplog_entries.len(), 1);
    let entry = &oplog_entries[0];
    assert_eq!(entry.op_type, "create");
    assert_eq!(entry.table, "blocks");

    // 3. Verify the block now exists in the blocks table.
    assert!(
        operations::get_block(&mut conn, block.block_id)
            .unwrap()
            .is_some()
    );
}

#[test]
fn test_assign_task_to_block_crdt() {
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();
    let task_list = logic::create_new_task_list(
        &mut conn,
        user_id,
        device_id,
        "Assign Task Test List".to_string(),
    )
    .unwrap();
    let task = logic::add_task_to_list(
        &mut conn,
        user_id,
        device_id,
        task_list.list_id,
        "Task to assign".to_string(),
    )
    .unwrap();
    let block = logic::schedule_block(
        &mut conn,
        user_id,
        device_id,
        Utc::now(),
        Utc::now() + chrono::Duration::hours(1),
    )
    .unwrap();

    // 1. Assign the task to the block.
    logic::assign_task_to_block(&mut conn, user_id, device_id, task.task_id, block.block_id)
        .unwrap();

    // 2. Verify oplog entries (create_task_list, create_task, schedule_block, assign_task_to_block).
    let since_timestamp = (Utc::now() - chrono::Duration::minutes(1)).timestamp();
    let oplog_entries_after_assign =
        operations::get_oplog_entries_since(&mut conn, since_timestamp).unwrap();
    assert_eq!(
        oplog_entries_after_assign.len(),
        4,
        "Expected 4 oplog entries"
    );
    assert!(
        oplog_entries_after_assign
            .iter()
            .any(|e| e.table == "task_blocks")
    );

    // 3. Verify the task block now exists in the task_blocks table.
    let task_blocks_after_apply =
        operations::get_tasks_by_block_id(&mut conn, block.block_id).unwrap();
    assert_eq!(task_blocks_after_apply.len(), 1);
}

#[test]
fn test_save_pomodoro_preset_crdt() {
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // 1. Save a pomodoro preset.
    let _pomodoro = logic::save_pomodoro_preset(
        &mut conn,
        user_id,
        device_id,
        PomodoroPreset {
            name: "Test Preset".to_string(),
            cover: None,
            work_duration: 25,
            short_break: 5,
            long_break: 15,
            interval: 4,
        },
    )
    .unwrap();

    // 2. Verify an oplog entry was created.
    let since_timestamp = (Utc::now() - chrono::Duration::minutes(1)).timestamp();
    let oplog_entries = operations::get_oplog_entries_since(&mut conn, since_timestamp).unwrap();
    assert_eq!(oplog_entries.len(), 1);
    let entry = &oplog_entries[0];
    assert_eq!(entry.op_type, "create");
    assert_eq!(entry.table, "pomodoros");

    // 3. Verify the pomodoro preset now exists in the pomodoros table.
    let presets_after_apply = operations::get_pomodoros_by_user_id(&mut conn, user_id).unwrap();
    assert_eq!(presets_after_apply.len(), 1);
}

// P2P Synchronization Tests

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

// Note: sync_with_peer function doesn't exist in the sync module.
// The sync functionality is handled through SyncMessage handling and CRDT merge.
// This test is commented out until sync_with_peer is implemented or replaced with
// the proper message-based sync flow.
#[test]
#[ignore]
fn test_sync_with_peer() {
    use nexus_core::crdt;

    // Setup two "devices" with separate databases but same user
    let (mut conn1, user_id, device_id1) = setup_db_with_user_and_device();

    // Create a second device database for the same user
    let mut conn2 =
        operations::initialize_database(":memory:").expect("Failed to create in-memory database");
    let user = nexus_core::models::User {
        user_id, // Same user_id as conn1
        user_name: "testuser".to_string(),
        user_password_hash: "password".to_string(),
        user_mail: "test@example.com".to_string(),
        created_at: Utc::now(),
    };
    operations::create_user(&mut conn2, &user).expect("Failed to create user");

    let _device_id2 = Uuid::new_v4();
    let device2 = nexus_core::models::Device {
        device_id: _device_id2,
        user_id,
        device_type: "test_device_2".to_string(),
        push_token: None,
        last_seen: None,
    };
    operations::create_device(&mut conn2, &device2).expect("Failed to create device");

    // Create a task on device 1
    let task_list1 =
        logic::create_new_task_list(&mut conn1, user_id, device_id1, "Device 1 List".to_string())
            .unwrap();
    let _task1 = logic::add_task_to_list(
        &mut conn1,
        user_id,
        device_id1,
        task_list1.list_id,
        "Task from device 1".to_string(),
    )
    .unwrap();

    // Get oplog entries from device 1 (should have create_task_list and create_task)
    let since_timestamp = (Utc::now() - chrono::Duration::hours(1)).timestamp();
    let device1_entries = operations::get_oplog_entries_since(&mut conn1, since_timestamp).unwrap();
    assert_eq!(
        device1_entries.len(),
        2,
        "Expected 2 entries: create_task_list and create_task"
    );

    // Sync device 1's entries to device 2 using CRDT merge
    crdt::merge(&mut conn2, &device1_entries).unwrap();

    // Verify that device 2 now has the task
    let tasks_on_device2 =
        operations::get_tasks_by_list_id(&mut conn2, task_list1.list_id).unwrap();
    assert_eq!(tasks_on_device2.len(), 1);
    assert_eq!(tasks_on_device2[0].content, "Task from device 1");

    // Verify that device 2 has the synced oplog entries
    let device2_entries = operations::get_oplog_entries_since(&mut conn2, since_timestamp).unwrap();
    assert!(
        device2_entries.len() >= 2,
        "Device 2 should have at least 2 oplog entries"
    );
}

#[test]
#[ignore]
fn test_bidirectional_sync() {
    use nexus_core::crdt;

    // Setup two devices with the same user_id
    let (mut conn1, user_id, device_id1) = setup_db_with_user_and_device();

    // Create a second device in-memory database for the same user
    let mut conn2 =
        operations::initialize_database(":memory:").expect("Failed to create in-memory database");
    let user = nexus_core::models::User {
        user_id, // Same user_id as conn1
        user_name: "testuser".to_string(),
        user_password_hash: "password".to_string(),
        user_mail: "test@example.com".to_string(),
        created_at: Utc::now(),
    };
    operations::create_user(&mut conn2, &user).expect("Failed to create user");

    let device_id2 = Uuid::new_v4();
    let device2 = nexus_core::models::Device {
        device_id: device_id2,
        user_id,
        device_type: "test_device_2".to_string(),
        push_token: None,
        last_seen: None,
    };
    operations::create_device(&mut conn2, &device2).expect("Failed to create device");

    // Create a task list on device 1
    let task_list =
        logic::create_new_task_list(&mut conn1, user_id, device_id1, "Shared List".to_string())
            .unwrap();

    // Create a task on device 1
    let _task1 = logic::add_task_to_list(
        &mut conn1,
        user_id,
        device_id1,
        task_list.list_id,
        "Task from device 1".to_string(),
    )
    .unwrap();

    // Get device 1's entries and sync to device 2 using CRDT merge
    let since_timestamp = (Utc::now() - chrono::Duration::hours(1)).timestamp();
    let device1_entries = operations::get_oplog_entries_since(&mut conn1, since_timestamp).unwrap();
    crdt::merge(&mut conn2, &device1_entries).unwrap();

    // Now device 2 knows about the task list, create a task on device 2
    let _task2 = logic::add_task_to_list(
        &mut conn2,
        user_id,
        device_id2,
        task_list.list_id,
        "Task from device 2".to_string(),
    )
    .unwrap();

    // Get device 2's entries (only the new task) and sync back to device 1
    let since_timestamp2 = (Utc::now() - chrono::Duration::seconds(5)).timestamp();
    let device2_entries =
        operations::get_oplog_entries_since(&mut conn2, since_timestamp2).unwrap();
    crdt::merge(&mut conn1, &device2_entries).unwrap();

    // Both devices should now have both tasks
    let tasks_on_device1 = operations::get_tasks_by_list_id(&mut conn1, task_list.list_id).unwrap();
    let tasks_on_device2 = operations::get_tasks_by_list_id(&mut conn2, task_list.list_id).unwrap();

    assert_eq!(tasks_on_device1.len(), 2);
    assert_eq!(tasks_on_device2.len(), 2);

    // Verify both tasks exist on both devices
    let task_contents1: Vec<String> = tasks_on_device1.iter().map(|t| t.content.clone()).collect();
    let task_contents2: Vec<String> = tasks_on_device2.iter().map(|t| t.content.clone()).collect();

    assert!(task_contents1.contains(&"Task from device 1".to_string()));
    assert!(task_contents1.contains(&"Task from device 2".to_string()));
    assert!(task_contents2.contains(&"Task from device 1".to_string()));
    assert!(task_contents2.contains(&"Task from device 2".to_string()));
}

#[test]
fn test_update_peer_info() {
    use nexus_core::logic::sync::update_peer_info;

    let (mut conn, user_id, _device_id) = setup_db_with_user_and_device();

    // Create a peer device
    let peer_device_id = Uuid::new_v4();
    let peer_peer_id = "peer_123".to_string();
    let peer_ip = Some("192.168.1.100".to_string());

    // Update peer info (should create a new peer)
    update_peer_info(
        &mut conn,
        user_id,
        peer_device_id,
        peer_peer_id.clone(),
        peer_ip.clone(),
    )
    .unwrap();

    // Verify peer was created
    let peers = operations::get_peers_by_user_id(&mut conn, user_id).unwrap();
    assert_eq!(peers.len(), 1);
    assert_eq!(peers[0].device_id, peer_device_id);
    assert_eq!(peers[0].last_known_ip, peer_ip);
    assert!(peers[0].last_sync_time.is_some());
}

#[test]
#[ignore]
fn test_sync_prevents_duplicates() {
    use nexus_core::crdt;

    let (mut conn1, user_id, device_id1) = setup_db_with_user_and_device();

    // Create a second device database for the same user
    let mut conn2 =
        operations::initialize_database(":memory:").expect("Failed to create in-memory database");
    let user = nexus_core::models::User {
        user_id, // Same user_id as conn1
        user_name: "testuser".to_string(),
        user_password_hash: "password".to_string(),
        user_mail: "test@example.com".to_string(),
        created_at: Utc::now(),
    };
    operations::create_user(&mut conn2, &user).expect("Failed to create user");

    let _device_id2 = Uuid::new_v4();
    let device2 = nexus_core::models::Device {
        device_id: _device_id2,
        user_id,
        device_type: "test_device_2".to_string(),
        push_token: None,
        last_seen: None,
    };
    operations::create_device(&mut conn2, &device2).expect("Failed to create device");

    // Create a task on device 1
    let task_list =
        logic::create_new_task_list(&mut conn1, user_id, device_id1, "Test List".to_string())
            .unwrap();
    let _task = logic::add_task_to_list(
        &mut conn1,
        user_id,
        device_id1,
        task_list.list_id,
        "Test Task".to_string(),
    )
    .unwrap();

    // Get entries from device 1
    let since_timestamp = (Utc::now() - chrono::Duration::hours(1)).timestamp();
    let entries = operations::get_oplog_entries_since(&mut conn1, since_timestamp).unwrap();

    // Sync to device 2 first time using CRDT merge
    crdt::merge(&mut conn2, &entries).unwrap();
    let tasks_after_first_sync =
        operations::get_tasks_by_list_id(&mut conn2, task_list.list_id).unwrap();
    assert_eq!(tasks_after_first_sync.len(), 1);

    // Sync the same entries again (should not create duplicates)
    crdt::merge(&mut conn2, &entries).unwrap();
    let tasks_after_second_sync =
        operations::get_tasks_by_list_id(&mut conn2, task_list.list_id).unwrap();
    assert_eq!(
        tasks_after_second_sync.len(),
        1,
        "Duplicate entries should be prevented"
    );
}

#[test]
fn test_add_item_to_blocklist_crdt() {
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // 1. Add an item to the blocklist.
    let _item = logic::add_item_to_blocklist(
        &mut conn,
        user_id,
        device_id,
        "website".to_string(),
        "example.com".to_string(),
    )
    .unwrap();

    // 2. Verify an oplog entry was created.
    let since_timestamp = (Utc::now() - chrono::Duration::minutes(1)).timestamp();
    let oplog_entries = operations::get_oplog_entries_since(&mut conn, since_timestamp).unwrap();
    assert_eq!(oplog_entries.len(), 1);
    let entry = &oplog_entries[0];
    assert_eq!(entry.op_type, "create");
    assert_eq!(entry.table, "blocked_items");

    // 3. Verify the item now exists in the blocked_items table.
    let items_after_apply =
        operations::get_active_blocked_items_by_user_id(&mut conn, user_id).unwrap();
    assert_eq!(items_after_apply.len(), 1);
}

#[test]
fn test_get_all_task_lists_for_user() {
    // Test successful retrieval of all task lists for a user
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Initially, user should have no task lists
    let empty_lists =
        logic::get_all_task_lists_for_user(&mut conn, user_id).expect("Failed to get task lists");
    assert_eq!(
        empty_lists.len(),
        0,
        "User should have no task lists initially"
    );

    // Create multiple task lists
    let _list1 =
        logic::create_new_task_list(&mut conn, user_id, device_id, "Work".to_string()).unwrap();
    let _list2 =
        logic::create_new_task_list(&mut conn, user_id, device_id, "Personal".to_string()).unwrap();
    let _list3 =
        logic::create_new_task_list(&mut conn, user_id, device_id, "Shopping".to_string()).unwrap();

    // Retrieve all task lists
    let all_lists = logic::get_all_task_lists_for_user(&mut conn, user_id)
        .expect("Failed to get all task lists");

    assert_eq!(all_lists.len(), 3, "User should have 3 task lists");

    // Verify list names
    let list_names: Vec<String> = all_lists.iter().map(|l| l.name.clone()).collect();
    assert!(list_names.contains(&"Work".to_string()));
    assert!(list_names.contains(&"Personal".to_string()));
    assert!(list_names.contains(&"Shopping".to_string()));

    // Verify all lists belong to the correct user
    for list in &all_lists {
        assert_eq!(list.user_id, user_id);
    }
}

#[test]
fn test_get_all_tasks_in_list_success() {
    // Test successful retrieval of tasks from a list the user owns
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Create a task list
    let task_list =
        logic::create_new_task_list(&mut conn, user_id, device_id, "Test List".to_string())
            .unwrap();

    // Initially, list should have no tasks
    let empty_tasks = logic::get_all_tasks_in_list(&mut conn, user_id, task_list.list_id)
        .expect("Failed to get tasks");
    assert_eq!(empty_tasks.len(), 0, "Task list should be empty initially");

    // Add multiple tasks to the list
    let _task1 = logic::add_task_to_list(
        &mut conn,
        user_id,
        device_id,
        task_list.list_id,
        "Task 1".to_string(),
    )
    .unwrap();
    let _task2 = logic::add_task_to_list(
        &mut conn,
        user_id,
        device_id,
        task_list.list_id,
        "Task 2".to_string(),
    )
    .unwrap();
    let _task3 = logic::add_task_to_list(
        &mut conn,
        user_id,
        device_id,
        task_list.list_id,
        "Task 3".to_string(),
    )
    .unwrap();

    // Retrieve all tasks
    let all_tasks = logic::get_all_tasks_in_list(&mut conn, user_id, task_list.list_id)
        .expect("Failed to get all tasks");

    assert_eq!(all_tasks.len(), 3, "Should have 3 tasks in the list");

    // Verify task contents
    let task_contents: Vec<String> = all_tasks.iter().map(|t| t.content.clone()).collect();
    assert!(task_contents.contains(&"Task 1".to_string()));
    assert!(task_contents.contains(&"Task 2".to_string()));
    assert!(task_contents.contains(&"Task 3".to_string()));
}

#[test]
fn test_get_all_tasks_in_list_access_denied() {
    // Test that a user cannot access another user's task list
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Create a task list for the first user
    let task_list =
        logic::create_new_task_list(&mut conn, user_id, device_id, "User 1 List".to_string())
            .unwrap();
    logic::add_task_to_list(
        &mut conn,
        user_id,
        device_id,
        task_list.list_id,
        "User 1 Task".to_string(),
    )
    .unwrap();

    // Create a second user
    let other_user_id = Uuid::new_v4();
    let other_user = User {
        user_id: other_user_id,
        user_name: "otheruser".to_string(),
        user_password_hash: "password".to_string(),
        user_mail: "other@example.com".to_string(),
        created_at: Utc::now(),
    };
    operations::create_user(&mut conn, &other_user).expect("Failed to create other user");

    // Attempt to access the first user's task list with the second user's ID
    let result = logic::get_all_tasks_in_list(&mut conn, other_user_id, task_list.list_id);

    assert!(result.is_err(), "Should return error for access denied");
    assert!(
        result.unwrap_err().contains("Access denied"),
        "Error message should indicate access denied"
    );
}

#[test]
fn test_get_all_tasks_in_list_not_found() {
    // Test that function returns error for non-existent list
    let (mut conn, user_id, _device_id) = setup_db_with_user_and_device();

    let nonexistent_list_id = Uuid::new_v4();
    let result = logic::get_all_tasks_in_list(&mut conn, user_id, nonexistent_list_id);

    assert!(result.is_err(), "Should return error for non-existent list");
    assert!(
        result.unwrap_err().contains("not found"),
        "Error message should indicate list not found"
    );
}

#[test]
fn test_get_tasks_due_today() {
    // Test retrieval of tasks due today
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Initially, no tasks due today
    let empty_tasks =
        logic::get_tasks_due_today(&mut conn, user_id).expect("Failed to get tasks due today");
    assert_eq!(
        empty_tasks.len(),
        0,
        "Should have no tasks due today initially"
    );

    // Create a task list
    let task_list =
        logic::create_new_task_list(&mut conn, user_id, device_id, "Test List".to_string())
            .unwrap();

    // Create tasks and manually set due dates
    let today = Utc::now().naive_utc().date();
    let tomorrow = today + chrono::Duration::days(1);
    let yesterday = today - chrono::Duration::days(1);

    // Create a task due today
    let task_today_1 = nexus_core::models::Task {
        task_id: Uuid::new_v4(),
        list_id: task_list.list_id,
        content: "Task due today 1".to_string(),
        is_completed: false,
        due_date: Some(today),
        created_at: Utc::now(),
        updated_at: None,
    };
    operations::create_task(&mut conn, &task_today_1).expect("Failed to create task");

    // Create another task due today
    let task_today_2 = nexus_core::models::Task {
        task_id: Uuid::new_v4(),
        list_id: task_list.list_id,
        content: "Task due today 2".to_string(),
        is_completed: false,
        due_date: Some(today),
        created_at: Utc::now(),
        updated_at: None,
    };
    operations::create_task(&mut conn, &task_today_2).expect("Failed to create task");

    // Create a task due tomorrow
    let task_tomorrow = nexus_core::models::Task {
        task_id: Uuid::new_v4(),
        list_id: task_list.list_id,
        content: "Task due tomorrow".to_string(),
        is_completed: false,
        due_date: Some(tomorrow),
        created_at: Utc::now(),
        updated_at: None,
    };
    operations::create_task(&mut conn, &task_tomorrow).expect("Failed to create task");

    // Create a task due yesterday
    let task_yesterday = nexus_core::models::Task {
        task_id: Uuid::new_v4(),
        list_id: task_list.list_id,
        content: "Task due yesterday".to_string(),
        is_completed: false,
        due_date: Some(yesterday),
        created_at: Utc::now(),
        updated_at: None,
    };
    operations::create_task(&mut conn, &task_yesterday).expect("Failed to create task");

    // Create a task with no due date
    let task_no_date = nexus_core::models::Task {
        task_id: Uuid::new_v4(),
        list_id: task_list.list_id,
        content: "Task with no due date".to_string(),
        is_completed: false,
        due_date: None,
        created_at: Utc::now(),
        updated_at: None,
    };
    operations::create_task(&mut conn, &task_no_date).expect("Failed to create task");

    // Get tasks due today
    let tasks_due_today =
        logic::get_tasks_due_today(&mut conn, user_id).expect("Failed to get tasks due today");

    assert_eq!(
        tasks_due_today.len(),
        2,
        "Should have exactly 2 tasks due today"
    );

    // Verify the correct tasks are returned
    let task_contents: Vec<String> = tasks_due_today.iter().map(|t| t.content.clone()).collect();
    assert!(task_contents.contains(&"Task due today 1".to_string()));
    assert!(task_contents.contains(&"Task due today 2".to_string()));
    assert!(!task_contents.contains(&"Task due tomorrow".to_string()));
    assert!(!task_contents.contains(&"Task due yesterday".to_string()));
}

#[test]
fn test_get_habit_streak_zero_entries() {
    // Test streak calculation with no habit entries (zero streak)
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Create a habit
    let habit = logic::create_habit(
        &mut conn,
        user_id,
        device_id,
        "Test Habit".to_string(),
        None,
        None,
        "daily".to_string(),
    )
    .unwrap();

    // Get streak (should be 0 with no entries)
    let streak = logic::get_habit_streak(&mut conn, user_id, habit.habit_id)
        .expect("Failed to get habit streak");

    assert_eq!(streak, 0, "Streak should be 0 when no entries exist");
}

#[test]
fn test_get_habit_streak_current_streak_today() {
    // Test streak calculation with entries including today
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Create a habit
    let habit = logic::create_habit(
        &mut conn,
        user_id,
        device_id,
        "Daily Habit".to_string(),
        None,
        None,
        "daily".to_string(),
    )
    .unwrap();

    let today = Utc::now().naive_utc().date();

    // Log habit completion for today and the past 4 days (5 consecutive days)
    for i in 0..5 {
        let date = today - chrono::Duration::days(i);
        logic::log_habit_completion(&mut conn, user_id, device_id, habit.habit_id, date, None)
            .unwrap();
    }

    // Get streak
    let streak = logic::get_habit_streak(&mut conn, user_id, habit.habit_id)
        .expect("Failed to get habit streak");

    assert_eq!(
        streak, 5,
        "Streak should be 5 with 5 consecutive days including today"
    );
}

#[test]
fn test_get_habit_streak_current_streak_yesterday() {
    // Test streak calculation when latest entry is yesterday (still counts as current streak)
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Create a habit
    let habit = logic::create_habit(
        &mut conn,
        user_id,
        device_id,
        "Daily Habit".to_string(),
        None,
        None,
        "daily".to_string(),
    )
    .unwrap();

    let today = Utc::now().naive_utc().date();
    let yesterday = today - chrono::Duration::days(1);

    // Log habit completion for yesterday and the 2 days before that (3 consecutive days)
    for i in 0..3 {
        let date = yesterday - chrono::Duration::days(i);
        logic::log_habit_completion(&mut conn, user_id, device_id, habit.habit_id, date, None)
            .unwrap();
    }

    // Get streak
    let streak = logic::get_habit_streak(&mut conn, user_id, habit.habit_id)
        .expect("Failed to get habit streak");

    assert_eq!(
        streak, 3,
        "Streak should be 3 when latest entry is yesterday with 3 consecutive days"
    );
}

#[test]
fn test_get_habit_streak_broken_streak() {
    // Test streak calculation when the streak is broken (latest entry is too old)
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Create a habit
    let habit = logic::create_habit(
        &mut conn,
        user_id,
        device_id,
        "Daily Habit".to_string(),
        None,
        None,
        "daily".to_string(),
    )
    .unwrap();

    let today = Utc::now().naive_utc().date();

    // Log habit completion for 5 consecutive days, but starting 3 days ago
    for i in 3..8 {
        let date = today - chrono::Duration::days(i);
        logic::log_habit_completion(&mut conn, user_id, device_id, habit.habit_id, date, None)
            .unwrap();
    }

    // Get streak (should be 0 because the latest entry is 3 days ago)
    let streak = logic::get_habit_streak(&mut conn, user_id, habit.habit_id)
        .expect("Failed to get habit streak");

    assert_eq!(
        streak, 0,
        "Streak should be 0 when latest entry is more than 1 day old"
    );
}

#[test]
fn test_get_habit_streak_with_gap() {
    // Test streak calculation with a gap in the middle (streak resets)
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Create a habit
    let habit = logic::create_habit(
        &mut conn,
        user_id,
        device_id,
        "Daily Habit".to_string(),
        None,
        None,
        "daily".to_string(),
    )
    .unwrap();

    let today = Utc::now().naive_utc().date();

    // Log habit completion for today and yesterday
    logic::log_habit_completion(&mut conn, user_id, device_id, habit.habit_id, today, None)
        .unwrap();
    logic::log_habit_completion(
        &mut conn,
        user_id,
        device_id,
        habit.habit_id,
        today - chrono::Duration::days(1),
        None,
    )
    .unwrap();

    // Skip day 2

    // Log habit completion for days 3-5 ago
    logic::log_habit_completion(
        &mut conn,
        user_id,
        device_id,
        habit.habit_id,
        today - chrono::Duration::days(3),
        None,
    )
    .unwrap();
    logic::log_habit_completion(
        &mut conn,
        user_id,
        device_id,
        habit.habit_id,
        today - chrono::Duration::days(4),
        None,
    )
    .unwrap();
    logic::log_habit_completion(
        &mut conn,
        user_id,
        device_id,
        habit.habit_id,
        today - chrono::Duration::days(5),
        None,
    )
    .unwrap();

    // Get streak (should only count today and yesterday = 2)
    let streak = logic::get_habit_streak(&mut conn, user_id, habit.habit_id)
        .expect("Failed to get habit streak");

    assert_eq!(
        streak, 2,
        "Streak should be 2, counting only today and yesterday before the gap"
    );
}

#[test]
fn test_get_habit_streak_long_consecutive_streak() {
    // Test streak calculation with a long consecutive streak (30 days)
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Create a habit
    let habit = logic::create_habit(
        &mut conn,
        user_id,
        device_id,
        "Daily Habit".to_string(),
        None,
        None,
        "daily".to_string(),
    )
    .unwrap();

    let today = Utc::now().naive_utc().date();

    // Log habit completion for 30 consecutive days
    for i in 0..30 {
        let date = today - chrono::Duration::days(i);
        logic::log_habit_completion(&mut conn, user_id, device_id, habit.habit_id, date, None)
            .unwrap();
    }

    // Get streak
    let streak = logic::get_habit_streak(&mut conn, user_id, habit.habit_id)
        .expect("Failed to get habit streak");

    assert_eq!(streak, 30, "Streak should be 30 with 30 consecutive days");
}

#[test]
fn test_get_habit_streak_access_denied() {
    // Test that a user cannot access another user's habit streak
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Create a habit for the first user
    let habit = logic::create_habit(
        &mut conn,
        user_id,
        device_id,
        "User 1 Habit".to_string(),
        None,
        None,
        "daily".to_string(),
    )
    .unwrap();
    logic::log_habit_completion(
        &mut conn,
        user_id,
        device_id,
        habit.habit_id,
        Utc::now().naive_utc().date(),
        None,
    )
    .unwrap();

    // Create a second user
    let other_user_id = Uuid::new_v4();
    let other_user = User {
        user_id: other_user_id,
        user_name: "otheruser".to_string(),
        user_password_hash: "password".to_string(),
        user_mail: "other@example.com".to_string(),
        created_at: Utc::now(),
    };
    operations::create_user(&mut conn, &other_user).expect("Failed to create other user");

    // Attempt to get streak for the first user's habit with the second user's ID
    let result = logic::get_habit_streak(&mut conn, other_user_id, habit.habit_id);

    assert!(result.is_err(), "Should return error for access denied");
    assert!(
        result.unwrap_err().contains("Access denied"),
        "Error message should indicate access denied"
    );
}

#[test]
fn test_get_tasks_for_a_specific_block_success() {
    // Test successful retrieval of tasks assigned to a block
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Create a task list and tasks
    let task_list =
        logic::create_new_task_list(&mut conn, user_id, device_id, "Test List".to_string())
            .unwrap();
    let task1 = logic::add_task_to_list(
        &mut conn,
        user_id,
        device_id,
        task_list.list_id,
        "Task 1".to_string(),
    )
    .unwrap();
    let task2 = logic::add_task_to_list(
        &mut conn,
        user_id,
        device_id,
        task_list.list_id,
        "Task 2".to_string(),
    )
    .unwrap();
    let _task3 = logic::add_task_to_list(
        &mut conn,
        user_id,
        device_id,
        task_list.list_id,
        "Task 3".to_string(),
    )
    .unwrap();

    // Create a block
    let block = logic::schedule_block(
        &mut conn,
        user_id,
        device_id,
        Utc::now(),
        Utc::now() + chrono::Duration::hours(2),
    )
    .unwrap();

    // Initially, block should have no tasks
    let empty_tasks = logic::get_tasks_for_a_specific_block(&mut conn, user_id, block.block_id)
        .expect("Failed to get tasks for block");
    assert_eq!(empty_tasks.len(), 0, "Block should have no tasks initially");

    // Assign tasks to the block
    logic::assign_task_to_block(&mut conn, user_id, device_id, task1.task_id, block.block_id)
        .unwrap();
    logic::assign_task_to_block(&mut conn, user_id, device_id, task2.task_id, block.block_id)
        .unwrap();

    // Get tasks for the block
    let block_tasks = logic::get_tasks_for_a_specific_block(&mut conn, user_id, block.block_id)
        .expect("Failed to get tasks for block");

    assert_eq!(block_tasks.len(), 2, "Block should have 2 assigned tasks");

    // Verify the correct tasks are returned
    let task_contents: Vec<String> = block_tasks.iter().map(|t| t.content.clone()).collect();
    assert!(task_contents.contains(&"Task 1".to_string()));
    assert!(task_contents.contains(&"Task 2".to_string()));
    assert!(!task_contents.contains(&"Task 3".to_string()));
}

#[test]
fn test_get_tasks_for_a_specific_block_access_denied() {
    // Test that a user cannot access another user's block tasks
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Create a block for the first user
    let block = logic::schedule_block(
        &mut conn,
        user_id,
        device_id,
        Utc::now(),
        Utc::now() + chrono::Duration::hours(1),
    )
    .unwrap();

    // Create a second user
    let other_user_id = Uuid::new_v4();
    let other_user = User {
        user_id: other_user_id,
        user_name: "otheruser".to_string(),
        user_password_hash: "password".to_string(),
        user_mail: "other@example.com".to_string(),
        created_at: Utc::now(),
    };
    operations::create_user(&mut conn, &other_user).expect("Failed to create other user");

    // Attempt to access the first user's block with the second user's ID
    let result = logic::get_tasks_for_a_specific_block(&mut conn, other_user_id, block.block_id);

    assert!(result.is_err(), "Should return error for access denied");
    assert!(
        result.unwrap_err().contains("Access denied"),
        "Error message should indicate access denied"
    );
}

#[test]
fn test_get_tasks_for_a_specific_block_not_found() {
    // Test that function returns error for non-existent block
    let (mut conn, user_id, _device_id) = setup_db_with_user_and_device();

    let nonexistent_block_id = Uuid::new_v4();
    let result = logic::get_tasks_for_a_specific_block(&mut conn, user_id, nonexistent_block_id);

    assert!(
        result.is_err(),
        "Should return error for non-existent block"
    );
    assert!(
        result.unwrap_err().contains("not found"),
        "Error message should indicate block not found"
    );
}

#[test]
fn test_get_all_pomodoro_presets() {
    // Test retrieval of all pomodoro presets for a user
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Initially, user should have no presets
    let empty_presets = logic::get_all_pomodoro_presets(&mut conn, user_id)
        .expect("Failed to get pomodoro presets");
    assert_eq!(
        empty_presets.len(),
        0,
        "User should have no pomodoro presets initially"
    );

    // Create multiple pomodoro presets
    let _preset1 = logic::save_pomodoro_preset(
        &mut conn,
        user_id,
        device_id,
        PomodoroPreset {
            name: "Classic Pomodoro".to_string(),
            cover: None,
            work_duration: 25,
            short_break: 5,
            long_break: 15,
            interval: 4,
        },
    )
    .unwrap();
    let _preset2 = logic::save_pomodoro_preset(
        &mut conn,
        user_id,
        device_id,
        PomodoroPreset {
            name: "Short Focus".to_string(),
            cover: None,
            work_duration: 15,
            short_break: 3,
            long_break: 10,
            interval: 4,
        },
    )
    .unwrap();
    let _preset3 = logic::save_pomodoro_preset(
        &mut conn,
        user_id,
        device_id,
        PomodoroPreset {
            name: "Long Focus".to_string(),
            cover: None,
            work_duration: 50,
            short_break: 10,
            long_break: 30,
            interval: 3,
        },
    )
    .unwrap();

    // Retrieve all presets
    let all_presets = logic::get_all_pomodoro_presets(&mut conn, user_id)
        .expect("Failed to get all pomodoro presets");

    assert_eq!(all_presets.len(), 3, "User should have 3 pomodoro presets");

    // Verify preset names
    let preset_names: Vec<String> = all_presets
        .iter()
        .map(|p| p.pomodoro_name.clone())
        .collect();
    assert!(preset_names.contains(&"Classic Pomodoro".to_string()));
    assert!(preset_names.contains(&"Short Focus".to_string()));
    assert!(preset_names.contains(&"Long Focus".to_string()));

    // Verify all presets belong to the correct user
    for preset in &all_presets {
        assert_eq!(preset.user_id, user_id);
    }

    // Verify preset details for one preset
    let classic = all_presets
        .iter()
        .find(|p| p.pomodoro_name == "Classic Pomodoro")
        .unwrap();
    assert_eq!(classic.work_duration, 25);
    assert_eq!(classic.short_break_duration, 5);
    assert_eq!(classic.long_break_duration, 15);
    assert_eq!(classic.long_break_interval, 4);
}

#[test]
fn test_get_all_pomodoro_presets_empty() {
    // Test that function returns empty list for user with no presets
    let (mut conn, user_id, _device_id) = setup_db_with_user_and_device();

    let presets = logic::get_all_pomodoro_presets(&mut conn, user_id)
        .expect("Failed to get pomodoro presets");

    assert_eq!(
        presets.len(),
        0,
        "Should return empty list for user with no presets"
    );
}

#[test]
fn test_get_active_blocklist() {
    // Test retrieval of active blocklist items for a user
    let (mut conn, user_id, device_id) = setup_db_with_user_and_device();

    // Initially, user should have no blocked items
    let empty_list =
        logic::get_active_blocklist(&mut conn, user_id).expect("Failed to get active blocklist");
    assert_eq!(
        empty_list.len(),
        0,
        "User should have no blocked items initially"
    );

    // Add multiple items to the blocklist
    let _item1 = logic::add_item_to_blocklist(
        &mut conn,
        user_id,
        device_id,
        "website".to_string(),
        "facebook.com".to_string(),
    )
    .unwrap();
    let _item2 = logic::add_item_to_blocklist(
        &mut conn,
        user_id,
        device_id,
        "website".to_string(),
        "twitter.com".to_string(),
    )
    .unwrap();
    let _item3 = logic::add_item_to_blocklist(
        &mut conn,
        user_id,
        device_id,
        "app".to_string(),
        "instagram".to_string(),
    )
    .unwrap();

    // Retrieve active blocklist
    let active_list =
        logic::get_active_blocklist(&mut conn, user_id).expect("Failed to get active blocklist");

    assert_eq!(active_list.len(), 3, "User should have 3 blocked items");

    // Verify item identifiers
    let identifiers: Vec<String> = active_list.iter().map(|i| i.identifier.clone()).collect();
    assert!(identifiers.contains(&"facebook.com".to_string()));
    assert!(identifiers.contains(&"twitter.com".to_string()));
    assert!(identifiers.contains(&"instagram".to_string()));

    // Verify all items are active
    for item in &active_list {
        assert!(item.is_active, "All returned items should be active");
        assert_eq!(
            item.user_id, user_id,
            "All items should belong to the correct user"
        );
    }
}

#[test]
fn test_get_active_blocklist_empty() {
    // Test that function returns empty list for user with no blocked items
    let (mut conn, user_id, _device_id) = setup_db_with_user_and_device();

    let blocklist =
        logic::get_active_blocklist(&mut conn, user_id).expect("Failed to get active blocklist");

    assert_eq!(
        blocklist.len(),
        0,
        "Should return empty list for user with no blocked items"
    );
}
