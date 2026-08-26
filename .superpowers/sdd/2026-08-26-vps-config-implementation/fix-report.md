# Fix Report — VPS Config Code Review Findings

**Date:** 2026-08-26

## Changes Made

### CRITICAL

**1. Password authentication not implemented** — `src/ssh/client.rs`
- Added `password: Option<String>` field to `VpsConfig` in `src/config/schema.rs`
- Rewrote `SshClient::connect()` to match on `config.auth`:
  - `"password"` → uses `session.authenticate_password()` with the password from config
  - `"key"` (or any other value) → uses `session.authenticate_publickey()` with key file (existing behavior)
- Updated wizard in `src/cli/prompts.rs` to prompt for password when auth method is "password"
- Files: `src/ssh/client.rs`, `src/config/schema.rs`, `src/cli/prompts.rs`

### IMPORTANT

**2. Fail2ban hardcoded Debian log path** — `src/modules/security/fail2ban.rs`
- Made `Fail2BanModule` generic over distro (added `distro: &'a Distro` field)
- Set `logpath` to `/var/log/auth.log` on Debian/Ubuntu, `/var/log/secure` on RHEL/Fedora
- Changed `apply()` from `r#"..."#` literal to `format!()` for dynamic logpath
- Updated `modules/mod.rs` to pass `distro` to `Fail2BanModule::new()`
- Files: `src/modules/security/fail2ban.rs`, `src/modules/mod.rs`

**3. Go module hardcodes amd64 and fixed version** — `src/modules/devtools/go.rs`
- Made `GoModule` generic with a `version: &'a str` field
- Added architecture detection via `uname -m`: maps `x86_64` → `amd64`, `aarch64` → `arm64`
- Downloads `go{version}.linux-{arch}.tar.gz` instead of hardcoded URL
- Added `go_version: Option<String>` to `DevtoolsConfig` in schema
- Updated wizard to prompt for Go version (default `1.22.0`)
- Updated `modules/mod.rs` to pass version to `GoModule::new()`
- Files: `src/modules/devtools/go.rs`, `src/config/schema.rs`, `src/cli/prompts.rs`, `src/modules/mod.rs`

**4. Docker module CentOS match arm** — `src/modules/services/docker.rs`
- CentOS, Rocky, and AlmaLinux all map to `Distro::RHEL` in `parse_os_release()` (existing behavior in `src/os/detect.rs:23`), so the `Distro::RHEL | Distro::Fedora` match arm already handles them correctly
- No code change needed in docker.rs itself — the match was already correct
- See finding #8 for the related CentOS enum cleanup

**5. PostgreSQL service name differs between distros** — `src/modules/services/postgres.rs`
- Moved `enable_service`/`start_service` calls inside the distro match arms
- On Debian/Ubuntu: uses `postgresql` (existing behavior)
- On RHEL/Fedora: detects actual service name via `systemctl list-unit-files | grep '^postgresql'` (handles `postgresql-16`, `postgresql-15`, etc.)
- Files: `src/modules/services/postgres.rs`

**6. No rollback on SSH hardening failure** — `src/modules/security/ssh_harden.rs`
- Wrapped `systemctl restart sshd` in error handler
- On failure: restores `/etc/ssh/sshd_config.bak` → `/etc/ssh/sshd_config`, attempts restart with original config, then bails with descriptive error
- Files: `src/modules/security/ssh_harden.rs`

**7. Config file path doesn't match spec** — `src/main.rs`
- Now checks `$XDG_CONFIG_HOME/vps-config/config.toml` (via `dirs::config_dir()`) first
- Falls back to `./config.toml` in CWD if XDG path doesn't exist
- Falls back to interactive wizard if neither exists
- Files: `src/main.rs`

### MINOR

**8. CentOS enum variant is dead code** — `src/os/detect.rs`
- Removed `CentOS` variant from `Distro` enum (CentOS/Rocky/AlmaLinux already map to `RHEL` in `parse_os_release()`)
- Removed `Distro::CentOS` from all match arms in `src/main.rs`, `src/modules/security/firewall.rs`
- Files: `src/os/detect.rs`, `src/main.rs`, `src/modules/security/firewall.rs`

**9. PhantomData in SshHardenModule is unnecessary** — `src/modules/security/ssh_harden.rs`
- Removed `marker: std::marker::PhantomData<&'a ()>` field and its associated lifetime parameter from `SshHardenModule`
- Files: `src/modules/security/ssh_harden.rs`

## Compilation

```
$ cargo check
warning: unreachable pattern (x3)  — all Distro variants now covered explicitly
warning: dead code (x3)            — pre-existing, unrelated
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.11s
```

No errors. All findings resolved.
