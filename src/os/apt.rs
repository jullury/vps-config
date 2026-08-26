use anyhow::Result;
use crate::os::PackageManager;

pub struct AptManager;

impl PackageManager for AptManager {
    fn update(&self) -> Result<()> { Ok(()) }
    fn install(&self, _packages: &[&str]) -> Result<()> { Ok(()) }
    fn is_installed(&self, _package: &str) -> Result<bool> { Ok(false) }
    fn enable_service(&self, _service: &str) -> Result<()> { Ok(()) }
    fn start_service(&self, _service: &str) -> Result<()> { Ok(()) }
}
