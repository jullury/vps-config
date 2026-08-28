use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::{PackageManager, detect::Distro};
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct RedisModule<'a> {
    distro: &'a Distro,
}

impl<'a> RedisModule<'a> {
    pub fn new(distro: &'a Distro) -> Self {
        Self { distro }
    }
}

#[async_trait(?Send)]
impl<'a> Module for RedisModule<'a> {
    async fn apply(&self, executor: &mut Executor<'_>, pkg: &dyn PackageManager) -> Result<()> {
        if pkg.is_installed(executor, "redis").await? {
            println!("  {} Redis already installed", "✓".green());
            return Ok(());
        }

        pkg.install(executor, &["redis"]).await?;

        let conf_path = match self.distro {
            Distro::Debian | Distro::Ubuntu => "/etc/redis/redis.conf",
            Distro::RHEL | Distro::Fedora => "/etc/redis.conf",
        };

        executor.run(&format!("sudo sed -i 's/^bind .*/bind 127.0.0.1/' {conf_path}")).await?;

        pkg.enable_service(executor, "redis").await?;
        pkg.start_service(executor, "redis").await?;

        println!("  {} Redis installed", "✓".green());
        Ok(())
    }
}
