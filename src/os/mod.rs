pub mod detect;
pub mod apt;
pub mod dnf;

use anyhow::Result;

pub trait PackageManager: Send + Sync {
    fn update(&self) -> Result<()>;
    fn install(&self, packages: &[&str]) -> Result<()>;
    fn is_installed(&self, package: &str) -> Result<bool>;
    fn enable_service(&self, service: &str) -> Result<()>;
    fn start_service(&self, service: &str) -> Result<()>;
}
