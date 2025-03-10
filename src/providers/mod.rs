mod claude;
pub use claude::*;

use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptResponse {
    pub content: String,
    pub model: String,
}

pub trait Provider {
    fn generate_prompt(
        &self,
        input: &str,
    ) -> impl std::future::Future<Output = Result<PromptResponse>> + Send;
}
