#!/bin/bash
set -e

echo "================================"
echo "Rubik's Cube Solver - iOS Build"
echo "================================"
echo ""

# Source Rust environment
source ~/.cargo/env 2>/dev/null || true

# Check for required tools
echo "Checking prerequisites..."

# Check macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "❌ iOS builds require macOS"
    exit 1
fi
echo "✓ macOS detected: $(sw_vers -productVersion)"

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install from https://rustup.rs/"
    exit 1
fi
echo "✓ Rust found: $(cargo --version)"

# Check Dioxus CLI
if ! command -v dx &> /dev/null; then
    echo "❌ Dioxus CLI not found. Install with: cargo install dioxus-cli"
    exit 1
fi
echo "✓ Dioxus CLI found: $(dx --version)"

# Check Xcode
if ! command -v xcodebuild &> /dev/null; then
    echo "❌ Xcode not found. Please install from Mac App Store or run: xcode-select --install"
    exit 1
fi
echo "✓ Xcode found: $(xcodebuild -version | head -1)"

# Check iOS targets
echo ""
echo "Checking Rust iOS targets..."
REQUIRED_TARGETS=("aarch64-apple-ios" "aarch64-apple-ios-sim")
MISSING_TARGETS=()

for target in "${REQUIRED_TARGETS[@]}"; do
    if ! rustup target list --installed | grep -q "$target"; then
        MISSING_TARGETS+=("$target")
    else
        echo "✓ $target installed"
    fi
done

if [ ${#MISSING_TARGETS[@]} -ne 0 ]; then
    echo "❌ Missing iOS targets. Installing..."
    for target in "${MISSING_TARGETS[@]}"; do
        echo "  Installing $target..."
        rustup target add "$target"
    done
fi

echo ""
echo "All prerequisites satisfied!"
echo ""

# Build
BUILD_TYPE="${1:-release}"

if [ "$BUILD_TYPE" = "debug" ]; then
    echo "Building iOS app (debug for simulator)..."
    dx build --ios
elif [ "$BUILD_TYPE" = "release" ]; then
    echo "Building iOS app (release for device)..."
    dx build --ios --release
elif [ "$BUILD_TYPE" = "bundle" ]; then
    echo "Creating iOS bundle (release IPA)..."
    dx bundle --platform ios --release
elif [ "$BUILD_TYPE" = "simulator" ]; then
    echo "Building and launching in iOS simulator..."
    dx build --ios
    echo ""
    echo "To view logs:"
    echo "  xcrun simctl spawn booted log stream --level debug"
else
    echo "❌ Unknown build type: $BUILD_TYPE"
    echo "Usage: $0 [debug|release|bundle|simulator]"
    exit 1
fi

echo ""
echo "================================"
echo "Build completed successfully!"
echo "================================"

if [ "$BUILD_TYPE" = "bundle" ]; then
    echo ""
    echo "IPA location: target/dx/rubiks-cube-solver/bundle/ios/"
    echo ""
    echo "To test on device:"
    echo "  1. Connect your iOS device"
    echo "  2. Open Xcode and trust the device"
    echo "  3. Install: dx build --ios --device \"Your Device Name\""
fi

if [ "$BUILD_TYPE" = "debug" ] || [ "$BUILD_TYPE" = "simulator" ]; then
    echo ""
    echo "App running in simulator!"
    echo "Use Simulator app controls to interact with the app"
fi
