use anyhow::Result;
use async_trait::async_trait;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct DockerModule;

#[async_trait(?Send)]
impl Module for DockerModule {
    fn name(&self) -> &str { "docker" }
    async fn apply(&self, _executor: &mut Executor<'_>, _pkg: &dyn PackageManager) -> Result<()> {
        Ok(())
    }
}
