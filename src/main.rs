use clap::Parser;
use colored::Colorize;
use std::path::Path;

mod cli;
mod config;
mod ssh;
mod os;
mod modules;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::args::Args::parse();

    println!("{}", "=== VPS Config Wizard ===".cyan().bold());

    let config = if let Some(ref path) = args.config {
        config::loader::load_config(Path::new(path))?
    } else {
        let default_path = Path::new("config.toml");
        if default_path.exists() {
            config::loader::load_config(default_path)?
        } else {
            cli::prompts::run_wizard()?
        }
    };

    println!("\n{} Connecting to {}...", "->".blue(), config.vps.ip);
    let client = ssh::client::SshClient::new(&config.vps)?;
    let mut session = client.connect().await?;
    let mut executor = ssh::executor::Executor::new(&mut session);

    println!("{} Detecting OS...", "->".blue());
    let distro = os::detect::detect_distro(&mut executor).await?;
    println!("{} Detected: {:?}", "✓".green(), distro);

    let pkg: Box<dyn os::PackageManager> = match distro {
        os::detect::Distro::Debian | os::detect::Distro::Ubuntu => {
            Box::new(os::apt::AptManager::new())
        }
        os::detect::Distro::RHEL | os::detect::Distro::CentOS | os::detect::Distro::Fedora => {
            Box::new(os::dnf::DnfManager::new())
        }
    };

    println!("{} Updating package lists...", "->".blue());
    pkg.update(&mut executor).await?;

    modules::run_modules(&mut executor, &*pkg, &config, &distro).await?;

    Ok(())
}
