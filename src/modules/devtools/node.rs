use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct NodeModule<'a> {
    version: &'a str,
}

impl<'a> NodeModule<'a> {
    pub fn new(version: &'a str) -> Self {
        Self { version }
    }
}

#[async_trait(?Send)]
impl<'a> Module for NodeModule<'a> {
    fn name(&self) -> &str { "node" }

    async fn apply(&self, executor: &mut Executor<'_>, _pkg: &dyn PackageManager) -> Result<()> {
        let (output, _) = executor.run_with_output("command -v nvm || true").await?;
        if !output.contains("nvm") {
            executor.run("curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash").await?;
        }

        executor.run(&format!(
            "export NVM_DIR=\"$HOME/.nvm\" && [ -s \"$NVM_DIR/nvm.sh\" ] && . \"$NVM_DIR/nvm.sh\" && nvm install {} && nvm use {}",
            self.version, self.version
        )).await?;

        println!("  {} Node.js {} installed", "✓".green(), self.version);
        Ok(())
    }
}
