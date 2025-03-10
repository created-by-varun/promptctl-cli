use crate::{
    error::Result,
    providers::{PromptResponse, Provider, WebClaudeClient},
    ui,
};

pub async fn generate_prompt(
    client: &WebClaudeClient,
    input: &str,
    refine: bool,
    spinner: &mut ui::LoadingSpinner,
) -> Result<PromptResponse> {
    if refine {
        // Get refinement questions from web API
        let questions = client.get_refinement_questions(input).await?;

        // Stop spinner before asking questions
        spinner.stop_with_message("");

        // Ask each question and collect answers
        let mut questions_and_answers = Vec::new();
        for question in questions {
            if !question.trim().is_empty() {
                let answer = ui::get_input(&question)?;
                questions_and_answers.push((question, answer));
            }
        }

        // Restart spinner for final prompt generation
        *spinner = ui::LoadingSpinner::new("Improving your prompt with Claude...");

        // Generate final prompt with refinements
        client
            .generate_refined_prompt(input, questions_and_answers)
            .await
    } else {
        // Generate improved prompt without refinement
        client.generate_prompt(input).await
    }
}
