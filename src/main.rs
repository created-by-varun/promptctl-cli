use clap::Parser;
use colored::Colorize;
use promptctl::{
    cli::{Cli, Commands},
    config::Config,
    error::Result,
    providers::ClaudeClient,
    ui::{self, LoadingSpinner, PromptDisplay},
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut config = Config::load()?;

    match cli.command {
        Commands::Generate(args) => {
            let prompt_text = ui::get_input("Enter your prompt")?
;

            let api_key = config.get_claude_api_key()?;
            let client = ClaudeClient::new(api_key);

            let mut spinner = LoadingSpinner::new(if args.refine {
                "Getting follow-up questions from Claude..."
            } else {
                "Improving your prompt with Claude..."
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
        Commands::Config(args) => {
            if let Some(key) = args.claude_key {
                config.claude_api_key = Some(key);
                config.save()?;
                println!("{}", "✓ Claude API key saved".green());
            } else if args.view {
                println!("{}", "Current configuration:".blue());
                println!(
                    "{} {}",
                    "Claude API key:".dimmed(),
                    config.claude_api_key.as_deref().unwrap_or("[not set]")
                );
            } else {
                println!("{}", "No configuration changes specified".yellow());
                println!(
                    "{}",
                    "Use --claude-key to set API key or --view to view current config".dimmed()
                );
            }
        }
    }

    Ok(())
}
