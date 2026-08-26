use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::{PackageManager, detect::Distro};
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct Fail2BanModule<'a> {
    distro: &'a Distro,
}

impl<'a> Fail2BanModule<'a> {
    pub fn new(distro: &'a Distro) -> Self {
        Self { distro }
    }
}

#[async_trait(?Send)]
impl<'a> Module for Fail2BanModule<'a> {
    fn name(&self) -> &str { "fail2ban" }

    async fn apply(&self, executor: &mut Executor<'_>, pkg: &dyn PackageManager) -> Result<()> {
        pkg.install(executor, &["fail2ban"]).await?;

        let logpath = match self.distro {
            Distro::Debian | Distro::Ubuntu => "/var/log/auth.log",
            Distro::RHEL | Distro::Fedora => "/var/log/secure",
        };

        let jail_config = format!(r#"[DEFAULT]
bantime = 3600
findtime = 600
maxretry = 5

[sshd]
enabled = true
port = ssh
filter = sshd
logpath = {logpath}
maxretry = 3
"#);

        executor.run(&format!(
            "echo '{}' > /etc/fail2ban/jail.local",
            jail_config.replace('\n', "\\n")
        )).await?;

        pkg.enable_service(executor, "fail2ban").await?;
        pkg.start_service(executor, "fail2ban").await?;

        println!("  {} fail2ban installed and configured", "✓".green());
        Ok(())
    }
}
