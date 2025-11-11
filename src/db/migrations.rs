use chrono::Utc;
use rusqlite::{Connection, Result};

/// Represents a single database migration
pub struct Migration {
    pub version: i32,
    pub description: &'static str,
    pub sql: &'static str,
}

/// List of all migrations in order
/// Each migration should be numbered sequentially starting from 1
const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        description: "Initial schema - database synchronization infrastructure",
        sql: include_str!("migrations/001_initial_schema.sql"),
    },
];

/// Initialize the schema_version table if it doesn't exist
fn ensure_schema_version_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL,
            description TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

/// Get the current schema version
/// Returns 0 if no migrations have been applied yet
pub fn get_current_version(conn: &Connection) -> Result<i32> {
    ensure_schema_version_table(conn)?;

    let version: Result<Option<i32>> =
        conn.query_row("SELECT MAX(version) FROM schema_version", [], |row| {
            row.get(0)
        });

    match version {
        Ok(Some(v)) => Ok(v),
        Ok(None) => Ok(0), // Table is empty, no migrations applied yet
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
        Err(e) => Err(e),
    }
}

/// Apply a single migration
fn apply_migration(conn: &Connection, migration: &Migration) -> Result<()> {
    // Execute the migration SQL
    conn.execute_batch(migration.sql)?;

    // Record the migration in schema_version table
    conn.execute(
        "INSERT INTO schema_version (version, applied_at, description) VALUES (?1, ?2, ?3)",
        rusqlite::params![
            migration.version,
            Utc::now().to_rfc3339(),
            migration.description
        ],
    )?;

    Ok(())
}

/// Apply all pending migrations
pub fn apply_migrations(conn: &Connection) -> Result<()> {
    ensure_schema_version_table(conn)?;

    let current_version = get_current_version(conn)?;

    for migration in MIGRATIONS {
        if migration.version > current_version {
            println!(
                "Applying migration {}: {}",
                migration.version, migration.description
            );
            apply_migration(conn, migration)?;
            println!("Migration {} applied successfully", migration.version);
        }
    }

    Ok(())
}

/// Get migration history
pub fn get_migration_history(conn: &Connection) -> Result<Vec<(i32, String, String)>> {
    ensure_schema_version_table(conn)?;

    let mut stmt = conn.prepare(
        "SELECT version, applied_at, description FROM schema_version ORDER BY version ASC",
    )?;

    let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;

    let mut history = Vec::new();
    for row in rows {
        history.push(row?);
    }

    Ok(history)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_initial_version_is_zero() {
        let conn = Connection::open_in_memory().unwrap();
        let version = get_current_version(&conn).unwrap();
        assert_eq!(version, 0);
    }

    #[test]
    fn test_apply_migrations() {
        let conn = Connection::open_in_memory().unwrap();
        apply_migrations(&conn).unwrap();
        let version = get_current_version(&conn).unwrap();
        assert_eq!(version, MIGRATIONS.len() as i32);
    }

    #[test]
    fn test_migrations_are_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        apply_migrations(&conn).unwrap();
        let version1 = get_current_version(&conn).unwrap();

        // Apply migrations again
        apply_migrations(&conn).unwrap();
        let version2 = get_current_version(&conn).unwrap();

        // Version should not change
        assert_eq!(version1, version2);
    }

    #[test]
    fn test_migration_history() {
        let conn = Connection::open_in_memory().unwrap();
        apply_migrations(&conn).unwrap();

        let history = get_migration_history(&conn).unwrap();
        assert_eq!(history.len(), MIGRATIONS.len());

        // Verify migrations are in order
        for (i, (version, _, _)) in history.iter().enumerate() {
            assert_eq!(*version, (i + 1) as i32);
        }
    }
}
