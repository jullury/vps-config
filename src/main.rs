use clap::Parser;
use std::path::Path;

use cli::args::Args;
use config::loader::load_config;

mod cli;
mod config;
mod ssh;
mod os;
mod modules;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = if let Some(ref path) = args.config {
        load_config(Path::new(path))?
    } else {
        load_config(Path::new("config.toml"))?
    };

    println!("vps-config - Remote VPS Provisioning Wizard");
    println!("Loaded config for: {}", config.vps.ip);

    Ok(())
}
