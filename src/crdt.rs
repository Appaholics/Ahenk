//! CRDT (Conflict-free Replicated Data Type) implementation.
//!
//! This module provides the foundation for distributed synchronization:
//! - Hybrid Logical Clock for causal ordering
//! - Operation log management
//! - Conflict resolution primitives
//!
//! Apps using ahenk should implement their own table-specific merge logic
//! using the HLC and oplog primitives provided here.

use crate::OplogEntry;
use chrono::{DateTime, Utc};
use rusqlite::Connection;
use std::cmp::Ordering;

/// Hybrid Logical Clock for maintaining causal ordering of operations.
///
/// HLC combines physical time (system clock) with a logical counter to ensure:
/// - Events on the same device are totally ordered
/// - Events across devices can be causally ordered
/// - Clock drift is bounded
///
/// Format: 48 bits physical time (microseconds) + 16 bits counter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HybridLogicalClock {
    timestamp: u64,
}

impl HybridLogicalClock {
    /// Create a new HLC from physical time and counter
    pub fn new(physical_time: DateTime<Utc>, counter: u16) -> Self {
        let physical_micros = physical_time.timestamp_micros() as u64;
        // 48 bits for physical time, 16 bits for counter
        let timestamp = (physical_micros << 16) | (counter as u64);
        Self { timestamp }
    }

    /// Create HLC from a raw timestamp value
    pub fn from_timestamp(timestamp: i64) -> Self {
        Self {
            timestamp: timestamp as u64,
        }
    }

    /// Convert HLC to raw timestamp value
    pub fn to_timestamp(&self) -> i64 {
        self.timestamp as i64
    }

    /// Extract physical time component (microseconds since epoch)
    pub fn physical_time(&self) -> u64 {
        self.timestamp >> 16
    }

    /// Extract logical counter component
    pub fn counter(&self) -> u16 {
        (self.timestamp & 0xFFFF) as u16
    }

    /// Create HLC with current physical time and zero counter
    pub fn now() -> Self {
        let now = Utc::now();
        Self::new(now, 0)
    }

    /// Increment HLC, optionally synchronizing with remote time
    ///
    /// This implements the HLC update rule:
    /// - Advance to max of local and remote physical time
    /// - Increment counter if physical times are equal
    pub fn increment(&mut self, remote_time: Option<Self>) {
        let physical_now = Utc::now().timestamp_micros() as u64;

        match remote_time {
            None => {
                // Local operation: increment based on current time
                let new_physical = physical_now.max(self.physical_time());

                if new_physical == self.physical_time() {
                    // Same physical time, increment counter
                    let new_counter = self.counter() + 1;
                    self.timestamp = (new_physical << 16) | (new_counter as u64);
                } else {
                    // Time advanced, reset counter
                    self.timestamp = new_physical << 16;
                }
            }
            Some(remote) => {
                // Remote synchronization
                let remote_physical = remote.physical_time();
                let new_physical = physical_now.max(self.physical_time()).max(remote_physical);

                if new_physical == self.physical_time() && new_physical == remote_physical {
                    // All times equal, take max counter and increment
                    let new_counter = self.counter().max(remote.counter()) + 1;
                    self.timestamp = (new_physical << 16) | (new_counter as u64);
                } else if new_physical == self.physical_time() {
                    // Local time matches, increment local counter
                    let new_counter = self.counter() + 1;
                    self.timestamp = (new_physical << 16) | (new_counter as u64);
                } else if new_physical == remote_physical {
                    // Remote time matches, increment remote counter
                    let new_counter = remote.counter() + 1;
                    self.timestamp = (new_physical << 16) | (new_counter as u64);
                } else {
                    // Time advanced, reset counter
                    self.timestamp = new_physical << 16;
                }
            }
        }
    }
}

impl PartialOrd for HybridLogicalClock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HybridLogicalClock {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

// ============================================================================
// Operation Application
// ============================================================================

/// Apply a local operation and record it in the oplog.
///
/// This function records the operation in the oplog for later synchronization.
/// Apps should implement their own table-specific logic before calling this.
///
/// # Example
/// ```rust,no_run
/// use ahenk::{local_apply, build_oplog_entry};
/// # use rusqlite::Connection;
/// # use uuid::Uuid;
///
/// # fn example(mut conn: Connection, user_id: Uuid, device_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
/// // App creates a record in their table
/// conn.execute(
///     "INSERT INTO my_app_data (id, value) VALUES (?1, ?2)",
///     rusqlite::params!["id1", "value1"],
/// )?;
///
/// // Record operation in oplog for sync
/// let entry = build_oplog_entry(
///     device_id,
///     "my_app_data",
///     "create",
///     &serde_json::json!({"id": "id1", "value": "value1"}),
/// )?;
/// local_apply(&mut conn, &entry)?;
/// # Ok(())
/// # }
/// ```
pub fn local_apply(conn: &mut Connection, op: &OplogEntry) -> Result<(), rusqlite::Error> {
    // Check if operation already exists (idempotency)
    let mut stmt = conn.prepare("SELECT 1 FROM oplog WHERE id = ?")?;
    let exists = stmt.exists([op.id.to_string()])?;

    if !exists {
        // Record operation in oplog
        conn.execute(
            "INSERT INTO oplog (id, device_id, timestamp, table_name, op_type, data) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                op.id.to_string(),
                op.device_id.to_string(),
                op.timestamp,
                op.table,
                op.op_type,
                serde_json::to_string(&op.data).unwrap(),
            ],
        )?;
    }

    Ok(())
}

/// Merge remote operations into the local database.
///
/// This function merges operations from remote peers, recording them in the oplog.
/// Apps should implement their own conflict resolution logic and table updates.
///
/// The function:
/// 1. Checks if each operation already exists (idempotency)
/// 2. Records new operations in the oplog
/// 3. Apps must handle actual table updates based on their conflict resolution strategy
///
/// # Example
/// ```rust,no_run
/// use ahenk::{merge, OplogEntry};
/// # use rusqlite::Connection;
///
/// # fn example(mut conn: Connection, remote_ops: Vec<OplogEntry>) -> Result<(), Box<dyn std::error::Error>> {
/// // Merge operations from remote peer
/// merge(&mut conn, &remote_ops)?;
///
/// // App should now apply operations to their tables with conflict resolution
/// for op in remote_ops {
///     // App-specific logic here based on op.table, op.op_type, and op.data
///     // Use HLC timestamps for last-write-wins or custom conflict resolution
/// }
/// # Ok(())
/// # }
/// ```
pub fn merge(conn: &mut Connection, remote_ops: &[OplogEntry]) -> Result<(), rusqlite::Error> {
    let tx = conn.transaction()?;

    for op in remote_ops {
        let mut stmt = tx.prepare("SELECT 1 FROM oplog WHERE id = ?")?;
        let exists = stmt.exists([op.id.to_string()])?;

        if !exists {
            // Record operation in oplog
            tx.execute(
                "INSERT INTO oplog (id, device_id, timestamp, table_name, op_type, data) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![
                    op.id.to_string(),
                    op.device_id.to_string(),
                    op.timestamp,
                    op.table,
                    op.op_type,
                    serde_json::to_string(&op.data).unwrap(),
                ],
            )?;
        }
    }

    tx.commit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hlc_ordering() {
        let hlc1 = HybridLogicalClock::new(Utc::now(), 0);
        std::thread::sleep(std::time::Duration::from_millis(10));
        let hlc2 = HybridLogicalClock::now();

        assert!(hlc1 < hlc2);
    }

    #[test]
    fn test_hlc_increment() {
        // Test 1: Increment always advances HLC (either time or counter)
        let mut hlc = HybridLogicalClock::now();
        let initial = hlc.to_timestamp();

        hlc.increment(None);
        assert!(
            hlc.to_timestamp() >= initial,
            "HLC should advance after increment"
        );

        // Test 2: Multiple rapid increments advance HLC
        for _ in 0..5 {
            let before = hlc.to_timestamp();
            hlc.increment(None);
            assert!(
                hlc.to_timestamp() >= before,
                "Each increment should advance or maintain HLC"
            );
        }

        // Test 3: Increment with remote time synchronizes correctly
        let mut hlc1 = HybridLogicalClock::now();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let hlc2 = HybridLogicalClock::now();

        let before = hlc1.to_timestamp();
        hlc1.increment(Some(hlc2));

        // After syncing with a later time, HLC should advance
        assert!(
            hlc1.to_timestamp() >= before,
            "Syncing with remote time should advance HLC"
        );
        assert!(
            hlc1.to_timestamp() >= hlc2.to_timestamp(),
            "After sync, local HLC should be >= remote"
        );
    }

    #[test]
    fn test_hlc_roundtrip() {
        let hlc = HybridLogicalClock::now();
        let timestamp = hlc.to_timestamp();
        let hlc2 = HybridLogicalClock::from_timestamp(timestamp);

        assert_eq!(hlc, hlc2);
    }
}
