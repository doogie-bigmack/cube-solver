# Android Build Instructions

## Prerequisites

1. **Java Development Kit (JDK)**
   ```bash
   brew install --cask temurin
   ```
   Or download from: https://adoptium.net/

2. **Android Command Line Tools**
   ```bash
   brew install --cask android-commandlinetools
   ```

3. **Android SDK and NDK**
   ```bash
   export ANDROID_HOME=/opt/homebrew/share/android-commandlinetools
   sdkmanager --sdk_root=$ANDROID_HOME "platform-tools" "platforms;android-34" "build-tools;34.0.0" "ndk;27.0.12077973"
   sdkmanager --sdk_root=$ANDROID_HOME --licenses
   ```

4. **Rust Android Targets**
   ```bash
   rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
   ```

## Environment Variables

Add to your `~/.zshrc` or `~/.bashrc`:

```bash
export ANDROID_HOME=/opt/homebrew/share/android-commandlinetools
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/27.0.12077973
export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools
```

## Building for Android

### Debug Build
```bash
source ~/.cargo/env
dx build --android
```

### Release Build
```bash
source ~/.cargo/env
dx build --android --release
```

### Create APK Bundle
```bash
source ~/.cargo/env
dx bundle --platform android --release
```

The APK will be located in `target/dx/rubiks-cube-solver/bundle/android/`

## Testing on Device/Emulator

### Create Android Emulator
```bash
sdkmanager "system-images;android-34;google_apis;arm64-v8a"
avdmanager create avd -n pixel_5 -k "system-images;android-34;google_apis;arm64-v8a"
```

### Run Emulator
```bash
emulator -avd pixel_5
```

### Install APK
```bash
adb install target/dx/rubiks-cube-solver/bundle/android/rubiks-cube-solver.apk
```

## Troubleshooting

### "ANDROID_NDK_HOME not set"
Make sure you've installed the NDK and set the environment variable:
```bash
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/27.0.12077973
```

### "Java not found"
Install Java JDK using brew or download from Adoptium.

### Build Errors
- Ensure all Rust targets are installed
- Check that NDK version matches the one in ANDROID_NDK_HOME
- Try cleaning the build: `cargo clean && dx build --android`

## Requirements

- Android 10+ (API level 29+)
- ARMv7 or ARM64 architecture
- Minimum 100MB free space
- OpenGL ES 3.0 support (for 3D rendering)
