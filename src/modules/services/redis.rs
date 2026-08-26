use anyhow::Result;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct RedisModule;

impl Module for RedisModule {
    fn name(&self) -> &str { "redis" }
    fn apply(&self, _executor: &Executor, _pkg: &dyn PackageManager) -> Result<()> {
        Ok(())
    }
}
