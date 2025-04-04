use clap::Parser;

mod commands;
pub use commands::*;

#[derive(Parser)]
#[command(
    name = "pctl",
    about = "A CLI tool for generating and improving AI prompts",
    version,
    author
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Generate an improved prompt
    #[command(alias = "g")]
    Generate(commands::generate::GenerateArgs),
}
