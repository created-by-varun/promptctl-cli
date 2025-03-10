use crate::{
    error::Result,
    providers::{ClaudeClient, PromptResponse, Provider},
    ui,
};

pub async fn generate_prompt(
    client: &ClaudeClient,
    input: &str,
    refine: bool,
    spinner: &mut ui::LoadingSpinner,
) -> Result<PromptResponse> {
    if refine {
        // Get refinement questions from Claude
        let questions_prompt = format!(
            "You are helping improve a user's prompt. You will ask TWO short questions to make the prompt more effective.\n\nGuidelines for questions:\n1. Focus on technical requirements, context, or specific details\n2. Keep questions concise and direct\n3. Questions should help clarify the user's intent\n\nOriginal prompt: {}\n\nFormat your response as exactly two questions, each on a new line, no other text or explanations.",
            input
        );

        let questions = client.generate_prompt(&questions_prompt).await?;

        // Stop spinner before asking questions
        spinner.stop_with_message("");

        // Ask each question and collect answers
        let mut refined_context = String::new();
        for question in questions.content.lines() {
            if !question.trim().is_empty() {
                let answer = ui::get_input(question)?;
                refined_context.push_str(&format!("\n- {}: {}", question, answer));
            }
        }

        // Restart spinner for final prompt generation
        *spinner = ui::LoadingSpinner::new("Improving your prompt with Claude...");

        // Generate final prompt with refinements
        let final_prompt = format!(
            "You are a prompt improvement expert. Enhance this prompt to be more specific, clear, and effective.\n\nOriginal prompt: {}\n\nAdditional context from user:{}\n\nGuidelines for improvement:\n1. Incorporate the user's clarifications naturally\n2. Add relevant technical details and requirements\n3. Structure the prompt logically\n4. Keep the same intent but make it more comprehensive\n\nProvide only the improved prompt, no explanations.",
            input, refined_context
        );

        client.generate_prompt(&final_prompt).await
    } else {
        // Generate improved prompt without refinement
        let system_prompt = format!(
            "You are a prompt improvement assistant. Your task is to enhance the following prompt to be more specific, clear, and effective. Keep the same general intent but make it more comprehensive and well-structured.\n\nPrompt to improve:\n{}\n\nProvide only the improved prompt, no explanations.",
            input
        );

        client.generate_prompt(&system_prompt).await
    }
}
