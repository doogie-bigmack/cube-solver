# Build script for Windows desktop binary
#
# This script builds the Rubik's Cube Solver for Windows (Windows 10/11)
#
# Requirements:
# - Rust 1.75+ with cargo
# - Visual Studio Build Tools 2019+ (with C++ support)
# - Or: MSVC toolchain via rustup
#
# Usage:
#   .\build-windows.ps1              # Build release binary
#   .\build-windows.ps1 -Docker      # Build in Docker container (from macOS/Linux)

param(
    [switch]$Docker
)

Write-Host "🦀 Rubik's Cube Solver - Windows Build Script" -ForegroundColor Cyan
Write-Host "=============================================" -ForegroundColor Cyan
Write-Host ""

# Check if running in Docker mode
if ($Docker) {
    Write-Host "🐳 Building in Docker container..." -ForegroundColor Yellow
    Write-Host "Note: This uses cross-compilation from Linux to Windows" -ForegroundColor Yellow
    Write-Host ""

    # Use cross-compilation container
    docker run --rm `
        -v "${PWD}:/app" `
        -w /app `
        rust:1.75-slim-bookworm `
        bash -c @"
            set -e
            echo '📦 Installing cross-compilation dependencies...'
            apt-get update -qq
            apt-get install -y -qq mingw-w64 > /dev/null 2>&1

            echo '🔧 Adding Windows target...'
            rustup target add x86_64-pc-windows-gnu

            echo '🔨 Building release binary for Windows...'
            cargo build --release --target x86_64-pc-windows-gnu --features desktop

            echo '✅ Build complete!'
            echo 'Binary location: target/x86_64-pc-windows-gnu/release/rubiks-cube-solver.exe'
            ls -lh target/x86_64-pc-windows-gnu/release/rubiks-cube-solver.exe 2>/dev/null || echo 'Binary built successfully'
"@

    exit $LASTEXITCODE
}

# Native Windows build
Write-Host "🔨 Building natively for Windows..." -ForegroundColor Green
Write-Host ""

# Check if cargo is available
try {
    $null = Get-Command cargo -ErrorAction Stop
} catch {
    Write-Host "❌ Error: cargo not found" -ForegroundColor Red
    Write-Host "   Install Rust from: https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

# Check for MSVC toolchain
$rustcVersion = rustc --version 2>&1
if ($rustcVersion -notmatch "msvc") {
    Write-Host "⚠️  Warning: MSVC toolchain not detected" -ForegroundColor Yellow
    Write-Host "   Install with: rustup toolchain install stable-x86_64-pc-windows-msvc" -ForegroundColor Yellow
    Write-Host "   Then set default: rustup default stable-x86_64-pc-windows-msvc" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "   Continuing with current toolchain..." -ForegroundColor Yellow
    Write-Host ""
}

# Build the release binary
Write-Host "📦 Building release binary with desktop features..." -ForegroundColor Cyan
cargo build --release --features desktop

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host ""
Write-Host "✅ Build successful!" -ForegroundColor Green
Write-Host "📍 Binary location: target\release\rubiks-cube-solver.exe" -ForegroundColor Cyan

# Get binary size
$binaryPath = "target\release\rubiks-cube-solver.exe"
if (Test-Path $binaryPath) {
    $size = (Get-Item $binaryPath).Length
    $sizeMB = [math]::Round($size / 1MB, 2)
    Write-Host "📊 Binary size: $sizeMB MB" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "To run: .\target\release\rubiks-cube-solver.exe" -ForegroundColor Yellow
Write-Host ""
