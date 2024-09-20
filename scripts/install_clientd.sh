#!/bin/bash

# Default package name
PACKAGE_NAME="clientd"
VERSION="0.3.1"

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

# Define the URL for the raw binary based on the architecture
BINARY_URL="https://github.com/0xC0ffee1/sessio/releases/download/v${VERSION}/sessio-${PACKAGE_NAME}-${VERSION}.${ARCH}"

# Download the binary using curl or wget
if command -v curl > /dev/null; then
    curl -L -o "/tmp/sessio-${PACKAGE_NAME}" "$BINARY_URL"
elif command -v wget > /dev/null; then
    wget -O "/tmp/sessio-${PACKAGE_NAME}" "$BINARY_URL"
else
    echo "Error: Neither curl nor wget is installed."
    exit 1
fi

# Move the binary to ~/.local/bin (user's local bin directory)
mkdir -p $HOME/.local/bin
mv "/tmp/sessio-${PACKAGE_NAME}" $HOME/.local/bin/sessio-${PACKAGE_NAME}
chmod +x $HOME/.local/bin/sessio-${PACKAGE_NAME}

# Create the systemd user service file
mkdir -p $HOME/.config/systemd/user
cat <<EOF > $HOME/.config/systemd/user/sessio-clientd.service
[Unit]
Description="Sessio client daemon"
After=network.target

[Service]
Type=simple
Environment="TERM=xterm-256color"
ExecStart=$HOME/.local/bin/sessio-clientd
Restart=always

[Install]
WantedBy=default.target
EOF

# Reload systemd for user services, enable and start the service
systemctl --user daemon-reload
systemctl --user enable sessio-clientd
systemctl --user start sessio-clientd

echo "Sessio clientd installed and user service started successfully. Stop with: systemctl --user stop sessio-clientd"
