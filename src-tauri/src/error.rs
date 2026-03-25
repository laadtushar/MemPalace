use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("LLM provider error: {0}")]
    LlmProvider(String),

    #[error("Import error: {0}")]
    Import(String),

    #[error("Analysis error: {0}")]
    Analysis(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("{0}")]
    Other(String),
}

// Convert AppError to String for Tauri command returns
impl From<AppError> for String {
    fn from(err: AppError) -> String {
        err.to_string()
    }
}
