use anyhow::Result;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct DockerModule;

impl Module for DockerModule {
    fn name(&self) -> &str { "docker" }
    fn apply(&self, _executor: &Executor, _pkg: &dyn PackageManager) -> Result<()> {
        Ok(())
    }
}
