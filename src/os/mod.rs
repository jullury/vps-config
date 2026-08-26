use anyhow::Result;
use async_trait::async_trait;

pub mod apt;
pub mod detect;
pub mod dnf;

#[async_trait(?Send)]
pub trait PackageManager {
    async fn update(&self, executor: &mut crate::ssh::executor::Executor<'_>) -> Result<()>;
    async fn install(
        &self,
        executor: &mut crate::ssh::executor::Executor<'_>,
        packages: &[&str],
    ) -> Result<()>;
    async fn is_installed(
        &self,
        executor: &mut crate::ssh::executor::Executor<'_>,
        package: &str,
    ) -> Result<bool>;
    async fn enable_service(
        &self,
        executor: &mut crate::ssh::executor::Executor<'_>,
        service: &str,
    ) -> Result<()>;
    async fn start_service(
        &self,
        executor: &mut crate::ssh::executor::Executor<'_>,
        service: &str,
    ) -> Result<()>;
}
