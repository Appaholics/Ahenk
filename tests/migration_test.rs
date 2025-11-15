//! Migration system tests for ahenk.
//!
//! These tests verify:
//! - Fresh database initialization
//! - Migration application and version tracking
//! - Schema correctness
//! - Data preservation across migrations

use ahenk::db::migrations::{apply_migrations, get_current_version, get_migration_history};
use ahenk::db::operations::{create_user, get_user};
use ahenk::models::User;
use chrono::Utc;
use rusqlite::Connection;
use uuid::Uuid;

#[test]
fn test_fresh_database_migration() {
    // Test that a fresh database gets all migrations applied
    let conn = Connection::open_in_memory().unwrap();

    // Apply migrations
    apply_migrations(&conn).unwrap();

    // Verify schema version
    let version = get_current_version(&conn).unwrap();
    assert_eq!(version, 1, "Fresh database should be at version 1");

    // Verify core tables exist by checking sqlite_master
    let table_count: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
            [],
            |row| row.get(0),
        )
        .unwrap();

    // We should have: users, devices, oplog, peers, schema_version = 5 tables
    assert_eq!(table_count, 5, "Should have 5 tables in core sync schema");
}

#[test]
fn test_migration_creates_functional_database() {
    // Test that migrated database is fully functional
    let conn = Connection::open_in_memory().unwrap();
    apply_migrations(&conn).unwrap();

    // Create a user to verify database is functional
    let user = User {
        user_id: Uuid::new_v4(),
        user_name: "testuser".to_string(),
        user_password_hash: "hash".to_string(),
        user_mail: "test@example.com".to_string(),
        created_at: Utc::now(),
    };
    create_user(&conn, &user).unwrap();

    // Verify user can be retrieved
    let retrieved = get_user(&conn, user.user_id).unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().user_name, "testuser");
}

#[test]
fn test_idempotent_migrations() {
    // Test that running migrations multiple times is safe
    let conn = Connection::open_in_memory().unwrap();

    // Apply migrations first time
    apply_migrations(&conn).unwrap();
    let version1 = get_current_version(&conn).unwrap();

    // Apply migrations again (should be a no-op)
    apply_migrations(&conn).unwrap();
    let version2 = get_current_version(&conn).unwrap();

    assert_eq!(
        version1, version2,
        "Version should not change on re-applying migrations"
    );

    // Verify no duplicate data or errors
    let table_count: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(table_count, 5);
}

#[test]
fn test_migration_history_tracking() {
    // Test that migration history is correctly tracked
    let conn = Connection::open_in_memory().unwrap();
    apply_migrations(&conn).unwrap();

    let history = get_migration_history(&conn).unwrap();

    // Verify history entries exist
    assert!(!history.is_empty(), "Migration history should not be empty");

    // Verify first migration
    let (version, _timestamp, description) = &history[0];
    assert_eq!(*version, 1);
    assert!(description.contains("Initial") || description.contains("schema"));

    // Verify migrations are in order
    for i in 0..history.len() - 1 {
        assert!(
            history[i].0 < history[i + 1].0,
            "Migration versions should be ordered"
        );
    }
}

#[test]
fn test_schema_version_table_structure() {
    // Test the schema_version table has correct columns
    let conn = Connection::open_in_memory().unwrap();
    apply_migrations(&conn).unwrap();

    // Query the table structure
    let mut stmt = conn.prepare("PRAGMA table_info(schema_version)").unwrap();

    let columns: Vec<String> = stmt
        .query_map([], |row| row.get(1))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    assert!(columns.contains(&"version".to_string()));
    assert!(columns.contains(&"applied_at".to_string()));
    assert!(columns.contains(&"description".to_string()));
}

#[test]
fn test_foreign_key_constraints() {
    // Test that foreign key constraints are properly set up
    let conn = Connection::open_in_memory().unwrap();

    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", []).unwrap();

    apply_migrations(&conn).unwrap();

    // Create a user
    let user_id = Uuid::new_v4();
    let user = User {
        user_id,
        user_name: "fktest".to_string(),
        user_password_hash: "hash".to_string(),
        user_mail: "fk@example.com".to_string(),
        created_at: Utc::now(),
    };
    create_user(&conn, &user).unwrap();

    // Try to insert device with invalid user_id (should fail)
    let invalid_user_id = Uuid::new_v4();
    let result = conn.execute(
        "INSERT INTO devices (device_id, user_id, device_type) VALUES (?1, ?2, ?3)",
        [
            invalid_user_id.to_string(),
            invalid_user_id.to_string(),
            "test".to_string(),
        ],
    );

    assert!(
        result.is_err(),
        "Should fail to insert device with nonexistent user_id"
    );
}

#[test]
fn test_all_required_core_tables_exist() {
    // Test that all required core sync tables are created
    let conn = Connection::open_in_memory().unwrap();
    apply_migrations(&conn).unwrap();

    let required_tables = vec![
        "users",          // User authentication
        "devices",        // Device management
        "oplog",          // CRDT operation log
        "peers",          // P2P peer tracking
        "schema_version", // Migration tracking
    ];

    for table in required_tables {
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                [table],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(count, 1, "Core table '{}' should exist", table);
    }
}

#[test]
fn test_database_upgrade_preserves_data() {
    // Simulate upgrading a database: create data, apply migrations again, verify data intact
    let conn = Connection::open_in_memory().unwrap();

    // Apply initial migrations
    apply_migrations(&conn).unwrap();

    // Create test data
    let user = User {
        user_id: Uuid::new_v4(),
        user_name: "upgradetest".to_string(),
        user_password_hash: "hash".to_string(),
        user_mail: "upgrade@example.com".to_string(),
        created_at: Utc::now(),
    };
    create_user(&conn, &user).unwrap();

    // Create a device
    conn.execute(
        "INSERT INTO devices (device_id, user_id, device_type) VALUES (?1, ?2, ?3)",
        [
            Uuid::new_v4().to_string(),
            user.user_id.to_string(),
            "test_device".to_string(),
        ],
    )
    .unwrap();

    // "Simulate" a restart/upgrade by re-applying migrations (should be idempotent)
    apply_migrations(&conn).unwrap();

    // Verify data is still intact
    let retrieved_user = get_user(&conn, user.user_id).unwrap();
    assert!(retrieved_user.is_some());
    assert_eq!(retrieved_user.unwrap().user_name, "upgradetest");

    // Verify device still exists
    let device_count: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM devices WHERE user_id = ?1",
            [user.user_id.to_string()],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(device_count, 1, "Device should still exist after upgrade");
}

#[test]
fn test_users_table_structure() {
    // Verify users table has correct columns
    let conn = Connection::open_in_memory().unwrap();
    apply_migrations(&conn).unwrap();

    let mut stmt = conn.prepare("PRAGMA table_info(users)").unwrap();
    let columns: Vec<String> = stmt
        .query_map([], |row| row.get(1))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    assert!(columns.contains(&"user_id".to_string()));
    assert!(columns.contains(&"user_name".to_string()));
    assert!(columns.contains(&"user_password".to_string()));
    assert!(columns.contains(&"user_mail".to_string()));
    assert!(columns.contains(&"created_at".to_string()));
}

#[test]
fn test_oplog_table_structure() {
    // Verify oplog table has correct columns for CRDT-based sync infrastructure
    let conn = Connection::open_in_memory().unwrap();
    apply_migrations(&conn).unwrap();

    let mut stmt = conn.prepare("PRAGMA table_info(oplog)").unwrap();
    let columns: Vec<String> = stmt
        .query_map([], |row| row.get(1))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    // Verify simplified CRDT oplog columns
    assert!(columns.contains(&"id".to_string()));
    assert!(columns.contains(&"device_id".to_string()));
    assert!(columns.contains(&"timestamp".to_string()));
    assert!(columns.contains(&"table_name".to_string()));
    assert!(columns.contains(&"op_type".to_string()));
    assert!(columns.contains(&"data".to_string()));
}
