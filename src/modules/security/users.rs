use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct UsersModule<'a> {
    username: &'a str,
    password: Option<&'a str>,
    source_user: &'a str,
}

impl<'a> UsersModule<'a> {
    pub fn new(username: &'a str, password: Option<&'a str>, source_user: &'a str) -> Self {
        Self { username, password, source_user }
    }
}

#[async_trait(?Send)]
impl<'a> Module for UsersModule<'a> {
    async fn apply(&self, executor: &mut Executor<'_>, _pkg: &dyn PackageManager) -> Result<()> {
        println!("  Creating user: {}", self.username);

        // Ensure sudo group exists (Debian) or wheel (RHEL)
        executor.run("sudo groupadd -f sudo || sudo groupadd -f wheel").await?;

        // Create user with sudo
        executor.run(&format!(
            "id {} >/dev/null 2>&1 || sudo useradd -m -s /bin/bash -G sudo {}",
            self.username, self.username
        )).await?;
        executor.run(&format!("sudo usermod -aG sudo {} || sudo usermod -aG wheel {}", self.username, self.username)).await?;

        // Set up passwordless sudo
        executor.run(&format!(
            "sudo sh -c 'echo \"{} ALL=(ALL) NOPASSWD:ALL\" > /etc/sudoers.d/{} && chmod 440 /etc/sudoers.d/{}'",
            self.username, self.username, self.username
        )).await?;

        // Set user password if provided
        if let Some(password) = self.password {
            let esc_user = self.username.replace('\'', "'\\''");
            let esc_pass = password.replace('\'', "'\\''");
            executor.run(&format!(
                "sudo sh -c 'echo \"{}:{}\" | chpasswd'",
                esc_user, esc_pass
            )).await?;
        }

        // Copy source user's SSH keys to new user
        executor.run(&format!(
            "sudo mkdir -p /home/{}/.ssh && sudo cp -a /home/{}/.ssh/. /home/{}/.ssh/ && sudo chown -R {}:{} /home/{}/.ssh && sudo chmod 700 /home/{}/.ssh && sudo chmod 600 /home/{}/.ssh/*",
            self.username, self.source_user, self.username, self.username, self.username, self.username, self.username, self.username
        )).await?;

        println!("  {} User '{}' created with sudo access", "✓".green(), self.username);
        Ok(())
    }
}
