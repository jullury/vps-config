use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct RedisModule;

#[async_trait(?Send)]
impl Module for RedisModule {
    fn name(&self) -> &str { "redis" }

    async fn apply(&self, executor: &mut Executor<'_>, pkg: &dyn PackageManager) -> Result<()> {
        if pkg.is_installed(executor, "redis").await? {
            println!("  {} Redis already installed", "✓".green());
            return Ok(());
        }

        pkg.install(executor, &["redis"]).await?;

        executor.run("sed -i 's/^bind .*/bind 127.0.0.1/' /etc/redis/redis.conf").await?;
        executor.run("sed -i 's/^# requirepass .*/requirepass/' /etc/redis/redis.conf").await?;

        pkg.enable_service(executor, "redis").await?;
        pkg.start_service(executor, "redis").await?;

        println!("  {} Redis installed", "✓".green());
        Ok(())
    }
}
