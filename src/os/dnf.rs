use anyhow::Result;
use async_trait::async_trait;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;

pub struct DnfManager;

impl Default for DnfManager {
    fn default() -> Self {
        Self
    }
}

impl DnfManager {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait(?Send)]
impl PackageManager for DnfManager {
    async fn update(&self, executor: &mut Executor<'_>) -> Result<()> {
        executor.run("dnf makecache -q").await?;
        Ok(())
    }

    async fn install(&self, executor: &mut Executor<'_>, packages: &[&str]) -> Result<()> {
        let pkg_str = packages.join(" ");
        executor
            .run(&format!("dnf install -y -q {}", pkg_str))
            .await?;
        Ok(())
    }

    async fn is_installed(&self, executor: &mut Executor<'_>, package: &str) -> Result<bool> {
        let (_output, status) = executor
            .run_with_output(&format!("rpm -q {} 2>/dev/null", package))
            .await?;
        Ok(status == 0)
    }

    async fn enable_service(&self, executor: &mut Executor<'_>, service: &str) -> Result<()> {
        executor
            .run(&format!("systemctl enable {}", service))
            .await?;
        Ok(())
    }

    async fn start_service(&self, executor: &mut Executor<'_>, service: &str) -> Result<()> {
        executor
            .run(&format!("systemctl start {}", service))
            .await?;
        Ok(())
    }
}
