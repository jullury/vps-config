use clap::Parser;
use std::path::{Path, PathBuf};

use cli::args::Args;
use config::loader::load_config;

mod cli;
mod config;
mod ssh;
mod os;
mod modules;

fn default_config_path() -> PathBuf {
    let local = PathBuf::from("config.toml");
    if local.exists() {
        return local;
    }
    dirs::config_dir()
        .map(|d| d.join("vps-config/config.toml"))
        .filter(|p| p.exists())
        .unwrap_or(local)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = if let Some(ref path) = args.config {
        load_config(Path::new(path))?
    } else {
        load_config(&default_config_path())?
    };

    println!("vps-config - Remote VPS Provisioning Wizard");
    println!("Loaded config for: {}", config.vps.ip);

    Ok(())
}
