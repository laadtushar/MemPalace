use std::sync::Arc;

use crate::domain::ports::document_store::IDocumentStore;
use crate::domain::ports::embedding_provider::IEmbeddingProvider;
use crate::domain::ports::page_index::IPageIndex;
use crate::domain::ports::timeline_store::ITimelineStore;
use crate::error::AppError;

use super::chunker::{chunk_text, ChunkerConfig};
use super::dedup::deduplicate;
use super::normalizer::normalize_documents;
use super::source_adapters::SourceAdapter;

/// Summary of an import operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImportSummary {
    pub documents_imported: usize,
    pub chunks_created: usize,
    pub duplicates_skipped: usize,
    pub errors: Vec<String>,
    pub duration_ms: u64,
}

/// Progress callback type for reporting import progress.
pub type ProgressCallback = Box<dyn Fn(&str, usize, usize, &str) + Send>;

/// Orchestrates the full ingestion pipeline:
/// parse → dedup → normalize → chunk → store → FTS index
///
/// Embedding generation is optional (requires Ollama running).
pub struct IngestionOrchestrator<'a> {
    document_store: &'a dyn IDocumentStore,
    timeline_store: &'a dyn ITimelineStore,
    _page_index: &'a dyn IPageIndex, // FTS5 auto-syncs via triggers
    embedding_provider: Option<Arc<dyn IEmbeddingProvider>>,
    chunker_config: ChunkerConfig,
    embedding_batch_size: usize,
}

impl<'a> IngestionOrchestrator<'a> {
    pub fn new(
        document_store: &'a dyn IDocumentStore,
        timeline_store: &'a dyn ITimelineStore,
        page_index: &'a dyn IPageIndex,
    ) -> Self {
        Self {
            document_store,
            timeline_store,
            _page_index: page_index,
            embedding_provider: None,
            chunker_config: ChunkerConfig::default(),
            embedding_batch_size: 10,
        }
    }

    pub fn with_embedding_provider(mut self, provider: Arc<dyn IEmbeddingProvider>) -> Self {
        self.embedding_provider = Some(provider);
        self
    }

    pub fn with_chunker_config(mut self, config: ChunkerConfig) -> Self {
        self.chunker_config = config;
        self
    }

    /// Run the full ingestion pipeline for a given source adapter and path.
    pub async fn ingest(
        &self,
        adapter: &dyn SourceAdapter,
        path: &std::path::Path,
        on_progress: Option<&ProgressCallback>,
    ) -> Result<ImportSummary, AppError> {
        let start = std::time::Instant::now();
        let mut errors = Vec::new();

        // Stage 1: Parse
        report_progress(on_progress, "parsing", 0, 0, &format!("Parsing {} files...", adapter.name()));
        let mut documents = adapter.parse(path)?;
        let total_parsed = documents.len();
        report_progress(on_progress, "parsing", total_parsed, total_parsed, &format!("Parsed {} documents", total_parsed));

        // Stage 2: Dedup
        report_progress(on_progress, "dedup", 0, total_parsed, "Checking for duplicates...");
        let dedup_result = deduplicate(documents, self.document_store);
        let duplicates_skipped = dedup_result.duplicates_skipped;
        documents = dedup_result.new_documents;
        report_progress(on_progress, "dedup", documents.len(), total_parsed,
            &format!("{} new, {} duplicates skipped", documents.len(), duplicates_skipped));

        // Stage 3: Normalize
        report_progress(on_progress, "normalize", 0, documents.len(), "Normalizing text...");
        normalize_documents(&mut documents);
        report_progress(on_progress, "normalize", documents.len(), documents.len(), "Normalization complete");

        // Stage 4: Store documents + chunk + index
        let total_docs = documents.len();
        let mut chunks_created = 0;

        for (i, doc) in documents.iter().enumerate() {
            report_progress(on_progress, "storing", i, total_docs,
                &format!("Processing document {}/{}", i + 1, total_docs));

            // Save document
            if let Err(e) = self.document_store.save_document(doc) {
                errors.push(format!("Failed to save document {}: {}", doc.id, e));
                continue;
            }

            // Index in timeline
            if let Err(e) = self.timeline_store.index_document(doc) {
                errors.push(format!("Failed to index document {}: {}", doc.id, e));
            }

            // Chunk the document
            let chunks = chunk_text(&doc.id, &doc.raw_text, &self.chunker_config);
            let chunk_count = chunks.len();

            // Save chunks (FTS5 auto-indexes via trigger)
            if let Err(e) = self.document_store.save_chunks(&chunks) {
                errors.push(format!("Failed to save chunks for {}: {}", doc.id, e));
                continue;
            }

            chunks_created += chunk_count;
        }

        report_progress(on_progress, "storing", total_docs, total_docs,
            &format!("Stored {} documents, {} chunks", total_docs, chunks_created));

        // Stage 5: Generate embeddings (if provider available)
        if let Some(ref _provider) = self.embedding_provider {
            report_progress(on_progress, "embedding", 0, chunks_created, "Generating embeddings...");
            // Embedding generation will be implemented when vector store is added.
            // For now, skip this stage.
            report_progress(on_progress, "embedding", chunks_created, chunks_created,
                "Embedding generation deferred (vector store not yet configured)");
        }

        let duration_ms = start.elapsed().as_millis() as u64;

        report_progress(on_progress, "complete", total_docs, total_docs,
            &format!("Import complete in {}ms", duration_ms));

        Ok(ImportSummary {
            documents_imported: total_docs,
            chunks_created,
            duplicates_skipped,
            errors,
            duration_ms,
        })
    }
}

fn report_progress(
    on_progress: Option<&ProgressCallback>,
    stage: &str,
    current: usize,
    total: usize,
    message: &str,
) {
    if let Some(cb) = on_progress {
        cb(stage, current, total, message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::sqlite::connection::SqliteConnection;
    use crate::adapters::sqlite::document_store::SqliteDocumentStore;
    use crate::adapters::sqlite::page_index::SqliteFts5Index;
    use crate::adapters::sqlite::timeline_store::SqliteTimelineStore;
    use crate::domain::models::common::SourcePlatform;
    use crate::domain::models::document::Document;
    use chrono::Utc;
    use sha2::{Digest, Sha256};

    /// A mock source adapter that returns pre-built documents.
    struct MockAdapter {
        docs: Vec<Document>,
    }

    impl SourceAdapter for MockAdapter {
        fn parse(&self, _path: &std::path::Path) -> Result<Vec<Document>, AppError> {
            Ok(self.docs.clone())
        }
        fn name(&self) -> &str {
            "mock"
        }
    }

    fn make_doc(text: &str) -> Document {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        Document {
            id: uuid::Uuid::new_v4().to_string(),
            source_platform: SourcePlatform::Markdown,
            raw_text: text.to_string(),
            timestamp: Utc::now(),
            participants: vec![],
            metadata: serde_json::json!({}),
            content_hash: hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_full_pipeline() {
        let db = Arc::new(SqliteConnection::open_in_memory().unwrap());
        let doc_store = SqliteDocumentStore::new(db.clone());
        let timeline_store = SqliteTimelineStore::new(db.clone());
        let page_index = SqliteFts5Index::new(db);

        let orchestrator = IngestionOrchestrator::new(&doc_store, &timeline_store, &page_index);

        let adapter = MockAdapter {
            docs: vec![
                make_doc("Today I thought about the meaning of life. It was a beautiful day for reflection."),
                make_doc("I went hiking in the mountains. The air was fresh and clear."),
                make_doc("Meeting with the team about the project roadmap. We decided to focus on quality."),
            ],
        };

        let summary = orchestrator
            .ingest(&adapter, std::path::Path::new("/fake"), None)
            .await
            .unwrap();

        assert_eq!(summary.documents_imported, 3);
        assert!(summary.chunks_created >= 3); // at least one chunk per doc
        assert_eq!(summary.duplicates_skipped, 0);
        assert!(summary.errors.is_empty());
    }

    #[tokio::test]
    async fn test_dedup_in_pipeline() {
        let db = Arc::new(SqliteConnection::open_in_memory().unwrap());
        let doc_store = SqliteDocumentStore::new(db.clone());
        let timeline_store = SqliteTimelineStore::new(db.clone());
        let page_index = SqliteFts5Index::new(db);

        let orchestrator = IngestionOrchestrator::new(&doc_store, &timeline_store, &page_index);

        let doc = make_doc("Unique content for dedup test.");
        let adapter = MockAdapter {
            docs: vec![doc.clone()],
        };

        // First import
        let s1 = orchestrator
            .ingest(&adapter, std::path::Path::new("/fake"), None)
            .await
            .unwrap();
        assert_eq!(s1.documents_imported, 1);

        // Second import of same content — should be deduped
        let adapter2 = MockAdapter {
            docs: vec![doc],
        };
        let s2 = orchestrator
            .ingest(&adapter2, std::path::Path::new("/fake"), None)
            .await
            .unwrap();
        assert_eq!(s2.documents_imported, 0);
        assert_eq!(s2.duplicates_skipped, 1);
    }
}
