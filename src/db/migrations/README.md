# Database Migrations

This directory contains SQL migration files for the nexus-core database schema.

## Migration Naming Convention

Migrations are numbered sequentially with the format:
```
XXX_description.sql
```

Where:
- `XXX` is a zero-padded 3-digit version number (e.g., 001, 002, 003)
- `description` is a brief description of what the migration does (use underscores)

Examples:
- `001_initial_schema.sql` - Initial database schema
- `002_add_task_priority.sql` - Add priority column to tasks table
- `003_create_tags_table.sql` - Create a new tags table

## Creating a New Migration

1. **Determine the next version number**: Check existing migrations and use the next sequential number.

2. **Create the migration file**: Create a new `.sql` file in this directory with the appropriate name.

3. **Write the migration SQL**: Include all DDL statements needed for the migration.
   - Use `CREATE TABLE IF NOT EXISTS` for new tables
   - Use `ALTER TABLE` for modifying existing tables
   - Include comments explaining the migration

4. **Add to migrations.rs**: Update `nexus-core/src/db/migrations.rs` to include the new migration:
   ```rust
   const MIGRATIONS: &[Migration] = &[
       // ... existing migrations ...
       Migration {
           version: 2,
           description: "Add priority column to tasks",
           sql: include_str!("migrations/002_add_task_priority.sql"),
       },
   ];
   ```

## Example Migration

Here's a template for a new migration:

```sql
-- Migration XXX: Brief Description
-- Description: Detailed description of what this migration does and why
-- Applied: YYYY-MM-DD (date when added to codebase)

-- Add new column example
ALTER TABLE tasks ADD COLUMN priority INTEGER DEFAULT 0;

-- Create new table example
CREATE TABLE IF NOT EXISTS tags (
    tag_id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    name VARCHAR(50) NOT NULL,
    color VARCHAR(7),
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- Create index example
CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks(priority);
```

## Migration Best Practices

1. **Idempotency**: Use `IF NOT EXISTS` clauses where possible to make migrations idempotent.

2. **Backward Compatibility**: Consider how changes affect existing data and older client versions.
   - Add new columns with DEFAULT values
   - Don't remove columns that older clients might need
   - Use feature flags or version checks when introducing breaking changes

3. **Data Migration**: If migrating data, include both DDL and DML in the same migration.

4. **Testing**: Always test migrations on a copy of production data before deploying.

5. **Documentation**: Include clear comments explaining why the migration is needed.

## Rollback Strategy

Currently, migrations only support forward migration (up). For critical deployments:

1. **Backup**: Always backup the database before applying migrations.
2. **Testing**: Test migrations in a staging environment first.
3. **Monitoring**: Monitor application behavior after migration deployment.

Future enhancement: Add support for down migrations for rollback capability.

## Checking Migration Status

To check which migrations have been applied:

```sql
SELECT * FROM schema_version ORDER BY version;
```

Or use the Rust API:
```rust
use cfost::db::migrations::{get_current_version, get_migration_history};

let conn = initialize_database("nexus.db")?;
let current_version = get_current_version(&conn)?;
let history = get_migration_history(&conn)?;
```

## P2P Sync Considerations

When working with P2P synchronized databases:

1. **Version Compatibility**: Ensure oplogs from newer versions can be applied to older schemas.
2. **Graceful Degradation**: New fields should have defaults so older clients can function.
3. **Schema Negotiation**: Consider implementing version negotiation in P2P protocol (future enhancement).
