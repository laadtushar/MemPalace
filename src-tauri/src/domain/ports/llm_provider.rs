use crate::error::AppError;
use async_trait::async_trait;

/// Parameters for LLM completion
#[derive(Debug, Clone)]
pub struct LlmParams {
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<usize>,
    pub system_prompt: Option<String>,
}

impl Default for LlmParams {
    fn default() -> Self {
        Self {
            model: None,
            temperature: Some(0.7),
            max_tokens: Some(4096),
            system_prompt: None,
        }
    }
}

/// Classification result from the LLM
#[derive(Debug, Clone)]
pub struct Classification {
    pub category: String,
    pub confidence: f64,
}

/// Port for LLM text generation, summarization, and analysis
#[async_trait]
pub trait ILlmProvider: Send + Sync {
    async fn complete(&self, prompt: &str, params: &LlmParams) -> Result<String, AppError>;
    async fn classify(
        &self,
        text: &str,
        categories: &[String],
        params: &LlmParams,
    ) -> Result<Classification, AppError>;
    async fn is_available(&self) -> bool;
    fn provider_name(&self) -> &str;
}
