use std::collections::HashMap;
use std::sync::Arc;

use rusqlite::params;

use crate::domain::ports::vector_store::{IVectorStore, ScoredResult};
use crate::error::AppError;

use super::connection::SqliteConnection;

/// SQLite-backed vector store using serialized float arrays.
/// Uses brute-force cosine similarity for search.
/// This is a lightweight fallback; can be swapped for LanceDB when available.
pub struct SqliteVectorStore {
    db: Arc<SqliteConnection>,
}

impl SqliteVectorStore {
    pub fn new(db: Arc<SqliteConnection>) -> Result<Self, AppError> {
        // Create the vectors table if it doesn't exist
        db.with_conn(|conn| {
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS vectors (
                    id TEXT PRIMARY KEY,
                    vector BLOB NOT NULL,
                    dimensions INTEGER NOT NULL,
                    metadata TEXT NOT NULL DEFAULT '{}'
                );",
            )?;
            Ok(())
        })?;

        Ok(Self { db })
    }
}

impl IVectorStore for SqliteVectorStore {
    fn upsert(
        &self,
        id: &str,
        vector: &[f32],
        metadata: HashMap<String, String>,
    ) -> Result<(), AppError> {
        let blob = floats_to_blob(vector);
        let meta_json = serde_json::to_string(&metadata)?;

        self.db.with_conn(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO vectors (id, vector, dimensions, metadata) VALUES (?1, ?2, ?3, ?4)",
                params![id, blob, vector.len() as i64, meta_json],
            )?;
            Ok(())
        })
    }

    fn upsert_batch(
        &self,
        items: &[(String, Vec<f32>, HashMap<String, String>)],
    ) -> Result<(), AppError> {
        self.db.with_conn(|conn| {
            let tx = conn.unchecked_transaction()?;
            {
                let mut stmt = tx.prepare(
                    "INSERT OR REPLACE INTO vectors (id, vector, dimensions, metadata) VALUES (?1, ?2, ?3, ?4)",
                )?;
                for (id, vector, metadata) in items {
                    let blob = floats_to_blob(vector);
                    let meta_json = serde_json::to_string(metadata)?;
                    stmt.execute(params![id, blob, vector.len() as i64, meta_json])?;
                }
            }
            tx.commit()?;
            Ok(())
        })
    }

    fn search(
        &self,
        query_vector: &[f32],
        top_k: usize,
        _filter: Option<&HashMap<String, String>>,
    ) -> Result<Vec<ScoredResult>, AppError> {
        self.db.with_conn(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, vector, metadata FROM vectors WHERE dimensions = ?1",
            )?;
            let dims = query_vector.len() as i64;

            let rows = stmt.query_map(params![dims], |row| {
                let id: String = row.get(0)?;
                let blob: Vec<u8> = row.get(1)?;
                let meta_str: String = row.get(2)?;
                Ok((id, blob, meta_str))
            })?;

            let mut scored: Vec<(String, f32, HashMap<String, String>)> = Vec::new();
            for row in rows {
                let (id, blob, meta_str) = row?;
                let stored_vector = blob_to_floats(&blob);
                let similarity = cosine_similarity(query_vector, &stored_vector);
                let metadata: HashMap<String, String> =
                    serde_json::from_str(&meta_str).unwrap_or_default();
                scored.push((id, similarity, metadata));
            }

            // Sort by similarity descending
            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            scored.truncate(top_k);

            Ok(scored
                .into_iter()
                .map(|(id, score, metadata)| ScoredResult {
                    id,
                    score,
                    metadata,
                })
                .collect())
        })
    }

    fn delete(&self, id: &str) -> Result<(), AppError> {
        self.db.with_conn(|conn| {
            conn.execute("DELETE FROM vectors WHERE id = ?1", params![id])?;
            Ok(())
        })
    }
}

fn floats_to_blob(floats: &[f32]) -> Vec<u8> {
    floats
        .iter()
        .flat_map(|f| f.to_le_bytes())
        .collect()
}

fn blob_to_floats(blob: &[u8]) -> Vec<f32> {
    blob.chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect()
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);

        let c = vec![0.0, 1.0, 0.0];
        assert!(cosine_similarity(&a, &c).abs() < 0.001); // orthogonal

        let d = vec![-1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &d) + 1.0).abs() < 0.001); // opposite
    }

    #[test]
    fn test_upsert_and_search() {
        let db = Arc::new(SqliteConnection::open_in_memory().unwrap());
        let store = SqliteVectorStore::new(db).unwrap();

        // Insert vectors
        store
            .upsert("v1", &[1.0, 0.0, 0.0], HashMap::from([("doc".into(), "d1".into())]))
            .unwrap();
        store
            .upsert("v2", &[0.0, 1.0, 0.0], HashMap::from([("doc".into(), "d2".into())]))
            .unwrap();
        store
            .upsert("v3", &[0.9, 0.1, 0.0], HashMap::from([("doc".into(), "d3".into())]))
            .unwrap();

        // Search: closest to [1, 0, 0] should be v1 then v3
        let results = store.search(&[1.0, 0.0, 0.0], 2, None).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, "v1");
        assert_eq!(results[1].id, "v3");
        assert!(results[0].score > results[1].score);
    }

    #[test]
    fn test_batch_upsert() {
        let db = Arc::new(SqliteConnection::open_in_memory().unwrap());
        let store = SqliteVectorStore::new(db).unwrap();

        let items = vec![
            ("v1".to_string(), vec![1.0, 0.0], HashMap::new()),
            ("v2".to_string(), vec![0.0, 1.0], HashMap::new()),
        ];
        store.upsert_batch(&items).unwrap();

        let results = store.search(&[1.0, 0.0], 10, None).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_delete() {
        let db = Arc::new(SqliteConnection::open_in_memory().unwrap());
        let store = SqliteVectorStore::new(db).unwrap();

        store.upsert("v1", &[1.0, 0.0], HashMap::new()).unwrap();
        store.delete("v1").unwrap();

        let results = store.search(&[1.0, 0.0], 10, None).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_blob_roundtrip() {
        let original = vec![1.5, -2.3, 0.0, 42.0];
        let blob = floats_to_blob(&original);
        let restored = blob_to_floats(&blob);
        assert_eq!(original, restored);
    }
}
