# Migration Quick Start Guide

Quick reference for common migration tasks in ahenk.

## For End Users

### Check Your Database Version

```rust
use ahenk::{initialize_database, get_current_version};

let conn = initialize_database("nexus.db")?;
let version = get_current_version(&conn)?;
println!("Database schema version: {}", version);
```

### Upgrade Database

Just open the database - migrations run automatically:

```rust
use ahenk::initialize_database;

// Opens database and applies any pending migrations
let conn = initialize_database("nexus.db")?;
```

### View Migration History

```rust
use ahenk::{initialize_database, get_migration_history};

let conn = initialize_database("nexus.db")?;
let history = get_migration_history(&conn)?;

for (version, applied_at, description) in history {
    println!("Version {}: {}", version, description);
    println!("  Applied: {}", applied_at);
}
```

### Backup Before Upgrading

```bash
# Create backup before running new version
cp nexus.db nexus.db.backup.$(date +%Y%m%d)

# Run your application (migrations apply automatically)
./your-app

# If something goes wrong:
# cp nexus.db.backup.20241021 nexus.db
```

---

## For Developers

### Creating a New Migration

**1. Create migration file:**

```bash
cd ahenk/src/db/migrations
nano 002_add_task_tags.sql
```

```sql
-- Migration 002: Add task tagging system
-- Description: Creates tags table and task_tags junction table
-- Applied: 2024-10-21

CREATE TABLE IF NOT EXISTS tags (
    tag_id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    color TEXT DEFAULT '#3B82F6',
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

CREATE TABLE IF NOT EXISTS task_tags (
    task_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    PRIMARY KEY (task_id, tag_id),
    FOREIGN KEY (task_id) REFERENCES tasks(task_id),
    FOREIGN KEY (tag_id) REFERENCES tags(tag_id)
);

CREATE INDEX IF NOT EXISTS idx_tags_user ON tags(user_id);
CREATE INDEX IF NOT EXISTS idx_task_tags_task ON task_tags(task_id);
```

**2. Register in migrations.rs:**

```rust
const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        description: "Initial schema with all core tables",
        sql: include_str!("migrations/001_initial_schema.sql"),
    },
    Migration {
        version: 2,
        description: "Add task tagging system",
        sql: include_str!("migrations/002_add_task_tags.sql"),
    },
];
```

**3. Test:**

```bash
cd ahenk
cargo test migrations
```

**4. Update models (if needed):**

```rust
// In models.rs
pub struct Tag {
    pub tag_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub color: String,
}

pub struct TaskTag {
    pub task_id: Uuid,
    pub tag_id: Uuid,
}
```

**5. Add CRUD operations (if needed):**

```rust
// In operations.rs
pub fn create_tag(conn: &Connection, tag: &Tag) -> Result<()> { ... }
pub fn get_tags_for_task(conn: &Connection, task_id: Uuid) -> Result<Vec<Tag>> { ... }
pub fn add_tag_to_task(conn: &Connection, task_id: Uuid, tag_id: Uuid) -> Result<()> { ... }
```

### Testing Migrations

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_002_tags() {
        let conn = Connection::open_in_memory().unwrap();

        // Apply all migrations
        apply_migrations(&conn).unwrap();

        // Verify tags table exists
        let result: i32 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='tags'",
            [],
            |row| row.get(0)
        ).unwrap();

        assert_eq!(result, 1);
    }
}
```

### Common Migration Patterns

#### Add Column

```sql
ALTER TABLE tasks ADD COLUMN priority INTEGER DEFAULT 0;
```

#### Add Table

```sql
CREATE TABLE IF NOT EXISTS new_table (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);
```

#### Add Index

```sql
CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date);
```

#### Add Column with Data Migration

```sql
-- Add column
ALTER TABLE tasks ADD COLUMN tags TEXT DEFAULT '[]';

-- Migrate existing data (if needed)
UPDATE tasks SET tags = '[]' WHERE tags IS NULL;
```

#### Rename Table (SQLite Limitation Workaround)

SQLite doesn't support `ALTER TABLE RENAME COLUMN`, so:

```sql
-- Create new table with desired schema
CREATE TABLE tasks_new (
    task_id TEXT PRIMARY KEY,
    list_id TEXT NOT NULL,
    content TEXT NOT NULL,
    is_completed INTEGER NOT NULL DEFAULT 0,
    due_date TEXT,
    created_at TEXT NOT NULL,
    priority INTEGER DEFAULT 0  -- New column
);

-- Copy data
INSERT INTO tasks_new (task_id, list_id, content, is_completed, due_date, created_at)
SELECT task_id, list_id, content, is_completed, due_date, created_at FROM tasks;

-- Drop old table
DROP TABLE tasks;

-- Rename new table
ALTER TABLE tasks_new RENAME TO tasks;

-- Recreate indexes
CREATE INDEX IF NOT EXISTS idx_tasks_list ON tasks(list_id);
```

---

## For P2P Sync Developers

### Ensure P2P Compatibility

When creating migrations, ensure older clients can sync:

**✅ Good - Backward Compatible:**

```sql
-- New optional column with default
ALTER TABLE tasks ADD COLUMN priority INTEGER DEFAULT 0;
```

**❌ Bad - Breaking Change:**

```sql
-- Removing column breaks older clients
ALTER TABLE tasks DROP COLUMN content;

-- Changing type breaks older clients
ALTER TABLE tasks ALTER COLUMN priority TYPE TEXT;
```

### Testing Cross-Version Sync

```rust
#[test]
fn test_cross_version_sync() {
    // Client A: Old version (v1)
    let conn_a = Connection::open_in_memory().unwrap();
    apply_migrations_up_to(&conn_a, 1).unwrap();

    // Client B: New version (v2)
    let conn_b = Connection::open_in_memory().unwrap();
    apply_migrations(&conn_b).unwrap();

    // Create task on old client
    let task = Task { /* v1 fields only */ };
    create_task(&conn_a, &task).unwrap();

    // Generate oplog
    let oplog = create_oplog_entry(&conn_a, /* ... */).unwrap();

    // Apply to new client
    apply_oplog_entry(&conn_b, &oplog).unwrap();

    // Verify sync succeeded
    let synced_task = get_task(&conn_b, task.task_id).unwrap();
    assert_eq!(synced_task.content, task.content);
    assert_eq!(synced_task.priority, 0); // Default value
}
```

---

## Common Issues

### Migration Already Applied

**Error**: `UNIQUE constraint failed: schema_version.version`

**Cause**: Trying to re-apply a migration that's already been applied.

**Solution**: This is fine - migrations are idempotent. The error is caught and ignored.

### SQLite Lock Error

**Error**: `database is locked`

**Cause**: Another process has the database open.

**Solution**: Close all connections to the database and try again.

### Foreign Key Violation

**Error**: `FOREIGN KEY constraint failed`

**Cause**: Migration tries to create data that violates foreign key constraints.

**Solution**: Ensure referenced rows exist before inserting dependent rows:

```sql
-- Wrong order
INSERT INTO tasks (task_id, list_id, ...) VALUES (...);
INSERT INTO task_lists (list_id, ...) VALUES (...);

-- Correct order
INSERT INTO task_lists (list_id, ...) VALUES (...);
INSERT INTO tasks (task_id, list_id, ...) VALUES (...);
```

---

## Cheat Sheet

| Task | Command |
|------|---------|
| Check version | `get_current_version(&conn)?` |
| Apply migrations | `apply_migrations(&conn)?` (automatic in `initialize_database`) |
| View history | `get_migration_history(&conn)?` |
| Create migration | Create `XXX_name.sql`, update `migrations.rs` |
| Test migrations | `cargo test migrations` |
| Backup database | `cp nexus.db nexus.db.backup` |

---

## Best Practices

1. **Always backup** before running migrations
2. **Test migrations** on a copy of production data
3. **Use IF NOT EXISTS** for idempotency
4. **Provide defaults** for new columns (P2P compatibility)
5. **Never remove columns** - deprecate instead
6. **Document breaking changes** in migration comments
7. **Version models** alongside schema changes
8. **Test cross-version sync** for P2P systems

---

## Further Reading

- [Full Migration Guide](./DATABASE_MIGRATIONS.md)
- [Migration File Examples](../src/db/migrations/README.md)
- [P2P Sync Architecture](./P2P_SYNC.md)
