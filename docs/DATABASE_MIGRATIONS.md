# Database Migration & Versioning Guide

This document describes the database migration system for `nexus-core` and provides step-by-step procedures for client upgrades.

## Table of Contents

1. [Overview](#overview)
2. [Migration System Architecture](#migration-system-architecture)
3. [Client Upgrade Procedures](#client-upgrade-procedures)
4. [Creating New Migrations](#creating-new-migrations)
5. [P2P Sync Considerations](#p2p-sync-considerations)
6. [Troubleshooting](#troubleshooting)

---

## Overview

The nexus-core database uses a migration-based versioning system to manage schema evolution. This ensures:

- **Seamless upgrades**: Clients can upgrade their local database schema automatically
- **Version tracking**: The system knows which version each database is at
- **Safe deployments**: Migrations are applied sequentially and idempotently
- **P2P compatibility**: Different client versions can sync data (with graceful degradation)

### Key Components

1. **`schema_version` table**: Tracks which migrations have been applied
2. **Migration files**: Numbered SQL files in `src/db/migrations/`
3. **Migration runner**: Rust module that applies pending migrations
4. **Auto-migration**: Database upgrades happen automatically on app start

---

## Migration System Architecture

### Schema Version Table

Every database has a `schema_version` table:

```sql
CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL,
    description TEXT NOT NULL
);
```

Each applied migration creates a row in this table, allowing the system to track which migrations have been applied.

### Migration Files

Located in `nexus-core/src/db/migrations/`:

```
migrations/
├── 001_initial_schema.sql
├── 002_add_feature_x.sql
├── 003_update_table_y.sql
└── README.md
```

Each migration:
- Is numbered sequentially (001, 002, 003, ...)
- Contains SQL DDL statements
- Is embedded in the binary at compile time
- Is idempotent (uses `CREATE TABLE IF NOT EXISTS`, etc.)

### Migration Runner

The `migrations.rs` module:

```rust
pub fn apply_migrations(conn: &Connection) -> Result<()>
pub fn get_current_version(conn: &Connection) -> Result<i32>
pub fn get_migration_history(conn: &Connection) -> Result<Vec<(i32, String, String)>>
```

Automatically called by `initialize_database()`.

---

## Client Upgrade Procedures

### Automatic Upgrade (Recommended)

**For most users, upgrades happen automatically.**

When a client starts with a new version of the app:

1. App calls `initialize_database("nexus.db")`
2. Migration system checks current version
3. Applies any pending migrations
4. App is ready to use

**No manual intervention required.**

#### Example

```rust
use nexus_core::initialize_database;

// This will automatically upgrade the database if needed
let conn = initialize_database("nexus.db")?;
```

### Checking Current Version

To check which version your database is at:

```rust
use nexus_core::{initialize_database, get_current_version};

let conn = initialize_database("nexus.db")?;
let version = get_current_version(&conn)?;
println!("Current schema version: {}", version);
```

Or query directly:

```sql
SELECT MAX(version) FROM schema_version;
```

### Viewing Migration History

```rust
use nexus_core::{initialize_database, get_migration_history};

let conn = initialize_database("nexus.db")?;
let history = get_migration_history(&conn)?;

for (version, applied_at, description) in history {
    println!("v{}: {} (applied: {})", version, description, applied_at);
}
```

### Manual Migration (Advanced)

For advanced users who want explicit control:

```rust
use rusqlite::Connection;
use nexus_core::db::migrations::{apply_migrations, get_current_version};

// Open connection without auto-migration
let conn = Connection::open("nexus.db")?;

// Check current version
let current = get_current_version(&conn)?;
println!("Current version: {}", current);

// Manually apply migrations
apply_migrations(&conn)?;

let new_version = get_current_version(&conn)?;
println!("Upgraded to version: {}", new_version);
```

---

## Creating New Migrations

### Step 1: Create Migration File

Create a new file in `nexus-core/src/db/migrations/`:

```sql
-- Migration 002: Add task priority feature
-- Description: Adds priority column to tasks table for task ordering
-- Applied: 2024-10-21

ALTER TABLE tasks ADD COLUMN priority INTEGER DEFAULT 0;

CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks(priority);
```

**Naming Convention**: `XXX_description.sql` where `XXX` is the next sequential number.

### Step 2: Register Migration

Update `nexus-core/src/db/migrations.rs`:

```rust
const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        description: "Initial schema with all core tables",
        sql: include_str!("migrations/001_initial_schema.sql"),
    },
    Migration {
        version: 2,
        description: "Add task priority feature",
        sql: include_str!("migrations/002_add_priority.sql"),
    },
];
```

### Step 3: Test Migration

```bash
cd nexus-core
cargo test migrations
```

### Step 4: Update Models (if needed)

If you added new fields, update `models.rs`:

```rust
pub struct Task {
    pub task_id: Uuid,
    pub list_id: Uuid,
    pub content: String,
    pub is_completed: bool,
    pub due_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub priority: i32,  // New field
}
```

### Step 5: Document Breaking Changes

If the migration introduces breaking changes, document them in:
- `CHANGELOG.md`
- Migration file comments
- This document

---

## P2P Sync Considerations

The nexus-core system uses P2P synchronization via oplogs (operation logs). This creates unique challenges for schema versioning.

### Version Compatibility Matrix

| Client A | Client B | Sync Status | Notes |
|----------|----------|-------------|-------|
| v1 | v1 | ✅ Full sync | Perfect compatibility |
| v1 | v2 | ⚠️ Partial sync | v2 fields ignored by v1 |
| v2 | v1 | ⚠️ Partial sync | v1 can't write to v2 fields |
| v2 | v2 | ✅ Full sync | Perfect compatibility |

### Best Practices for P2P-Compatible Migrations

#### 1. Always Add, Rarely Remove

**Good**:
```sql
-- Add new optional column
ALTER TABLE tasks ADD COLUMN priority INTEGER DEFAULT 0;
```

**Bad**:
```sql
-- Removing column breaks older clients
ALTER TABLE tasks DROP COLUMN content;
```

#### 2. Use Default Values

Always provide defaults for new columns:

```sql
ALTER TABLE tasks ADD COLUMN tags TEXT DEFAULT '[]';
```

This allows older clients to continue creating records without knowledge of the new field.

#### 3. Handle Missing Fields Gracefully

In your application logic:

```rust
// When syncing from an older client
let priority = oplog.get_value("priority").unwrap_or(0);
```

#### 4. Oplog Compatibility

The oplog table structure should remain stable. When adding new tables:

```sql
-- Oplogs work automatically because they reference:
-- - table_name (VARCHAR)
-- - row_id (UUID)
-- - column_name (VARCHAR)
-- - new_value (TEXT)
```

#### 5. Version Negotiation (Future Enhancement)

Consider implementing version exchange during P2P handshake:

```rust
pub struct PeerInfo {
    pub peer_id: Uuid,
    pub schema_version: i32,  // Exchange versions
    pub capabilities: Vec<String>,  // Feature flags
}
```

### Handling Version Mismatches

When clients with different versions sync:

1. **Newer → Older**: Newer client oplogs may reference unknown columns
   - Older client should log a warning and skip unknown operations
   - Core functionality remains intact

2. **Older → Newer**: Older client oplogs work perfectly
   - Newer client simply doesn't receive new field updates

**Example: Priority Field**

```
v1 Client (no priority) ←→ v2 Client (has priority)

v1 creates task:
  oplog: {table: "tasks", column: "content", value: "Buy milk"}
  ✅ v2 applies successfully (priority defaults to 0)

v2 creates task:
  oplog: {table: "tasks", column: "content", value: "Buy eggs"}
  oplog: {table: "tasks", column: "priority", value: "5"}
  ✅ v1 applies content
  ⚠️ v1 skips priority (unknown column)
```

### Migration Rollout Strategy

For production deployments:

1. **Phase 1 - Add Fields Only**: Deploy migration with new fields (DEFAULT values)
2. **Phase 2 - Wait for Adoption**: Allow 80%+ of clients to upgrade
3. **Phase 3 - Use New Fields**: Update app logic to use new features
4. **Phase 4 - Deprecate**: Mark old fields as deprecated (don't remove)

---

## Troubleshooting

### Database Won't Open

**Symptom**: `initialize_database()` fails with migration error

**Solution**:
```bash
# Check database version
sqlite3 nexus.db "SELECT * FROM schema_version;"

# Manually run migrations
cargo run --example check_migrations
```

### Migration Failed Midway

**Symptom**: Migration partially applied, database in inconsistent state

**Solution**:
```bash
# Option 1: Restore from backup
cp nexus.db.backup nexus.db

# Option 2: Fix manually
sqlite3 nexus.db
> DELETE FROM schema_version WHERE version = X;
> -- Manually fix tables
> -- Re-run migration
```

**Prevention**: Always backup before major migrations

### P2P Sync Not Working

**Symptom**: Peers can't sync after upgrade

**Causes**:
1. Breaking schema change
2. Oplog incompatibility
3. Version mismatch too large

**Solution**:
```rust
// Check peer schema version
let peer_version = get_peer_schema_version(peer_id)?;
let my_version = get_current_version(&conn)?;

if (my_version - peer_version).abs() > 3 {
    warn!("Large version gap: consider full resync");
}
```

### Unknown Column in Oplog

**Symptom**: Logs show "unknown column in oplog"

**This is normal** when syncing with newer clients.

**Action**: Update your client to the latest version to gain new features.

---

## Migration Checklist

Before deploying a new migration:

- [ ] Migration file created with proper naming
- [ ] Migration registered in `migrations.rs`
- [ ] Tests written and passing
- [ ] Models updated (if schema changed)
- [ ] Default values provided for new columns
- [ ] P2P compatibility considered
- [ ] Documentation updated
- [ ] Tested on copy of production database
- [ ] Backup strategy in place
- [ ] Rollback plan documented

---

## Future Enhancements

Planned improvements:

1. **Down Migrations**: Support for rollback
2. **Schema Validation**: Verify schema matches expected state
3. **Version Negotiation**: P2P protocol enhancement
4. **Migration Hooks**: Pre/post migration callbacks
5. **Online Migrations**: Apply migrations without downtime
6. **Conflict Resolution**: Better handling of oplog conflicts across versions

---

## Support

For issues related to database migrations:

1. Check this documentation
2. Review migration history: `SELECT * FROM schema_version`
3. Check logs for migration errors
4. File an issue on GitHub with:
   - Current schema version
   - Target schema version
   - Error messages
   - Steps to reproduce
