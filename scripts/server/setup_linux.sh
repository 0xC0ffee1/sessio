#!/bin/bash

# Check if the script is running in an interactive terminal
if [[ ! -t 0 ]]; then
    echo "Relaunching script in an interactive shell..."
    exec bash -i "$0" "$@"
fi

# Variables
SERVICE_NAME="sessio-server"
BINARY_BASE_URL="https://github.com/0xC0ffee1/sessio/releases/download/v0.3.1-beta/sessio-server-"
INSTALL_DIR="$HOME/.sessio"
BINARY_PATH="$INSTALL_DIR/$SERVICE_NAME"
CONFIG_FILE="$INSTALL_DIR/config.toml"

# Determine service file location
if [ "$(id -u)" -eq 0 ]; then
    # Root user, use system-level service
    echo "Installing as root"
    SERVICE_FILE="/etc/systemd/system/$SERVICE_NAME.service"
else
    # Regular user, use user-level service
    SERVICE_FILE="$HOME/.config/systemd/user/$SERVICE_NAME.service"
fi

# Create the installation directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Detect system architecture
architecture=$(uname -m)

case "$architecture" in
  x86_64)
    ARCH="x86_64"
    ;;
  aarch64)
    ARCH="arm64"
    ;;
  armv7l)
    ARCH="armhf"
    ;;
  *)
    echo "Error: Unsupported architecture $architecture"
    exit 1
    ;;
esac

BINARY_URL="${BINARY_BASE_URL}${ARCH}"

# Download the binary using curl
if command -v curl > /dev/null; then
    curl -L -o "$BINARY_PATH" "$BINARY_URL"
elif command -v wget > /dev/null; then
    wget -O "$BINARY_PATH" "$BINARY_URL"
else
    echo "Error: Neither curl nor wget is installed."
    exit 1
fi

# Make the binary executable
chmod +x "$BINARY_PATH"

# Prompt the user for the id and coordinator url
read -p "Enter the device id: " DEVICE_ID
read -p "Enter the coordinator URL: " COORDINATOR_URL

# Create the default config file
cat > "$CONFIG_FILE" <<EOF
# Configuration for $SERVICE_NAME

coordinator = "$COORDINATOR_URL"
id = "$DEVICE_ID"
private_key = "$INSTALL_DIR/ssh_host_ed25519_key"
EOF

echo "Config file created at $CONFIG_FILE"

# Create the systemd service file
if [ "$(id -u)" -eq 0 ]; then
    # System-level service for root
    sudo tee "$SERVICE_FILE" > /dev/null << EOF
[Unit]
Description=$SERVICE_NAME
After=network.target

[Service]
ExecStart=$BINARY_PATH --config $CONFIG_FILE
Restart=always

[Install]
WantedBy=multi-user.target
EOF
else
    # User-level service
    mkdir -p "$(dirname "$SERVICE_FILE")"
    tee "$SERVICE_FILE" > /dev/null << EOF
[Unit]
Description=$SERVICE_NAME
After=network.target

[Service]
ExecStart=$BINARY_PATH --config $CONFIG_FILE
Restart=always

[Install]
WantedBy=default.target
EOF
fi

# Reload systemd, enable, and start the service
if [ "$(id -u)" -eq 0 ]; then
    # System-level service
    sudo systemctl daemon-reload
    sudo systemctl enable "$SERVICE_NAME"
    sudo systemctl start "$SERVICE_NAME"
else
    # User-level service
    systemctl --user daemon-reload
    systemctl --user enable "$SERVICE_NAME"
    systemctl --user start "$SERVICE_NAME"
fi

echo "$SERVICE_NAME has been installed and started."
