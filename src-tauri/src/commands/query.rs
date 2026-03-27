use tauri::Manager;

use crate::adapters::sqlite::activity_store::ActivityEntry;
use crate::app_state::AppState;
use crate::query::rag_pipeline;

#[derive(Debug, Clone, serde::Serialize)]
pub struct AskResponse {
    pub answer: String,
    pub sources: Vec<crate::query::rag_pipeline::RagSource>,
    pub conversation_id: Option<String>,
}

#[tauri::command]
pub async fn ask(
    query: String,
    conversation_id: Option<String>,
    mode: Option<String>,
    app_handle: tauri::AppHandle,
) -> Result<AskResponse, String> {
    let rag_mode = mode.as_deref().unwrap_or("auto");
    tracing::info!(query_len = query.len(), mode = rag_mode, "RAG query received");
    let start = std::time::Instant::now();

    let query_clone = query.clone();
    let mode_clone = rag_mode.to_string();
    let handle = app_handle.clone();

    let result = tokio::task::spawn_blocking(move || {
        let state = handle.state::<AppState>();
        let llm = state.llm_provider.read().map_err(|e| format!("Lock error: {}", e))?;

        match mode_clone.as_str() {
            "keyword" => {
                // BM25-only: no embeddings needed
                tauri::async_runtime::block_on(rag_pipeline::query_rag_bm25(
                    &query_clone,
                    state.document_store.as_ref(),
                    state.page_index.as_ref(),
                    state.memory_store.as_ref(),
                    llm.as_ref(),
                    5,
                )).map_err(|e| e.to_string())
            }
            "reasoning" => {
                // PageIndex-inspired: BM25 + LLM re-ranking
                tauri::async_runtime::block_on(rag_pipeline::query_rag_reasoning(
                    &query_clone,
                    state.document_store.as_ref(),
                    state.page_index.as_ref(),
                    state.memory_store.as_ref(),
                    llm.as_ref(),
                    5,
                )).map_err(|e| e.to_string())
            }
            _ => {
                // "auto" or "hybrid": standard RAG with embedding fallback
                let embedding = state.embedding_provider.read().map_err(|e| format!("Lock error: {}", e))?;
                tauri::async_runtime::block_on(rag_pipeline::query_rag(
                    &query_clone,
                    state.document_store.as_ref(),
                    state.vector_store.as_ref(),
                    state.page_index.as_ref(),
                    state.memory_store.as_ref(),
                    embedding.as_ref(),
                    llm.as_ref(),
                    5,
                )).map_err(|e| e.to_string())
            }
        }
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
    .map_err(|e: String| {
        tracing::error!(error = %e, "RAG query failed");
        e
    })?;

    let duration_ms = start.elapsed().as_millis() as u64;
    tracing::info!(
        sources = result.sources.len(),
        answer_len = result.answer.len(),
        duration_ms = duration_ms,
        "RAG query complete"
    );

    // Persist to chat history
    let state = app_handle.state::<AppState>();
    let conv_id = if let Some(cid) = conversation_id {
        // Existing conversation
        let _ = state.chat_store.add_message(&cid, "user", &query, &[]);
        let _ = state.chat_store.add_message(&cid, "assistant", &result.answer, &result.sources);
        Some(cid)
    } else {
        // Create new conversation with first few words as title
        let title: String = query.chars().take(60).collect();
        match state.chat_store.create_conversation(&title) {
            Ok(conv) => {
                let _ = state.chat_store.add_message(&conv.id, "user", &query, &[]);
                let _ = state.chat_store.add_message(&conv.id, "assistant", &result.answer, &result.sources);
                Some(conv.id)
            }
            Err(e) => {
                tracing::warn!(error = %e, "Failed to create conversation");
                None
            }
        }
    };

    // Log activity
    let truncated_query: String = query.chars().take(100).collect();
    let _ = state.activity_store.log_activity(&ActivityEntry {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        action_type: "ask".to_string(),
        title: truncated_query,
        description: String::new(),
        result_summary: format!("{} sources", result.sources.len()),
        metadata: serde_json::json!({}),
        duration_ms: duration_ms as i64,
        status: "success".to_string(),
    });

    Ok(AskResponse {
        answer: result.answer,
        sources: result.sources,
        conversation_id: conv_id,
    })
}
