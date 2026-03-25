use crate::error::AppError;
use std::collections::HashMap;

/// A scored search result from vector similarity search
#[derive(Debug, Clone)]
pub struct ScoredResult {
    pub id: String,
    pub score: f32,
    pub metadata: HashMap<String, String>,
}

/// Port for semantic embedding storage and similarity search
pub trait IVectorStore: Send + Sync {
    fn upsert(
        &self,
        id: &str,
        vector: &[f32],
        metadata: HashMap<String, String>,
    ) -> Result<(), AppError>;

    fn upsert_batch(
        &self,
        items: &[(String, Vec<f32>, HashMap<String, String>)],
    ) -> Result<(), AppError>;

    fn search(
        &self,
        query_vector: &[f32],
        top_k: usize,
        filter: Option<&HashMap<String, String>>,
    ) -> Result<Vec<ScoredResult>, AppError>;

    fn delete(&self, id: &str) -> Result<(), AppError>;
}
