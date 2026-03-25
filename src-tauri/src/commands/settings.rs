use tauri::State;

use crate::app_state::AppState;

#[derive(serde::Serialize)]
pub struct OllamaStatus {
    pub connected: bool,
    pub models: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct AppStats {
    pub total_documents: usize,
    pub total_memory_facts: usize,
    pub date_range: Option<(String, String)>,
}

#[tauri::command]
pub fn test_ollama_connection() -> Result<OllamaStatus, String> {
    // Run a quick synchronous HTTP check
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .map_err(|e| e.to_string())?;

    match client.get("http://localhost:11434/api/tags").send() {
        Ok(resp) if resp.status().is_success() => {
            let models: Vec<String> = resp
                .json::<serde_json::Value>()
                .ok()
                .and_then(|v| v.get("models")?.as_array().cloned())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|m| m.get("name")?.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();

            Ok(OllamaStatus {
                connected: true,
                models,
            })
        }
        _ => Ok(OllamaStatus {
            connected: false,
            models: vec![],
        }),
    }
}

#[tauri::command]
pub fn get_app_stats(
    state: State<'_, AppState>,
) -> Result<AppStats, String> {
    let months = state
        .timeline_store
        .get_document_count_by_month()
        .map_err(|e| e.to_string())?;

    let total_documents: usize = months.iter().map(|(_, c)| c).sum();

    let date_range = state
        .timeline_store
        .get_date_range()
        .map_err(|e| e.to_string())?
        .map(|r| (r.start.to_rfc3339(), r.end.to_rfc3339()));

    let total_facts = state
        .memory_store
        .get_all(None, None)
        .map_err(|e| e.to_string())?
        .len();

    Ok(AppStats {
        total_documents,
        total_memory_facts: total_facts,
        date_range,
    })
}
