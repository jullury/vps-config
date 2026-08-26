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
        let xdg_config = dirs::config_dir()
            .map(|p| p.join("vps-config").join("config.toml"));
        let config_path = xdg_config
            .as_ref()
            .filter(|p| p.exists())
            .cloned()
            .or_else(|| {
                let cwd_path = Path::new("config.toml").to_path_buf();
                if cwd_path.exists() {
                    Some(cwd_path)
                } else {
                    None
                }
            });
        match config_path {
            Some(path) => config::loader::load_config(&path)?,
            None => cli::prompts::run_wizard()?,
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
        os::detect::Distro::RHEL | os::detect::Distro::Fedora => {
            Box::new(os::dnf::DnfManager::new())
        }
    };

    println!("{} Updating package lists...", "->".blue());
    pkg.update(&mut executor).await?;

    println!("{} Applying security configuration...", "->".blue());
    let ssh_hardened = modules::run_security_modules(&mut executor, &*pkg, &config, &distro).await?;

    if ssh_hardened {
        println!("\n{} Reconnecting after SSH hardening...", "->".blue());
        drop(executor);
        drop(session);
        let mut session = client.connect().await?;
        let mut executor = ssh::executor::Executor::new(&mut session);
        modules::run_services_and_devtools(&mut executor, &*pkg, &config, &distro).await?;
    } else {
        modules::run_services_and_devtools(&mut executor, &*pkg, &config, &distro).await?;
    }

    Ok(())
}
