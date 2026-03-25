use crate::domain::models::common::{SourcePlatform, TimeRange};
use crate::domain::models::document::{Chunk, Document};
use crate::error::AppError;

/// Port for raw document persistence and retrieval
pub trait IDocumentStore: Send + Sync {
    fn save_document(&self, doc: &Document) -> Result<(), AppError>;
    fn get_by_id(&self, id: &str) -> Result<Option<Document>, AppError>;
    fn get_by_source(
        &self,
        platform: &SourcePlatform,
        time_range: Option<&TimeRange>,
    ) -> Result<Vec<Document>, AppError>;
    fn get_by_content_hash(&self, hash: &str) -> Result<Option<Document>, AppError>;
    fn delete_document(&self, id: &str) -> Result<bool, AppError>;

    fn save_chunk(&self, chunk: &Chunk) -> Result<(), AppError>;
    fn save_chunks(&self, chunks: &[Chunk]) -> Result<(), AppError>;
    fn get_chunks_by_document(&self, document_id: &str) -> Result<Vec<Chunk>, AppError>;
    fn get_chunk_by_id(&self, id: &str) -> Result<Option<Chunk>, AppError>;
    fn get_chunks_by_ids(&self, ids: &[String]) -> Result<Vec<Chunk>, AppError>;
}
