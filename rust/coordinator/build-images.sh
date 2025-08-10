#!/bin/bash

# Sessio Docker Images Build Script
set -e

echo "Building Sessio Docker Images"

# Configuration
REGISTRY=${DOCKER_REGISTRY:-"sessio"}
VERSION=${VERSION:-"latest"}
COORDINATOR_TAG="${REGISTRY}/coordinator:${VERSION}"
FRONTEND_TAG="${REGISTRY}/frontend:${VERSION}"

# Parse command line arguments
PUSH_IMAGES=false
BUILD_COORDINATOR=true
BUILD_FRONTEND=true

while [[ $# -gt 0 ]]; do
    case $1 in
        --push)
            PUSH_IMAGES=true
            shift
            ;;
        --coordinator-only)
            BUILD_FRONTEND=false
            shift
            ;;
        --frontend-only)
            BUILD_COORDINATOR=false
            shift
            ;;
        --registry)
            REGISTRY="$2"
            COORDINATOR_TAG="${REGISTRY}/coordinator:${VERSION}"
            FRONTEND_TAG="${REGISTRY}/frontend:${VERSION}"
            shift 2
            ;;
        --version)
            VERSION="$2"
            COORDINATOR_TAG="${REGISTRY}/coordinator:${VERSION}"
            FRONTEND_TAG="${REGISTRY}/frontend:${VERSION}"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --push                Push images to registry after building"
            echo "  --coordinator-only    Only build coordinator image"
            echo "  --frontend-only       Only build frontend image"
            echo "  --registry REGISTRY   Docker registry to use (default: sessio)"
            echo "  --version VERSION     Image version tag (default: latest)"
            echo "  --help               Show this help message"
            echo ""
            echo "Environment variables:"
            echo "  DOCKER_REGISTRY      Docker registry (default: sessio)"
            echo "  VERSION              Image version (default: latest)"
            echo ""
            echo "Examples:"
            echo "  # Build all images"
            echo "  ./build-images.sh"
            echo ""
            echo "  # Build and push to Docker Hub"
            echo "  ./build-images.sh --push --registry your-username"
            echo ""
            echo "  # Build specific version"
            echo "  ./build-images.sh --version v1.0.0"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

echo " Build Configuration:"
echo "  Registry: $REGISTRY"
echo "  Version: $VERSION"
echo "  Build Coordinator: $BUILD_COORDINATOR"
echo "  Build Frontend: $BUILD_FRONTEND"
echo "  Push Images: $PUSH_IMAGES"
echo ""

# Build coordinator image
if [ "$BUILD_COORDINATOR" = true ]; then
    echo "🔨 Building coordinator image: $COORDINATOR_TAG"
    docker build -t "$COORDINATOR_TAG" -f Dockerfile ..
    
    if [ $? -eq 0 ]; then
        echo "✅ Successfully built coordinator image"
    else
        echo "❌ Failed to build coordinator image"
        exit 1
    fi
fi

# Build frontend image
if [ "$BUILD_FRONTEND" = true ]; then
    echo "🔨 Building frontend image: $FRONTEND_TAG"
    docker build -t "$FRONTEND_TAG" ../../coordinator-frontend/
    
    if [ $? -eq 0 ]; then
        echo "✅ Successfully built frontend image"
    else
        echo "❌ Failed to build frontend image"
        exit 1
    fi
fi

# Push images if requested
if [ "$PUSH_IMAGES" = true ]; then
    echo "📤 Pushing images to registry..."
    
    if [ "$BUILD_COORDINATOR" = true ]; then
        echo "Pushing $COORDINATOR_TAG..."
        docker push "$COORDINATOR_TAG"
    fi
    
    if [ "$BUILD_FRONTEND" = true ]; then
        echo "Pushing $FRONTEND_TAG..."
        docker push "$FRONTEND_TAG"
    fi
    
    echo "✅ Successfully pushed images to registry"
fi

echo ""
echo "🏷️  Built Images:"
if [ "$BUILD_COORDINATOR" = true ]; then
    echo "  - $COORDINATOR_TAG"
fi
if [ "$BUILD_FRONTEND" = true ]; then
    echo "  - $FRONTEND_TAG"
fi


echo ""
echo "✅ Build complete!"
