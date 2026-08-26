use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::{PackageManager, detect::Distro};
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct FirewallModule<'a> {
    distro: &'a Distro,
}

impl<'a> FirewallModule<'a> {
    pub fn new(distro: &'a Distro) -> Self {
        Self { distro }
    }
}

#[async_trait(?Send)]
impl<'a> Module for FirewallModule<'a> {
    async fn apply(&self, executor: &mut Executor<'_>, pkg: &dyn PackageManager) -> Result<()> {
        match self.distro {
            Distro::Debian | Distro::Ubuntu => {
                pkg.install(executor, &["ufw"]).await?;
                executor.run("ufw default deny incoming").await?;
                executor.run("ufw default allow outgoing").await?;
                executor.run("ufw allow ssh").await?;
                executor.run("ufw allow 80/tcp").await?;
                executor.run("ufw allow 443/tcp").await?;
                executor.run("ufw --force enable").await?;
            }
            Distro::RHEL | Distro::Fedora => {
                pkg.install(executor, &["firewalld"]).await?;
                pkg.enable_service(executor, "firewalld").await?;
                pkg.start_service(executor, "firewalld").await?;
                executor.run("firewall-cmd --permanent --add-service=ssh").await?;
                executor.run("firewall-cmd --permanent --add-service=http").await?;
                executor.run("firewall-cmd --permanent --add-service=https").await?;
                executor.run("firewall-cmd --reload").await?;
            }
        }
        println!("  {} Firewall configured", "✓".green());
        Ok(())
    }
}
