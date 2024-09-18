#!/bin/bash

# Default package name
PACKAGE_NAME="server"
VERSION="0.3.1"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    key="$1"

    case $key in
        --package)
        PACKAGE_NAME="$2"
        shift # past argument
        shift # past value
        ;;
        *)
        echo "Unknown option $1"
        echo "Usage: $0 [--package <server|coordinator|clientd>]"
        exit 1
        ;;
    esac
done

# Detect system architecture
architecture=$(uname -m)

case "$architecture" in
    x86_64)
        ARCH="x86_64"
        PKG_ARCH="amd64"
        ;;
    aarch64)
        ARCH="arm64"
        PKG_ARCH="arm64"
        ;;
    armv7l)
        ARCH="armhf"
        PKG_ARCH="armhf"
        ;;
    *)
        echo "Error: Unsupported architecture $architecture"
        exit 1
        ;;
esac

# Detect package manager (RPM or DPKG)
if command -v rpm &> /dev/null; then
    PKG_TYPE="rpm"
    BINARY_URL="https://github.com/0xC0ffee1/sessio/releases/download/v${VERSION}/sessio-${PACKAGE_NAME}-${VERSION}-1.${ARCH}.rpm"
elif command -v dpkg &> /dev/null; then
    PKG_TYPE="deb"
    BINARY_URL="https://github.com/0xC0ffee1/sessio/releases/download/v${VERSION}/sessio-${PACKAGE_NAME}_${VERSION}-1_${PKG_ARCH}.deb"
else
    echo "Error: Neither RPM nor DPKG package manager found."
    exit 1
fi

# Download the package using curl or wget
if command -v curl > /dev/null; then
    curl -L -o "/tmp/sessio-${PACKAGE_NAME}.${PKG_TYPE}" "$BINARY_URL"
elif command -v wget > /dev/null; then
    wget -O "/tmp/sessio-${PACKAGE_NAME}.${PKG_TYPE}" "$BINARY_URL"
else
    echo "Error: Neither curl nor wget is installed."
    exit 1
fi

echo "/tmp/sessio-${PACKAGE_NAME}.${PKG_TYPE}" "$BINARY_URL"

# Install the package
if command -v rpm &> /dev/null; then
    if [ "$(id -u)" -eq 0 ]; then
        rpm -i "/tmp/sessio-${PACKAGE_NAME}.${PKG_TYPE}"
    else
        echo "Error: You need to be root to install RPM packages."
        exit 1
    fi
elif command -v dpkg &> /dev/null; then
    if [ "$(id -u)" -eq 0 ]; then
        dpkg -i "/tmp/sessio-${PACKAGE_NAME}.${PKG_TYPE}"
    else
        echo "Error: You need to be root to install DEB packages."
        exit 1
    fi
fi
