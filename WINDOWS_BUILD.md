# Windows Desktop Build Guide

This guide explains how to build and run the Rubik's Cube Solver on Windows (Windows 10/11).

## Prerequisites

### System Requirements
- Windows 10 or Windows 11
- Rust 1.75+ with cargo
- Visual Studio Build Tools 2019+ (with C++ support) OR
- MinGW-w64 toolchain

### Install Rust

Download and install Rust from [rustup.rs](https://rustup.rs/):

```powershell
# Download and run rustup-init.exe
# Follow the prompts to install Rust
```

The installer will automatically set up the MSVC toolchain if you have Visual Studio Build Tools installed.

### Install Visual Studio Build Tools (Recommended)

Download from: https://visualstudio.microsoft.com/downloads/

Select "Build Tools for Visual Studio 2022" and install with:
- Desktop development with C++
- Windows 10 SDK or Windows 11 SDK

### Alternative: MinGW-w64 Toolchain

If you don't want to install Visual Studio, you can use MinGW:

```powershell
# Install MinGW target
rustup target add x86_64-pc-windows-gnu

# You'll need to install MinGW-w64 separately
# Download from: https://www.mingw-w64.org/downloads/
```

## Building

### Quick Build (PowerShell)

Use the provided build script:

```powershell
.\build-windows.ps1
```

### Manual Build

```powershell
# Build release binary with desktop features
cargo build --release --features desktop

# The binary will be at: target\release\rubiks-cube-solver.exe
```

### Cross-Compilation from macOS/Linux

If you're building on macOS or Linux for Windows:

```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu --features desktop

# Binary will be at: target/x86_64-pc-windows-gnu/release/rubiks-cube-solver.exe
```

Or use the build script with Docker:

```powershell
.\build-windows.ps1 -Docker
```

## Running

After building, run the application:

```powershell
.\target\release\rubiks-cube-solver.exe
```

Or double-click `rubiks-cube-solver.exe` in Windows Explorer.

## Build Verification

Run the Windows-specific integration tests to verify the build:

```powershell
cargo test --test windows_build_tests
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

### Missing MSVC Toolchain

If you see errors about missing linker:
```
error: linker `link.exe` not found
```

Install Visual Studio Build Tools with C++ support, or switch to MinGW:

```powershell
rustup target add x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

### Windows Defender / Antivirus Warnings

Some antivirus software may flag Rust binaries as suspicious. This is a false positive. You may need to:
1. Add an exception for the `target\release` folder
2. Disable real-time scanning temporarily during builds

### Missing DLL Errors

If you see "The code execution cannot proceed because VCRUNTIME140.dll was not found":

Install the Visual C++ Redistributable:
- Download from: https://aka.ms/vs/17/release/vc_redist.x64.exe
- Or install via Visual Studio Build Tools

### Slow Build Times

First build can take 5-10 minutes. Subsequent builds are much faster (seconds to minutes).

To speed up builds:
1. Use `cargo build --release` only for final builds
2. Use `cargo build` (debug mode) during development
3. Consider using `cargo-watch` for auto-rebuilding

## Package Distribution

### Creating an Installer

#### Using Inno Setup

1. Download Inno Setup: https://jrsoftware.org/isdl.php

2. Create an installer script (`installer.iss`):

```iss
[Setup]
AppName=Rubik's Cube Solver
AppVersion=0.1.0
DefaultDirName={autopf}\RubiksCubeSolver
DefaultGroupName=Rubik's Cube Solver
OutputDir=installer
OutputBaseFilename=RubiksCubeSolver-Setup

[Files]
Source: "target\release\rubiks-cube-solver.exe"; DestDir: "{app}"

[Icons]
Name: "{group}\Rubik's Cube Solver"; Filename: "{app}\rubiks-cube-solver.exe"
```

3. Compile with Inno Setup

#### Using WiX Toolset

1. Install WiX: https://wixtoolset.org/

2. Use `cargo-wix`:
```powershell
cargo install cargo-wix
cargo wix
```

### Creating a Portable .zip

For a simple portable distribution:

```powershell
# Build release
cargo build --release --features desktop

# Create zip
Compress-Archive -Path target\release\rubiks-cube-solver.exe -DestinationPath RubiksCubeSolver-Windows-x64.zip
```

## Platform-Specific Notes

### ARM64 (aarch64)

Windows on ARM support:

```powershell
# Add ARM64 target
rustup target add aarch64-pc-windows-msvc

# Build for ARM64
cargo build --release --target aarch64-pc-windows-msvc --features desktop
```

### x86_64

Standard build for x86_64 (most common):

```powershell
cargo build --release --features desktop
```

### 32-bit (x86)

For older 32-bit Windows systems:

```powershell
# Add 32-bit target
rustup target add i686-pc-windows-msvc

# Build for 32-bit
cargo build --release --target i686-pc-windows-msvc --features desktop
```

## Performance

- Binary size: ~6-8MB (release build)
- Memory usage: ~50-100MB (typical)
- Startup time: <1 second
- Solver performance: 2x2 solves in <10ms, 3x3 simple scrambles in <100ms

## Supported Features

On Windows desktop, all features are supported:
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

## Windows-Specific Features

### High DPI Support

The application automatically scales for high-DPI displays (4K, etc.).

### Desktop Integration

- Taskbar integration
- System notifications (future feature)
- File associations for .cube files (future feature)

## Debugging

### Enable Logging

Set environment variable for debug output:

```powershell
$env:RUST_LOG="debug"
.\target\release\rubiks-cube-solver.exe
```

### Performance Profiling

Use Windows Performance Analyzer or:

```powershell
# Install cargo-flamegraph
cargo install flamegraph

# Profile (requires admin)
cargo flamegraph --features desktop
```

## Continuous Integration

For CI/CD pipelines (GitHub Actions, Azure Pipelines, etc.):

```yaml
# Example GitHub Actions workflow
- name: Build Windows
  run: |
    cargo build --release --features desktop

- name: Test Windows
  run: |
    cargo test --test windows_build_tests
```

## Additional Resources

- [Main README](README.md)
- [Project Requirements (prd.json)](prd.json)
- [Build Script (build-windows.ps1)](build-windows.ps1)
- [Integration Tests (tests/windows_build_tests.rs)](tests/windows_build_tests.rs)

## Support

For issues specific to Windows builds:
1. Check the troubleshooting section above
2. Verify all prerequisites are installed
3. Try both MSVC and MinGW toolchains
4. Check Windows version compatibility (requires Windows 10/11)
