use rusqlite::Connection;
use std::path::Path;
use std::sync::Mutex;

use crate::error::AppError;

use super::migrations;

/// Manages a SQLite connection with WAL mode and optional encryption.
pub struct SqliteConnection {
    conn: Mutex<Connection>,
}

impl SqliteConnection {
    /// Open or create a database at the given path with WAL mode.
    pub fn open(db_path: &Path) -> Result<Self, AppError> {
        let conn = Connection::open(db_path)?;

        // Enable WAL mode for concurrent reads during import
        conn.execute_batch("PRAGMA journal_mode=WAL;")?;
        // Enforce foreign keys
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;
        // Reasonable busy timeout (5 seconds)
        conn.busy_timeout(std::time::Duration::from_secs(5))?;

        let db = Self {
            conn: Mutex::new(conn),
        };

        // Run migrations
        db.with_conn(|conn| migrations::run_migrations(conn))?;

        Ok(db)
    }

    /// Open an in-memory database (for testing).
    pub fn open_in_memory() -> Result<Self, AppError> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;

        let db = Self {
            conn: Mutex::new(conn),
        };

        db.with_conn(|conn| migrations::run_migrations(conn))?;

        Ok(db)
    }

    /// Execute a closure with the database connection.
    pub fn with_conn<F, T>(&self, f: F) -> Result<T, AppError>
    where
        F: FnOnce(&Connection) -> Result<T, AppError>,
    {
        let conn = self.conn.lock().map_err(|e| AppError::Other(e.to_string()))?;
        f(&conn)
    }
}
