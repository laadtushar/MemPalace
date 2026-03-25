use tauri::State;

use crate::app_state::AppState;
use crate::query::rag_pipeline::{self, RagResponse};

#[tauri::command]
pub fn ask(
    query: String,
    state: State<'_, AppState>,
) -> Result<RagResponse, String> {
    let llm = state
        .llm_provider
        .read()
        .map_err(|e| format!("Lock error: {}", e))?;

    let embedding = state
        .embedding_provider
        .read()
        .map_err(|e| format!("Lock error: {}", e))?;

    tauri::async_runtime::block_on(rag_pipeline::query_rag(
        &query,
        state.document_store.as_ref(),
        state.vector_store.as_ref(),
        state.page_index.as_ref(),
        state.memory_store.as_ref(),
        embedding.as_ref(),
        llm.as_ref(),
        5,
    ))
    .map_err(|e| e.to_string())
}
