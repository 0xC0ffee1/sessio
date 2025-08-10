#!/bin/bash

# Script Constants for Sessio Installation Scripts
# This file provides shared constants and functions for all installation scripts

# Read version from VERSION file in parent directory
VERSION_FILE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/../VERSION"

if [ -f "$VERSION_FILE" ]; then
    export SESSIO_VERSION=$(cat "$VERSION_FILE" | tr -d '\n\r')
else
    echo "Error: VERSION file not found at $VERSION_FILE" >&2
    exit 1
fi

# Export other version-related variables
export SESSIO_GITHUB_REPO="0xC0ffee1/sessio"
export SESSIO_RELEASE_URL="https://github.com/${SESSIO_GITHUB_REPO}/releases/download/v${SESSIO_VERSION}"

# Component versions (can be overridden for specific components)
export SESSIO_CLIENT_VERSION="${SESSIO_CLIENT_VERSION:-$SESSIO_VERSION}"
export SESSIO_SERVER_VERSION="${SESSIO_SERVER_VERSION:-$SESSIO_VERSION}"
export SESSIO_COORDINATOR_VERSION="${SESSIO_COORDINATOR_VERSION:-$SESSIO_VERSION}"
export SESSIO_CLI_VERSION="${SESSIO_CLI_VERSION:-$SESSIO_VERSION}"

# Function to get version
get_version() {
    echo "$SESSIO_VERSION"
}

# Function to get component version
get_component_version() {
    local component="$1"
    case "$component" in
        client|clientd)
            echo "$SESSIO_CLIENT_VERSION"
            ;;
        server)
            echo "$SESSIO_SERVER_VERSION"
            ;;
        coordinator)
            echo "$SESSIO_COORDINATOR_VERSION"
            ;;
        cli)
            echo "$SESSIO_CLI_VERSION"
            ;;
        *)
            echo "$SESSIO_VERSION"
            ;;
    esac
}

# Function to get download URL for component AppImage
get_download_url() {
    local component="$1"
    local arch="$2"
    
    local component_version=$(get_component_version "$component")
    local base_url="https://github.com/${SESSIO_GITHUB_REPO}/releases/download/v${component_version}"
    
    # Only AppImages are supported
    echo "${base_url}/sessio-${component}-${arch}.AppImage"
}

# If script is executed directly, show version information
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    echo "Sessio Script Constants"
    echo "======================"
    echo "Main Version: $SESSIO_VERSION"
    echo "GitHub Repo: $SESSIO_GITHUB_REPO"
    echo "Release URL: $SESSIO_RELEASE_URL"
    echo ""
    echo "Component Versions:"
    echo "  Client:      $(get_component_version client)"
    echo "  Server:      $(get_component_version server)"
    echo "  Coordinator: $(get_component_version coordinator)"
    echo "  CLI:         $(get_component_version cli)"
    echo ""
    echo "Usage:"
    echo "  source script_consts.sh             # Load script constants"
    echo "  get_version                         # Get main version"
    echo "  get_component_version client        # Get component version"
    echo "  get_download_url server x86_64     # Get AppImage download URL"
fi