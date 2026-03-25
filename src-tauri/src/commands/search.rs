use tauri::State;

use crate::app_state::AppState;

#[derive(serde::Serialize)]
pub struct SearchResult {
    pub chunk_id: String,
    pub document_id: String,
    pub text: String,
    pub score: f64,
    pub timestamp: String,
    pub source_platform: String,
}

#[tauri::command]
pub fn keyword_search(
    query: String,
    top_k: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<SearchResult>, String> {
    let k = top_k.unwrap_or(10);
    let fts_results = state
        .page_index
        .search(&query, k)
        .map_err(|e| e.to_string())?;

    let chunk_ids: Vec<String> = fts_results.iter().map(|r| r.chunk_id.clone()).collect();
    let chunks = state
        .document_store
        .get_chunks_by_ids(&chunk_ids)
        .map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for fts_result in &fts_results {
        if let Some(chunk) = chunks.iter().find(|c| c.id == fts_result.chunk_id) {
            let (timestamp, platform) = match state.document_store.get_by_id(&chunk.document_id) {
                Ok(Some(doc)) => (doc.timestamp.to_rfc3339(), doc.source_platform.to_string()),
                _ => (String::new(), String::new()),
            };

            results.push(SearchResult {
                chunk_id: chunk.id.clone(),
                document_id: chunk.document_id.clone(),
                text: fts_result.snippet.clone(),
                score: fts_result.rank_score,
                timestamp,
                source_platform: platform,
            });
        }
    }

    Ok(results)
}

#[tauri::command]
pub fn get_document_text(
    document_id: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let doc = state
        .document_store
        .get_by_id(&document_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Document not found".to_string())?;
    Ok(doc.raw_text)
}
