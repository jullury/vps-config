use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use russh::keys::*;
use russh::*;

use crate::config::schema::VpsConfig;

struct ClientHandler;

impl client::Handler for ClientHandler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &ssh_key::PublicKey,
    ) -> std::result::Result<bool, Self::Error> {
        Ok(true)
    }
}

pub struct SshSession {
    session: client::Handle<ClientHandler>,
}

impl SshSession {
    async fn read_channel(&self, channel: &mut Channel<client::Msg>) -> Result<(String, Option<u32>)> {
        let mut output = String::new();
        let mut exit_status: Option<u32> = None;

        loop {
            let Some(msg) = channel.wait().await else {
                break;
            };
            match msg {
                ChannelMsg::Data { ref data } => {
                    output.push_str(&String::from_utf8_lossy(data));
                }
                ChannelMsg::ExitStatus { exit_status: status } => {
                    exit_status = Some(status);
                }
                _ => {}
            }
        }

        Ok((output, exit_status))
    }

    pub async fn call(&mut self, command: &str) -> Result<String> {
        let mut channel = self.session.channel_open_session().await?;
        channel.exec(true, command).await?;

        let (output, exit_status) = self.read_channel(&mut channel).await?;
        let status = exit_status.context("Channel did not exit cleanly")?;
        if status != 0 {
            anyhow::bail!("Command failed (exit {}): {}", status, output);
        }
        Ok(output)
    }

    pub async fn call_with_output(&mut self, command: &str) -> Result<(String, i32)> {
        let mut channel = self.session.channel_open_session().await?;
        channel.exec(true, command).await?;

        let (output, exit_status) = self.read_channel(&mut channel).await?;
        let status = exit_status.context("Channel did not exit cleanly")?;
        Ok((output, status as i32))
    }

    pub async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}

pub struct SshClient {
    config: VpsConfig,
}

impl SshClient {
    pub fn new(config: &VpsConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
        })
    }

    pub fn config(&self) -> &VpsConfig {
        &self.config
    }

    pub async fn connect(&self) -> Result<SshSession> {
        let key_path = self
            .config
            .key_path
            .as_ref()
            .context("SSH key path required")?;
        let key_path = shellexpand::tilde(key_path);
        let key_pair = load_secret_key(key_path.as_ref(), None)
            .context("Failed to load SSH private key")?;

        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(30)),
            ..<_>::default()
        };
        let config = Arc::new(config);
        let handler = ClientHandler;

        let addr = (&*self.config.ip, self.config.port);
        let mut session = client::connect(config, addr, handler).await?;

        let auth_res = session
            .authenticate_publickey(
                &self.config.user,
                PrivateKeyWithHashAlg::new(
                    Arc::new(key_pair),
                    session.best_supported_rsa_hash().await?.flatten(),
                ),
            )
            .await?;

        if !auth_res.success() {
            anyhow::bail!("SSH public key authentication failed");
        }

        Ok(SshSession { session })
    }
}
