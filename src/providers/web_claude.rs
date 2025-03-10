use crate::error::Result;
use crate::providers::{PromptResponse, Provider};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const API_URL: &str = "http://localhost:3000/api";

#[derive(Debug, Serialize)]
struct GenerateRequest {
    prompt: String,
    refine: bool,
}

#[derive(Debug, Deserialize)]
struct RefinementQuestionsResponse {
    questions: Vec<String>,
}

#[derive(Debug, Serialize)]
struct RefineRequest {
    prompt: String,
    answers: Vec<QuestionAnswer>,
}

#[derive(Debug, Serialize)]
struct QuestionAnswer {
    question: String,
    answer: String,
}

pub struct WebClaudeClient {
    client: Client,
}

impl WebClaudeClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_refinement_questions(&self, prompt: &str) -> Result<Vec<String>> {
        let request = GenerateRequest {
            prompt: prompt.to_string(),
            refine: true,
        };

        let response = self
            .client
            .post(&format!("{}/generate", API_URL))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(crate::error::Error::Claude(format!(
                "API error: {}",
                response.status()
            )));
        }

        let questions: RefinementQuestionsResponse = response.json().await?;
        Ok(questions.questions)
    }

    pub async fn generate_refined_prompt(
        &self,
        prompt: &str,
        questions_and_answers: Vec<(String, String)>,
    ) -> Result<PromptResponse> {
        let answers = questions_and_answers
            .into_iter()
            .map(|(question, answer)| QuestionAnswer { question, answer })
            .collect();

        let request = RefineRequest {
            prompt: prompt.to_string(),
            answers,
        };

        let response = self
            .client
            .post(&format!("{}/generate/refine", API_URL))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(crate::error::Error::Claude(format!(
                "API error: {}",
                response.status()
            )));
        }

        response.json().await.map_err(Into::into)
    }
}

#[async_trait::async_trait]
impl Provider for WebClaudeClient {
    async fn generate_prompt(&self, input: &str) -> Result<PromptResponse> {
        let request = GenerateRequest {
            prompt: input.to_string(),
            refine: false,
        };

        let response = self
            .client
            .post(&format!("{}/generate", API_URL))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(crate::error::Error::Claude(format!(
                "API error: {}",
                response.status()
            )));
        }

        response.json().await.map_err(Into::into)
    }
}
