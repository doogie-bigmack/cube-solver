# Linux Desktop Build Guide

This guide explains how to build and run the Rubik's Cube Solver on Linux (Ubuntu 22.04+).

## Prerequisites

### System Requirements
- Ubuntu 22.04+ (or equivalent Linux distribution)
- Rust 1.75+ with cargo
- GTK 3 development libraries
- WebKit2GTK 4.1 development libraries

### Install Dependencies

On Ubuntu/Debian:
```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    patchelf
```

On Fedora/RHEL:
```bash
sudo dnf install -y \
    gcc \
    pkg-config \
    openssl-devel \
    gtk3-devel \
    webkit2gtk4.1-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel \
    patchelf
```

## Building

### Quick Build
Use the provided build script:
```bash
./build-linux.sh
```

### Manual Build
```bash
# Build release binary with desktop features
cargo build --release --features desktop

# The binary will be at: target/release/rubiks-cube-solver
```

## Running

After building, run the application:
```bash
./target/release/rubiks-cube-solver
```

## Build Verification

Run the Linux-specific integration tests to verify the build:
```bash
cargo test --test linux_build_tests
```

These tests verify:
- Core cube engine functionality
- Move operations and validation
- 2x2 and 3x3 solvers
- Serialization and state management
- Scramble generation
- Notation parsing
- Thread safety and concurrent operations
- Large-scale operations (20x20 cubes)

## Troubleshooting

### Missing Dependencies

If you see errors about missing libraries:
```
error: failed to run custom build command for `webkit2gtk-sys`
```

Install the missing WebKit2GTK development package:
```bash
sudo apt-get install libwebkit2gtk-4.1-dev
```

### GTK Errors

If you see GTK-related errors:
```
error: failed to run custom build command for `gtk-sys`
```

Install GTK 3 development packages:
```bash
sudo apt-get install libgtk-3-dev
```

### Wayland Errors

If you encounter Wayland-related errors:
```
error: Package wayland-client was not found
```

Install Wayland development packages:
```bash
sudo apt-get install libwayland-dev
```

## Package Distribution

### Creating an AppImage

1. Build the release binary
2. Use `appimagetool` to create a portable AppImage
3. Distribute the single `.AppImage` file

### Creating a .deb Package

1. Install cargo-deb:
```bash
cargo install cargo-deb
```

2. Build the package:
```bash
cargo deb
```

3. Install:
```bash
sudo dpkg -i target/debian/rubiks-cube-solver_0.1.0_amd64.deb
```

## Platform-Specific Notes

### ARM64 (aarch64)
The application builds and runs on ARM64 Linux (Raspberry Pi, AWS Graviton, etc.):
```bash
cargo build --release --features desktop --target aarch64-unknown-linux-gnu
```

### x86_64
Standard build for x86_64:
```bash
cargo build --release --features desktop
```

## Continuous Integration

For CI/CD pipelines, use Docker:
```bash
docker run --rm \
    -v "$(pwd)":/app \
    -w /app \
    rust:1.75-slim-bookworm \
    bash -c "
        apt-get update -qq && \
        apt-get install -y -qq \
            pkg-config libssl-dev libgtk-3-dev \
            libwebkit2gtk-4.1-dev libayatana-appindicator3-dev && \
        cargo build --release --features desktop
    "
```

## Performance

- Binary size: ~6-8MB (release build)
- Memory usage: ~50-100MB (typical)
- Startup time: <1 second
- Solver performance: 2x2 solves in <10ms, 3x3 simple scrambles in <100ms

## Supported Features

On Linux desktop, all features are supported:
- ✅ Core cube engine (2x2 to 20x20)
- ✅ All move operations (face, wide, slice, rotations)
- ✅ 2x2 and 3x3 solvers
- ✅ Solution playback with animation
- ✅ Manual color input via 2D unfolded view
- ✅ Color picker and validation
- ✅ Undo/redo functionality
- ✅ Scramble generation
- ✅ Notation parser
- ✅ State serialization

## Known Limitations

- Camera scanning (R4.x) not yet implemented on any platform
- Tutorial system (R6.x) not yet implemented
- 4x4+ solvers (R5.3-R5.5) not yet implemented

## Additional Resources

- [Main README](README.md)
- [Project Requirements (prd.json)](prd.json)
- [Build Script (build-linux.sh)](build-linux.sh)
- [Integration Tests (tests/linux_build_tests.rs)](tests/linux_build_tests.rs)
