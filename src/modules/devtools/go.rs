use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct GoModule<'a> {
    version: &'a str,
}

impl<'a> GoModule<'a> {
    pub fn new(version: &'a str) -> Self {
        Self { version }
    }
}

#[async_trait(?Send)]
impl<'a> Module for GoModule<'a> {
    async fn apply(&self, executor: &mut Executor<'_>, _pkg: &dyn PackageManager) -> Result<()> {
        let (output, _) = executor.run_with_output("go version 2>/dev/null || true").await?;
        if output.contains("go") {
            println!("  {} Go already installed", "✓".green());
            return Ok(());
        }

        let (arch_output, _) = executor.run_with_output("uname -m").await?;
        let arch = match arch_output.trim() {
            "x86_64" => "amd64",
            "aarch64" => "arm64",
            other => anyhow::bail!("Unsupported architecture: {}", other),
        };

        let url = format!(
            "https://go.dev/dl/go{}.linux-{}.tar.gz",
            self.version, arch
        );
        executor.run(&format!("wget -q {url} -O /tmp/go.tar.gz")).await?;
        executor.run("sudo sh -c 'rm -rf /usr/local/go && tar -C /usr/local -xzf /tmp/go.tar.gz'").await?;
        executor.run("sudo sh -c 'echo \"export PATH=$PATH:/usr/local/go/bin\" > /etc/profile.d/go.sh'").await?;
        executor.run("rm /tmp/go.tar.gz").await?;

        println!("  {} Go {} installed", "✓".green(), self.version);
        Ok(())
    }
}
