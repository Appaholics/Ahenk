-- Migration 003: Add User Preferences Table
-- Description: Adds table for storing user preferences that sync across devices
-- Applied: Settings Module Implementation

-- User Preferences Table: Stores user settings that sync via CRDT
CREATE TABLE IF NOT EXISTS user_preferences (
    preference_id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    preference_key VARCHAR(100) NOT NULL,
    preference_value TEXT NOT NULL,
    preference_type VARCHAR(20) NOT NULL, -- 'string', 'number', 'boolean', 'json'
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id),
    UNIQUE(user_id, preference_key)
);

CREATE INDEX IF NOT EXISTS idx_user_preferences_user_key
ON user_preferences(user_id, preference_key);
