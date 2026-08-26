use anyhow::{Context, Result};
use std::path::Path;
use super::schema::Config;

pub fn load_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;
    let config: Config = toml::from_str(&content)
        .with_context(|| "Failed to parse config file")?;
    Ok(config)
}
