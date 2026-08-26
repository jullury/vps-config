use anyhow::Result;
use async_trait::async_trait;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct NodeModule;

#[async_trait(?Send)]
impl Module for NodeModule {
    fn name(&self) -> &str { "node" }
    async fn apply(&self, _executor: &mut Executor<'_>, _pkg: &dyn PackageManager) -> Result<()> {
        Ok(())
    }
}
