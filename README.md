# CFOST

**Conflict-Free Offline Synchronization Tool**

[![Tests](https://img.shields.io/badge/tests-passing-brightgreen)](tests/)
[![Rust](https://img.shields.io/badge/rust-nightly%202024-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)
[![crates.io](https://img.shields.io/crates/v/cfost)](https://crates.io/crates/cfost)

> Cross-platform database synchronization infrastructure with P2P networking, CRDT conflict resolution, and offline-first architecture.

## Overview

**CFOST** (Conflict-Free Offline Synchronization Tool) is a batteries-included Rust library that provides everything you need to synchronize databases across devices in your application ecosystem. It handles the complex distributed systems challenges so you can focus on building your app.

### Key Features

- **🔄 P2P Synchronization** - Built on libp2p with mDNS discovery, relay support, and NAT traversal
- **🎯 CRDT Conflict Resolution** - Hybrid Logical Clocks ensure causal ordering and conflict-free merges
- **📴 Offline-First** - Operation log tracks all changes for synchronization when devices reconnect
- **🔐 User Authentication** - Argon2 password hashing with timing-safe verification
- **📱 Device Management** - Secure device pairing with QR code-based authorization
- **🚀 Zero Configuration** - Automatic schema migrations and peer discovery
- **🌍 Cross-Platform** - Works on iOS, Android, macOS, Windows, Linux, and embedded devices

## Quick Start

### Prerequisites

- **Rust Nightly** (2024 edition required)
- SQLite 3.x

```bash
# Install Rust nightly
rustup install nightly
rustup default nightly
```

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cfost = { git = "https://github.com/kodfikirsanat/cfost" }
# Or from crates.io (when published):
# cfost = "0.1"
```

### Basic Usage

```rust
use cfost::{initialize_database, register_user, add_device_to_user};

// Initialize database (auto-migrates schema)
let conn = initialize_database("app.db")?;

// Register a user
let user = register_user(
    &conn,
    "alice".to_string(),
    "alice@example.com".to_string(),
    "secure_password".to_string(),
)?;

// Register a device
let device = add_device_to_user(
    &conn,
    user.user_id,
    "ios".to_string(),
    None,
)?;

println!("User {} registered on device {}", user.user_name, device.device_id);
```

## Architecture

### Core Components

```
┌─────────────────────────────────────────────────────────────┐
│                      Your Application                        │
│                   (Tables, Business Logic)                   │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│                     nexus-core API                           │
├─────────────────────────────────────────────────────────────┤
│  ┌────────────┐  ┌─────────────┐  ┌───────────────────┐    │
│  │   Users    │  │   Devices   │  │  Operation Log    │    │
│  │   & Auth   │  │  Management │  │   (CRDT Oplog)    │    │
│  └────────────┘  └─────────────┘  └───────────────────┘    │
│                                                               │
│  ┌────────────┐  ┌─────────────┐  ┌───────────────────┐    │
│  │  P2P Sync  │  │    CRDT     │  │  Device Auth      │    │
│  │  (libp2p)  │  │    (HLC)    │  │  (QR Pairing)     │    │
│  └────────────┘  └─────────────┘  └───────────────────┘    │
└─────────────────────────────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│                     SQLite Database                          │
│  ┌─────────────┐  ┌───────────────┐  ┌──────────────┐      │
│  │   users     │  │   devices     │  │    oplog     │      │
│  └─────────────┘  └───────────────┘  └──────────────┘      │
│  ┌─────────────┐     Your App Tables                        │
│  │   peers     │     .....................                   │
│  └─────────────┘     .....................                   │
└─────────────────────────────────────────────────────────────┘
```

### How It Works

1. **Your app creates/updates data** in your own tables
2. **Records operations** in the oplog using `build_oplog_entry()`
3. **P2P sync** automatically exchanges oplogs between devices
4. **CRDT merge** applies remote operations with conflict resolution
5. **Your app handles** table-specific updates based on oplog entries

## Integration Guide

### 1. Initialize Database

```rust
use cfost::initialize_database;

let conn = initialize_database("app.db")?;
// Creates users, devices, oplog, and peers tables automatically
```

### 2. User & Device Management

```rust
use cfost::{register_user, add_device_to_user, login_user};

// Register user
let user = register_user(&conn, username, email, password)?;

// Login
let user = login_user(&conn, username_or_email, password)?;

// Add device
let device = add_device_to_user(&conn, user.user_id, "android", None)?;
```

### 3. Track Operations in Your App

```rust
use cfost::{build_oplog_entry, local_apply};

// Your app creates a record
conn.execute(
    "INSERT INTO my_app_table (id, value) VALUES (?1, ?2)",
    params![id, value],
)?;

// Record the operation for sync
let entry = build_oplog_entry(
    device_id,
    "my_app_table",
    "create",
    &serde_json::json!({"id": id, "value": value}),
)?;
local_apply(&mut conn, &entry)?;
```

### 4. Set Up P2P Sync

```rust
use cfost::{create_swarm, P2PConfig, SyncMessage};

// Create P2P swarm with auto-discovery
let mut swarm = create_swarm().await?;

// Or with custom config
let config = P2PConfig {
    enable_mdns: true,
    enable_relay: true,
    relay_servers: vec!["relay.example.com".to_string()],
    ..Default::default()
};
let mut swarm = create_swarm_with_config(config).await?;

// Handle incoming sync messages
while let Some(event) = swarm.select_next_some().await {
    match event {
        SwarmEvent::Behaviour(event) => {
            // Handle sync messages, merge operations, etc.
        }
        _ => {}
    }
}
```

### 5. Implement Conflict Resolution

```rust
use cfost::{merge, OplogEntry, HybridLogicalClock};

// Receive operations from peer
let remote_ops: Vec<OplogEntry> = get_from_peer();

// Merge into oplog
merge(&mut conn, &remote_ops)?;

// Apply to your tables with your conflict resolution strategy
for op in remote_ops {
    match op.table.as_str() {
        "my_app_table" => {
            // Your app-specific logic
            // Use op.timestamp (HLC) for last-write-wins or custom logic
            apply_my_app_table_op(&conn, &op)?;
        }
        _ => {}
    }
}
```

## CLI Tool

Nexus-core includes a CLI for managing the sync daemon:

```bash
# Install CLI
cargo install --path . --features cli

# Initialize with user
nexus-cli init --user alice --email alice@example.com

# Start sync daemon
nexus-cli start --daemon

# Check status
nexus-cli status

# Manage devices
nexus-cli device list
nexus-cli device add --type ios

# Manage peers
nexus-cli peer list
nexus-cli peer add /ip4/192.168.1.100/tcp/4001/p2p/12D3...

# View logs
nexus-cli logs --follow
```

See [docs/CLI_USAGE.md](docs/CLI_USAGE.md) for complete CLI documentation.

## Advanced Usage

### Custom CRDT Implementation

```rust
use cfost::{HybridLogicalClock, OplogEntry};

// Use HLC for causal ordering
let hlc = HybridLogicalClock::now();

// Create operation with causal timestamp
let entry = OplogEntry {
    id: Uuid::new_v4(),
    device_id,
    timestamp: hlc.to_timestamp(),
    table: "my_table".to_string(),
    op_type: "update".to_string(),
    data: serde_json::to_value(&my_data)?,
};
```

### Device Authorization Workflow

```rust
use cfost::{DeviceAuthManager, AuthorizerWorkflow, NewDeviceWorkflow};

// On device with account (authorizer)
let manager = DeviceAuthManager::new(&conn, user_id, device_id);
let mut workflow = AuthorizerWorkflow::new();

// Generate QR code
let challenge = workflow.create_challenge()?;
let qr_code = workflow.get_qr_code_string()?;
display_qr_code(&qr_code); // Show to user

// On new device
let mut new_workflow = NewDeviceWorkflow::new();
let scanned_challenge = scan_qr_code()?;
let response = new_workflow.respond_to_challenge(&scanned_challenge, device_id)?;

// Complete authorization
workflow.verify_response(&response)?;
```

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Test specific module
cargo test --lib --test integration_test
```

## Platform Support

| Platform | Architectures | Status |
|----------|--------------|--------|
| **iOS** | arm64, x86_64-sim | ✅ Tested |
| **Android** | arm64-v8a, armeabi-v7a, x86_64 | ✅ Tested |
| **macOS** | arm64 (M1/M2/M3), x86_64 | ✅ Tested |
| **Windows** | x64, arm64 | ✅ Tested |
| **Linux** | x86_64, arm64 | ✅ Tested |
| **WatchOS** | arm64, arm64-sim | ⚙️ Supported |
| **WearOS** | arm64-v8a, armeabi-v7a | ⚙️ Supported |

## Performance

- **Binary size**: Optimized with LTO and size optimization
- **Network efficiency**: Only syncs changes since last sync (incremental)
- **Memory efficient**: Streaming oplog processing
- **Offline capable**: All operations work offline, sync when connected

## Security

- **Password hashing**: Argon2 with cryptographic salts
- **Timing-safe comparison**: Constant-time password verification
- **SQL injection prevention**: Parameterized queries only
- **Encrypted transport**: TLS/Noise protocol for P2P communication
- **Device authorization**: Challenge-response authentication
- **UUID primary keys**: Prevents enumeration attacks

## Documentation

| Document | Description |
|----------|-------------|
| [CLI_USAGE.md](docs/CLI_USAGE.md) | Complete CLI tool guide |
| [DATABASE_MIGRATIONS.md](docs/DATABASE_MIGRATIONS.md) | Migration system guide |
| [API Documentation](https://docs.rs/nexus-core) | Complete API reference |

### Generate Local Docs

```bash
cargo doc --open
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Write tests for new functionality
4. Ensure all tests pass: `cargo test`
5. Format code: `cargo fmt`
6. Check for issues: `cargo clippy`
7. Submit a pull request

## Versioning

This project uses semantic versioning:

- **0.1.0**: Initial development release

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [SQLite](https://www.sqlite.org/) - Embedded database
- [rusqlite](https://github.com/rusqlite/rusqlite) - Rust SQLite bindings
- [libp2p](https://libp2p.io/) - P2P networking library
- [Argon2](https://github.com/P-H-C/phc-winner-argon2) - Password hashing
- [Chrono](https://github.com/chronotope/chrono) - Date and time library
- [UUID](https://github.com/uuid-rs/uuid) - Unique identifiers
- [Serde](https://serde.rs/) - Serialization framework

---

**Database synchronization made simple.**
