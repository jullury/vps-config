use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct Fail2BanModule;

#[async_trait(?Send)]
impl Module for Fail2BanModule {
    fn name(&self) -> &str { "fail2ban" }

    async fn apply(&self, executor: &mut Executor<'_>, pkg: &dyn PackageManager) -> Result<()> {
        pkg.install(executor, &["fail2ban"]).await?;

        let jail_config = r#"[DEFAULT]
bantime = 3600
findtime = 600
maxretry = 5

[sshd]
enabled = true
port = ssh
filter = sshd
logpath = /var/log/auth.log
maxretry = 3
"#;

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
