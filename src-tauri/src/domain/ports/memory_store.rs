use crate::domain::models::common::TimeRange;
use crate::domain::models::memory::{FactCategory, MemoryFact};
use crate::error::AppError;

/// Port for persistent memory / long-term context (Mem0-style)
pub trait IMemoryStore: Send + Sync {
    fn store(&self, fact: &MemoryFact) -> Result<(), AppError>;
    fn recall(&self, query: &str, top_k: usize) -> Result<Vec<MemoryFact>, AppError>;
    fn update(&self, id: &str, updated_fact: &MemoryFact) -> Result<(), AppError>;
    fn contradict(&self, id: &str, contradicting_fact_id: &str) -> Result<(), AppError>;
    fn forget(&self, id: &str) -> Result<(), AppError>;
    fn get_all(
        &self,
        category: Option<&FactCategory>,
        time_range: Option<&TimeRange>,
    ) -> Result<Vec<MemoryFact>, AppError>;
    fn get_by_id(&self, id: &str) -> Result<Option<MemoryFact>, AppError>;
}
