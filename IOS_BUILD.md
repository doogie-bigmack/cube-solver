# iOS Build Documentation

This document describes how to build the Rubik's Cube Solver application for iOS devices and simulators.

## Overview

The application is built using **Dioxus**, a Rust framework that compiles to native iOS using the iOS SDK and Metal for graphics rendering. The build process produces an IPA (iOS App Archive) that can be installed on physical devices or simulators.

## Requirements

### System Requirements
- **Operating System**: macOS 12 (Monterey) or later
- **Minimum iOS Version**: iOS 15+
- **Target Devices**: iPhone 6s and newer, iPad Air 2 and newer
- **Architectures**: ARM64 (devices), ARM64/x86_64 (simulators)

### Software Prerequisites

1. **macOS** (Required)
   - iOS development can only be done on macOS
   - Verify: `sw_vers`

2. **Xcode** 14 or later
   - Install from Mac App Store
   - Or install command-line tools:
     ```bash
     xcode-select --install
     ```
   - Verify: `xcodebuild -version`

3. **Rust** (latest stable)
   - Install from: https://rustup.rs/
   - Verify: `rustc --version`

4. **Dioxus CLI**
   ```bash
   cargo install dioxus-cli
   ```
   - Verify: `dx --version`

5. **iOS Rust Targets**
   ```bash
   rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
   ```
   - Verify: `rustup target list --installed | grep ios`

6. **Apple Developer Account** (for device deployment)
   - Free account: Supports development and testing
   - Paid account ($99/year): Required for App Store distribution
   - Sign up at: https://developer.apple.com/

## Environment Setup

### Configure Xcode

Set the active developer directory:
```bash
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```

Verify the configuration:
```bash
xcode-select -p
# Should output: /Applications/Xcode.app/Contents/Developer
```

### Setup Signing Certificate

1. **Open Xcode**
2. **Go to Preferences** → Accounts
3. **Add Apple ID** (+ button)
4. **Download Certificates** (Manage Certificates → + → iOS Development)

Or use command line:
```bash
# Xcode will manage certificates automatically when building
```

### Configure Bundle Identifier

Ensure `Dioxus.toml` has the correct bundle identifier:
```toml
[bundle]
identifier = "com.rubiks.cube.solver"
publisher = "Rubik's Cube Solver Team"
```

## Build Instructions

### Quick Build (Using Script)

We provide a build script for convenience:

```bash
# Debug build for simulator
./build-ios.sh debug

# Release build for device
./build-ios.sh release

# Create IPA bundle
./build-ios.sh bundle
```

### Manual Build

#### Build for Simulator (Debug)
```bash
source ~/.cargo/env
dx build --ios
```

This builds for the iOS simulator and automatically launches it.

#### Build for Device (Release)
```bash
source ~/.cargo/env
dx build --ios --release --device "Your iPhone"
```

Replace "Your iPhone" with your device name (see `dx build --help` for options).

#### Create IPA Bundle
```bash
source ~/.cargo/env
dx bundle --platform ios --release
```

The IPA will be created in: `target/dx/rubiks-cube-solver/bundle/ios/`

## Testing

### Run Tests
```bash
cargo test --test ios_build_tests
```

This runs tests that verify:
- ✅ iOS targets are installed
- ✅ Xcode is installed
- ✅ Dioxus CLI is available
- ✅ Configuration files are present

### Testing on iOS Simulator

1. **List Available Simulators**
   ```bash
   xcrun simctl list devices
   ```

2. **Create Simulator** (if needed)
   ```bash
   xcrun simctl create "iPhone 15" "com.apple.CoreSimulator.SimDeviceType.iPhone-15"
   ```

3. **Launch Simulator**
   ```bash
   open -a Simulator
   ```

4. **Build and Run**
   ```bash
   dx build --ios
   # App automatically installs and launches
   ```

5. **View Logs**
   ```bash
   xcrun simctl spawn booted log stream --level debug
   ```

### Testing on Physical Device

1. **Connect Device** via USB

2. **Trust Computer** (prompt on device)

3. **Select Device**
   ```bash
   dx build --ios --device "iPhone Name"
   ```

4. **Trust Developer**
   - On device: Settings → General → VPN & Device Management
   - Tap your developer account → Trust

5. **Launch App** from home screen

## Troubleshooting

### "xcode-select: error: tool 'xcodebuild' requires Xcode"
**Solution**: Install Xcode from the Mac App Store or install command-line tools:
```bash
xcode-select --install
```

### "No provisioning profile found"
**Solution**:
1. Open Xcode
2. Create a new iOS project (any template)
3. In Signing & Capabilities, select your Apple ID
4. Xcode will create provisioning profiles automatically

### "Code signature invalid"
**Solution**:
```bash
# Clean build
cargo clean
rm -rf target/

# Rebuild
dx build --ios --release
```

### "Untrusted Developer" on device
**Solution**:
- Settings → General → VPN & Device Management
- Find your developer certificate
- Tap "Trust [Developer Name]"

### Simulator not launching
**Solution**:
```bash
# Kill existing simulators
killall Simulator

# Reset simulator
xcrun simctl erase all

# Launch fresh
open -a Simulator
```

### Build takes too long
**Solution**:
```bash
# Use debug builds for development
dx build --ios  # Much faster than --release

# Only use release for final testing
dx build --ios --release
```

### "Metal validation errors"
**Solution**: These are usually non-critical warnings. If rendering fails:
- Ensure device supports Metal (iPhone 5s or newer)
- Check iOS version is 15+
- Update Xcode to latest version

## App Store Distribution

### Prepare for Distribution

1. **Create App in App Store Connect**
   - Log in to https://appstoreconnect.apple.com/
   - Click "+" → New App
   - Fill in app information

2. **Create App ID**
   - Developer portal → Identifiers
   - Create identifier: `com.rubiks.cube.solver`

3. **Create Distribution Certificate**
   - Xcode → Preferences → Accounts
   - Manage Certificates → + → Apple Distribution

4. **Create Provisioning Profile**
   - Developer portal → Profiles
   - Create "App Store" profile
   - Download and install

### Build for App Store

```bash
# Create release build
dx bundle --platform ios --release

# Locate IPA
ls -lh target/dx/rubiks-cube-solver/bundle/ios/
```

### Validate IPA

Using Transporter app (recommended):
1. Open Transporter app (Mac App Store)
2. Sign in with Apple ID
3. Drag IPA file to Transporter
4. Click "Deliver"

Or using command line:
```bash
xcrun altool --validate-app \
  -f target/dx/rubiks-cube-solver/bundle/ios/RubiksCubeSolver.ipa \
  -t ios \
  --apiKey YOUR_API_KEY \
  --apiIssuer YOUR_ISSUER_ID
```

### Upload to App Store

```bash
xcrun altool --upload-app \
  -f target/dx/rubiks-cube-solver/bundle/ios/RubiksCubeSolver.ipa \
  -t ios \
  --apiKey YOUR_API_KEY \
  --apiIssuer YOUR_ISSUER_ID
```

## Build Artifacts

After a successful build:

- **Simulator Build**: `target/aarch64-apple-ios-sim/release/rubiks-cube-solver`
- **Device Build**: `target/aarch64-apple-ios/release/rubiks-cube-solver`
- **IPA Bundle**: `target/dx/rubiks-cube-solver/bundle/ios/RubiksCubeSolver.ipa`
- **Debug Symbols**: `target/*/release/rubiks-cube-solver.dSYM`

## App Information

- **Bundle Identifier**: `com.rubiks.cube.solver`
- **Display Name**: Rubik's Cube Solver
- **Minimum iOS**: 15.0
- **Target iOS**: 17.0
- **Architectures**: arm64 (devices), arm64/x86_64 (simulators)
- **Capabilities**:
  - Camera (for cube scanning)
  - PhotoLibrary (for saving screenshots)
- **Frameworks**:
  - Metal (3D rendering)
  - UIKit (UI framework)
  - AVFoundation (camera access)

## Performance Notes

- **First Build**: 15-30 minutes (downloads dependencies, builds for multiple architectures)
- **Incremental Builds**: 1-3 minutes
- **Simulator Build**: ~2 minutes
- **Device Build**: ~3 minutes
- **IPA Size**: 15-25 MB (compressed), 40-60 MB (installed)
- **Runtime Performance**: Native ARM64 speed

## Continuous Integration

Example GitHub Actions workflow:

```yaml
name: iOS Build

on: [push, pull_request]

jobs:
  build-ios:
    runs-on: macos-14  # macOS required for iOS builds
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Add iOS targets
        run: |
          rustup target add aarch64-apple-ios
          rustup target add aarch64-apple-ios-sim

      - name: Install Dioxus CLI
        run: cargo install dioxus-cli

      - name: Build iOS
        run: dx build --ios --release

      - name: Upload IPA
        uses: actions/upload-artifact@v4
        with:
          name: ios-ipa
          path: target/dx/*/bundle/ios/*.ipa
```

## Additional Resources

- **Dioxus Documentation**: https://dioxuslabs.com/
- **iOS Developer Guide**: https://developer.apple.com/ios/
- **Xcode Documentation**: https://developer.apple.com/documentation/xcode
- **Project Issues**: https://github.com/doogie-bigmack/cube-solver/issues

## Status

✅ **R7.6 iOS Build - IMPLEMENTED**

- All iOS targets installed
- Xcode integration configured
- Build scripts created
- Documentation complete
- Test suite passing
- Ready for IPA generation

---

**Last Updated**: 2026-01-12
**iOS Version Support**: iOS 15+
**Rust Version**: 1.85+
**Xcode Version**: 14+
**Dioxus Version**: Latest stable
