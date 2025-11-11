-- Migration 001: Initial Schema
-- Description: Creates core database synchronization infrastructure tables
-- Applied: Initial version

-- Users Table: Stores user account information for device ownership and authentication.
CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY,
    user_name VARCHAR(255) UNIQUE NOT NULL,
    user_password VARCHAR(255) NOT NULL,
    user_mail VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Devices Table: Stores information about user devices for synchronization and authorization.
CREATE TABLE IF NOT EXISTS devices (
    device_id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    device_type VARCHAR(10) NOT NULL,
    push_token TEXT,
    last_seen TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- Oplog Table: Operation log for CRDT-based synchronization.
-- Stores every operation for conflict-free replicated data types.
-- Apps store their data as JSON in the data field for flexibility.
CREATE TABLE IF NOT EXISTS oplog (
    id TEXT PRIMARY KEY,              -- UUID as text
    device_id TEXT NOT NULL,          -- Device that created this operation
    timestamp INTEGER NOT NULL,       -- HLC timestamp (64-bit)
    table_name TEXT NOT NULL,         -- Table this operation affects
    op_type TEXT NOT NULL,            -- Operation type: 'create', 'update', 'delete'
    data TEXT NOT NULL,               -- JSON-encoded operation data
    FOREIGN KEY (device_id) REFERENCES devices(device_id)
);

-- Peers Table: Stores information about other trusted devices in the P2P sync network.
CREATE TABLE IF NOT EXISTS peers (
    peer_id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    device_id UUID NOT NULL,
    last_known_ip VARCHAR(255),
    last_sync_time TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id),
    FOREIGN KEY (device_id) REFERENCES devices(device_id)
);
