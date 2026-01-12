# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rubik's Cube Solver & Tutorial - a Dioxus (Rust) cross-platform app for kids to learn cube solving (2x2 to 20x20). Targets web, desktop (macOS/Windows/Linux), and mobile (iOS/Android).

## Build & Run Commands

```bash
# Development (desktop)
dx serve

# Development (web)
dx serve --platform web

# Build release
dx build --release

# Run tests
cargo test

# Run single test
cargo test cube_001

# Run test category
cargo test cube_      # All cube tests
cargo test solver_    # All solver tests

# Run with output
cargo test -- --nocapture

# Coverage
cargo tarpaulin --out Html

# Benchmarks
cargo bench
```

## Architecture

```
src/
├── cube/           # Core cube logic (state, moves, notation, validation)
├── solver/         # Solving algorithms (kociemba, reduction, parity)
├── renderer/       # WGPU 3D rendering (mesh, camera, animations)
├── camera/         # Webcam capture and color detection (nokhwa)
├── components/     # Dioxus UI components
├── tutorial/       # Lesson system and practice mode
│   └── lessons/    # Individual tutorials (cross, OLL, PLL, etc.)
└── state/          # App state management (history, progress)
```

## Key Dependencies

- **dioxus** - Cross-platform UI framework
- **wgpu** - 3D rendering
- **nokhwa** - Webcam access (conditional: native vs wasm)
- **glam** - Linear algebra for 3D math

## Requirements Tracking

See `prd.json` for 56 requirements (R1-R7) with `implemented` and `tests_passing` status flags. Update these when completing features.

## Test IDs

Tests use prefixed IDs that map to requirements:
- `cube_*` → R1 (core engine)
- `nota_*` → R1.6 (notation)
- `valid_*` → R1.7 (validation)
- `solv_*` → R5 (solvers)
- `color_*` → R4.3 (color detection)

## Platform-Specific Notes

Camera feature uses conditional compilation:
- Native: `nokhwa` with `input-native`
- WASM: `nokhwa` with `input-jscam`

Enable camera: `cargo build --features camera`

## Docker Deployment

```bash
# Install Docker (macOS) if not present
brew install --cask docker

# Build container
docker build -t rubiks-cube-solver .

# Run container (web)
docker run -p 8080:8080 rubiks-cube-solver

# Or use docker-compose
docker-compose up --build
```

Container serves the WASM web build on port 8080.
