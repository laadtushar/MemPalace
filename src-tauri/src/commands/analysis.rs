use tauri::State;

use crate::app_state::AppState;
use crate::pipeline::analysis::orchestrator::{self, AnalysisResult};

#[tauri::command]
pub fn run_analysis(
    state: State<'_, AppState>,
) -> Result<AnalysisResult, String> {
    let llm = state
        .llm_provider
        .read()
        .map_err(|e| format!("Lock error: {}", e))?;

    tauri::async_runtime::block_on(orchestrator::run_analysis(
        state.document_store.as_ref(),
        state.timeline_store.as_ref(),
        state.memory_store.as_ref(),
        state.graph_store.as_ref(),
        llm.as_ref(),
    ))
    .map_err(|e| e.to_string())
}
