use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Distro {
    Debian,
    Ubuntu,
    RHEL,
    CentOS,
    Fedora,
}

pub fn detect_distro(executor: &crate::ssh::executor::Executor<'_>) -> Result<Distro> {
    // Placeholder
    Ok(Distro::Debian)
}
