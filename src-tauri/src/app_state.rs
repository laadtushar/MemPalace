use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use crate::adapters::llm::ollama::OllamaProvider;
use crate::adapters::sqlite::connection::SqliteConnection;
use crate::adapters::sqlite::document_store::SqliteDocumentStore;
use crate::adapters::sqlite::graph_store::SqliteGraphStore;
use crate::adapters::sqlite::memory_store::SqliteMemoryStore;
use crate::adapters::sqlite::page_index::SqliteFts5Index;
use crate::adapters::sqlite::timeline_store::SqliteTimelineStore;
use crate::adapters::sqlite::vector_store::SqliteVectorStore;
use crate::domain::ports::document_store::IDocumentStore;
use crate::domain::ports::embedding_provider::IEmbeddingProvider;
use crate::domain::ports::graph_store::IGraphStore;
use crate::domain::ports::llm_provider::ILlmProvider;
use crate::domain::ports::memory_store::IMemoryStore;
use crate::domain::ports::page_index::IPageIndex;
use crate::domain::ports::timeline_store::ITimelineStore;
use crate::domain::ports::vector_store::IVectorStore;
use crate::error::AppError;

/// Central application state holding all adapter instances.
pub struct AppState {
    pub document_store: Box<dyn IDocumentStore>,
    pub vector_store: Box<dyn IVectorStore>,
    pub memory_store: Box<dyn IMemoryStore>,
    pub page_index: Box<dyn IPageIndex>,
    pub graph_store: Box<dyn IGraphStore>,
    pub timeline_store: Box<dyn ITimelineStore>,
    pub llm_provider: Arc<RwLock<Box<dyn ILlmProvider>>>,
    pub embedding_provider: Arc<RwLock<Box<dyn IEmbeddingProvider>>>,
}

impl AppState {
    /// Initialize AppState with default adapters (SQLite + Ollama).
    pub fn new(data_dir: PathBuf) -> Result<Self, AppError> {
        std::fs::create_dir_all(&data_dir)?;
        let db_path = data_dir.join("memory_palace.db");
        let db = Arc::new(SqliteConnection::open(&db_path)?);

        let vector_store = SqliteVectorStore::new(db.clone())?;

        let ollama_llm: Box<dyn ILlmProvider> = Box::new(OllamaProvider::new(
            "http://localhost:11434",
            "llama3.1:8b",
            "nomic-embed-text",
        ));

        let ollama_embed: Box<dyn IEmbeddingProvider> = Box::new(OllamaProvider::new(
            "http://localhost:11434",
            "llama3.1:8b",
            "nomic-embed-text",
        ));

        Ok(Self {
            document_store: Box::new(SqliteDocumentStore::new(db.clone())),
            vector_store: Box::new(vector_store),
            memory_store: Box::new(SqliteMemoryStore::new(db.clone())),
            page_index: Box::new(SqliteFts5Index::new(db.clone())),
            graph_store: Box::new(SqliteGraphStore::new(db.clone())),
            timeline_store: Box::new(SqliteTimelineStore::new(db)),
            llm_provider: Arc::new(RwLock::new(ollama_llm)),
            embedding_provider: Arc::new(RwLock::new(ollama_embed)),
        })
    }
}
