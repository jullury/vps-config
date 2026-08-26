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
    async fn apply(&self, executor: &mut Executor<'_>, pkg: &dyn PackageManager) -> Result<()> {
        if pkg.is_installed(executor, "docker").await? {
            println!("  {} Docker already installed", "✓".green());
            return Ok(());
        }

        match self.distro {
            Distro::Debian | Distro::Ubuntu => {
                let repo_url = match self.distro {
                    Distro::Ubuntu => "https://download.docker.com/linux/ubuntu",
                    Distro::Debian => "https://download.docker.com/linux/debian",
                    _ => unreachable!(),
                };
                pkg.install(executor, &["ca-certificates", "curl", "gnupg"]).await?;
                executor.run("sudo install -m 0755 -d /etc/apt/keyrings").await?;
                executor.run(&format!("curl -fsSL {repo_url}/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg")).await?;
                executor.run("sudo chmod a+r /etc/apt/keyrings/docker.gpg").await?;
                executor.run(&format!("echo \"deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] {repo_url} $(lsb_release -cs) stable\" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null")).await?;
                pkg.update(executor).await?;
                pkg.install(executor, &["docker-ce", "docker-ce-cli", "containerd.io", "docker-compose-plugin"]).await?;
            }
            Distro::RHEL | Distro::Fedora => {
                pkg.install(executor, &["dnf-plugins-core"]).await?;
                executor.run("sudo dnf config-manager --add-repo https://download.docker.com/linux/rhel/docker-ce.repo").await?;
                pkg.install(executor, &["docker-ce", "docker-ce-cli", "containerd.io", "docker-compose-plugin"]).await?;
            }
        }

        pkg.enable_service(executor, "docker").await?;
        pkg.start_service(executor, "docker").await?;

        println!("  {} Docker installed", "✓".green());
        Ok(())
    }
}
