use clap::Args;

/// Command to manage API keys and settings
#[derive(Args)]
#[command(
    about = "Configure API keys and settings for the CLI. Use this to set up your Claude API key."
)]
pub struct ConfigArgs {
    /// Set your Claude API key (required for prompt generation)
    #[arg(
        short = 'k',
        long,
        help = "Set your Claude API key. You can get this from https://claude.ai"
    )]
    pub claude_key: Option<String>,

    /// View current configuration including API keys
    #[arg(short, long, help = "Display current configuration settings")]
    pub view: bool,
}
