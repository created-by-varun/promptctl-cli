use std::path::PathBuf;
use crate::error::{Result, Error};

pub fn get_config_dir() -> Result<PathBuf> {
    dirs::config_dir()
        .map(|d| d.join("promptctl"))
        .ok_or_else(|| Error::Config("Could not determine config directory".into()))
}

pub fn get_config_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("config.json"))
}
