-- Migration 004: Add Soundscape Presets
-- Description: Adds tables for storing soundscape presets and their track compositions
-- Applied: Soundscapes Module Implementation

-- Soundscape Presets Table: Stores user-created soundscape presets
CREATE TABLE IF NOT EXISTS soundscape_presets (
    preset_id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- Soundscape Tracks Table: Stores individual tracks for each preset
CREATE TABLE IF NOT EXISTS soundscape_tracks (
    track_id TEXT PRIMARY KEY,
    preset_id TEXT NOT NULL,
    track_type TEXT NOT NULL CHECK(track_type IN ('frequency', 'ambient')),
    identifier TEXT NOT NULL,
    volume REAL NOT NULL CHECK(volume >= 0 AND volume <= 1),
    FOREIGN KEY (preset_id) REFERENCES soundscape_presets(preset_id) ON DELETE CASCADE
);

-- Index for efficient preset lookups by user
CREATE INDEX IF NOT EXISTS idx_soundscape_presets_user_id
ON soundscape_presets(user_id);

-- Index for efficient track lookups by preset
CREATE INDEX IF NOT EXISTS idx_soundscape_tracks_preset_id
ON soundscape_tracks(preset_id);
