#Must be run from this directory
docker build -t x86_64-unknown-linux-gnu-protoc -f docker/Dockerfile_x86_64 .
docker build -t aarch64-unknown-linux-gnu-protoc -f docker/Dockerfile_aarch64 .
docker build -t i686-unknown-linux-gnu-protoc -f docker/Dockerfile_i686 .