use crate::error::AppError;

/// A full-text search result with BM25 ranking
#[derive(Debug, Clone)]
pub struct FtsResult {
    pub chunk_id: String,
    pub snippet: String,
    pub rank_score: f64,
}

/// Port for full-text search with BM25 ranking
pub trait IPageIndex: Send + Sync {
    fn index_text(&self, chunk_id: &str, text: &str) -> Result<(), AppError>;
    fn index_batch(&self, items: &[(String, String)]) -> Result<(), AppError>;
    fn search(&self, query: &str, top_k: usize) -> Result<Vec<FtsResult>, AppError>;
    fn remove(&self, chunk_id: &str) -> Result<(), AppError>;
}
