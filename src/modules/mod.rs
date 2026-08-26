pub mod security;
pub mod services;
pub mod devtools;

use anyhow::Result;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;

pub trait Module {
    fn name(&self) -> &str;
    fn apply(&self, executor: &Executor, pkg: &dyn PackageManager) -> Result<()>;
}
