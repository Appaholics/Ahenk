use chrono::Utc;
use nexus_core::db::operations;
use nexus_core::models::{Device, FavoriteSound, OplogEntry, Peer, Sound, User};
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
}

#[test]
fn test_device_crud_operations() {
    // Create a temporary database
    let conn =
        operations::initialize_database(":memory:").expect("Failed to create in-memory database");

    // Create test device
    let device_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
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
fn test_sound_crud_operations() {
    // Create a temporary database
    let conn =
        operations::initialize_database(":memory:").expect("Failed to create in-memory database");

    // Create test sounds
    let sound1_id = Uuid::new_v4();
    let sound1 = Sound {
        sound_id: sound1_id,
        name: "Ocean Waves".to_string(),
        category: Some("Nature".to_string()),
        file_url: "https://example.com/ocean.mp3".to_string(),
    };

    let sound2_id = Uuid::new_v4();
    let sound2 = Sound {
        sound_id: sound2_id,
        name: "Rain".to_string(),
        category: Some("Nature".to_string()),
        file_url: "https://example.com/rain.mp3".to_string(),
    };

    let sound3_id = Uuid::new_v4();
    let sound3 = Sound {
        sound_id: sound3_id,
        name: "White Noise".to_string(),
        category: Some("Ambient".to_string()),
        file_url: "https://example.com/whitenoise.mp3".to_string(),
    };

    // Test CREATE
    operations::create_sound(&conn, &sound1).expect("Failed to create sound1");
    operations::create_sound(&conn, &sound2).expect("Failed to create sound2");
    operations::create_sound(&conn, &sound3).expect("Failed to create sound3");

    // Test READ by sound_id
    let retrieved_sound = operations::get_sound(&conn, sound1_id)
        .expect("Failed to get sound")
        .expect("Sound not found");

    assert_eq!(retrieved_sound.sound_id, sound1_id);
    assert_eq!(retrieved_sound.name, "Ocean Waves");
    assert_eq!(retrieved_sound.category, Some("Nature".to_string()));

    // Test READ all sounds
    let all_sounds = operations::get_all_sounds(&conn).expect("Failed to get all sounds");

    assert_eq!(all_sounds.len(), 3);

    // Test READ by category
    let nature_sounds = operations::get_sounds_by_category(&conn, "Nature")
        .expect("Failed to get sounds by category");

    assert_eq!(nature_sounds.len(), 2);

    let ambient_sounds =
        operations::get_sounds_by_category(&conn, "Ambient").expect("Failed to get ambient sounds");

    assert_eq!(ambient_sounds.len(), 1);
    assert_eq!(ambient_sounds[0].name, "White Noise");
}

#[test]
fn test_favorite_sound_crud_operations() {
    // Create a temporary database
    let conn =
        operations::initialize_database(":memory:").expect("Failed to create in-memory database");

    // Create test user and sounds
    let user_id = Uuid::new_v4();
    let sound1_id = Uuid::new_v4();
    let sound2_id = Uuid::new_v4();

    let sound1 = Sound {
        sound_id: sound1_id,
        name: "Forest".to_string(),
        category: Some("Nature".to_string()),
        file_url: "https://example.com/forest.mp3".to_string(),
    };

    let sound2 = Sound {
        sound_id: sound2_id,
        name: "Thunder".to_string(),
        category: Some("Nature".to_string()),
        file_url: "https://example.com/thunder.mp3".to_string(),
    };

    operations::create_sound(&conn, &sound1).expect("Failed to create sound1");
    operations::create_sound(&conn, &sound2).expect("Failed to create sound2");

    // Test CREATE favorite
    let favorite1 = FavoriteSound {
        user_id,
        sound_id: sound1_id,
    };
    let favorite2 = FavoriteSound {
        user_id,
        sound_id: sound2_id,
    };

    operations::create_favorite_sound(&conn, &favorite1).expect("Failed to create favorite1");
    operations::create_favorite_sound(&conn, &favorite2).expect("Failed to create favorite2");

    // Test READ favorites by user_id
    let favorites = operations::get_favorite_sounds_by_user_id(&conn, user_id)
        .expect("Failed to get favorite sounds");

    assert_eq!(favorites.len(), 2);

    // Verify the favorite sounds contain the correct data
    let favorite_names: Vec<String> = favorites.iter().map(|s| s.name.clone()).collect();
    assert!(favorite_names.contains(&"Forest".to_string()));
    assert!(favorite_names.contains(&"Thunder".to_string()));

    // Test DELETE favorite
    operations::delete_favorite_sound(&conn, user_id, sound1_id)
        .expect("Failed to delete favorite sound");

    let remaining_favorites = operations::get_favorite_sounds_by_user_id(&conn, user_id)
        .expect("Failed to get remaining favorites");

    assert_eq!(remaining_favorites.len(), 1);
    assert_eq!(remaining_favorites[0].name, "Thunder");
}

#[test]
fn test_oplog_and_peer_crud_operations() {
    let conn =
        operations::initialize_database(":memory:").expect("Failed to create in-memory database");

    let user_id = Uuid::new_v4();
    let user = User {
        user_id,
        user_name: "sync_user".to_string(),
        user_password_hash: "sync_password".to_string(),
        user_mail: "sync@example.com".to_string(),
        created_at: Utc::now(),
    };
    operations::create_user(&conn, &user).expect("Failed to create user");

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
        table: "tasks".to_string(),
        op_type: "create".to_string(),
        data: serde_json::json!({
            "content": "New Task Content",
            "task_id": Uuid::new_v4().to_string(),
        }),
    };
    operations::create_oplog_entry(&conn, &oplog_entry).expect("Failed to create oplog entry");

    // Test OplogEntry READ
    let entries =
        operations::get_oplog_entries_since(&conn, chrono::Utc::now().timestamp_millis() - 60000)
            .unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].table, "tasks");

    // Test Peer CREATE
    let peer = Peer {
        peer_id: Uuid::new_v4(),
        user_id,
        device_id,
        last_known_ip: Some("192.168.1.100".to_string()),
        last_sync_time: Some(Utc::now().timestamp_millis()),
    };
    operations::create_peer(&conn, &peer).expect("Failed to create peer");

    // Test Peer READ
    let peers = operations::get_peers_by_user_id(&conn, user_id).unwrap();
    assert_eq!(peers.len(), 1);
    assert_eq!(peers[0].last_known_ip, Some("192.168.1.100".to_string()));
}

#[test]
fn test_get_task_lists_by_user_id() {
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
    operations::create_user(&conn, &user).expect("Failed to create user");

    // Test with no task lists (empty result)
    let empty_lists =
        operations::get_task_lists_by_user_id(&conn, user_id).expect("Failed to get task lists");
    assert_eq!(
        empty_lists.len(),
        0,
        "Should return empty list when no task lists exist"
    );

    // Create multiple task lists for the user
    let list1_id = Uuid::new_v4();
    let list1 = nexus_core::models::TaskList {
        list_id: list1_id,
        user_id,
        name: "Work Tasks".to_string(),
    };
    operations::create_task_list(&conn, &list1).expect("Failed to create list1");

    let list2_id = Uuid::new_v4();
    let list2 = nexus_core::models::TaskList {
        list_id: list2_id,
        user_id,
        name: "Personal Tasks".to_string(),
    };
    operations::create_task_list(&conn, &list2).expect("Failed to create list2");

    let list3_id = Uuid::new_v4();
    let list3 = nexus_core::models::TaskList {
        list_id: list3_id,
        user_id,
        name: "Shopping List".to_string(),
    };
    operations::create_task_list(&conn, &list3).expect("Failed to create list3");

    // Test READ - should return all 3 task lists
    let retrieved_lists =
        operations::get_task_lists_by_user_id(&conn, user_id).expect("Failed to get task lists");

    assert_eq!(
        retrieved_lists.len(),
        3,
        "Should return all 3 task lists for the user"
    );

    // Verify all list names are present
    let list_names: Vec<String> = retrieved_lists.iter().map(|l| l.name.clone()).collect();
    assert!(list_names.contains(&"Work Tasks".to_string()));
    assert!(list_names.contains(&"Personal Tasks".to_string()));
    assert!(list_names.contains(&"Shopping List".to_string()));

    // Verify all lists belong to the correct user
    for list in &retrieved_lists {
        assert_eq!(
            list.user_id, user_id,
            "All lists should belong to the test user"
        );
    }

    // Test with different user - should return empty result
    let other_user_id = Uuid::new_v4();
    let empty_for_other_user = operations::get_task_lists_by_user_id(&conn, other_user_id)
        .expect("Failed to get task lists for other user");
    assert_eq!(
        empty_for_other_user.len(),
        0,
        "Should return empty list for user with no task lists"
    );
}

#[test]
fn test_get_tasks_due_on_date_for_user() {
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
    operations::create_user(&conn, &user).expect("Failed to create user");

    // Create task list
    let list_id = Uuid::new_v4();
    let task_list = nexus_core::models::TaskList {
        list_id,
        user_id,
        name: "Test List".to_string(),
    };
    operations::create_task_list(&conn, &task_list).expect("Failed to create task list");

    // Define test dates
    let today = chrono::NaiveDate::from_ymd_opt(2025, 10, 21).unwrap();
    let tomorrow = chrono::NaiveDate::from_ymd_opt(2025, 10, 22).unwrap();
    let yesterday = chrono::NaiveDate::from_ymd_opt(2025, 10, 20).unwrap();

    // Test with no tasks (empty result)
    let empty_tasks = operations::get_tasks_due_on_date_for_user(&conn, user_id, today)
        .expect("Failed to get tasks");
    assert_eq!(
        empty_tasks.len(),
        0,
        "Should return empty list when no tasks exist"
    );

    // Create tasks with different due dates
    let task1_id = Uuid::new_v4();
    let task1 = nexus_core::models::Task {
        task_id: task1_id,
        list_id,
        content: "Task due today 1".to_string(),
        is_completed: false,
        due_date: Some(today),
        created_at: Utc::now(),
        updated_at: None,
    };
    operations::create_task(&conn, &task1).expect("Failed to create task1");

    let task2_id = Uuid::new_v4();
    let task2 = nexus_core::models::Task {
        task_id: task2_id,
        list_id,
        content: "Task due today 2".to_string(),
        is_completed: false,
        due_date: Some(today),
        created_at: Utc::now(),
        updated_at: None,
    };
    operations::create_task(&conn, &task2).expect("Failed to create task2");

    let task3_id = Uuid::new_v4();
    let task3 = nexus_core::models::Task {
        task_id: task3_id,
        list_id,
        content: "Task due tomorrow".to_string(),
        is_completed: false,
        due_date: Some(tomorrow),
        created_at: Utc::now(),
        updated_at: None,
    };
    operations::create_task(&conn, &task3).expect("Failed to create task3");

    let task4_id = Uuid::new_v4();
    let task4 = nexus_core::models::Task {
        task_id: task4_id,
        list_id,
        content: "Task due yesterday".to_string(),
        is_completed: false,
        due_date: Some(yesterday),
        created_at: Utc::now(),
        updated_at: None,
    };
    operations::create_task(&conn, &task4).expect("Failed to create task4");

    let task5_id = Uuid::new_v4();
    let task5 = nexus_core::models::Task {
        task_id: task5_id,
        list_id,
        content: "Task with no due date".to_string(),
        is_completed: false,
        due_date: None,
        created_at: Utc::now(),
        updated_at: None,
    };
    operations::create_task(&conn, &task5).expect("Failed to create task5");

    // Test READ - tasks due today
    let tasks_due_today = operations::get_tasks_due_on_date_for_user(&conn, user_id, today)
        .expect("Failed to get tasks due today");

    assert_eq!(
        tasks_due_today.len(),
        2,
        "Should return exactly 2 tasks due today"
    );

    // Verify the correct tasks are returned
    let task_contents: Vec<String> = tasks_due_today.iter().map(|t| t.content.clone()).collect();
    assert!(task_contents.contains(&"Task due today 1".to_string()));
    assert!(task_contents.contains(&"Task due today 2".to_string()));

    // Verify all returned tasks have the correct due date
    for task in &tasks_due_today {
        assert_eq!(
            task.due_date,
            Some(today),
            "All returned tasks should have due date set to today"
        );
    }

    // Test READ - tasks due tomorrow (should return 1 task)
    let tasks_due_tomorrow = operations::get_tasks_due_on_date_for_user(&conn, user_id, tomorrow)
        .expect("Failed to get tasks due tomorrow");
    assert_eq!(
        tasks_due_tomorrow.len(),
        1,
        "Should return exactly 1 task due tomorrow"
    );
    assert_eq!(tasks_due_tomorrow[0].content, "Task due tomorrow");

    // Test READ - tasks due yesterday (should return 1 task)
    let tasks_due_yesterday = operations::get_tasks_due_on_date_for_user(&conn, user_id, yesterday)
        .expect("Failed to get tasks due yesterday");
    assert_eq!(
        tasks_due_yesterday.len(),
        1,
        "Should return exactly 1 task due yesterday"
    );
    assert_eq!(tasks_due_yesterday[0].content, "Task due yesterday");

    // Test READ - date with no tasks
    let future_date = chrono::NaiveDate::from_ymd_opt(2025, 12, 25).unwrap();
    let no_tasks = operations::get_tasks_due_on_date_for_user(&conn, user_id, future_date)
        .expect("Failed to get tasks for future date");
    assert_eq!(
        no_tasks.len(),
        0,
        "Should return empty list for date with no tasks"
    );

    // Test with different user - should return no tasks
    let other_user_id = Uuid::new_v4();
    let no_tasks_for_other_user =
        operations::get_tasks_due_on_date_for_user(&conn, other_user_id, today)
            .expect("Failed to get tasks for other user");
    assert_eq!(
        no_tasks_for_other_user.len(),
        0,
        "Should return empty list for different user"
    );
}
