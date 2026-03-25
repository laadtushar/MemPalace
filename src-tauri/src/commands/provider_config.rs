use tauri::State;

use crate::adapters::llm::claude::ClaudeProvider;
use crate::adapters::llm::ollama::OllamaProvider;
use crate::app_state::AppState;

/// Persisted LLM configuration.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LlmConfig {
    pub active_provider: String,      // "ollama" or "claude"
    pub ollama_url: String,
    pub ollama_model: String,
    pub embedding_model: String,
    pub claude_api_key: Option<String>, // masked in response
    pub claude_model: String,
}

/// Get the current LLM configuration.
#[tauri::command]
pub fn get_llm_config(state: State<'_, AppState>) -> Result<LlmConfig, String> {
    let cs = &state.config_store;

    let active = cs.get("llm.active_provider").ok().flatten().unwrap_or_else(|| "ollama".into());
    let ollama_url = cs.get("llm.ollama_url").ok().flatten().unwrap_or_else(|| "http://localhost:11434".into());
    let ollama_model = cs.get("llm.model").ok().flatten().unwrap_or_else(|| "llama3.1:8b".into());
    let embed_model = cs.get("llm.embedding_model").ok().flatten().unwrap_or_else(|| "nomic-embed-text".into());
    let claude_key = cs.get("llm.claude_api_key").ok().flatten();
    let claude_model = cs.get("llm.claude_model").ok().flatten().unwrap_or_else(|| "claude-sonnet-4-20250514".into());

    Ok(LlmConfig {
        active_provider: active,
        ollama_url,
        ollama_model,
        embedding_model: embed_model,
        claude_api_key: claude_key.map(|k| mask_api_key(&k)),
        claude_model,
    })
}

/// Save LLM configuration and switch the active provider at runtime.
#[tauri::command]
pub fn save_llm_config(
    config: LlmConfig,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let cs = &state.config_store;

    cs.set("llm.active_provider", &config.active_provider).map_err(|e| e.to_string())?;
    cs.set("llm.ollama_url", &config.ollama_url).map_err(|e| e.to_string())?;
    cs.set("llm.model", &config.ollama_model).map_err(|e| e.to_string())?;
    cs.set("llm.embedding_model", &config.embedding_model).map_err(|e| e.to_string())?;
    cs.set("llm.claude_model", &config.claude_model).map_err(|e| e.to_string())?;

    // Only update API key if a non-masked value is provided
    if let Some(ref key) = config.claude_api_key {
        if !key.contains('*') && !key.is_empty() {
            cs.set("llm.claude_api_key", key).map_err(|e| e.to_string())?;
        }
    }

    // Swap the active LLM provider at runtime
    let new_provider: Box<dyn crate::domain::ports::llm_provider::ILlmProvider> =
        if config.active_provider == "claude" {
            let real_key = cs
                .get("llm.claude_api_key")
                .ok()
                .flatten()
                .ok_or("Claude API key not set")?;
            Box::new(ClaudeProvider::new(&real_key, &config.claude_model))
        } else {
            Box::new(OllamaProvider::new(
                &config.ollama_url,
                &config.ollama_model,
                &config.embedding_model,
            ))
        };

    // Swap the provider behind the RwLock
    let mut provider = state
        .llm_provider
        .write()
        .map_err(|e| format!("Lock error: {}", e))?;
    *provider = new_provider;

    // Also update embedding provider with new Ollama URL/model
    let new_embed: Box<dyn crate::domain::ports::embedding_provider::IEmbeddingProvider> =
        Box::new(OllamaProvider::new(
            &config.ollama_url,
            &config.ollama_model,
            &config.embedding_model,
        ));
    let mut embed = state
        .embedding_provider
        .write()
        .map_err(|e| format!("Lock error: {}", e))?;
    *embed = new_embed;

    Ok(())
}

fn mask_api_key(key: &str) -> String {
    if key.len() <= 8 {
        "*".repeat(key.len())
    } else {
        format!("{}...{}", &key[..4], &key[key.len() - 4..])
    }
}
