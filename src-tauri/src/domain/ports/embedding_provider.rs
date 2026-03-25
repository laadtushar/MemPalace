use crate::error::AppError;
use async_trait::async_trait;

/// Port for text-to-vector encoding
#[async_trait]
pub trait IEmbeddingProvider: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, AppError>;
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, AppError>;
    fn dimensions(&self) -> usize;
    fn model_name(&self) -> &str;
}
