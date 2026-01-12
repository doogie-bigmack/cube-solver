#!/bin/bash
# Build script for Linux desktop binary
#
# This script builds the Rubik's Cube Solver for Linux (Ubuntu 22.04+)
# It can be run on Linux directly, or in a Docker container from macOS/Windows
#
# Requirements:
# - Rust 1.75+ with cargo
# - pkg-config
# - libssl-dev
# - GTK 3 development libraries (libgtk-3-dev)
# - WebKit2GTK development libraries (libwebkit2gtk-4.1-dev)
# - AppIndicator development libraries (libayatana-appindicator3-dev)
#
# Usage:
#   ./build-linux.sh              # Build release binary
#   ./build-linux.sh --docker     # Build in Docker container

set -e

echo "🦀 Rubik's Cube Solver - Linux Build Script"
echo "==========================================="

# Check if running in Docker
if [ "$1" = "--docker" ]; then
    echo "🐳 Building in Docker container..."

    # Use a Rust container with required dependencies
    docker run --rm \
        -v "$(pwd)":/app \
        -w /app \
        rust:1.75-slim-bookworm \
        bash -c "
            set -e
            echo '📦 Installing build dependencies...'
            apt-get update -qq
            apt-get install -y -qq \
                pkg-config \
                libssl-dev \
                libgtk-3-dev \
                libwebkit2gtk-4.1-dev \
                libayatana-appindicator3-dev \
                librsvg2-dev \
                patchelf \
                > /dev/null 2>&1

            echo '🔨 Building release binary...'
            cargo build --release --features desktop

            echo '✅ Build complete!'
            echo 'Binary location: target/release/rubiks-cube-solver'
            ls -lh target/release/rubiks-cube-solver
        "

    exit 0
fi

# Native Linux build
echo "🔨 Building natively for Linux..."

# Check for required dependencies
if ! command -v pkg-config &> /dev/null; then
    echo "❌ Error: pkg-config not found"
    echo "   Install with: sudo apt-get install pkg-config"
    exit 1
fi

if ! pkg-config --exists gtk+-3.0; then
    echo "❌ Error: GTK 3 development libraries not found"
    echo "   Install with: sudo apt-get install libgtk-3-dev"
    exit 1
fi

if ! pkg-config --exists webkit2gtk-4.1; then
    echo "❌ Error: WebKit2GTK development libraries not found"
    echo "   Install with: sudo apt-get install libwebkit2gtk-4.1-dev"
    exit 1
fi

# Build the release binary
echo "📦 Building release binary with desktop features..."
cargo build --release --features desktop

echo ""
echo "✅ Build successful!"
echo "📍 Binary location: target/release/rubiks-cube-solver"
echo "📊 Binary size: $(ls -lh target/release/rubiks-cube-solver | awk '{print $5}')"
echo ""
echo "To run: ./target/release/rubiks-cube-solver"
