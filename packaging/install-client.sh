#!/bin/bash

# Sessio Client Installation Script
# Downloads and installs the client binaries from GitHub releases
# Usage: ./install-client.sh --install-key "KEY" --coordinator "URL"

set -e

# Get script directory and source script constants
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/script_consts.sh"

# Configuration  
VERSION="$SESSIO_VERSION"
GITHUB_REPO="$SESSIO_GITHUB_REPO"
RELEASE_URL="$SESSIO_RELEASE_URL"

# Default values
COORDINATOR="https://127.0.0.1:2223"
DEVICE_ID=""
INSTALL_KEY=""
USER_INSTALL=false
SERVICE_NAME="sessio-clientd"
CLI_BIN_NAME="sessio-cli"
DAEMON_BIN_NAME="sessio-clientd"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log() {
    echo -e "${BLUE}[$(date '+%Y-%m-%d %H:%M:%S')]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Parse command line arguments
usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Install Sessio Client binaries from GitHub releases

Required Options:
    -k, --install-key KEY   Install key from coordinator (required)
    -c, --coordinator URL   Coordinator URL (default: $COORDINATOR)

Optional:
    -i, --id ID            Device ID (optional, auto-generated if not provided)
    -u, --user             Install as user service (default: system-wide)
    -v, --version VERSION  Version to install (default: $VERSION)
    -h, --help             Show this help message

Examples:
    $0 --install-key "abc123" --coordinator "https://coordinator.example.com"
    $0 -k "key123" -c "https://coord.local" -i "my-device" --user
EOF
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -k|--install-key)
            INSTALL_KEY="$2"
            shift 2
            ;;
        -c|--coordinator)
            COORDINATOR="$2"
            shift 2
            ;;
        -i|--id)
            DEVICE_ID="$2"
            shift 2
            ;;
        -u|--user)
            USER_INSTALL=true
            shift
            ;;
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            error "Unknown option: $1"
            ;;
    esac
done

# Validate required arguments
if [[ -z "$INSTALL_KEY" ]]; then
    error "Install key is required. Use -k or --install-key option."
fi

# Generate device ID if not provided
if [[ -z "$DEVICE_ID" ]]; then
    DEVICE_ID="$(hostname)-client"
    log "Generated device ID: $DEVICE_ID"
fi

# Check for root permissions if system-wide install
if [[ "$USER_INSTALL" == false ]] && [[ $EUID -ne 0 ]]; then
    error "System-wide installation requires root permissions. Use sudo or add --user flag."
fi

# Detect architecture
ARCH=$(uname -m)

case "$ARCH" in
    x86_64)
        ARCH_SUFFIX="x86_64"
        ;;
    aarch64|arm64)
        ARCH_SUFFIX="aarch64"
        ;;
    *)
        error "Unsupported architecture: $ARCH. Only x86_64 and aarch64 are supported."
        ;;
esac

# Download URLs for plain binaries
CLI_DOWNLOAD_URL="${RELEASE_URL}/${CLI_BIN_NAME}-${ARCH_SUFFIX}"
DAEMON_DOWNLOAD_URL="${RELEASE_URL}/${DAEMON_BIN_NAME}-${ARCH_SUFFIX}"

# Set installation paths based on user or system install
if [[ "$USER_INSTALL" == true ]]; then
    BIN_DIR="$HOME/.local/bin"
    CONFIG_DIR="$HOME/.sessio"
    SERVICE_DIR="$HOME/.config/systemd/user"
    SYSTEMCTL_CMD="systemctl --user"
else
    BIN_DIR="/usr/local/bin"
    CONFIG_DIR="/etc/sessio"
    SERVICE_DIR="/etc/systemd/system"
    SYSTEMCTL_CMD="systemctl"
fi

log "Starting Sessio Client installation..."
log "Version: $VERSION"
log "Architecture: $ARCH_SUFFIX"
log "Install type: $([ "$USER_INSTALL" == true ] && echo "User" || echo "System-wide")"
log "CLI Binary URL: $CLI_DOWNLOAD_URL"
log "Daemon Binary URL: $DAEMON_DOWNLOAD_URL"
log "Binary directory: $BIN_DIR"
log "Config directory: $CONFIG_DIR"

# Create directories
log "Creating directories..."
mkdir -p "$BIN_DIR"
mkdir -p "$CONFIG_DIR"
mkdir -p "$SERVICE_DIR"
mkdir -p "$HOME/.sessio"  # Always create user config dir for socket

# Create temporary directory for downloads
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# Download binaries
log "Downloading client binaries from GitHub releases..."

# Download CLI binary
log "Downloading $CLI_BIN_NAME binary..."
if command -v wget > /dev/null; then
    if ! wget -q --show-progress -O "$TEMP_DIR/$CLI_BIN_NAME" "$CLI_DOWNLOAD_URL"; then
        error "Failed to download $CLI_BIN_NAME binary from $CLI_DOWNLOAD_URL"
    fi
elif command -v curl > /dev/null; then
    if ! curl -L --progress-bar -o "$TEMP_DIR/$CLI_BIN_NAME" "$CLI_DOWNLOAD_URL"; then
        error "Failed to download $CLI_BIN_NAME binary from $CLI_DOWNLOAD_URL"
    fi
else
    error "Neither wget nor curl is available. Please install one of them."
fi

# Download daemon binary
log "Downloading $DAEMON_BIN_NAME binary..."
if command -v wget > /dev/null; then
    if ! wget -q --show-progress -O "$TEMP_DIR/$DAEMON_BIN_NAME" "$DAEMON_DOWNLOAD_URL"; then
        error "Failed to download $DAEMON_BIN_NAME binary from $DAEMON_DOWNLOAD_URL"
    fi
elif command -v curl > /dev/null; then
    if ! curl -L --progress-bar -o "$TEMP_DIR/$DAEMON_BIN_NAME" "$DAEMON_DOWNLOAD_URL"; then
        error "Failed to download $DAEMON_BIN_NAME binary from $DAEMON_DOWNLOAD_URL"
    fi
else
    error "Neither wget nor curl is available. Please install one of them."
fi

# Install binaries
log "Installing binaries..."
cp "$TEMP_DIR/$CLI_BIN_NAME" "$BIN_DIR/sessio"
cp "$TEMP_DIR/$DAEMON_BIN_NAME" "$BIN_DIR/$DAEMON_BIN_NAME"
chmod +x "$BIN_DIR/sessio"
chmod +x "$BIN_DIR/$DAEMON_BIN_NAME"

# Create systemd service file
log "Creating systemd service..."
if [[ "$USER_INSTALL" == true ]]; then
    cat > "$SERVICE_DIR/$SERVICE_NAME.service" << EOF
[Unit]
Description=Sessio Client Daemon
After=network.target

[Service]
Type=simple
ExecStart=$BIN_DIR/$DAEMON_BIN_NAME
Restart=always
RestartSec=10
Environment="RUST_LOG=info"

[Install]
WantedBy=default.target
EOF
else
    cat > "$SERVICE_DIR/$SERVICE_NAME.service" << EOF
[Unit]
Description=Sessio Client Daemon
After=network.target

[Service]
Type=simple
ExecStart=$BIN_DIR/$DAEMON_BIN_NAME
Restart=always
RestartSec=10
User=sessio
Group=sessio
Environment="RUST_LOG=info"

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=read-only
ReadWritePaths=$CONFIG_DIR
ReadWritePaths=/home/sessio/.sessio

[Install]
WantedBy=multi-user.target
EOF

    # Create service user for system-wide install
    log "Creating service user..."
    if ! id "sessio" &>/dev/null; then
        useradd --system --create-home --shell /bin/false sessio
        log "Created user: sessio"
    else
        log "User already exists: sessio"
    fi
    
    # Create .sessio directory for the service user
    mkdir -p /home/sessio/.sessio
    chown -R sessio:sessio /home/sessio/.sessio
    
    # Set ownership for system-wide install
    chown -R sessio:sessio "$CONFIG_DIR"
fi

# Start the daemon
log "Starting daemon..."
$SYSTEMCTL_CMD daemon-reload
$SYSTEMCTL_CMD enable "$SERVICE_NAME"
$SYSTEMCTL_CMD start "$SERVICE_NAME"

# Wait for daemon to start
sleep 3

# Run the install command
log "Running install command..."
if [[ "$USER_INSTALL" == true ]]; then
    "$BIN_DIR/sessio" install --install-key "$INSTALL_KEY" --coordinator "$COORDINATOR" --device-id "$DEVICE_ID"
else
    sudo -u sessio "$BIN_DIR/sessio" install --install-key "$INSTALL_KEY" --coordinator "$COORDINATOR" --device-id "$DEVICE_ID"
fi

# Check if installation was successful
if [[ $? -eq 0 ]]; then
    success "Sessio Client installed successfully!"
    echo ""
    echo "📦 Version: $VERSION"
    echo "🏗️ Architecture: $ARCH_SUFFIX"
    echo "📁 Binaries: $BIN_DIR"
    echo "🔧 Service: $SERVICE_NAME"
    echo "📋 Status: $SYSTEMCTL_CMD status $SERVICE_NAME"
    echo "📄 Logs: journalctl $([ "$USER_INSTALL" == true ] && echo "--user") -u $SERVICE_NAME -f"
    echo ""
    echo "You can now use the 'sessio' command to interact with devices."
    echo "Try 'sessio status' to see available devices."
    
    # Add to PATH reminder if user install
    if [[ "$USER_INSTALL" == true ]] && [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
        warning "Please add $HOME/.local/bin to your PATH to use the 'sessio' command"
        echo "You can do this by adding the following line to your ~/.bashrc or ~/.zshrc:"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    fi
else
    error "Installation failed. Check the logs for details."
fi