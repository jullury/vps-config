use anyhow::Result;
use async_trait::async_trait;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct Fail2BanModule;

#[async_trait(?Send)]
impl Module for Fail2BanModule {
    fn name(&self) -> &str { "fail2ban" }
    async fn apply(&self, _executor: &mut Executor<'_>, _pkg: &dyn PackageManager) -> Result<()> {
        Ok(())
    }
}
