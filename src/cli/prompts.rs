use anyhow::Result;
use dialoguer::{Input, Select, Confirm, Password};
use console::style;
use colored::Colorize;
use russh::keys::load_secret_key;
use crate::config::schema::*;

fn detect_local_keys() -> Vec<String> {
    let mut keys = Vec::new();
    if let Ok(home) = std::env::var("HOME") {
        let ssh_dir = std::path::PathBuf::from(&home).join(".ssh");
        if let Ok(entries) = std::fs::read_dir(&ssh_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.ends_with(".pub") {
                    if let Some(stem) = name_str.strip_suffix(".pub") {
                        // Prefer ed25519 > ecdsa > rsa
                        if stem == "id_ed25519" || stem == "id_ecdsa" || stem == "id_rsa" || stem.starts_with("id_") {
                            keys.push(entry.path().to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }
    // Sort: ed25519 first, then ecdsa, then rsa, then others
    keys.sort_by(|a, b| {
        let priority = |p: &str| -> u8 {
            if p.contains("ed25519") { 0 }
            else if p.contains("ecdsa") { 1 }
            else if p.contains("rsa") { 2 }
            else { 3 }
        };
        priority(a).cmp(&priority(b))
    });
    keys
}

pub fn run_wizard() -> Result<Config> {
    println!("\n{}", style("=== VPS Config Wizard ===").bold().cyan());

    let ip: String = Input::new()
        .with_prompt("VPS IP address")
        .interact_text()?;

    let port: u16 = Input::new()
        .with_prompt("SSH port")
        .default(22)
        .interact_text()?;

    let user: String = Input::new()
        .with_prompt("SSH user")
        .default("root".to_string())
        .interact_text()?;

    let auth_options = vec!["Password", "SSH Key"];
    let auth_idx = Select::new()
        .with_prompt("Auth method")
        .items(&auth_options)
        .default(0)
        .interact()?;
    let auth = if auth_idx == 0 { "password" } else { "key" }.to_string();

    let key_path = if auth == "key" {
        Some(Input::new()
            .with_prompt("SSH key path")
            .default("~/.ssh/id_ed25519".to_string())
            .interact_text()?)
    } else {
        None
    };

    let key_passphrase = if auth == "key" {
        if let Some(ref path) = key_path {
            let expanded = shellexpand::tilde(path);
            // Try loading without passphrase first to detect if encrypted
            match load_secret_key(expanded.as_ref(), None) {
                Ok(_) => None, // Key is not encrypted
                Err(e) => {
                    let err_str = e.to_string();
                    if err_str.contains("encrypted") || err_str.contains("Encrypted") || err_str.contains("decrypt") {
                        Some(Password::new()
                            .with_prompt("Key passphrase (key is encrypted)")
                            .interact()?)
                    } else {
                        // Some other error, let it fail later with a clear message
                        None
                    }
                }
            }
        } else {
            None
        }
    } else {
        None
    };

    let password = if auth == "password" {
        Some(Password::new()
            .with_prompt("SSH password")
            .interact()?)
    } else {
        None
    };

    println!("\n{}", style("--- Security ---").yellow().bold());

    let create_user: String = Input::new()
        .with_prompt("Create non-root user (empty to skip)")
        .allow_empty(true)
        .interact_text()?;

    let user_password = if !create_user.is_empty() {
        Some(Password::new()
            .with_prompt(&format!("Password for '{}'", create_user))
            .interact()?)
    } else {
        None
    };

    let ssh_password_auth = Confirm::new()
        .with_prompt("Allow SSH password authentication?")
        .default(false)
        .interact()?;

    let ssh_allow_root_login = if create_user.is_empty() {
        println!("  {} Root login must stay enabled (no non-root user created)", "⚠".yellow());
        true
    } else {
        Confirm::new()
            .with_prompt("Allow SSH root login?")
            .default(false)
            .interact()?
    };

    let ssh_public_key_path = if !ssh_password_auth {
        let keys = detect_local_keys();
        if keys.is_empty() {
            println!("  {} No SSH public keys found in ~/.ssh/", "⚠".yellow());
            println!("  You'll need to add one manually before re-running with SSH hardening.");
            None
        } else {
            let mut items: Vec<String> = keys.iter().map(|k| {
                let path = std::path::Path::new(k);
                let stem = path.file_stem().unwrap().to_string_lossy();
                let content = std::fs::read_to_string(k).unwrap_or_default();
                let truncated = content.trim().chars().take(50).collect::<String>();
                format!("{stem}  ({truncated}...)")
            }).collect();
            items.push("Enter path manually".to_string());

            let idx = Select::new()
                .with_prompt("Select public key to copy to VPS")
                .items(&items)
                .default(0)
                .interact()?;

            if idx < keys.len() {
                Some(keys[idx].clone())
            } else {
                Some(Input::new()
                    .with_prompt("Public key path")
                    .interact_text()?)
            }
        }
    } else {
        None
    };

    let firewall = Confirm::new()
        .with_prompt("Enable firewall (ufw/firewalld)?")
        .default(true)
        .interact()?;

    let fail2ban = Confirm::new()
        .with_prompt("Install fail2ban?")
        .default(true)
        .interact()?;

    println!("\n{}", style("--- Services ---").green().bold());

    let docker = Confirm::new()
        .with_prompt("Install Docker?")
        .default(true)
        .interact()?;

    let nginx = Confirm::new()
        .with_prompt("Install nginx?")
        .default(true)
        .interact()?;

    let postgres = Confirm::new()
        .with_prompt("Install PostgreSQL?")
        .default(true)
        .interact()?;

    let redis = Confirm::new()
        .with_prompt("Install Redis?")
        .default(false)
        .interact()?;

    println!("\n{}", style("--- Dev Tools ---").blue().bold());

    let node = Confirm::new()
        .with_prompt("Install Node.js?")
        .default(true)
        .interact()?;

    let node_version = if node {
        Some(Input::new()
            .with_prompt("Node.js version")
            .default("22".to_string())
            .interact_text()?)
    } else {
        None
    };

    let python = Confirm::new()
        .with_prompt("Install Python?")
        .default(true)
        .interact()?;

    let python_version = if python {
        Some(Input::new()
            .with_prompt("Python version")
            .default("3.12".to_string())
            .interact_text()?)
    } else {
        None
    };

    let go = Confirm::new()
        .with_prompt("Install Go?")
        .default(true)
        .interact()?;

    let go_version = if go {
        Some(Input::new()
            .with_prompt("Go version")
            .default("1.22.0".to_string())
            .interact_text()?)
    } else {
        None
    };

    Ok(Config {
        vps: VpsConfig {
            ip,
            port,
            user,
            auth,
            key_path,
            key_passphrase,
            password,
        },
        security: SecurityConfig {
            create_user: if create_user.is_empty() { None } else { Some(create_user) },
            user_password,
            ssh_password_auth,
            ssh_allow_root_login,
            ssh_public_key_path,
            firewall,
            fail2ban,
        },
        services: ServicesConfig {
            docker,
            nginx,
            postgres,
            redis,
        },
        devtools: DevtoolsConfig {
            node,
            node_version,
            python,
            python_version,
            go,
            go_version,
        },
    })
}
