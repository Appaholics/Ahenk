use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: Uuid,
    pub user_name: String,
    pub user_password_hash: String,
    pub user_mail: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub device_id: Uuid,
    pub user_id: Uuid,
    pub device_type: String,
    pub push_token: Option<String>,
    pub last_seen: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskList {
    pub list_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub task_id: Uuid,
    pub list_id: Uuid,
    pub content: String,
    pub is_completed: bool,
    pub due_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub block_id: Uuid,
    pub user_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct TaskBlock {
    pub task_id: Uuid,
    pub block_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct BlockedItem {
    pub item_id: Uuid,
    pub user_id: Uuid,
    pub item_type: String,
    pub identifier: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Sound {
    pub sound_id: Uuid,
    pub name: String,
    pub category: Option<String>,
    pub file_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct FavoriteSound {
    pub user_id: Uuid,
    pub sound_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Soundscape {
    pub soundscape_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub file_path: String,
    pub volume: f32,
    pub is_playing: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Habit {
    pub habit_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub habit_cover: Option<String>,
    pub frequency_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct HabitEntry {
    pub entry_id: Uuid,
    pub habit_id: Uuid,
    pub completion_date: NaiveDate,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Pomodoro {
    pub pomodoro_id: Uuid,
    pub user_id: Uuid,
    pub pomodoro_name: String,
    pub pomodoro_cover: Option<String>,
    pub work_duration: i32,
    pub short_break_duration: i32,
    pub long_break_duration: i32,
    pub long_break_interval: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PomodoroPreset {
    pub name: String,
    pub cover: Option<String>,
    pub work_duration: i32,
    pub short_break: i32,
    pub long_break: i32,
    pub interval: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PomodoroSession {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub pomodoro_id: Option<Uuid>,
    pub session_type: String,
    pub duration_seconds: i32,
    pub completed: bool,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OplogEntry {
    pub id: Uuid,                // Unique ID for the operation itself
    pub device_id: Uuid,         // The device that created the operation
    pub timestamp: i64,          // Hybrid Logical Clock (HLC) timestamp
    pub table: String,           // e.g., "tasks", "habits"
    pub op_type: String,         // e.g., "create", "update", "delete"
    pub data: serde_json::Value, // The full JSON representation of the entity
}

#[derive(Serialize, Deserialize)]
pub struct Peer {
    pub peer_id: Uuid,
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub last_known_ip: Option<String>,
    pub last_sync_time: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPreference {
    pub preference_id: Uuid,
    pub user_id: Uuid,
    pub preference_key: String,
    pub preference_value: String,
    pub preference_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conflict {
    pub conflict_id: Uuid,
    pub user_id: Uuid,
    pub table_name: String,
    pub item_id: Uuid,
    pub local_data: serde_json::Value,
    pub remote_data: serde_json::Value,
    pub resolved_data: Option<serde_json::Value>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoundTrack {
    pub id: String,
    #[serde(rename = "type")]
    pub track_type: String, // 'frequency' or 'ambient'
    pub identifier: String, // '500hz', '528hz', '832hz', 'rain', 'fire', 'noise', 'sweep'
    pub volume: f32,
    #[serde(rename = "isPlaying")]
    pub is_playing: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoundscapePreset {
    pub preset_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub tracks: Vec<SoundTrack>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoundTrackDto {
    pub id: String,
    #[serde(rename = "type")]
    pub track_type: String,
    pub identifier: String,
    pub volume: f32,
    #[serde(rename = "isPlaying")]
    pub is_playing: bool,
}
