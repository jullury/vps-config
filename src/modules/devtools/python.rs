use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct PythonModule<'a> {
    version: &'a str,
}

impl<'a> PythonModule<'a> {
    pub fn new(version: &'a str) -> Self {
        Self { version }
    }
}

#[async_trait(?Send)]
impl<'a> Module for PythonModule<'a> {
    fn name(&self) -> &str { "python" }

    async fn apply(&self, executor: &mut Executor<'_>, pkg: &dyn PackageManager) -> Result<()> {
        pkg.install(executor, &[
            "build-essential", "libssl-dev", "zlib1g-dev", "libbz2-dev",
            "libreadline-dev", "libsqlite3-dev", "wget", "curl", "llvm",
            "libncursesw5-dev", "xz-utils", "tk-dev", "libxml2-dev",
            "libxmlsec1-dev", "libffi-dev", "liblzma-dev",
        ]).await?;

        let (output, _) = executor.run_with_output("command -v pyenv || true").await?;
        if !output.contains("pyenv") {
            executor.run("curl https://pyenv.run | bash").await?;
        }

        executor.run(&format!(
            "export PYENV_ROOT=\"$HOME/.pyenv\" && export PATH=\"$PYENV_ROOT/bin:$PATH\" && pyenv install -s {} && pyenv global {}",
            self.version, self.version
        )).await?;

        println!("  {} Python {} installed", "✓".green(), self.version);
        Ok(())
    }
}
