use clap::Parser;

#[derive(Parser)]
#[command(name = "vps-config", about = "Remote VPS provisioning wizard")]
pub struct Args {
    /// Path to config file
    #[arg(short, long)]
    pub config: Option<String>,

    /// VPS IP address
    #[arg(short, long)]
    pub ip: Option<String>,

    /// SSH port
    #[arg(short, long, default_value_t = 22)]
    pub port: u16,

    /// SSH user
    #[arg(short, long, default_value = "root")]
    pub user: String,
}
