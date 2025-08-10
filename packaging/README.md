# Sessio Installation Scripts

This directory contains installation scripts for Sessio client and server components. The scripts automatically download the appropriate binaries from GitHub releases and configure them on your system.

## Quick Start

### Client Installation

For user installation (recommended for desktop use):
```bash
curl -sSL https://raw.githubusercontent.com/0xC0ffee1/sessio/main/packaging/install-client.sh | bash -s -- --install-key "YOUR_KEY" --coordinator "https://your.coordinator.com" --user
```

For system-wide installation:
```bash
curl -sSL https://raw.githubusercontent.com/0xC0ffee1/sessio/main/packaging/install-client.sh | sudo bash -s -- --install-key "YOUR_KEY" --coordinator "https://your.coordinator.com"
```

### Server Installation

Server installation requires root privileges:
```bash
curl -sSL https://raw.githubusercontent.com/0xC0ffee1/sessio/main/packaging/install-server.sh | sudo bash -s -- --install-key "YOUR_KEY" --coordinator "https://your.coordinator.com"
```

## Installation Scripts

### install-client.sh

Downloads and installs the Sessio client components (CLI and daemon) from GitHub releases.

**Usage:**
```bash
./install-client.sh --install-key KEY --coordinator URL [OPTIONS]
```

**Options:**
- `-k, --install-key KEY` - Install key from coordinator (required)
- `-c, --coordinator URL` - Coordinator URL (default: https://127.0.0.1:2223)
- `-i, --id ID` - Device ID (optional, auto-generated if not provided)
- `-u, --user` - Install as user service (default: system-wide)
- `-v, --version VERSION` - Version to install (default: 0.3.1)
- `-h, --help` - Show help message

**What it does:**
1. Downloads `sessio-cli` and `sessio-clientd` binaries from GitHub releases
2. Installs binaries to appropriate directories
3. Creates and starts a systemd service for the daemon
4. Runs the `sessio install` command to register with the coordinator
5. Configures the client with the provided install key

### install-server.sh

Downloads and installs the Sessio server from GitHub releases.

**Usage:**
```bash
sudo ./install-server.sh --install-key KEY --coordinator URL [OPTIONS]
```

**Options:**
- `-k, --install-key KEY` - Install key from coordinator (required)
- `-c, --coordinator URL` - Coordinator URL (default: https://127.0.0.1:2223)
- `-i, --id ID` - Device ID (optional, auto-generated if not provided)
- `-v, --version VERSION` - Version to install (default: 0.3.1)
- `-h, --help` - Show help message

**What it does:**
1. Downloads `sessio-server` binary from GitHub releases
2. Creates a dedicated `sessio` user for the service
3. Runs the `sessio-server install` command to configure the server
4. Creates and starts a systemd service
5. Sets up proper permissions and security settings

## Architecture Support

The scripts automatically detect your system architecture and download the appropriate binary:
- `x86_64` - 64-bit Intel/AMD
- `aarch64` - 64-bit ARM (ARM64)
- `armv7` - 32-bit ARM
- `i686` - 32-bit Intel/AMD

## File Locations

### Client Installation

**User Installation (`--user`):**
- Binaries: `~/.local/bin/`
- Configuration: `~/.sessio/`
- Service: `~/.config/systemd/user/sessio-clientd.service`

**System Installation:**
- Binaries: `/usr/local/bin/`
- Configuration: `/etc/sessio/`
- Service: `/etc/systemd/system/sessio-clientd.service`

### Server Installation

- Binary: `/usr/local/bin/sessio-server`
- Configuration: `/etc/sessio/`
- Data: `/var/lib/sessio/`
- Service: `/etc/systemd/system/sessio-server.service`

## Service Management

### Client Service

```bash
# User installation
systemctl --user status sessio-clientd
systemctl --user start sessio-clientd
systemctl --user stop sessio-clientd
systemctl --user restart sessio-clientd
journalctl --user -u sessio-clientd -f

# System installation
sudo systemctl status sessio-clientd
sudo systemctl start sessio-clientd
sudo systemctl stop sessio-clientd
sudo systemctl restart sessio-clientd
sudo journalctl -u sessio-clientd -f
```

### Server Service

```bash
sudo systemctl status sessio-server
sudo systemctl start sessio-server
sudo systemctl stop sessio-server
sudo systemctl restart sessio-server
sudo journalctl -u sessio-server -f
```

## Using the CLI

After installation, you can use the `sessio` command:

```bash
# Show status of services and available devices
sessio status

# List active sessions
sessio list

# Connect to a device
sessio pty device-id

# Server management (requires server installation)
sessio server status
sessio server start
sessio server stop
sessio server restart
sessio server uninstall [--purge]
```

## Uninstallation

### Client

For complete removal:
```bash
# Stop and disable service
systemctl --user stop sessio-clientd  # or sudo systemctl for system install
systemctl --user disable sessio-clientd

# Remove files
rm ~/.local/bin/sessio ~/.local/bin/sessio-clientd
rm -rf ~/.sessio
rm ~/.config/systemd/user/sessio-clientd.service
```

### Server

```bash
# Use the CLI
sessio server uninstall --purge

# Or manually:
sudo systemctl stop sessio-server
sudo systemctl disable sessio-server
sudo rm /usr/local/bin/sessio-server
sudo rm -rf /etc/sessio /var/lib/sessio
sudo rm /etc/systemd/system/sessio-server.service
sudo userdel sessio
```

## Troubleshooting

### Check service logs
```bash
# Client
journalctl --user -u sessio-clientd -n 50  # or sudo journalctl for system install

# Server
sudo journalctl -u sessio-server -n 50
```

### Common Issues

1. **Permission denied**: Make sure to use `sudo` for server installation
2. **Service won't start**: Check logs and ensure the coordinator URL is accessible
3. **Install key invalid**: Get a fresh install key from your coordinator admin
4. **Binary not found**: Add `~/.local/bin` to your PATH for user installations

## AppImage Support

The scripts will automatically fall back to downloading AppImage versions if regular binaries are not available. AppImages are self-contained and work across different Linux distributions.