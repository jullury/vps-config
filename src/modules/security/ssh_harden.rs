use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct SshHardenModule<'a> {
    password_auth: bool,
    new_port: Option<u16>,
    marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> SshHardenModule<'a> {
    pub fn new(password_auth: bool, new_port: Option<u16>) -> Self {
        Self {
            password_auth,
            new_port,
            marker: std::marker::PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<'a> Module for SshHardenModule<'a> {
    fn name(&self) -> &str { "ssh_harden" }

    async fn apply(&self, executor: &mut Executor<'_>, _pkg: &dyn PackageManager) -> Result<()> {
        println!("  Hardening SSH configuration...");

        executor.run("cp /etc/ssh/sshd_config /etc/ssh/sshd_config.bak").await?;

        if !self.password_auth {
            executor.run("sed -i 's/^#\\?PermitRootLogin.*/PermitRootLogin prohibit-password/' /etc/ssh/sshd_config").await?;
            executor.run("sed -i 's/^#\\?PasswordAuthentication.*/PasswordAuthentication no/' /etc/ssh/sshd_config").await?;
        }

        if let Some(port) = self.new_port {
            executor.run(&format!("sed -i 's/^#\\?Port .*/Port {}/' /etc/ssh/sshd_config", port)).await?;
        }

        executor.run("systemctl restart sshd").await?;

        println!("  {} SSH hardened", "✓".green());
        println!("  {} WARNING: SSH restart with new config. Verify you can still connect!", "⚠".yellow());
        Ok(())
    }
}
