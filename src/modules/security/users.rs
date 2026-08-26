use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct UsersModule<'a> {
    username: &'a str,
}

impl<'a> UsersModule<'a> {
    pub fn new(username: &'a str) -> Self {
        Self { username }
    }
}

#[async_trait(?Send)]
impl<'a> Module for UsersModule<'a> {
    fn name(&self) -> &str { "users" }

    async fn apply(&self, executor: &mut Executor<'_>, _pkg: &dyn PackageManager) -> Result<()> {
        println!("  Creating user: {}", self.username);

        // Ensure sudo group exists (Debian) or wheel (RHEL)
        executor.run("groupadd -f sudo || groupadd -f wheel").await?;

        // Create user with sudo
        executor.run(&format!(
            "id {} >/dev/null 2>&1 || useradd -m -s /bin/bash -G sudo {}",
            self.username, self.username
        )).await?;
        executor.run(&format!("usermod -aG sudo {} || usermod -aG wheel {}", self.username, self.username)).await?;

        // Set up passwordless sudo
        executor.run(&format!(
            "echo '{} ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/{}",
            self.username, self.username
        )).await?;
        executor.run(&format!("chmod 440 /etc/sudoers.d/{}", self.username)).await?;

        // Copy root's SSH keys to new user
        executor.run(&format!(
            "mkdir -p /home/{}/.ssh && cp -a /root/.ssh/. /home/{}/.ssh/ && chown -R {}:{} /home/{}/.ssh && chmod 700 /home/{}/.ssh && chmod 600 /home/{}/.ssh/*",
            self.username, self.username, self.username, self.username, self.username, self.username, self.username
        )).await?;

        println!("  {} User '{}' created with sudo access", "✓".green(), self.username);
        Ok(())
    }
}
