use anyhow::Result;

use super::client::SshSession;

pub struct Executor<'a> {
    session: &'a mut SshSession,
}

impl<'a> Executor<'a> {
    pub fn new(session: &'a mut SshSession) -> Self {
        Self { session }
    }

    pub async fn run(&mut self, command: &str) -> Result<String> {
        self.session.call(command).await
    }

    pub async fn run_with_output(&mut self, command: &str) -> Result<(String, i32)> {
        self.session.call_with_output(command).await
    }
}
