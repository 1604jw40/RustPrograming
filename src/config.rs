use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("toml parse error: {0}")]
    Toml(#[from] toml::de::Error),
}

#[derive(Debug, Clone, Deserialize)]
pub struct PipelineConfig {
    pub input_dir: PathBuf,
    pub file_pattern: String,
    pub output_parquet: PathBuf,
    pub chunk_size: usize,
}

impl PipelineConfig {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let raw = fs::read_to_string(path)?;
        let mut config: PipelineConfig = toml::from_str(&raw)?;
        if config.chunk_size == 0 {
            config.chunk_size = 1;
        }
        Ok(config)
    }
}
