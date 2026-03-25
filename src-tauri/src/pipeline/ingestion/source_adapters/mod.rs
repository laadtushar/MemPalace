pub mod obsidian;
pub mod markdown;
pub mod dayone;

use crate::domain::models::document::Document;
use crate::error::AppError;

/// Trait for source adapters that parse platform-specific exports into Documents.
pub trait SourceAdapter: Send + Sync {
    /// Parse the source at the given path into a list of Documents.
    fn parse(&self, path: &std::path::Path) -> Result<Vec<Document>, AppError>;

    /// Human-readable name of this source adapter.
    fn name(&self) -> &str;
}
