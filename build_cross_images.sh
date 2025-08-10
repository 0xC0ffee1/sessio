# Build the images needed for cross compilation. See the build_images.sh script in rust folder.
# Must be run from this directory
docker build -t x86_64-unknown-linux-gnu-protoc -f docker/Dockerfile_x86_64 .
docker build -t aarch64-unknown-linux-gnu-protoc -f docker/Dockerfile_aarch64 .
docker build -t i686-unknown-linux-gnu-protoc -f docker/Dockerfile_i686 .
