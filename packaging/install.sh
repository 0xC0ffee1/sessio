#!/bin/bash

# Sessio Universal Installation Script
# Detects and installs either client or server based on context
# Usage: ./install.sh --type [client|server] --install-key "KEY" --coordinator "URL"

set -e

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
Sessio Universal Installation Script

Usage: $0 --type [client|server] --install-key KEY --coordinator URL [OPTIONS]

Required Options:
    -t, --type TYPE         Installation type: 'client' or 'server'
    -k, --install-key KEY   Install key from coordinator
    -c, --coordinator URL   Coordinator URL

Optional:
    -i, --id ID            Device ID (auto-generated if not provided)
    -u, --user             Install as user service (for both client and server)
    -v, --version VERSION  Version to install (default: auto-detected)
    -h, --help             Show this help message

Examples:
    # Install client (user mode)
    $0 --type client --install-key "abc123" --coordinator "https://coord.example.com" --user

    # Install client (system-wide)
    sudo $0 --type client --install-key "abc123" --coordinator "https://coord.example.com"

    # Install server (system-wide)
    sudo $0 --type server --install-key "xyz789" --coordinator "https://coord.example.com"

    # Install server (user mode)
    $0 --type server --install-key "xyz789" --coordinator "https://coord.example.com" --user

For direct installation:
    # Client installation
    ./install-client.sh --install-key KEY --coordinator URL [--user]

    # Server installation (system-wide)
    sudo ./install-server.sh --install-key KEY --coordinator URL

    # Server installation (user mode)
    ./install-server.sh --install-key KEY --coordinator URL --user
EOF
}

# Variables
TYPE=""
INSTALL_KEY=""
COORDINATOR=""
DEVICE_ID=""
USER_INSTALL=""
VERSION=""

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -t|--type)
            TYPE="$2"
            shift 2
            ;;
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
            USER_INSTALL="--user"
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
if [[ -z "$TYPE" ]]; then
    error "Installation type is required. Use -t or --type option with 'client' or 'server'"
fi

if [[ "$TYPE" != "client" ]] && [[ "$TYPE" != "server" ]]; then
    error "Invalid type '$TYPE'. Must be 'client' or 'server'"
fi

if [[ -z "$INSTALL_KEY" ]]; then
    error "Install key is required. Use -k or --install-key option."
fi

if [[ -z "$COORDINATOR" ]]; then
    error "Coordinator URL is required. Use -c or --coordinator option."
fi

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Build the appropriate command
if [[ "$TYPE" == "client" ]]; then
    INSTALL_SCRIPT="$SCRIPT_DIR/install-client.sh"
    if [[ ! -f "$INSTALL_SCRIPT" ]]; then
        error "Client installation script not found at $INSTALL_SCRIPT"
    fi
    
    CMD="$INSTALL_SCRIPT --install-key \"$INSTALL_KEY\" --coordinator \"$COORDINATOR\""
    
    if [[ -n "$DEVICE_ID" ]]; then
        CMD="$CMD --id \"$DEVICE_ID\""
    fi
    
    if [[ -n "$USER_INSTALL" ]]; then
        CMD="$CMD $USER_INSTALL"
    fi
    
    if [[ -n "$VERSION" ]]; then
        CMD="$CMD --version \"$VERSION\""
    fi
    
    log "Installing Sessio Client..."
    eval "$CMD"
    
elif [[ "$TYPE" == "server" ]]; then
    INSTALL_SCRIPT="$SCRIPT_DIR/install-server.sh"
    if [[ ! -f "$INSTALL_SCRIPT" ]]; then
        error "Server installation script not found at $INSTALL_SCRIPT"
    fi
    
    # Check for root when installing server in system mode
    if [[ -z "$USER_INSTALL" ]] && [[ $EUID -ne 0 ]]; then
        error "System-wide server installation requires root privileges. Please run with sudo or use --user flag."
    fi
    
    CMD="$INSTALL_SCRIPT --install-key \"$INSTALL_KEY\" --coordinator \"$COORDINATOR\""
    
    if [[ -n "$DEVICE_ID" ]]; then
        CMD="$CMD --id \"$DEVICE_ID\""
    fi
    
    if [[ -n "$USER_INSTALL" ]]; then
        CMD="$CMD $USER_INSTALL"
    fi
    
    if [[ -n "$VERSION" ]]; then
        CMD="$CMD --version \"$VERSION\""
    fi
    
    log "Installing Sessio Server..."
    eval "$CMD"
fi