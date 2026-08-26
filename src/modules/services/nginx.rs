use anyhow::Result;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct NginxModule;

impl Module for NginxModule {
    fn name(&self) -> &str { "nginx" }
    fn apply(&self, _executor: &Executor, _pkg: &dyn PackageManager) -> Result<()> {
        Ok(())
    }
}
