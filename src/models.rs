//! Core data models for nexus-core database synchronization infrastructure.
//!
//! This module contains the essential types for P2P database synchronization:
//! - User and Device models for authentication and device management
//! - OplogEntry for CRDT-based operation logging
//! - Peer for P2P network peer tracking

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User account for device ownership and authentication
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub user_name: String,
    pub user_password_hash: String,
    pub user_mail: String,
    pub created_at: DateTime<Utc>,
}

/// Device registered to a user for synchronization
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Device {
    pub device_id: Uuid,
    pub user_id: Uuid,
    pub device_type: String,
    pub push_token: Option<String>,
    pub last_seen: Option<DateTime<Utc>>,
}

/// Operation log entry for CRDT-based synchronization.
///
/// Each entry represents a single operation (create, update, delete) on any table,
/// enabling conflict-free replication across devices using hybrid logical clocks.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OplogEntry {
    /// Unique ID for the operation itself
    pub id: Uuid,
    /// The device that created the operation
    pub device_id: Uuid,
    /// Hybrid Logical Clock (HLC) timestamp for causal ordering
    pub timestamp: i64,
    /// Table name (e.g., "users", "app_data")
    pub table: String,
    /// Operation type (e.g., "create", "update", "delete")
    pub op_type: String,
    /// The full JSON representation of the entity
    pub data: serde_json::Value,
}

/// Peer device in the P2P synchronization network
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Peer {
    pub peer_id: Uuid,
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub last_known_ip: Option<String>,
    pub last_sync_time: Option<i64>,
}
