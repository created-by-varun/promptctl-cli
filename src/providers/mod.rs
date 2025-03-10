mod web_claude;
pub use web_claude::*;

use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptResponse {
    pub content: String,
    pub model: String,
}

#[async_trait::async_trait]
pub trait Provider {
    async fn generate_prompt(&self, input: &str) -> Result<PromptResponse>;
}
