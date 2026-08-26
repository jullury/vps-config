use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::{PackageManager, detect::Distro};
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct DockerModule<'a> {
    distro: &'a Distro,
}

impl<'a> DockerModule<'a> {
    pub fn new(distro: &'a Distro) -> Self {
        Self { distro }
    }
}

#[async_trait(?Send)]
impl<'a> Module for DockerModule<'a> {
    fn name(&self) -> &str { "docker" }

    async fn apply(&self, executor: &mut Executor<'_>, pkg: &dyn PackageManager) -> Result<()> {
        if pkg.is_installed(executor, "docker").await? {
            println!("  {} Docker already installed", "✓".green());
            return Ok(());
        }

        match self.distro {
            Distro::Debian | Distro::Ubuntu => {
                pkg.install(executor, &["ca-certificates", "curl", "gnupg"]).await?;
                executor.run("install -m 0755 -d /etc/apt/keyrings").await?;
                executor.run("curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg").await?;
                executor.run("chmod a+r /etc/apt/keyrings/docker.gpg").await?;
                executor.run("echo \"deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable\" > /etc/apt/sources.list.d/docker.list").await?;
                pkg.update(executor).await?;
                pkg.install(executor, &["docker-ce", "docker-ce-cli", "containerd.io", "docker-compose-plugin"]).await?;
            }
            Distro::RHEL | Distro::Fedora => {
                pkg.install(executor, &["dnf-plugins-core"]).await?;
                executor.run("dnf config-manager --add-repo https://download.docker.com/linux/rhel/docker-ce.repo").await?;
                pkg.install(executor, &["docker-ce", "docker-ce-cli", "containerd.io", "docker-compose-plugin"]).await?;
            }
            _ => anyhow::bail!("Docker not supported on this distro"),
        }

        pkg.enable_service(executor, "docker").await?;
        pkg.start_service(executor, "docker").await?;

        println!("  {} Docker installed", "✓".green());
        Ok(())
    }
}
