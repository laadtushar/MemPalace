use crate::error::AppError;
use async_trait::async_trait;

/// Status of an analysis run
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnalysisStatus {
    pub stage: String,
    pub progress: f64,
    pub message: String,
    pub is_complete: bool,
    pub is_cancelled: bool,
}

/// Port for pluggable analysis stages
#[async_trait]
pub trait IAnalysisStage: Send + Sync {
    fn name(&self) -> &str;
    async fn execute(&self, on_progress: &dyn Fn(f64, &str)) -> Result<(), AppError>;
}
