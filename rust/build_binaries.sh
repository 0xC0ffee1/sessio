
# Requires cross (cargo install cross --git https://github.com/cross-rs/cross)
# Also you need to run the ./build_cross_images.sh in the project root folder if those images don't exist on your machine.

# Architectures to build for
ARCHITECTURES=("x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu")

PACKAGES=("sessio-server" "sessio-clientd" "sessio-cli")

# Cross compile, create deb and rpm packages for each architecture
for ARCH in "${ARCHITECTURES[@]}"; do
    echo "Building for architecture: $ARCH"

    # Cross build each package
    for PACKAGE in "${PACKAGES[@]}"; do
        echo "Building $PACKAGE for $ARCH"
        cross build --target "$ARCH" --release -p "$PACKAGE"
    done
done

echo "Build process completed."
