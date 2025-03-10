use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

mod paths;
pub use paths::*;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub claude_api_key: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;
        if !config_path.exists() {
            return Ok(Config::default());
        }

        let config = config::Config::builder()
            .add_source(config::File::from(config_path))
            .build()?;

        Ok(config.try_deserialize()?)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path()?;
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, contents)?;
        Ok(())
    }

    pub fn get_claude_api_key(&self) -> Result<String> {
        self.claude_api_key.clone().ok_or_else(|| {
            Error::MissingApiKey(
                "Claude API key not found. Use 'promptctl config --claude-key <key>' to set it."
                    .into(),
            )
        })
    }
}
