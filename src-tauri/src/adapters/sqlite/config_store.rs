use std::sync::Arc;

use rusqlite::params;

use crate::error::AppError;

use super::connection::SqliteConnection;

/// SQLite-backed key-value config store.
pub struct SqliteConfigStore {
    db: Arc<SqliteConnection>,
}

impl SqliteConfigStore {
    pub fn new(db: Arc<SqliteConnection>) -> Self {
        Self { db }
    }

    /// Get a config value by key.
    pub fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        self.db.with_conn(|conn| {
            let mut stmt = conn.prepare("SELECT value FROM config WHERE key = ?1")?;
            let result = stmt
                .query_row(params![key], |row| row.get::<_, String>(0))
                .optional();
            match result {
                Ok(val) => Ok(val),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(AppError::from(e)),
            }
        })
    }

    /// Set a config value (upsert).
    pub fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        self.db.with_conn(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO config (key, value, updated_at) VALUES (?1, ?2, datetime('now'))",
                params![key, value],
            )?;
            Ok(())
        })
    }

    /// Delete a config value.
    pub fn delete(&self, key: &str) -> Result<(), AppError> {
        self.db.with_conn(|conn| {
            conn.execute("DELETE FROM config WHERE key = ?1", params![key])?;
            Ok(())
        })
    }

    /// Get all config values matching a prefix.
    pub fn get_by_prefix(&self, prefix: &str) -> Result<Vec<(String, String)>, AppError> {
        self.db.with_conn(|conn| {
            let pattern = format!("{}%", prefix);
            let mut stmt =
                conn.prepare("SELECT key, value FROM config WHERE key LIKE ?1 ORDER BY key")?;
            let rows = stmt.query_map(params![pattern], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;
            let mut results = Vec::new();
            for row in rows {
                results.push(row?);
            }
            Ok(results)
        })
    }
}

use rusqlite::OptionalExtension;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let db = Arc::new(SqliteConnection::open_in_memory().unwrap());
        let store = SqliteConfigStore::new(db);

        store.set("llm.provider", "ollama").unwrap();
        assert_eq!(store.get("llm.provider").unwrap(), Some("ollama".to_string()));
    }

    #[test]
    fn test_get_missing_key() {
        let db = Arc::new(SqliteConnection::open_in_memory().unwrap());
        let store = SqliteConfigStore::new(db);

        assert_eq!(store.get("nonexistent").unwrap(), None);
    }

    #[test]
    fn test_upsert() {
        let db = Arc::new(SqliteConnection::open_in_memory().unwrap());
        let store = SqliteConfigStore::new(db);

        store.set("key", "v1").unwrap();
        store.set("key", "v2").unwrap();
        assert_eq!(store.get("key").unwrap(), Some("v2".to_string()));
    }

    #[test]
    fn test_delete() {
        let db = Arc::new(SqliteConnection::open_in_memory().unwrap());
        let store = SqliteConfigStore::new(db);

        store.set("key", "val").unwrap();
        store.delete("key").unwrap();
        assert_eq!(store.get("key").unwrap(), None);
    }

    #[test]
    fn test_get_by_prefix() {
        let db = Arc::new(SqliteConnection::open_in_memory().unwrap());
        let store = SqliteConfigStore::new(db);

        store.set("llm.provider", "ollama").unwrap();
        store.set("llm.model", "llama3.1:8b").unwrap();
        store.set("embed.model", "nomic-embed-text").unwrap();

        let llm_configs = store.get_by_prefix("llm.").unwrap();
        assert_eq!(llm_configs.len(), 2);
    }
}
