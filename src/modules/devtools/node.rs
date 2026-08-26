use anyhow::Result;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct NodeModule;

impl Module for NodeModule {
    fn name(&self) -> &str { "node" }
    fn apply(&self, _executor: &Executor, _pkg: &dyn PackageManager) -> Result<()> {
        Ok(())
    }
}
