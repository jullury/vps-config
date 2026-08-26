use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct GoModule;

#[async_trait(?Send)]
impl Module for GoModule {
    fn name(&self) -> &str { "go" }

    async fn apply(&self, executor: &mut Executor<'_>, _pkg: &dyn PackageManager) -> Result<()> {
        let (output, _) = executor.run_with_output("go version 2>/dev/null || true").await?;
        if output.contains("go") {
            println!("  {} Go already installed", "✓".green());
            return Ok(());
        }

        executor.run("wget -q https://go.dev/dl/go1.22.0.linux-amd64.tar.gz -O /tmp/go.tar.gz").await?;
        executor.run("rm -rf /usr/local/go && tar -C /usr/local -xzf /tmp/go.tar.gz").await?;
        executor.run("echo 'export PATH=$PATH:/usr/local/go/bin' >> /etc/profile.d/go.sh").await?;
        executor.run("rm /tmp/go.tar.gz").await?;

        println!("  {} Go installed", "✓".green());
        Ok(())
    }
}
