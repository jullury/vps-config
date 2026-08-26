use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::{PackageManager, detect::Distro};
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct PostgresModule<'a> {
    distro: &'a Distro,
}

impl<'a> PostgresModule<'a> {
    pub fn new(distro: &'a Distro) -> Self {
        Self { distro }
    }
}

#[async_trait(?Send)]
impl<'a> Module for PostgresModule<'a> {
    async fn apply(&self, executor: &mut Executor<'_>, pkg: &dyn PackageManager) -> Result<()> {
        if pkg.is_installed(executor, "postgresql").await? {
            println!("  {} PostgreSQL already installed", "✓".green());
            return Ok(());
        }

        match self.distro {
            Distro::Debian | Distro::Ubuntu => {
                pkg.install(executor, &["postgresql", "postgresql-contrib"]).await?;
                pkg.enable_service(executor, "postgresql").await?;
                pkg.start_service(executor, "postgresql").await?;
            }
            Distro::RHEL | Distro::Fedora => {
                pkg.install(executor, &["postgresql-server", "postgresql-contrib"]).await?;
                executor.run("postgresql-setup --initdb").await?;
                let (svc_output, _) = executor.run_with_output(
                    "systemctl list-unit-files | grep '^postgresql' | head -1 | awk '{print $1}'"
                ).await?;
                let service = svc_output.trim();
                if service.is_empty() {
                    anyhow::bail!("Could not determine PostgreSQL service name on this system");
                }
                pkg.enable_service(executor, service).await?;
                pkg.start_service(executor, service).await?;
            }
        }

        println!("  {} PostgreSQL installed", "✓".green());
        Ok(())
    }
}
