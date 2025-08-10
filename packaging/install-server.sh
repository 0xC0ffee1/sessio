#!/bin/bash

# Sessio Server Installation Script
# Downloads and installs the server binary from GitHub releases
# Usage: ./install-server.sh --install-key "KEY" --coordinator "URL"

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
SERVICE_NAME="sessio-server"
BIN_NAME="sessio-server"

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

# Check if running as root for system install
check_root() {
    if [[ "$USER_INSTALL" == false ]] && [[ $EUID -ne 0 ]]; then
        error "System-wide installation requires root permissions. Use sudo or add --user flag."
    fi
}

# Parse command line arguments
usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Install Sessio Server binary from GitHub releases

Required Options:
    -k, --install-key KEY   Install key from coordinator (required)
    -c, --coordinator URL   Coordinator URL (default: $COORDINATOR)

Optional:
    -i, --id ID            Device ID (optional, auto-generated if not provided)
    -u, --user             Install as user service (default: system-wide)
    -v, --version VERSION  Version to install (default: $VERSION)
    -h, --help             Show this help message

Examples:
    sudo $0 --install-key "abc123" --coordinator "https://coordinator.example.com"
    $0 -k "key123" -c "https://coord.local" -i "my-server" --user
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
    DEVICE_ID="$(hostname)-server"
    log "Generated device ID: $DEVICE_ID"
fi

# Check for root permissions if system-wide install
check_root

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

# Download URL for plain binary
DOWNLOAD_URL="${RELEASE_URL}/${BIN_NAME}-${ARCH_SUFFIX}"

# Set installation paths based on user or system install
if [[ "$USER_INSTALL" == true ]]; then
    BIN_DIR="$HOME/.local/bin"
    CONFIG_DIR="$HOME/.sessio"
    SERVICE_DIR="$HOME/.config/systemd/user"
    DATA_DIR="$HOME/.sessio/server-data"
    SYSTEMCTL_CMD="systemctl --user"
else
    BIN_DIR="/usr/local/bin"
    CONFIG_DIR="/etc/sessio"
    SERVICE_DIR="/etc/systemd/system"
    DATA_DIR="/var/lib/sessio"
    SYSTEMCTL_CMD="systemctl"
fi

log "Starting Sessio Server installation..."
log "Version: $VERSION"
log "Architecture: $ARCH_SUFFIX"
log "Install type: $([ "$USER_INSTALL" == true ] && echo "User" || echo "System-wide")"
log "Binary URL: $DOWNLOAD_URL"
log "Binary directory: $BIN_DIR"
log "Config directory: $CONFIG_DIR"
log "Data directory: $DATA_DIR"

# Create directories
log "Creating directories..."
mkdir -p "$BIN_DIR"
mkdir -p "$CONFIG_DIR"
mkdir -p "$SERVICE_DIR"
mkdir -p "$DATA_DIR"

# Create temporary directory for downloads
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# Download binary
log "Downloading server binary from GitHub releases..."

if command -v wget > /dev/null; then
    if ! wget -q --show-progress -O "$TEMP_DIR/$BIN_NAME" "$DOWNLOAD_URL"; then
        error "Failed to download $BIN_NAME binary from $DOWNLOAD_URL"
    fi
elif command -v curl > /dev/null; then
    if ! curl -L --progress-bar -o "$TEMP_DIR/$BIN_NAME" "$DOWNLOAD_URL"; then
        error "Failed to download $BIN_NAME binary from $DOWNLOAD_URL"
    fi
else
    error "Neither wget nor curl is available. Please install one of them."
fi

# Install binary
log "Installing binary..."
cp "$TEMP_DIR/$BIN_NAME" "$BIN_DIR/$BIN_NAME"
chmod +x "$BIN_DIR/$BIN_NAME"

# Handle user creation and ownership for system install
if [[ "$USER_INSTALL" == false ]]; then
    # Create service user
    log "Creating service user..."
    if ! id "sessio" &>/dev/null; then
        useradd --system --no-create-home --shell /bin/false sessio
        log "Created user: sessio"
    else
        log "User already exists: sessio"
    fi
    
    # Set ownership
    chown -R sessio:sessio "$CONFIG_DIR"
    chown -R sessio:sessio "$DATA_DIR"
fi

# Run the install command
log "Running server installation..."
cd "$DATA_DIR"
if [[ "$USER_INSTALL" == true ]]; then
    if ! "$BIN_DIR/$BIN_NAME" install --install-key "$INSTALL_KEY" --coordinator "$COORDINATOR" --id "$DEVICE_ID"; then
        error "Server installation failed"
    fi
else
    if ! sudo -u sessio "$BIN_DIR/$BIN_NAME" install --install-key "$INSTALL_KEY" --coordinator "$COORDINATOR" --id "$DEVICE_ID"; then
        error "Server installation failed"
    fi
fi

# Create systemd service file
log "Creating systemd service..."
if [[ "$USER_INSTALL" == true ]]; then
    cat > "$SERVICE_DIR/$SERVICE_NAME.service" << EOF
[Unit]
Description=Sessio SSH Server (User)
After=network.target

[Service]
Type=simple
WorkingDirectory=$DATA_DIR
ExecStart=$BIN_DIR/$BIN_NAME run --coordinator "$COORDINATOR" --id "$DEVICE_ID"
Restart=always
RestartSec=10
Environment="RUST_LOG=info"

[Install]
WantedBy=default.target
EOF
else
    cat > "$SERVICE_DIR/$SERVICE_NAME.service" << EOF
[Unit]
Description=Sessio SSH Server
After=network.target
Wants=network.target

[Service]
Type=simple
User=sessio
Group=sessio
WorkingDirectory=$DATA_DIR
ExecStart=$BIN_DIR/$BIN_NAME run --coordinator "$COORDINATOR" --id "$DEVICE_ID"
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=$SERVICE_NAME
Environment="RUST_LOG=info"

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$DATA_DIR
ReadWritePaths=$CONFIG_DIR

# Network capabilities for port binding
AmbientCapabilities=CAP_NET_BIND_SERVICE
CapabilityBoundingSet=CAP_NET_BIND_SERVICE

[Install]
WantedBy=multi-user.target
EOF
fi

# Reload systemd and enable service
log "Configuring systemd service..."
$SYSTEMCTL_CMD daemon-reload
$SYSTEMCTL_CMD enable "$SERVICE_NAME"

# Start the service
log "Starting service..."
if $SYSTEMCTL_CMD start "$SERVICE_NAME"; then
    success "Service started successfully"
else
    error "Failed to start service"
fi

# Check service status
sleep 2
if $SYSTEMCTL_CMD is-active --quiet "$SERVICE_NAME"; then
    success "Service is running"
else
    warning "Service may not be running properly. Check logs with: journalctl $([ "$USER_INSTALL" == true ] && echo "--user") -u $SERVICE_NAME"
fi

# Installation complete
success "Sessio Server installation completed!"
echo ""
echo "📦 Version: $VERSION"
echo "🏗️ Architecture: $ARCH_SUFFIX"
echo "📁 Binary: $BIN_DIR/$BIN_NAME"
echo "🔧 Service: $SERVICE_NAME"
if [[ "$USER_INSTALL" == false ]]; then
    echo "👤 User: sessio"
fi
echo "📂 Data: $DATA_DIR"
echo "📋 Status: $SYSTEMCTL_CMD status $SERVICE_NAME"
echo "📄 Logs: journalctl $([ "$USER_INSTALL" == true ] && echo "--user") -u $SERVICE_NAME -f"
echo ""
echo "Service management:"
if [[ "$USER_INSTALL" == true ]]; then
    echo "  Start:   systemctl --user start $SERVICE_NAME"
    echo "  Stop:    systemctl --user stop $SERVICE_NAME"
    echo "  Restart: systemctl --user restart $SERVICE_NAME"
    echo "  Status:  systemctl --user status $SERVICE_NAME"
    
    # Add to PATH reminder if user install
    if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
        echo ""
        warning "Please add $HOME/.local/bin to your PATH"
        echo "You can do this by adding the following line to your ~/.bashrc or ~/.zshrc:"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    fi
else
    echo "  Start:   sudo systemctl start $SERVICE_NAME"
    echo "  Stop:    sudo systemctl stop $SERVICE_NAME"
    echo "  Restart: sudo systemctl restart $SERVICE_NAME"
    echo "  Status:  sudo systemctl status $SERVICE_NAME"
fi