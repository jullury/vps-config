use anyhow::Result;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct Fail2BanModule;

impl Module for Fail2BanModule {
    fn name(&self) -> &str { "fail2ban" }
    fn apply(&self, _executor: &Executor, _pkg: &dyn PackageManager) -> Result<()> {
        Ok(())
    }
}
