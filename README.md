# vps-config

Remote VPS provisioning wizard. Run locally, configure your VPS via SSH.

## Usage

```bash
# Build
cargo build --release

# Run interactive wizard
./target/release/vps-config

# Use config file
./target/release/vps-config --config config.toml
```

## Config File

See `config.example.toml` for available options.

## Features

- Security hardening (SSH, firewall, fail2ban, users)
- Services (Docker, nginx, PostgreSQL, Redis)
- Dev tools (Node.js, Python, Go)
- Multi-distro support (Debian/Ubuntu, RHEL/Fedora)
- Idempotent operations
