#!/bin/bash
set -e

echo "================================"
echo "Rubik's Cube Solver - Android Build"
echo "================================"
echo ""

# Source Rust environment
source ~/.cargo/env 2>/dev/null || true

# Check for required tools
echo "Checking prerequisites..."

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

# Check Java
if ! command -v java &> /dev/null; then
    echo "❌ Java not found. Please install JDK:"
    echo "   brew install --cask temurin"
    echo "   Or download from: https://adoptium.net/"
    exit 1
fi
echo "✓ Java found: $(java -version 2>&1 | head -1)"

# Check ANDROID_HOME
if [ -z "$ANDROID_HOME" ]; then
    echo "❌ ANDROID_HOME not set. Please set it to your Android SDK location:"
    echo "   export ANDROID_HOME=/opt/homebrew/share/android-commandlinetools"
    echo "   Or your custom SDK location"
    exit 1
fi
echo "✓ ANDROID_HOME set: $ANDROID_HOME"

# Check ANDROID_NDK_HOME
if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "❌ ANDROID_NDK_HOME not set. Please install NDK and set the path:"
    echo "   sdkmanager --sdk_root=\$ANDROID_HOME \"ndk;27.0.12077973\""
    echo "   export ANDROID_NDK_HOME=\$ANDROID_HOME/ndk/27.0.12077973"
    exit 1
fi
echo "✓ ANDROID_NDK_HOME set: $ANDROID_NDK_HOME"

# Check Android targets
echo ""
echo "Checking Rust Android targets..."
REQUIRED_TARGETS=("aarch64-linux-android" "armv7-linux-androideabi")
MISSING_TARGETS=()

for target in "${REQUIRED_TARGETS[@]}"; do
    if ! rustup target list --installed | grep -q "$target"; then
        MISSING_TARGETS+=("$target")
    else
        echo "✓ $target installed"
    fi
done

if [ ${#MISSING_TARGETS[@]} -ne 0 ]; then
    echo "❌ Missing Android targets. Installing..."
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
    echo "Building Android APK (debug)..."
    dx build --android
elif [ "$BUILD_TYPE" = "release" ]; then
    echo "Building Android APK (release)..."
    dx build --android --release
elif [ "$BUILD_TYPE" = "bundle" ]; then
    echo "Creating Android bundle (release)..."
    dx bundle --platform android --release
else
    echo "❌ Unknown build type: $BUILD_TYPE"
    echo "Usage: $0 [debug|release|bundle]"
    exit 1
fi

echo ""
echo "================================"
echo "Build completed successfully!"
echo "================================"

if [ "$BUILD_TYPE" = "bundle" ]; then
    echo ""
    echo "APK location: target/dx/rubiks-cube-solver/bundle/android/"
    echo ""
    echo "To install on device/emulator:"
    echo "  adb install target/dx/rubiks-cube-solver/bundle/android/rubiks-cube-solver.apk"
fi
