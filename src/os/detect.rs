use anyhow::{Result, Context};

#[derive(Debug, Clone, PartialEq)]
pub enum Distro {
    Debian,
    Ubuntu,
    RHEL,
    Fedora,
}

pub fn parse_os_release(content: &str) -> Result<Distro> {
    let mut id = None;
    for line in content.lines() {
        if let Some(val) = line.strip_prefix("ID=") {
            id = Some(val.trim_matches('"').to_string());
        }
    }
    let id = id.context("Could not find ID in os-release")?;
    match id.as_str() {
        "ubuntu" => Ok(Distro::Ubuntu),
        "debian" => Ok(Distro::Debian),
        "rhel" | "centos" | "rocky" | "almalinux" => Ok(Distro::RHEL),
        "fedora" => Ok(Distro::Fedora),
        _ => anyhow::bail!("Unsupported distro: {}", id),
    }
}

pub async fn detect_distro(executor: &mut crate::ssh::executor::Executor<'_>) -> Result<Distro> {
    let output = executor.run("cat /etc/os-release").await?;
    parse_os_release(&output)
}
