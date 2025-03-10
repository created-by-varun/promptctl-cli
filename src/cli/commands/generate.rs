use clap::Args;

/// Command to generate improved prompts using Claude AI
#[derive(Args)]
#[command(
    about = "Generate an improved prompt using Claude AI. You'll be guided through an interactive process to create and refine your prompt."
)]
pub struct GenerateArgs {
    /// Enable interactive refinement with follow-up questions
    #[arg(
        short = 'r',
        long,
        help = "Interactively refine the prompt with follow-up questions"
    )]
    pub refine: bool,
}
