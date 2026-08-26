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
    fn name(&self) -> &str { "postgres" }

    async fn apply(&self, executor: &mut Executor<'_>, pkg: &dyn PackageManager) -> Result<()> {
        if pkg.is_installed(executor, "postgresql").await? {
            println!("  {} PostgreSQL already installed", "✓".green());
            return Ok(());
        }

        match self.distro {
            Distro::Debian | Distro::Ubuntu => {
                pkg.install(executor, &["postgresql", "postgresql-contrib"]).await?;
            }
            Distro::RHEL | Distro::Fedora => {
                pkg.install(executor, &["postgresql-server", "postgresql-contrib"]).await?;
                executor.run("postgresql-setup --initdb").await?;
            }
            _ => anyhow::bail!("PostgreSQL not supported on this distro"),
        }

        pkg.enable_service(executor, "postgresql").await?;
        pkg.start_service(executor, "postgresql").await?;

        println!("  {} PostgreSQL installed", "✓".green());
        Ok(())
    }
}
