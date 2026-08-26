use anyhow::Result;
use async_trait::async_trait;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;

pub struct AptManager;

impl Default for AptManager {
    fn default() -> Self {
        Self
    }
}

impl AptManager {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait(?Send)]
impl PackageManager for AptManager {
    async fn update(&self, executor: &mut Executor<'_>) -> Result<()> {
        executor.run("apt-get update -qq").await?;
        Ok(())
    }

    async fn install(&self, executor: &mut Executor<'_>, packages: &[&str]) -> Result<()> {
        let pkg_str = packages.join(" ");
        executor
            .run(&format!(
                "DEBIAN_FRONTEND=noninteractive apt-get install -y -qq {}",
                pkg_str
            ))
            .await?;
        Ok(())
    }

    async fn is_installed(&self, executor: &mut Executor<'_>, package: &str) -> Result<bool> {
        let (_output, status) = executor
            .run_with_output(&format!("dpkg -l {} 2>/dev/null | grep -q ii", package))
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
