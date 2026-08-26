pub mod security;
pub mod services;
pub mod devtools;

use anyhow::Result;
use async_trait::async_trait;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;

#[async_trait(?Send)]
pub trait Module {
    fn name(&self) -> &str;
    async fn apply(&self, executor: &mut Executor<'_>, pkg: &dyn PackageManager) -> Result<()>;
}
