//! Database operations for nexus-core synchronization infrastructure.
//!
//! This module provides CRUD operations for the core sync types:
//! - Users: Authentication and device ownership
//! - Devices: Device registration and tracking
//! - OplogEntry: Operation log for CRDT synchronization
//! - Peer: P2P network peer management

use crate::models::{Device, OplogEntry, Peer, User};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result, Row, params, types::Type};
use uuid::Uuid;

/// Initialize the database with migrations
pub fn initialize_database(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    // Apply all pending migrations
    // This will create tables if they don't exist (new database)
    // or upgrade the schema to the latest version (existing database)
    super::migrations::apply_migrations(&conn)?;

    Ok(conn)
}

// ============================================================================
// Helper Functions
// ============================================================================

fn conversion_failure<E>(column_index: usize, err: E) -> rusqlite::Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    rusqlite::Error::FromSqlConversionFailure(column_index, Type::Text, Box::new(err))
}

fn parse_uuid_column(row: &Row, idx: usize) -> rusqlite::Result<Uuid> {
    let value: String = row.get(idx)?;
    Uuid::parse_str(&value).map_err(|e| conversion_failure(idx, e))
}

fn parse_datetime_column(row: &Row, idx: usize) -> rusqlite::Result<DateTime<Utc>> {
    let value: String = row.get(idx)?;
    DateTime::parse_from_rfc3339(&value)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| conversion_failure(idx, e))
}

fn parse_optional_datetime_column(
    row: &Row,
    idx: usize,
) -> rusqlite::Result<Option<DateTime<Utc>>> {
    let value: Option<String> = row.get(idx)?;
    match value {
        Some(raw) => DateTime::parse_from_rfc3339(&raw)
            .map(|dt| dt.with_timezone(&Utc))
            .map(Some)
            .map_err(|e| conversion_failure(idx, e)),
        None => Ok(None),
    }
}

// ============================================================================
// Row Mappers
// ============================================================================

fn row_to_user(row: &Row) -> rusqlite::Result<User> {
    Ok(User {
        user_id: parse_uuid_column(row, 0)?,
        user_name: row.get(1)?,
        user_password_hash: row.get(2)?,
        user_mail: row.get(3)?,
        created_at: parse_datetime_column(row, 4)?,
    })
}

fn row_to_device(row: &Row) -> rusqlite::Result<Device> {
    Ok(Device {
        device_id: parse_uuid_column(row, 0)?,
        user_id: parse_uuid_column(row, 1)?,
        device_type: row.get(2)?,
        push_token: row.get(3)?,
        last_seen: parse_optional_datetime_column(row, 4)?,
    })
}

fn row_to_oplog_entry(row: &Row) -> rusqlite::Result<OplogEntry> {
    let data_raw: String = row.get(5)?;
    let data = serde_json::from_str(&data_raw).map_err(|e| conversion_failure(5, e))?;

    Ok(OplogEntry {
        id: parse_uuid_column(row, 0)?,
        device_id: parse_uuid_column(row, 1)?,
        timestamp: row.get(2)?,
        table: row.get(3)?,
        op_type: row.get(4)?,
        data,
    })
}

fn row_to_peer(row: &Row) -> rusqlite::Result<Peer> {
    Ok(Peer {
        peer_id: parse_uuid_column(row, 0)?,
        user_id: parse_uuid_column(row, 1)?,
        device_id: parse_uuid_column(row, 2)?,
        last_known_ip: row.get(3)?,
        last_sync_time: row.get(4)?,
    })
}

// ============================================================================
// User Operations
// ============================================================================

/// Create a new user
pub fn create_user(conn: &Connection, user: &User) -> Result<()> {
    conn.execute(
        "INSERT INTO users (user_id, user_name, user_password, user_mail, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            &user.user_id.to_string(),
            &user.user_name,
            &user.user_password_hash,
            &user.user_mail,
            &user.created_at.to_rfc3339()
        ],
    )?;
    Ok(())
}

/// Get user by ID
pub fn get_user(conn: &Connection, user_id: Uuid) -> Result<Option<User>> {
    let mut stmt = conn.prepare(
        "SELECT user_id, user_name, user_password, user_mail, created_at FROM users WHERE user_id = ?1"
    )?;
    let mut rows = stmt.query_map(params![user_id.to_string()], row_to_user)?;
    rows.next().transpose()
}

/// Get user by username
pub fn get_user_by_name(conn: &Connection, user_name: &str) -> Result<Option<User>> {
    let mut stmt = conn.prepare(
        "SELECT user_id, user_name, user_password, user_mail, created_at FROM users WHERE user_name = ?1"
    )?;
    let mut rows = stmt.query_map(params![user_name], row_to_user)?;
    rows.next().transpose()
}

/// Get user by email
pub fn get_user_by_mail(conn: &Connection, user_mail: &str) -> Result<Option<User>> {
    let mut stmt = conn.prepare(
        "SELECT user_id, user_name, user_password, user_mail, created_at FROM users WHERE user_mail = ?1"
    )?;
    let mut rows = stmt.query_map(params![user_mail], row_to_user)?;
    rows.next().transpose()
}

// ============================================================================
// Device Operations
// ============================================================================

/// Create a new device
pub fn create_device(conn: &Connection, device: &Device) -> Result<()> {
    conn.execute(
        "INSERT INTO devices (device_id, user_id, device_type, push_token, last_seen) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            &device.device_id.to_string(),
            &device.user_id.to_string(),
            &device.device_type,
            &device.push_token,
            &device.last_seen.map(|dt| dt.to_rfc3339())
        ],
    )?;
    Ok(())
}

/// Get device by ID
pub fn get_device(conn: &Connection, device_id: Uuid) -> Result<Option<Device>> {
    let mut stmt = conn.prepare(
        "SELECT device_id, user_id, device_type, push_token, last_seen FROM devices WHERE device_id = ?1"
    )?;
    let mut rows = stmt.query_map(params![device_id.to_string()], row_to_device)?;
    rows.next().transpose()
}

/// Get all devices for a user
pub fn get_devices_by_user_id(conn: &Connection, user_id: Uuid) -> Result<Vec<Device>> {
    let mut stmt = conn.prepare(
        "SELECT device_id, user_id, device_type, push_token, last_seen FROM devices WHERE user_id = ?1"
    )?;
    let rows = stmt.query_map(params![user_id.to_string()], row_to_device)?;

    let mut devices = Vec::new();
    for row in rows {
        devices.push(row?);
    }

    Ok(devices)
}

/// Update device last seen timestamp
pub fn update_device_last_seen(
    conn: &Connection,
    device_id: Uuid,
    last_seen: DateTime<Utc>,
) -> Result<usize> {
    conn.execute(
        "UPDATE devices SET last_seen = ?1 WHERE device_id = ?2",
        params![last_seen.to_rfc3339(), device_id.to_string()],
    )
}

// ============================================================================
// OplogEntry Operations
// ============================================================================

/// Create a new operation log entry
pub fn create_oplog_entry(conn: &Connection, entry: &OplogEntry) -> Result<()> {
    let data = serde_json::to_string(&entry.data).map_err(|e| conversion_failure(5, e))?;

    conn.execute(
        "INSERT INTO oplog (id, device_id, timestamp, table_name, op_type, data) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            &entry.id.to_string(),
            &entry.device_id.to_string(),
            entry.timestamp,
            &entry.table,
            &entry.op_type,
            &data,
        ],
    )?;
    Ok(())
}

/// Get all oplog entries since a timestamp
pub fn get_oplog_entries_since(conn: &Connection, since: i64) -> Result<Vec<OplogEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, device_id, timestamp, table_name, op_type, data FROM oplog WHERE timestamp > ?1 ORDER BY timestamp ASC",
    )?;
    let rows = stmt.query_map(params![since], row_to_oplog_entry)?;

    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }

    Ok(entries)
}

// ============================================================================
// Peer Operations
// ============================================================================

/// Create a new peer
pub fn create_peer(conn: &Connection, peer: &Peer) -> Result<()> {
    conn.execute(
        "INSERT INTO peers (peer_id, user_id, device_id, last_known_ip, last_sync_time) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            &peer.peer_id.to_string(),
            &peer.user_id.to_string(),
            &peer.device_id.to_string(),
            &peer.last_known_ip,
            &peer.last_sync_time
        ],
    )?;
    Ok(())
}

/// Get peer by ID
pub fn get_peer(conn: &Connection, peer_id: Uuid) -> Result<Peer> {
    let mut stmt = conn.prepare(
        "SELECT peer_id, user_id, device_id, last_known_ip, last_sync_time FROM peers WHERE peer_id = ?1"
    )?;
    let peer = stmt.query_row(params![peer_id.to_string()], row_to_peer)?;
    Ok(peer)
}

/// Get all peers for a user
pub fn get_peers_by_user_id(conn: &Connection, user_id: Uuid) -> Result<Vec<Peer>> {
    let mut stmt = conn.prepare(
        "SELECT peer_id, user_id, device_id, last_known_ip, last_sync_time FROM peers WHERE user_id = ?1"
    )?;
    let rows = stmt.query_map(params![user_id.to_string()], row_to_peer)?;

    let mut peers = Vec::new();
    for row in rows {
        peers.push(row?);
    }

    Ok(peers)
}

/// Get all peers in the database
pub fn get_all_peers(conn: &Connection) -> Result<Vec<Peer>> {
    let mut stmt = conn
        .prepare("SELECT peer_id, user_id, device_id, last_known_ip, last_sync_time FROM peers")?;
    let rows = stmt.query_map(params![], row_to_peer)?;

    let mut peers = Vec::new();
    for row in rows {
        peers.push(row?);
    }

    Ok(peers)
}
