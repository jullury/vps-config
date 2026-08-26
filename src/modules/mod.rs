pub mod security;
pub mod services;
pub mod devtools;

use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::config::schema::Config;
use crate::os::PackageManager;
use crate::os::detect::Distro;
use crate::ssh::executor::Executor;

#[async_trait(?Send)]
pub trait Module {
    async fn apply(&self, executor: &mut Executor<'_>, pkg: &dyn PackageManager) -> Result<()>;
}

pub async fn run_modules(
    executor: &mut Executor<'_>,
    pkg: &dyn PackageManager,
    config: &Config,
    distro: &Distro,
) -> Result<()> {
    println!("\n{}", "=== Applying Configuration ===".cyan().bold());

    if config.security.create_user.is_some() || config.security.firewall || config.security.fail2ban || !config.security.ssh_password_auth {
        println!("\n{}", "Security:".yellow().bold());

        if let Some(ref user) = config.security.create_user {
            security::users::UsersModule::new(user).apply(executor, pkg).await?;
        }

        if config.security.firewall {
            security::firewall::FirewallModule::new(distro).apply(executor, pkg).await?;
        }

        if config.security.fail2ban {
            security::fail2ban::Fail2BanModule::new(distro).apply(executor, pkg).await?;
        }

        // SSH hardening requires explicit opt-in: set ssh_password_auth = false in config.
        // The schema default is true (password auth allowed), so hardening only runs
        // when the user deliberately disables it.
        if !config.security.ssh_password_auth {
            security::ssh_harden::SshHardenModule::new(
                config.security.ssh_password_auth,
                None,
            ).apply(executor, pkg).await?;
        }
    }

    if config.services.docker || config.services.nginx || config.services.postgres || config.services.redis {
        println!("\n{}", "Services:".green().bold());

        if config.services.docker {
            services::docker::DockerModule::new(distro).apply(executor, pkg).await?;
        }
        if config.services.nginx {
            services::nginx::NginxModule.apply(executor, pkg).await?;
        }
        if config.services.postgres {
            services::postgres::PostgresModule::new(distro).apply(executor, pkg).await?;
        }
        if config.services.redis {
            services::redis::RedisModule::new(distro).apply(executor, pkg).await?;
        }
    }

    if config.devtools.node || config.devtools.python || config.devtools.go {
        println!("\n{}", "Dev Tools:".blue().bold());

        if config.devtools.node {
            let ver = config.devtools.node_version.as_deref().unwrap_or("22");
            devtools::node::NodeModule::new(ver).apply(executor, pkg).await?;
        }
        if config.devtools.python {
            let ver = config.devtools.python_version.as_deref().unwrap_or("3.12");
            devtools::python::PythonModule::new(ver).apply(executor, pkg).await?;
        }
        if config.devtools.go {
            let ver = config.devtools.go_version.as_deref().unwrap_or("1.22.0");
            devtools::go::GoModule::new(ver).apply(executor, pkg).await?;
        }
    }

    println!("\n{}", "=== Configuration Complete ===".cyan().bold());
    Ok(())
}
