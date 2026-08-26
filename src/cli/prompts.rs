use anyhow::Result;
use dialoguer::{Input, Select, Confirm};
use console::style;
use crate::config::schema::*;

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
            .default("~/.ssh/id_rsa".to_string())
            .interact_text()?)
    } else {
        None
    };

    let password = if auth == "password" {
        Some(Input::new()
            .with_prompt("SSH password")
            .interact_text()?)
    } else {
        None
    };

    println!("\n{}", style("--- Security ---").yellow().bold());

    let create_user: String = Input::new()
        .with_prompt("Create non-root user (empty to skip)")
        .default("deploy".to_string())
        .interact_text()?;

    let ssh_password_auth = Confirm::new()
        .with_prompt("Allow SSH password authentication?")
        .default(false)
        .interact()?;

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
            password,
        },
        security: SecurityConfig {
            create_user: if create_user.is_empty() { None } else { Some(create_user) },
            ssh_password_auth,
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
