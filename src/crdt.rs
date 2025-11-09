use crate::{OplogEntry, Task};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HybridLogicalClock {
    timestamp: u64,
}

impl HybridLogicalClock {
    pub fn new(physical_time: DateTime<Utc>, counter: u16) -> Self {
        let physical_micros = physical_time.timestamp_micros() as u64;
        // 48 bits for physical time, 16 bits for counter
        let timestamp = (physical_micros << 16) | (counter as u64);
        Self { timestamp }
    }

    pub fn from_timestamp(timestamp: i64) -> Self {
        Self {
            timestamp: timestamp as u64,
        }
    }

    pub fn to_timestamp(&self) -> i64 {
        self.timestamp as i64
    }

    pub fn physical_time(&self) -> u64 {
        self.timestamp >> 16
    }

    pub fn counter(&self) -> u16 {
        (self.timestamp & 0xFFFF) as u16
    }

    pub fn now() -> Self {
        let now = Utc::now();
        Self::new(now, 0)
    }

    pub fn increment(&mut self, remote_time: Option<Self>) {
        let physical_now = Utc::now().timestamp_micros() as u64;
        let remote_physical = remote_time.map_or(0, |t| t.physical_time());

        let new_physical = physical_now.max(remote_physical);

        if new_physical == self.physical_time() && new_physical == remote_physical {
            let new_counter = self.counter().max(remote_time.unwrap().counter()) + 1;
            self.timestamp = (new_physical << 16) | (new_counter as u64);
        } else if new_physical == self.physical_time() {
            let new_counter = self.counter() + 1;
            self.timestamp = (new_physical << 16) | (new_counter as u64);
        } else if new_physical == remote_physical {
            let new_counter = remote_time.unwrap().counter() + 1;
            self.timestamp = (new_physical << 16) | (new_counter as u64);
        } else {
            self.timestamp = new_physical << 16;
        }
    }
}

impl PartialOrd for HybridLogicalClock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

impl Ord for HybridLogicalClock {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

fn apply_task_op(conn: &Connection, op: &OplogEntry) -> Result<(), rusqlite::Error> {
    let task: Task = serde_json::from_value(op.data.clone()).unwrap();

    let mut stmt = conn.prepare("SELECT updated_at FROM tasks WHERE task_id = ?")?;
    let maybe_existing_task_updated_at: Option<DateTime<Utc>> = stmt
        .query_row([task.task_id.to_string()], |row| {
            let timestamp: Option<i64> = row.get(0)?;
            Ok(timestamp.and_then(|ts| DateTime::from_timestamp(ts, 0)))
        })
        .optional()?
        .flatten();

    let remote_op_timestamp = HybridLogicalClock::from_timestamp(op.timestamp).physical_time();

    let should_apply = match maybe_existing_task_updated_at {
        Some(existing_task_updated_at) => {
            remote_op_timestamp > existing_task_updated_at.timestamp_micros() as u64
        }
        None => true,
    };

    if should_apply {
        conn.execute(
            "INSERT OR REPLACE INTO tasks (task_id, list_id, content, is_completed, due_date, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                task.task_id.to_string(),
                task.list_id.to_string(),
                task.content,
                task.is_completed,
                task.due_date.map(|d| d.to_string()),
                task.created_at.timestamp(),
                Utc::now().timestamp(),
            ],
        )?;
    }

    conn.execute(
        "INSERT INTO oplog (id, device_id, timestamp, table_name, op_type, data) VALUES (?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            op.id.to_string(),
            op.device_id.to_string(),
            op.timestamp,
            op.table,
            op.op_type,
            serde_json::to_string(&op.data).unwrap(),
        ],
    )?;

    Ok(())
}

pub fn merge(conn: &mut Connection, remote_ops: &[OplogEntry]) -> Result<(), rusqlite::Error> {
    let tx = conn.transaction()?;

    for op in remote_ops {
        let mut stmt = tx.prepare("SELECT 1 FROM oplog WHERE id = ?")?;
        let exists = stmt.exists([op.id.to_string()])?;

        if !exists {
            match op.table.as_str() {
                "tasks" => apply_task_op(&tx, op)?,
                // Add other tables here
                _ => (),
            }
        }
    }

    tx.commit()
}

pub fn local_apply(conn: &mut Connection, op: &OplogEntry) -> Result<(), rusqlite::Error> {
    let tx = conn.transaction()?;

    match op.table.as_str() {
        "tasks" => apply_task_op(&tx, op)?,
        // Add other tables here
        _ => (),
    }

    tx.commit()
}
