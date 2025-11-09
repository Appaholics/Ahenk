pub mod sync;
pub mod sync_manager;

use crate::crdt;
use crate::db::operations;
use crate::models::{
    Block, BlockedItem, Device, Habit, HabitEntry, OplogEntry, Pomodoro, PomodoroPreset,
    PomodoroSession, Task, TaskBlock, TaskList, User,
};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{DateTime, Duration, NaiveDate, Utc};
use rusqlite::Connection;
use serde::Serialize;
use serde_json;
use uuid::Uuid;

fn build_oplog_entry<T: Serialize>(
    device_id: Uuid,
    table: &str,
    op_type: &str,
    value: &T,
) -> Result<OplogEntry, String> {
    let data = serde_json::to_value(value)
        .map_err(|e| format!("Failed to serialize {} payload: {}", table, e))?;

    Ok(OplogEntry {
        id: Uuid::new_v4(),
        device_id,
        timestamp: crdt::HybridLogicalClock::now().to_timestamp(),
        table: table.to_string(),
        op_type: op_type.to_string(),
        data,
    })
}

/// Registers a new user after validating uniqueness and hashing their password.
pub fn register_user(
    conn: &Connection,
    user_name: String,
    user_mail: String,
    password: String,
) -> Result<User, String> {
    let normalized_name = user_name.trim();
    if normalized_name.is_empty() {
        return Err("Username cannot be empty".to_string());
    }

    let normalized_mail = user_mail.trim().to_lowercase();
    if normalized_mail.is_empty() {
        return Err("Email cannot be empty".to_string());
    }

    if password.trim().is_empty() {
        return Err("Password cannot be empty".to_string());
    }

    if operations::get_user_by_name(conn, normalized_name)
        .map_err(|e| format!("Database error: {}", e))?
        .is_some()
    {
        return Err("Username already exists".to_string());
    }

    if operations::get_user_by_mail(conn, &normalized_mail)
        .map_err(|e| format!("Database error: {}", e))?
        .is_some()
    {
        return Err("Email already registered".to_string());
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("Password hashing failed: {}", e))?
        .to_string();

    let new_user = User {
        user_id: Uuid::new_v4(),
        user_name: normalized_name.to_string(),
        user_password_hash: password_hash,
        user_mail: normalized_mail,
        created_at: Utc::now(),
    };

    operations::create_user(conn, &new_user)
        .map_err(|e| format!("Failed to create user: {}", e))?;

    Ok(new_user)
}

/// Validates credentials using Argon2 and returns the matching user record.
pub fn login_user(conn: &Connection, identifier: &str, password: &str) -> Result<User, String> {
    let trimmed_identifier = identifier.trim();
    if trimmed_identifier.is_empty() {
        return Err("Identifier cannot be empty".to_string());
    }

    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }

    let user_result = operations::get_user_by_name(conn, trimmed_identifier)
        .map_err(|e| format!("Database error: {}", e))?;

    let user = match user_result {
        Some(user) => user,
        None => {
            let email_lookup =
                operations::get_user_by_mail(conn, &trimmed_identifier.to_lowercase())
                    .map_err(|e| format!("Database error: {}", e))?;
            email_lookup.ok_or_else(|| "Invalid credentials".to_string())?
        }
    };

    let parsed_hash = PasswordHash::new(&user.user_password_hash)
        .map_err(|_| "Stored password hash is invalid".to_string())?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| "Invalid credentials".to_string())?;

    Ok(user)
}

/// Associates a new device with an existing user.
pub fn add_device_to_user(
    conn: &Connection,
    user_id: Uuid,
    device_type: String,
    push_token: Option<String>,
) -> Result<Device, String> {
    operations::get_user(conn, user_id)
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "User not found".to_string())?;

    let trimmed_type = device_type.trim();
    if trimmed_type.is_empty() {
        return Err("Device type cannot be empty".to_string());
    }

    let normalized_push_token = push_token.and_then(|token| {
        let trimmed = token.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    });

    let new_device = Device {
        device_id: Uuid::new_v4(),
        user_id,
        device_type: trimmed_type.to_string(),
        push_token: normalized_push_token,
        last_seen: Some(Utc::now()),
    };

    operations::create_device(conn, &new_device)
        .map_err(|e| format!("Failed to create device: {}", e))?;

    Ok(new_device)
}

/// Retrieves all devices currently associated with the user.
pub fn get_user_devices(conn: &Connection, user_id: Uuid) -> Result<Vec<Device>, String> {
    operations::get_devices_by_user_id(conn, user_id).map_err(|e| format!("Database error: {}", e))
}

pub fn get_all_task_lists_for_user(
    conn: &Connection,
    user_id: Uuid,
) -> Result<Vec<TaskList>, rusqlite::Error> {
    operations::get_task_lists_by_user_id(conn, user_id)
}

pub fn create_new_task_list(
    conn: &mut Connection,
    user_id: Uuid,
    device_id: Uuid,
    name: String,
) -> Result<TaskList, String> {
    let new_task_list = TaskList {
        list_id: Uuid::new_v4(),
        user_id,
        name: name.clone(),
    };

    let oplog_entry = OplogEntry {
        id: Uuid::new_v4(),
        device_id,
        timestamp: crdt::HybridLogicalClock::now().to_timestamp(),
        table: "task_lists".to_string(),
        op_type: "create".to_string(),
        data: serde_json::to_value(&new_task_list).unwrap(),
    };

    crdt::local_apply(conn, &oplog_entry).map_err(|e| e.to_string())?;

    Ok(new_task_list)
}

pub fn add_task_to_list(
    conn: &mut Connection,
    user_id: Uuid,
    device_id: Uuid,
    list_id: Uuid,
    content: String,
    due_date: Option<NaiveDate>,
) -> Result<Task, String> {
    match operations::get_task_list(conn, list_id) {
        Ok(Some(task_list)) => {
            if task_list.user_id != user_id {
                return Err("Access denied: Task list does not belong to the user".to_string());
            }

            let new_task = Task {
                task_id: Uuid::new_v4(),
                list_id,
                content,
                is_completed: false,
                due_date,
                created_at: Utc::now(),
                updated_at: None,
            };

            let oplog_entry = OplogEntry {
                id: Uuid::new_v4(),
                device_id,
                timestamp: crdt::HybridLogicalClock::now().to_timestamp(),
                table: "tasks".to_string(),
                op_type: "create".to_string(),
                data: serde_json::to_value(&new_task).unwrap(),
            };

            crdt::local_apply(conn, &oplog_entry).map_err(|e| e.to_string())?;

            Ok(new_task)
        }
        Ok(None) => Err("Task list not found".to_string()),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

pub fn mark_task_as_complete(
    conn: &mut Connection,
    user_id: Uuid,
    device_id: Uuid,
    task_id: Uuid,
) -> Result<(), String> {
    match operations::get_task(conn, task_id) {
        Ok(Some(mut task)) => match operations::get_task_list(conn, task.list_id) {
            Ok(Some(task_list)) => {
                if task_list.user_id != user_id {
                    return Err("Access denied: Task does not belong to the user".to_string());
                }

                task.is_completed = true;
                task.updated_at = Some(Utc::now());

                let oplog_entry = OplogEntry {
                    id: Uuid::new_v4(),
                    device_id,
                    timestamp: crdt::HybridLogicalClock::now().to_timestamp(),
                    table: "tasks".to_string(),
                    op_type: "update".to_string(),
                    data: serde_json::to_value(&task).unwrap(),
                };

                crdt::local_apply(conn, &oplog_entry).map_err(|e| e.to_string())?;

                Ok(())
            }
            Ok(None) => Err("Internal error: Task list not found for task".to_string()),
            Err(e) => Err(format!("Database error: {}", e)),
        },
        Ok(None) => Err("Task not found".to_string()),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

pub fn get_all_tasks_in_list(
    conn: &Connection,
    user_id: Uuid,
    list_id: Uuid,
) -> Result<Vec<Task>, String> {
    match operations::get_task_list(conn, list_id) {
        Ok(Some(task_list)) => {
            if task_list.user_id != user_id {
                return Err("Access denied: Task list does not belong to the user".to_string());
            }

            match operations::get_tasks_by_list_id(conn, list_id) {
                Ok(tasks) => Ok(tasks),
                Err(e) => Err(format!("Failed to retrieve tasks: {}", e)),
            }
        }
        Ok(None) => Err("Task list not found".to_string()),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

pub fn get_tasks_due_today(conn: &Connection, user_id: Uuid) -> Result<Vec<Task>, String> {
    let today = Utc::now().naive_utc().date();
    match operations::get_tasks_due_on_date_for_user(conn, user_id, today) {
        Ok(tasks) => Ok(tasks),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

pub fn create_habit(
    conn: &Connection,
    user_id: Uuid,
    device_id: Uuid,
    name: String,
    description: Option<String>,
    habit_cover: Option<String>,
    frequency_type: String,
) -> Result<Habit, String> {
    let new_habit = Habit {
        habit_id: Uuid::new_v4(),
        user_id,
        name,
        description,
        habit_cover,
        frequency_type,
    };

    operations::create_habit(conn, &new_habit)
        .map_err(|e| format!("Failed to create habit: {}", e))?;

    let entry = build_oplog_entry(device_id, "habits", "create_habit", &new_habit)?;

    operations::create_oplog_entry(conn, &entry)
        .map_err(|e| format!("Failed to create oplog entry: {}", e))?;

    Ok(new_habit)
}

pub fn get_all_habits_for_user(conn: &Connection, user_id: Uuid) -> Result<Vec<Habit>, String> {
    operations::get_habits_by_user_id(conn, user_id).map_err(|e| format!("Database error: {}", e))
}

pub fn log_habit_completion(
    conn: &mut Connection,
    user_id: Uuid,
    device_id: Uuid,
    habit_id: Uuid,
    completion_date: NaiveDate,
    notes: Option<String>,
) -> Result<HabitEntry, String> {
    match operations::get_habit(conn, habit_id) {
        Ok(Some(habit)) => {
            if habit.user_id != user_id {
                return Err("Access denied: Habit does not belong to the user".to_string());
            }

            let new_habit_entry = HabitEntry {
                entry_id: Uuid::new_v4(),
                habit_id,
                completion_date,
                notes,
            };

            let oplog_entry =
                build_oplog_entry(device_id, "habit_entries", "create", &new_habit_entry)?;

            crdt::local_apply(conn, &oplog_entry).map_err(|e| e.to_string())?;

            Ok(new_habit_entry)
        }
        Ok(None) => Err("Habit not found".to_string()),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

pub fn get_habit_streak(conn: &Connection, user_id: Uuid, habit_id: Uuid) -> Result<i32, String> {
    match operations::get_habit(conn, habit_id) {
        Ok(Some(habit)) => {
            if habit.user_id != user_id {
                return Err("Access denied: Habit does not belong to the user".to_string());
            }

            let entries = match operations::get_habit_entries_sorted_by_date(conn, habit_id) {
                Ok(entries) => entries,
                Err(e) => return Err(format!("Database error: {}", e)),
            };

            let mut streak = 0;
            let today = Utc::now().naive_utc().date();

            if let Some(latest_entry) = entries.first()
                && (latest_entry.completion_date == today
                    || latest_entry.completion_date == (today - Duration::days(1)))
            {
                streak += 1;
                let mut expected_date = latest_entry.completion_date - Duration::days(1);

                for entry in entries.iter().skip(1) {
                    if entry.completion_date == expected_date {
                        streak += 1;
                        expected_date -= Duration::days(1);
                    } else {
                        break;
                    }
                }
            }

            Ok(streak)
        }
        Ok(None) => Err("Habit not found".to_string()),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

pub fn schedule_block(
    conn: &mut Connection,
    user_id: Uuid,
    device_id: Uuid,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> Result<Block, String> {
    if start_time >= end_time {
        return Err("Start time must be before end time".to_string());
    }

    let new_block = Block {
        block_id: Uuid::new_v4(),
        user_id,
        start_time,
        end_time,
    };

    let oplog_entry = build_oplog_entry(device_id, "blocks", "create", &new_block)?;

    crdt::local_apply(conn, &oplog_entry).map_err(|e| e.to_string())?;

    Ok(new_block)
}

pub fn assign_task_to_block(
    conn: &mut Connection,
    user_id: Uuid,
    device_id: Uuid,
    task_id: Uuid,
    block_id: Uuid,
) -> Result<TaskBlock, String> {
    // Verify task exists and belongs to the user
    let task = match operations::get_task(conn, task_id) {
        Ok(Some(task)) => task,
        Ok(None) => return Err("Task not found".to_string()),
        Err(e) => return Err(format!("Database error: {}", e)),
    };
    let task_list = match operations::get_task_list(conn, task.list_id) {
        Ok(Some(task_list)) => task_list,
        Ok(None) => return Err("Internal error: Task list not found".to_string()),
        Err(e) => return Err(format!("Database error: {}", e)),
    };
    if task_list.user_id != user_id {
        return Err("Access denied: Task does not belong to the user".to_string());
    }

    // Verify block exists and belongs to the user
    let block = match operations::get_block(conn, block_id) {
        Ok(Some(block)) => block,
        Ok(None) => return Err("Block not found".to_string()),
        Err(e) => return Err(format!("Database error: {}", e)),
    };
    if block.user_id != user_id {
        return Err("Access denied: Block does not belong to the user".to_string());
    }

    // Verify that the task is not already assigned to the block
    match operations::get_task_block(conn, task_id, block_id) {
        Ok(Some(_)) => return Err("Task is already assigned to this block".to_string()),
        Ok(None) => (),
        Err(e) => return Err(format!("Database error: {}", e)),
    };

    let new_task_block = TaskBlock { task_id, block_id };

    let oplog_entry = build_oplog_entry(device_id, "task_blocks", "create", &new_task_block)?;

    crdt::local_apply(conn, &oplog_entry).map_err(|e| e.to_string())?;

    Ok(new_task_block)
}

pub fn get_tasks_for_a_specific_block(
    conn: &Connection,
    user_id: Uuid,
    block_id: Uuid,
) -> Result<Vec<Task>, String> {
    // Verify block exists and belongs to the user
    let block = match operations::get_block(conn, block_id) {
        Ok(Some(block)) => block,
        Ok(None) => return Err("Block not found".to_string()),
        Err(e) => return Err(format!("Database error: {}", e)),
    };
    if block.user_id != user_id {
        return Err("Access denied: Block does not belong to the user".to_string());
    }

    match operations::get_tasks_by_block_id(conn, block_id) {
        Ok(tasks) => Ok(tasks),
        Err(e) => Err(format!("Failed to retrieve tasks: {}", e)),
    }
}

pub fn get_all_blocks_for_user(conn: &Connection, user_id: Uuid) -> Result<Vec<Block>, String> {
    match operations::get_all_blocks_by_user_id(conn, user_id) {
        Ok(blocks) => Ok(blocks),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

pub fn remove_block(
    conn: &mut Connection,
    user_id: Uuid,
    device_id: Uuid,
    block_id: Uuid,
) -> Result<(), String> {
    // Verify block exists and belongs to the user
    let block = match operations::get_block(conn, block_id) {
        Ok(Some(block)) => block,
        Ok(None) => return Err("Block not found".to_string()),
        Err(e) => return Err(format!("Database error: {}", e)),
    };

    if block.user_id != user_id {
        return Err("Access denied: Block does not belong to the user".to_string());
    }

    let oplog_entry = build_oplog_entry(device_id, "blocks", "delete", &block)?;

    crdt::local_apply(conn, &oplog_entry).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn save_pomodoro_preset(
    conn: &mut Connection,
    user_id: Uuid,
    device_id: Uuid,
    preset: PomodoroPreset,
) -> Result<Pomodoro, String> {
    let new_pomodoro = Pomodoro {
        pomodoro_id: Uuid::new_v4(),
        user_id,
        pomodoro_name: preset.name,
        pomodoro_cover: preset.cover,
        work_duration: preset.work_duration,
        short_break_duration: preset.short_break,
        long_break_duration: preset.long_break,
        long_break_interval: preset.interval,
        created_at: Utc::now(),
        updated_at: None,
    };

    let oplog_entry = build_oplog_entry(device_id, "pomodoros", "create", &new_pomodoro)?;

    crdt::local_apply(conn, &oplog_entry).map_err(|e| e.to_string())?;

    Ok(new_pomodoro)
}

pub fn get_all_pomodoro_presets(conn: &Connection, user_id: Uuid) -> Result<Vec<Pomodoro>, String> {
    match operations::get_pomodoros_by_user_id(conn, user_id) {
        Ok(presets) => Ok(presets),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

pub fn add_item_to_blocklist(
    conn: &mut Connection,
    user_id: Uuid,
    device_id: Uuid,
    item_type: String,
    identifier: String,
) -> Result<BlockedItem, String> {
    let new_item = BlockedItem {
        item_id: Uuid::new_v4(),
        user_id,
        item_type,
        identifier,
        is_active: true,
    };

    let oplog_entry = build_oplog_entry(device_id, "blocked_items", "create", &new_item)?;

    crdt::local_apply(conn, &oplog_entry).map_err(|e| e.to_string())?;

    Ok(new_item)
}

pub fn get_active_blocklist(conn: &Connection, user_id: Uuid) -> Result<Vec<BlockedItem>, String> {
    match operations::get_active_blocked_items_by_user_id(conn, user_id) {
        Ok(items) => Ok(items),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

pub fn get_all_blocklist(conn: &Connection, user_id: Uuid) -> Result<Vec<BlockedItem>, String> {
    match operations::get_all_blocked_items_by_user_id(conn, user_id) {
        Ok(items) => Ok(items),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

pub fn toggle_blocked_item(
    conn: &mut Connection,
    user_id: Uuid,
    device_id: Uuid,
    item_id: Uuid,
    is_active: bool,
) -> Result<(), String> {
    // Verify item exists and belongs to the user
    let item = match operations::get_blocked_item(conn, item_id) {
        Ok(Some(item)) => item,
        Ok(None) => return Err("Blocked item not found".to_string()),
        Err(e) => return Err(format!("Database error: {}", e)),
    };

    if item.user_id != user_id {
        return Err("Access denied: Blocked item does not belong to the user".to_string());
    }

    let updated_item = BlockedItem {
        item_id: item.item_id,
        user_id: item.user_id,
        item_type: item.item_type,
        identifier: item.identifier,
        is_active,
    };

    let oplog_entry = build_oplog_entry(device_id, "blocked_items", "update", &updated_item)?;

    crdt::local_apply(conn, &oplog_entry).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn remove_blocked_item(
    conn: &mut Connection,
    user_id: Uuid,
    device_id: Uuid,
    item_id: Uuid,
) -> Result<(), String> {
    // Verify item exists and belongs to the user
    let item = match operations::get_blocked_item(conn, item_id) {
        Ok(Some(item)) => item,
        Ok(None) => return Err("Blocked item not found".to_string()),
        Err(e) => return Err(format!("Database error: {}", e)),
    };

    if item.user_id != user_id {
        return Err("Access denied: Blocked item does not belong to the user".to_string());
    }

    let oplog_entry = build_oplog_entry(device_id, "blocked_items", "delete", &item)?;

    crdt::local_apply(conn, &oplog_entry).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn record_pomodoro_session(
    conn: &mut Connection,
    user_id: Uuid,
    device_id: Uuid,
    pomodoro_id: Option<Uuid>,
    session_type: String,
    duration_seconds: i32,
    completed: bool,
    started_at: DateTime<Utc>,
    notes: Option<String>,
) -> Result<PomodoroSession, String> {
    let new_session = PomodoroSession {
        session_id: Uuid::new_v4(),
        user_id,
        pomodoro_id,
        session_type,
        duration_seconds,
        completed,
        started_at,
        completed_at: Utc::now(),
        notes,
    };

    let oplog_entry = build_oplog_entry(device_id, "pomodoro_sessions", "create", &new_session)?;

    crdt::local_apply(conn, &oplog_entry).map_err(|e| e.to_string())?;

    Ok(new_session)
}

pub fn get_recent_sessions(
    conn: &Connection,
    user_id: Uuid,
    limit: i32,
) -> Result<Vec<PomodoroSession>, String> {
    match operations::get_recent_pomodoro_sessions(conn, user_id, limit) {
        Ok(sessions) => Ok(sessions),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

pub fn get_pomodoro_statistics(
    conn: &Connection,
    user_id: Uuid,
    days: i32,
) -> Result<(i32, i32, i32), String> {
    match operations::get_pomodoro_stats_for_user(conn, user_id, days) {
        Ok(stats) => Ok(stats),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

// ============= Soundscape Preset Management =============

pub fn get_soundscape_presets_for_user(
    conn: &Connection,
    user_id: Uuid,
) -> Result<Vec<crate::models::SoundscapePreset>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare(
        "SELECT preset_id, user_id, name, created_at, updated_at
         FROM soundscape_presets
         WHERE user_id = ?1
         ORDER BY created_at DESC",
    )?;

    let preset_rows = stmt.query_map([user_id.to_string()], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, Option<String>>(4)?,
        ))
    })?;

    let mut presets = Vec::new();
    for preset_result in preset_rows {
        let (preset_id_str, user_id_str, name, created_at_str, updated_at_str) = preset_result?;
        let preset_id = Uuid::parse_str(&preset_id_str)?;

        // Get tracks for this preset
        let mut track_stmt = conn.prepare(
            "SELECT track_id, track_type, identifier, volume
             FROM soundscape_tracks
             WHERE preset_id = ?1",
        )?;

        let track_rows = track_stmt.query_map([preset_id_str.clone()], |row| {
            Ok(crate::models::SoundTrack {
                id: row.get(0)?,
                track_type: row.get(1)?,
                identifier: row.get(2)?,
                volume: row.get(3)?,
                is_playing: false,
            })
        })?;

        let mut tracks = Vec::new();
        for track in track_rows {
            tracks.push(track?);
        }

        let preset = crate::models::SoundscapePreset {
            preset_id,
            user_id: Uuid::parse_str(&user_id_str)?,
            name,
            tracks,
            created_at: DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc),
            updated_at: updated_at_str
                .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                .transpose()?,
        };

        presets.push(preset);
    }

    Ok(presets)
}

pub fn create_soundscape_preset(
    conn: &mut Connection,
    user_id: Uuid,
    device_id: Uuid,
    name: String,
    tracks: Vec<crate::models::SoundTrackDto>,
) -> Result<crate::models::SoundscapePreset, Box<dyn std::error::Error>> {
    let preset_id = Uuid::new_v4();
    let created_at = Utc::now();

    let tx = conn.transaction()?;

    // Insert preset
    tx.execute(
        "INSERT INTO soundscape_presets (preset_id, user_id, name, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![
            preset_id.to_string(),
            user_id.to_string(),
            name,
            created_at.to_rfc3339(),
            None::<String>,
        ],
    )?;

    // Insert tracks
    for track in &tracks {
        let track_id = Uuid::new_v4();
        tx.execute(
            "INSERT INTO soundscape_tracks (track_id, preset_id, track_type, identifier, volume)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                track_id.to_string(),
                preset_id.to_string(),
                track.track_type,
                track.identifier,
                track.volume,
            ],
        )?;
    }

    tx.commit()?;

    let sound_tracks: Vec<crate::models::SoundTrack> = tracks
        .into_iter()
        .map(|dto| crate::models::SoundTrack {
            id: dto.id,
            track_type: dto.track_type,
            identifier: dto.identifier,
            volume: dto.volume,
            is_playing: dto.is_playing,
        })
        .collect();

    Ok(crate::models::SoundscapePreset {
        preset_id,
        user_id,
        name,
        tracks: sound_tracks,
        created_at,
        updated_at: None,
    })
}

pub fn update_soundscape_preset(
    conn: &mut Connection,
    preset_id: Uuid,
    device_id: Uuid,
    name: String,
    tracks: Vec<crate::models::SoundTrackDto>,
) -> Result<crate::models::SoundscapePreset, Box<dyn std::error::Error>> {
    let updated_at = Utc::now();

    let tx = conn.transaction()?;

    // Update preset
    tx.execute(
        "UPDATE soundscape_presets SET name = ?1, updated_at = ?2 WHERE preset_id = ?3",
        rusqlite::params![name, updated_at.to_rfc3339(), preset_id.to_string()],
    )?;

    // Delete old tracks
    tx.execute(
        "DELETE FROM soundscape_tracks WHERE preset_id = ?1",
        [preset_id.to_string()],
    )?;

    // Insert new tracks
    for track in &tracks {
        let track_id = Uuid::new_v4();
        tx.execute(
            "INSERT INTO soundscape_tracks (track_id, preset_id, track_type, identifier, volume)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                track_id.to_string(),
                preset_id.to_string(),
                track.track_type,
                track.identifier,
                track.volume,
            ],
        )?;
    }

    tx.commit()?;

    // Fetch the updated preset to get user_id and created_at
    let mut stmt = conn.prepare(
        "SELECT user_id, created_at FROM soundscape_presets WHERE preset_id = ?1",
    )?;

    let (user_id, created_at): (String, String) =
        stmt.query_row([preset_id.to_string()], |row| Ok((row.get(0)?, row.get(1)?)))?;

    let sound_tracks: Vec<crate::models::SoundTrack> = tracks
        .into_iter()
        .map(|dto| crate::models::SoundTrack {
            id: dto.id,
            track_type: dto.track_type,
            identifier: dto.identifier,
            volume: dto.volume,
            is_playing: dto.is_playing,
        })
        .collect();

    Ok(crate::models::SoundscapePreset {
        preset_id,
        user_id: Uuid::parse_str(&user_id)?,
        name,
        tracks: sound_tracks,
        created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
        updated_at: Some(updated_at),
    })
}

pub fn delete_soundscape_preset(
    conn: &mut Connection,
    device_id: Uuid,
    preset_id: Uuid,
) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "DELETE FROM soundscape_presets WHERE preset_id = ?1",
        [preset_id.to_string()],
    )?;
    // Tracks will be automatically deleted due to CASCADE
    Ok(())
}
