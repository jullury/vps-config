use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct NginxModule;

#[async_trait(?Send)]
impl Module for NginxModule {
    fn name(&self) -> &str { "nginx" }

    async fn apply(&self, executor: &mut Executor<'_>, pkg: &dyn PackageManager) -> Result<()> {
        if pkg.is_installed(executor, "nginx").await? {
            println!("  {} nginx already installed", "✓".green());
            return Ok(());
        }

        pkg.install(executor, &["nginx"]).await?;
        pkg.enable_service(executor, "nginx").await?;
        pkg.start_service(executor, "nginx").await?;

        println!("  {} nginx installed", "✓".green());
        Ok(())
    }
}
