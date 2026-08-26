use anyhow::Result;
use async_trait::async_trait;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct GoModule;

#[async_trait(?Send)]
impl Module for GoModule {
    fn name(&self) -> &str { "go" }
    async fn apply(&self, _executor: &mut Executor<'_>, _pkg: &dyn PackageManager) -> Result<()> {
        Ok(())
    }
}
