use clap::Parser;
use colored::Colorize;
use promptctl::{
    cli::{Cli, Commands},
    error::Result,
    providers::WebClaudeClient,
    ui::{self, LoadingSpinner, PromptDisplay},
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(args) => {
            let prompt_text = ui::get_input("Enter your prompt")?;

            let client = WebClaudeClient::new();

            let mut spinner = LoadingSpinner::new(if args.refine {
                "Getting follow-up questions..."
            } else {
                "Improving your prompt..."
            });
            let response = promptctl::prompt::generate_prompt(
                &client,
                &prompt_text,
                args.refine,
                &mut spinner,
            )
            .await?;
            spinner.stop_with_message("");

            // Display improved prompt in a box
            let display = PromptDisplay::new(response.content.clone(), response.model.clone());
            display.display();

            // Copy to clipboard
            promptctl::utils::copy_to_clipboard(&response.content)?;
            println!("{}", "✓ Prompt copied to clipboard\n".green());

            let choices = &["Open with Claude", "Open with ChatGPT", "Exit"];
            match ui::prompt_for_action(choices)? {
                0 => {
                    let mut spinner = LoadingSpinner::new("Opening in Claude...");
                    let url = format!(
                        "https://claude.ai/new?q={}",
                        urlencoding::encode(&response.content)
                    );
                    promptctl::utils::open_url(&url)?;
                    spinner.stop_with_message("Opened in Claude")
                }
                1 => {
                    let mut spinner = LoadingSpinner::new("Opening in ChatGPT...");
                    let url = format!(
                        "https://chatgpt.com/?prompt={}",
                        urlencoding::encode(&response.content)
                    );
                    promptctl::utils::open_url(&url)?;
                    spinner.stop_with_message("Opened in ChatGPT")
                }
                _ => {}
            }
        }
    }

    Ok(())
}
