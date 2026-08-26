use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub vps: VpsConfig,
    pub security: SecurityConfig,
    pub services: ServicesConfig,
    pub devtools: DevtoolsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VpsConfig {
    pub ip: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_user")]
    pub user: String,
    pub auth: String,
    pub key_path: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SecurityConfig {
    pub create_user: Option<String>,
    #[serde(default = "default_true")]
    pub ssh_password_auth: bool,
    #[serde(default = "default_true")]
    pub firewall: bool,
    #[serde(default = "default_true")]
    pub fail2ban: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServicesConfig {
    #[serde(default)]
    pub docker: bool,
    #[serde(default)]
    pub nginx: bool,
    #[serde(default)]
    pub postgres: bool,
    #[serde(default)]
    pub redis: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DevtoolsConfig {
    #[serde(default)]
    pub node: bool,
    pub node_version: Option<String>,
    #[serde(default)]
    pub python: bool,
    pub python_version: Option<String>,
    #[serde(default)]
    pub go: bool,
    pub go_version: Option<String>,
}

fn default_port() -> u16 { 22 }
fn default_user() -> String { "root".to_string() }
fn default_true() -> bool { true }
