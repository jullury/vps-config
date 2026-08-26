# VPS Config Wizard — Design Spec

## Overview

A Rust CLI tool that remotely provisions and configures a fresh VPS via SSH. Runs locally on the user's machine, SSHs into the target VPS, detects the OS, and applies selected configuration modules (security hardening, services, dev tools).

## Goals

- Zero-dependency single binary distribution
- Interactive wizard with config file override (hybrid mode)
- Multi-distro support (Debian/Ubuntu, RHEL/CentOS/Fedora)
- Idempotent operations where possible
- Safe — never lock the user out of SSH

## Architecture

```
vps-config/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, CLI args
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── prompts.rs       # Interactive wizard prompts
│   │   └── args.rs          # CLI argument parsing (clap)
│   ├── config/
│   │   ├── mod.rs
│   │   ├── schema.rs        # Config struct definitions
│   │   ├── loader.rs        # Load from file / defaults
│   │   └── defaults.rs      # Default values per distro
│   ├── ssh/
│   │   ├── mod.rs
│   │   ├── client.rs        # SSH connection (russh)
│   │   └── executor.rs      # Remote command execution
│   ├── modules/
│   │   ├── mod.rs
│   │   ├── security/
│   │   │   ├── mod.rs
│   │   │   ├── ssh.rs       # Harden SSH (disable root pw, key auth)
│   │   │   ├── firewall.rs  # ufw / firewalld setup
│   │   │   ├── fail2ban.rs  # Install & configure fail2ban
│   │   │   └── users.rs     # Create non-root user, sudo
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── docker.rs    # Install Docker + Compose
│   │   │   ├── nginx.rs     # Install nginx
│   │   │   ├── postgres.rs  # Install PostgreSQL
│   │   │   └── redis.rs     # Install Redis
│   │   └── devtools/
│   │       ├── mod.rs
│   │       ├── node.rs      # nvm + Node.js
│   │       ├── python.rs    # pyenv + Python
│   │       └── go.rs        # Go install
│   └── os/
│       ├── mod.rs
│       ├── detect.rs        # OS detection via /etc/os-release
│       ├── apt.rs           # Debian/Ubuntu package manager
│       └── dnf.rs           # RHEL/Fedora package manager
└── config.example.toml      # Example config file
```

## User Flow

1. Clone repo, `cargo build --release`
2. Run `./target/release/vps-config`
3. Wizard asks:
   - VPS IP address
   - SSH port (default: 22)
   - Auth method: password or SSH key path
4. SSHs into VPS, detects OS
5. Prompts for each category (or reads from config file):
   - Security: Create user? SSH hardening? Firewall? Fail2ban?
   - Services: Docker? Nginx? PostgreSQL? Redis?
   - Dev tools: Node? Python? Go?
6. Applies selected modules in order
7. Reports summary of what was configured

## Key Design Decisions

| Decision | Choice | Reasoning |
|----------|--------|-----------|
| SSH library | `russh` | Pure Rust, no OpenSSL dep, async |
| CLI framework | `clap` | Standard, derive macros |
| Config format | TOML | Human-readable, Rust-friendly |
| Config location | `$XDG_CONFIG_HOME/vps-config/config.toml` or `./config.toml` | Flexible, respects XDG |
| OS detection | Parse `/etc/os-release` | Universal across distros |
| Package manager | Abstract over apt/dnf/yum | Single API, distro-specific impl |

## Config File Format (`config.toml`)

```toml
[vps]
ip = "1.2.3.4"
port = 22
user = "root"
auth = "password"  # or "key"
key_path = "~/.ssh/id_rsa"

[security]
create_user = "deploy"
ssh_password_auth = false
firewall = true
fail2ban = true

[services]
docker = true
nginx = true
postgres = true
redis = false

[devtools]
node = true
node_version = "22"
python = true
python_version = "3.12"
go = true
```

## Execution Model

- Runs **locally** on the user's machine
- SSHs into the **remote VPS** to apply changes
- Each module runs a sequence of remote commands
- Idempotent where possible (checks if already installed)
- Rolls back on critical failure (e.g., don't lock yourself out of SSH)

## Module Details

### Security

| Module | What it does |
|--------|--------------|
| `ssh.rs` | Disables root password login, enforces key auth, changes default port optionally |
| `firewall.rs` | Installs and configures ufw (Debian) or firewalld (RHEL), allows SSH + HTTP/HTTPS |
| `fail2ban.rs` | Installs fail2ban, configures SSH protection jail |
| `users.rs` | Creates a non-root user with sudo, sets up SSH keys for them |

**Critical safety rule**: SSH hardening is applied LAST, only after verifying key-based auth works. If key auth fails, the module rolls back.

### Services

| Module | What it does |
|--------|--------------|
| `docker.rs` | Installs Docker Engine + Docker Compose via official repos |
| `nginx.rs` | Installs nginx, enables and starts the service |
| `postgres.rs` | Installs PostgreSQL, sets up initial database and user |
| `redis.rs` | Installs Redis, configures bind address and basic security |

### Dev Tools

| Module | What it does |
|--------|--------------|
| `node.rs` | Installs nvm, then Node.js (LTS or specific version) |
| `python.rs` | Installs pyenv, then Python (specific version) |
| `go.rs` | Downloads and installs Go from official tarball |

## OS Abstraction

```rust
trait PackageManager {
    fn update(&self) -> Result<()>;
    fn install(&self, packages: &[&str]) -> Result<()>;
    fn is_installed(&self, package: &str) -> Result<bool>;
    fn enable_service(&self, service: &str) -> Result<()>;
    fn start_service(&self, service: &str) -> Result<()>;
}

struct AptManager;   // Debian/Ubuntu
struct DnfManager;   // RHEL/Fedora/CentOS
```

## Error Handling

- Each module returns `Result<()>` with descriptive errors
- Critical operations (SSH, firewall) have rollback logic
- If SSH config changes would lock the user out, the tool warns and asks for confirmation
- All remote command outputs are captured and displayed on failure

## Testing Strategy

- Unit tests for config parsing, OS detection
- Integration tests with a local Docker container (provision a Debian/Ubuntu container, run modules against it)
- Manual testing against a real VPS

## Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
russh = "0.50"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
toml = "0.8"
dialoguer = "0.11"
console = "0.15"
colored = "2"
anyhow = "1"
```

## Non-Goals (for now)

- Multi-VPS orchestration
- GUI interface
- Cloud provider integration (AWS, GCP, etc.)
- Ansible/Terraform compatibility
