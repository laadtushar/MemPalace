use crate::domain::models::insight::Insight;
use crate::domain::models::memory::MemoryFact;
use crate::domain::models::theme::ThemeSnapshot;
use crate::domain::ports::document_store::IDocumentStore;
use crate::domain::ports::llm_provider::ILlmProvider;
use crate::domain::ports::memory_store::IMemoryStore;
use crate::domain::ports::timeline_store::ITimelineStore;
use crate::error::AppError;

use super::belief_extractor;
use super::insight_generator;
use super::theme_extractor;

/// Result of a full analysis run.
#[derive(Debug, Clone, serde::Serialize)]
pub struct AnalysisResult {
    pub themes_extracted: usize,
    pub beliefs_extracted: usize,
    pub insights_generated: usize,
}

/// Run the full analysis pipeline: themes → beliefs → insights.
///
/// Sentiment tracking is deferred (requires per-chunk classification which
/// is expensive; will be added when batch LLM calls are optimized).
pub async fn run_analysis(
    document_store: &dyn IDocumentStore,
    timeline_store: &dyn ITimelineStore,
    memory_store: &dyn IMemoryStore,
    llm: &dyn ILlmProvider,
) -> Result<AnalysisResult, AppError> {
    // Check LLM availability
    if !llm.is_available().await {
        return Err(AppError::Analysis(
            "LLM provider is not available. Please start Ollama or configure an API key."
                .to_string(),
        ));
    }

    // Stage 1: Theme extraction
    log::info!("Analysis: extracting themes...");
    let themes = theme_extractor::extract_themes(
        document_store,
        timeline_store,
        llm,
        30, // max chunks per monthly window
    )
    .await?;
    log::info!("Analysis: extracted {} themes", themes.len());

    // Stage 2: Belief extraction from representative chunks
    log::info!("Analysis: extracting beliefs...");
    let months = timeline_store.get_document_count_by_month()?;
    let mut all_chunks: Vec<(String, String)> = Vec::new();

    // Sample chunks from each month for belief extraction
    for (_, _) in months.iter().take(12) {
        // Get chunks from recent documents
        let date_range = timeline_store.get_date_range()?;
        if let Some(range) = date_range {
            let doc_ids = timeline_store.get_documents_in_range(&range)?;
            for doc_id in doc_ids.iter().take(5) {
                let chunks = document_store.get_chunks_by_document(doc_id)?;
                for chunk in chunks.into_iter().take(2) {
                    all_chunks.push((chunk.id, chunk.text));
                }
            }
        }
        break; // Only sample once for now
    }

    let beliefs = if !all_chunks.is_empty() {
        belief_extractor::extract_beliefs(&all_chunks, llm).await?
    } else {
        Vec::new()
    };
    log::info!("Analysis: extracted {} beliefs", beliefs.len());

    // Store beliefs in memory store
    for fact in &beliefs {
        if let Err(e) = memory_store.store(fact) {
            log::warn!("Failed to store belief: {}", e);
        }
    }

    // Stage 3: Insight generation
    log::info!("Analysis: generating insights...");
    let insights = insight_generator::generate_insights(&themes, &beliefs, llm, 5).await?;
    log::info!("Analysis: generated {} insights", insights.len());

    Ok(AnalysisResult {
        themes_extracted: themes.len(),
        beliefs_extracted: beliefs.len(),
        insights_generated: insights.len(),
    })
}
