use super::{PromptResponse, Provider};
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

const CLAUDE_API_URL: &str = "https://api.anthropic.com/v1/messages";
const MODEL: &str = "claude-3-5-haiku-20241022";

#[derive(Debug)]
pub struct ClaudeClient {
    api_key: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    content: Vec<Content>,
}

#[derive(Debug, Deserialize)]
struct Content {
    text: String,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::new();
        Self { api_key, client }
    }

    fn create_system_prompt(input: &str) -> String {
        format!(
            "You are an expert at crafting effective prompts for Claude AI. Your task is to enhance this prompt to be more effective and generate better responses from Claude. Here's the input prompt:\n\n{}\n\nProvide ONLY the improved prompt, considering:\n\n1. Clarity and specificity\n2. Proper context and constraints\n3. Step-by-step breakdown for complex tasks\n4. Appropriate formatting (plain text format only)\n5. Clear expected outputs only in text format. Do not respond in markdown\n\nDo not include any explanations or text before/after the improved prompt.",
            input
        )
    }
}

impl Provider for ClaudeClient {
    fn generate_prompt(
        &self,
        input: &str,
    ) -> impl std::future::Future<Output = Result<PromptResponse>> + Send {
        let api_key = self.api_key.clone();
        let request = ClaudeRequest {
            model: MODEL.to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: format!("{} {}", Self::create_system_prompt(input), input),
            }],
            max_tokens: 1024,
        };
        let client = self.client.clone();

        async move {
            let response = client
                .post(CLAUDE_API_URL)
                .header("x-api-key", &api_key)
                .header("anthropic-version", "2023-06-01")
                .header("content-type", "application/json")
                .json(&request)
                .send()
                .await?;

            if !response.status().is_success() {
                let error = response.text().await?;
                return Err(Error::Claude(error));
            }

            let claude_response: ClaudeResponse = response.json().await?;
            let content = claude_response
                .content
                .first()
                .ok_or_else(|| Error::Claude("No content in response".to_string()))?;

            Ok(PromptResponse {
                content: content.text.clone(),
                model: MODEL.to_string(),
            })
        }
    }
}
