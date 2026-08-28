use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use crate::os::PackageManager;
use crate::ssh::executor::Executor;
use crate::modules::Module;

pub struct SshHardenModule {
    password_auth: bool,
    allow_root_login: bool,
    new_port: Option<u16>,
}

impl SshHardenModule {
    pub fn new(password_auth: bool, allow_root_login: bool, new_port: Option<u16>) -> Self {
        Self {
            password_auth,
            allow_root_login,
            new_port,
        }
    }
}

#[async_trait(?Send)]
impl Module for SshHardenModule {
    async fn apply(&self, executor: &mut Executor<'_>, _pkg: &dyn PackageManager) -> Result<()> {
        println!("  Hardening SSH configuration...");

        executor.run("sudo cp /etc/ssh/sshd_config /etc/ssh/sshd_config.bak").await?;

        if !self.password_auth {
            // SAFETY: Verify at least one SSH key exists before disabling password auth.
            // Without this check, users can be permanently locked out of their VPS.
            let (_, exit_code) = executor.run_with_output(
                "test -s ~/.ssh/authorized_keys"
            ).await?;
            if exit_code != 0 {
                anyhow::bail!(
                    "SSH hardening aborted: ~/.ssh/authorized_keys does not exist or is empty. \
                     Add an SSH public key first, then re-run. \
                     Without password auth, you would be locked out of your VPS."
                );
            }

            executor.run("sudo sed -i 's/^#\\?PermitRootLogin.*/PermitRootLogin prohibit-password/' /etc/ssh/sshd_config").await?;
            executor.run("sudo sed -i 's/^#\\?PasswordAuthentication.*/PasswordAuthentication no/' /etc/ssh/sshd_config").await?;
        }

        // Set PermitRootLogin based on config
        if self.allow_root_login {
            executor.run("sudo sed -i 's/^#\\?PermitRootLogin.*/PermitRootLogin yes/' /etc/ssh/sshd_config").await?;
        } else {
            executor.run("sudo sed -i 's/^#\\?PermitRootLogin.*/PermitRootLogin no/' /etc/ssh/sshd_config").await?;
        }

        if let Some(port) = self.new_port {
            executor.run(&format!("sudo sed -i 's/^#\\?Port .*/Port {}/' /etc/ssh/sshd_config", port)).await?;
        }

        // Detect correct SSH service name (ssh on Debian/Ubuntu, sshd on RHEL/Fedora)
        let (svc_output, _) = executor.run_with_output(
            "systemctl list-unit-files | grep -E '^ssh\\.service|^sshd\\.service' | head -1 | awk '{print $1}'"
        ).await?;
        let service = svc_output.trim().trim_end_matches(".service");
        if service.is_empty() {
            anyhow::bail!("Could not determine SSH service name");
        }

        let restart_result = executor.run(&format!("sudo systemctl restart {service}")).await;
        if let Err(e) = restart_result {
            eprintln!("  {} SSH restart failed, rolling back configuration: {}", "✗".red(), e);
            executor.run("sudo cp /etc/ssh/sshd_config.bak /etc/ssh/sshd_config").await?;
            executor.run(&format!("sudo systemctl restart {service}")).await?;
            anyhow::bail!("SSH hardening failed and configuration was rolled back: {}", e);
        }

        println!("  {} SSH hardened", "✓".green());
        println!("  {} WARNING: SSH restart with new config. Verify you can still connect!", "⚠".yellow());
        Ok(())
    }
}
