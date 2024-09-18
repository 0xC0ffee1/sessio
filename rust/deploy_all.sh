#!/bin/bash

# Architectures to build for
ARCHITECTURES=("x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu" "i686-unknown-linux-gnu")
PACKAGES=("sessio-server" "sessio-clientd" "sessio-coordinator")

# Cross compile, create deb and rpm packages for each architecture
for ARCH in "${ARCHITECTURES[@]}"; do
    echo "Building for architecture: $ARCH"

    # Cross build
    cross build --target "$ARCH" --release

    # DEB packages
    for PACKAGE in "${PACKAGES[@]}"; do
        echo "Creating DEB package for $PACKAGE ($ARCH)"
        cargo deb --target="$ARCH" --no-build -p "$PACKAGE" --no-strip
    done

    # RPM packages
    for PACKAGE_SHORT in "client" "server" "coordinator"; do
        echo "Creating RPM package for $PACKAGE_SHORT ($ARCH)"
        cargo generate-rpm --target "$ARCH" --package "$PACKAGE_SHORT"
    done
done

echo "Build process completed."
