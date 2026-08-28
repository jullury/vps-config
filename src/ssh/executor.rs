use anyhow::Result;

use super::client::SshSession;

pub struct Executor<'a> {
    session: &'a mut SshSession,
    sudo_password: Option<&'a str>,
}

impl<'a> Executor<'a> {
    pub fn new(session: &'a mut SshSession) -> Self {
        Self {
            session,
            sudo_password: None,
        }
    }

    pub fn with_sudo_password(&mut self, password: &'a str) -> &mut Self {
        self.sudo_password = Some(password);
        self
    }

    fn prepare(&self, command: &str) -> String {
        // If we have a sudo password and the command uses sudo, feed the password via stdin
        if let Some(pw) = self.sudo_password {
            if command.trim_start().starts_with("sudo ") {
                let escaped = pw.replace('\'', "'\\''");
                return format!("echo '{}' | sudo -S {}", escaped, command.trim_start().trim_start_matches("sudo "));
            }
        }
        command.to_string()
    }

    pub async fn run(&mut self, command: &str) -> Result<String> {
        let cmd = self.prepare(command);
        self.session.call(&cmd).await
    }

    pub async fn run_with_output(&mut self, command: &str) -> Result<(String, i32)> {
        let cmd = self.prepare(command);
        self.session.call_with_output(&cmd).await
    }
}
