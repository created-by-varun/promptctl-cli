pub mod cli;
pub mod error;
pub mod prompt;
pub mod providers;
pub mod ui;
pub mod utils;

// Re-export commonly used items
pub use error::{Error, Result};
pub use providers::WebClaudeClient;
