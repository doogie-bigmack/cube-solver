# iOS Build Instructions

## Prerequisites

1. **macOS Required**
   - iOS builds can only be created on macOS
   - macOS 12 (Monterey) or later recommended

2. **Xcode**
   ```bash
   # Install from Mac App Store or:
   xcode-select --install
   ```
   Required version: Xcode 14+ for iOS 15+ support

3. **Rust iOS Targets**
   ```bash
   rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
   ```

4. **Dioxus CLI**
   ```bash
   cargo install dioxus-cli
   ```

5. **iOS Development Certificate** (for device deployment)
   - Apple Developer account required
   - Create certificates in Xcode or Apple Developer portal

## Environment Setup

Ensure Xcode command-line tools are set up:

```bash
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```

## Building for iOS

### Debug Build (Simulator)
```bash
source ~/.cargo/env
dx build --ios
```

### Release Build (Device)
```bash
source ~/.cargo/env
dx build --ios --release
```

### Create IPA Bundle
```bash
source ~/.cargo/env
dx bundle --platform ios --release
```

The IPA will be located in `target/dx/rubiks-cube-solver/bundle/ios/`

## Testing on Simulator

### List Available Simulators
```bash
xcrun simctl list devices
```

### Launch Simulator
```bash
open -a Simulator
```

### Install and Run
```bash
dx build --ios
# The app will automatically install and launch in the simulator
```

## Testing on Physical Device

1. **Connect Device** via USB or WiFi

2. **Trust Computer** on device when prompted

3. **Build and Deploy**
   ```bash
   dx build --ios --device "Your iPhone Name"
   ```

4. **Trust Developer Certificate** on device:
   - Settings → General → VPN & Device Management
   - Trust the developer certificate

## App Store Preparation

### 1. Create App ID
- Log in to Apple Developer portal
- Register bundle identifier: `com.rubiks.cube.solver`

### 2. Generate Certificates and Provisioning Profiles
```bash
# In Xcode:
# - Open Preferences → Accounts
# - Add Apple ID
# - Download certificates
```

### 3. Build for Distribution
```bash
dx bundle --platform ios --release
```

### 4. Validate and Upload
```bash
# Use Xcode or Transporter app
# Validate IPA for App Store requirements
# Upload to App Store Connect
```

## Troubleshooting

### "No provisioning profiles found"
**Solution**: Create a provisioning profile in Xcode:
1. Open any iOS project in Xcode
2. Go to Signing & Capabilities
3. Select your Apple ID and team
4. Xcode will create profiles automatically

### "Untrusted Developer"
**Solution**: On iOS device:
- Settings → General → VPN & Device Management
- Tap developer name and Trust

### "Code signing failed"
**Solution**: In Dioxus.toml, ensure bundle identifier matches your Apple Developer account:
```toml
[bundle]
identifier = "com.rubiks.cube.solver"
```

### Simulator not launching
**Solution**:
```bash
# Reset simulator
xcrun simctl erase all

# Or create new simulator
xcrun simctl create "iPhone 15" "iPhone 15"
```

### Build errors
**Solutions**:
- Update Xcode: `softwareupdate --list`
- Clean build: `cargo clean && dx build --ios`
- Reset iOS targets: `rustup target remove aarch64-apple-ios && rustup target add aarch64-apple-ios`

## Requirements

- iOS 15+ (target version)
- iPhone 6s or newer (device support)
- 100MB+ free space
- Metal support for 3D rendering

## Features on iOS

- ✅ Full 3D cube rendering (Metal/WGPU)
- ✅ Touch gestures (drag, pinch, tap)
- ✅ Camera scanning (ARKit integration possible)
- ✅ Local storage for progress
- ✅ Native performance

## Performance Notes

- **First Build**: 10-20 minutes (downloads dependencies)
- **Incremental Builds**: 1-2 minutes
- **IPA Size**: Approximately 15-25 MB
- **Runtime**: Native ARM64 performance
