# Rubik's Cube Solver - Test Plan

## Overview

This document outlines the comprehensive testing strategy for the Rubik's Cube Solver & Tutorial application built with Dioxus (Rust).

## Test Categories

### 1. Unit Tests

Located in `tests/` directory, run with `cargo test`.

#### Cube State Tests (`tests/cube_tests.rs`)

| Test ID | Description | Requirement | Priority |
|---------|-------------|-------------|----------|
| cube_001 | Create 2x2 cube in solved state | R1.1 | High |
| cube_002 | Create 3x3 cube in solved state | R1.1 | High |
| cube_003 | Create 5x5 cube in solved state | R1.1 | High |
| cube_004 | Create 10x10 cube in solved state | R1.1 | Medium |
| cube_005 | Create 20x20 cube in solved state | R1.1 | Medium |
| cube_006 | R move rotates face correctly | R1.2 | High |
| cube_007 | L move rotates face correctly | R1.2 | High |
| cube_008 | U move rotates face correctly | R1.2 | High |
| cube_009 | D move rotates face correctly | R1.2 | High |
| cube_010 | F move rotates face correctly | R1.2 | High |
| cube_011 | B move rotates face correctly | R1.2 | High |
| cube_012 | R' (inverse) move works | R1.2 | High |
| cube_013 | R2 (double) move works | R1.2 | High |
| cube_014 | Rw (wide) move on 4x4 | R1.3 | High |
| cube_015 | Lw (wide) move on 4x4 | R1.3 | High |
| cube_016 | 3Rw (3-wide) move on 5x5 | R1.3 | Medium |
| cube_017 | M slice move on 3x3 | R1.4 | High |
| cube_018 | E slice move on 3x3 | R1.4 | High |
| cube_019 | S slice move on 3x3 | R1.4 | High |
| cube_020 | x rotation | R1.5 | High |
| cube_021 | y rotation | R1.5 | High |
| cube_022 | z rotation | R1.5 | High |
| cube_023 | Sexy move (R U R' U') 6x returns to solved | R1.2 | High |
| cube_024 | T-perm returns to solved after 2x | R1.2 | Medium |
| cube_025 | Adjacent face edges update on R move | R1.2 | High |
| cube_026 | Adjacent face edges update on U move | R1.2 | High |

#### Notation Parser Tests (`tests/notation_tests.rs`)

| Test ID | Description | Requirement | Priority |
|---------|-------------|-------------|----------|
| nota_001 | Parse single move "R" | R1.6 | High |
| nota_002 | Parse inverse "R'" | R1.6 | High |
| nota_003 | Parse double "R2" | R1.6 | High |
| nota_004 | Parse wide "Rw" | R1.6 | High |
| nota_005 | Parse wide with depth "3Rw" | R1.6 | Medium |
| nota_006 | Parse algorithm "R U R' U'" | R1.6 | High |
| nota_007 | Parse with extra spaces "R  U  R'" | R1.6 | Medium |
| nota_008 | Parse lowercase "r u r' u'" | R1.6 | Medium |
| nota_009 | Invalid notation "X" returns error | R1.6 | High |
| nota_010 | Empty string returns empty vec | R1.6 | High |
| nota_011 | Parse slice moves "M E S" | R1.6 | High |
| nota_012 | Parse rotations "x y z" | R1.6 | High |

#### Validation Tests (`tests/validation_tests.rs`)

| Test ID | Description | Requirement | Priority |
|---------|-------------|-------------|----------|
| valid_001 | Solved 3x3 cube is valid | R1.7 | High |
| valid_002 | Scrambled 3x3 cube is valid | R1.7 | High |
| valid_003 | Wrong color count (10 whites) is invalid | R1.7 | High |
| valid_004 | Twisted single corner is invalid | R1.7 | High |
| valid_005 | Flipped single edge is invalid | R1.7 | High |
| valid_006 | Two swapped edges is invalid | R1.7 | High |
| valid_007 | Two swapped corners is invalid | R1.7 | High |
| valid_008 | Valid 2x2 cube passes | R1.7 | High |
| valid_009 | Valid 4x4 cube passes | R1.7 | Medium |

#### Scramble Tests (`tests/scramble_tests.rs`)

| Test ID | Description | Requirement | Priority |
|---------|-------------|-------------|----------|
| scram_001 | Generate 20-move scramble | R1.8 | High |
| scram_002 | Scramble avoids R R sequence | R1.8 | High |
| scram_003 | Scramble avoids R R' sequence | R1.8 | High |
| scram_004 | Scramble is random (100 scrambles unique) | R1.8 | Medium |
| scram_005 | Scramble returns valid cube state | R1.8 | High |

#### Serialization Tests (`tests/serialization_tests.rs`)

| Test ID | Description | Requirement | Priority |
|---------|-------------|-------------|----------|
| serial_001 | Serialize solved cube to JSON | R1.9 | High |
| serial_002 | Deserialize JSON to solved cube | R1.9 | High |
| serial_003 | Round-trip scrambled cube | R1.9 | High |
| serial_004 | Handle invalid JSON gracefully | R1.9 | Medium |

#### Solver Tests (`tests/solver_tests.rs`)

| Test ID | Description | Requirement | Priority |
|---------|-------------|-------------|----------|
| solv_001 | Solve 2x2 from simple scramble | R5.1 | High |
| solv_002 | Solve 2x2 from complex scramble | R5.1 | High |
| solv_003 | 2x2 solution under 1 second | R5.1 | High |
| solv_004 | Solve 3x3 from simple scramble | R5.2 | High |
| solv_005 | Solve 3x3 from 20-move scramble | R5.2 | High |
| solv_006 | 3x3 solution <=20 moves | R5.2 | High |
| solv_007 | 3x3 solve under 2 seconds | R5.2 | High |
| solv_008 | Solve 4x4 centers correctly | R5.3 | High |
| solv_009 | Solve 4x4 edges correctly | R5.4 | High |
| solv_010 | Handle 4x4 OLL parity | R5.5 | High |
| solv_011 | Handle 4x4 PLL parity | R5.5 | High |
| solv_012 | Solve 5x5 from scramble | R5.3, R5.4 | Medium |
| solv_013 | Solve 7x7 from scramble | R5.3, R5.4 | Medium |
| solv_014 | Solve 10x10 from scramble | R5.3, R5.4 | Low |

#### Color Detection Tests (`tests/color_detect_tests.rs`)

| Test ID | Description | Requirement | Priority |
|---------|-------------|-------------|----------|
| color_001 | Detect white (RGB 255,255,255) | R4.3 | High |
| color_002 | Detect yellow (RGB 255,255,0) | R4.3 | High |
| color_003 | Detect red (RGB 255,0,0) | R4.3 | High |
| color_004 | Detect orange (RGB 255,165,0) | R4.3 | High |
| color_005 | Detect blue (RGB 0,0,255) | R4.3 | High |
| color_006 | Detect green (RGB 0,255,0) | R4.3 | High |
| color_007 | Detect off-white (RGB 240,240,240) | R4.6 | Medium |
| color_008 | Detect dark red in shadow (RGB 180,0,0) | R4.6 | Medium |
| color_009 | Distinguish red from orange | R4.3 | High |
| color_010 | 90%+ accuracy on test image set | R4.3 | High |

### 2. Integration Tests

#### Solve Workflow Tests (`tests/integration_tests.rs`)

| Test ID | Description | Requirements | Priority |
|---------|-------------|--------------|----------|
| int_001 | Create cube -> scramble -> solve -> verify solved | R1.1, R1.8, R5.2 | High |
| int_002 | Manual input -> validate -> solve | R3.1-R3.5, R5.2 | High |
| int_003 | Load saved state -> solve | R1.9, R5.2 | Medium |
| int_004 | Tutorial lesson progression | R6.1-R6.9 | Medium |
| int_005 | Camera scan -> validate -> solve | R4.1-R4.7, R5.2 | Medium |
| int_006 | Undo/redo through multiple changes | R3.7 | Medium |

### 3. Platform Tests

Run on each platform to verify builds work correctly.

#### Web (WASM) Tests

| Test ID | Browser | Description | Requirement | Priority |
|---------|---------|-------------|-------------|----------|
| web_001 | Chrome 120+ | App loads and renders | R7.1 | High |
| web_002 | Chrome 120+ | 3D cube rotates smoothly | R7.1, R2.3 | High |
| web_003 | Chrome 120+ | Solve button works | R7.1, R5.2 | High |
| web_004 | Firefox 120+ | App loads and renders | R7.1 | High |
| web_005 | Firefox 120+ | WebGPU/WebGL fallback works | R7.1 | Medium |
| web_006 | Safari 17+ | App loads and renders | R7.1 | High |
| web_007 | Safari 17+ | Touch controls work | R7.1, R7.7 | High |
| web_008 | Edge 120+ | App loads and renders | R7.1 | Medium |
| web_009 | Chrome Mobile (Android) | Touch controls work | R7.1, R7.7 | High |
| web_010 | Safari iOS | Touch controls work | R7.1, R7.7 | High |
| web_011 | Chrome | Camera permission request | R7.1, R4.1 | Medium |
| web_012 | All | Bundle size < 5MB | R7.1 | High |
| web_013 | All | Load time < 3 seconds | R7.1 | High |

#### Desktop Tests

| Test ID | Platform | Description | Requirement | Priority |
|---------|----------|-------------|-------------|----------|
| desk_001 | macOS 12+ (Intel) | App launches and runs | R7.2 | High |
| desk_002 | macOS 12+ (Apple Silicon) | App launches and runs | R7.2 | High |
| desk_003 | macOS | Camera access works | R7.2, R4.1 | Medium |
| desk_004 | macOS | File save/load works | R7.2, R1.9 | Medium |
| desk_005 | Windows 10 | App launches and runs | R7.3 | High |
| desk_006 | Windows 11 | App launches and runs | R7.3 | High |
| desk_007 | Windows | Camera access works | R7.3, R4.1 | Medium |
| desk_008 | Windows | File save/load works | R7.3, R1.9 | Medium |
| desk_009 | Ubuntu 22.04 | App launches and runs | R7.4 | High |
| desk_010 | Ubuntu 24.04 | App launches and runs | R7.4 | Medium |
| desk_011 | Ubuntu | Camera access works | R7.4, R4.1 | Medium |
| desk_012 | Fedora 39 | App launches and runs | R7.4 | Low |

#### Mobile Tests

| Test ID | Platform | Description | Requirement | Priority |
|---------|----------|-------------|-------------|----------|
| mob_001 | Android 10 | App installs and runs | R7.5 | High |
| mob_002 | Android 13 | App installs and runs | R7.5 | High |
| mob_003 | Android 14 | App installs and runs | R7.5 | High |
| mob_004 | Android | Touch rotate cube | R7.5, R7.7 | High |
| mob_005 | Android | Pinch zoom works | R7.5, R2.4 | High |
| mob_006 | Android | Camera scanning works | R7.5, R4.1 | Medium |
| mob_007 | iOS 15 | App installs and runs | R7.6 | High |
| mob_008 | iOS 17 | App installs and runs | R7.6 | High |
| mob_009 | iOS | Touch rotate cube | R7.6, R7.7 | High |
| mob_010 | iOS | Pinch zoom works | R7.6, R2.4 | High |
| mob_011 | iOS | Camera scanning works | R7.6, R4.1 | Medium |
| mob_012 | iPad | Tablet layout correct | R7.6, R7.8 | Medium |

### 4. Performance Tests

| Test ID | Description | Target | Requirement | Priority |
|---------|-------------|--------|-------------|----------|
| perf_001 | 3x3 cube renders at 60fps | >= 60fps | R2.1 | High |
| perf_002 | 5x5 cube renders at 60fps | >= 60fps | R2.1, R2.2 | High |
| perf_003 | 10x10 cube renders at 60fps | >= 60fps | R2.1, R2.2 | Medium |
| perf_004 | 20x20 cube renders at 30fps | >= 30fps | R2.1, R2.2 | Medium |
| perf_005 | Face rotation animation smooth | 60fps during animation | R2.5 | High |
| perf_006 | 3x3 solve time | < 2 seconds | R5.2 | High |
| perf_007 | 2x2 solve time | < 1 second | R5.1 | High |
| perf_008 | 4x4 solve time | < 10 seconds | R5.3-R5.5 | Medium |
| perf_009 | 5x5 solve time | < 30 seconds | R5.3-R5.5 | Medium |
| perf_010 | Web bundle size | < 5MB | R7.1 | High |
| perf_011 | Desktop binary size | < 50MB | R7.2-R7.4 | Medium |
| perf_012 | Initial load time (web) | < 3 seconds | R7.1 | High |
| perf_013 | Memory usage (3x3) | < 100MB | All | Medium |
| perf_014 | Memory usage (20x20) | < 500MB | All | Low |

### 5. Accessibility Tests

| Test ID | Description | Requirement | Priority |
|---------|-------------|-------------|----------|
| a11y_001 | All touch targets >= 44px | R6.12 | High |
| a11y_002 | Color contrast WCAG AA (4.5:1) | R6.12 | High |
| a11y_003 | Keyboard navigation (web/desktop) | R6.12 | Medium |
| a11y_004 | Screen reader labels (web) | R6.12 | Medium |
| a11y_005 | Focus indicators visible | R6.12 | Medium |
| a11y_006 | No seizure-inducing animations | R6.12 | High |

### 6. Responsive Layout Tests

| Test ID | Screen Size | Description | Requirement | Priority |
|---------|-------------|-------------|-------------|----------|
| resp_001 | 320px width | UI readable, no overflow | R7.8 | High |
| resp_002 | 375px width | iPhone SE layout correct | R7.8 | High |
| resp_003 | 390px width | iPhone 14 layout correct | R7.8 | High |
| resp_004 | 768px width | iPad portrait layout | R7.8 | High |
| resp_005 | 1024px width | iPad landscape / small desktop | R7.8 | High |
| resp_006 | 1440px width | Standard desktop | R7.8 | High |
| resp_007 | 1920px width | Large desktop | R7.8 | Medium |
| resp_008 | 2560px width | 4K monitor | R7.8 | Low |
| resp_009 | Landscape phone | Layout adapts | R7.8 | Medium |
| resp_010 | Portrait tablet | Layout adapts | R7.8 | Medium |

### 7. Tutorial Tests

| Test ID | Description | Requirement | Priority |
|---------|-------------|-------------|----------|
| tut_001 | Notation lesson loads | R6.1 | High |
| tut_002 | Notation lesson interactive demo works | R6.1 | High |
| tut_003 | Colors lesson quiz works | R6.2 | Medium |
| tut_004 | Cross tutorial step-by-step | R6.3 | High |
| tut_005 | First layer corners tutorial | R6.4 | High |
| tut_006 | Second layer tutorial | R6.5 | High |
| tut_007 | OLL tutorial pattern recognition | R6.6 | Medium |
| tut_008 | PLL tutorial | R6.7 | Medium |
| tut_009 | 2x2 tutorial complete flow | R6.8 | High |
| tut_010 | 4x4 tutorial complete flow | R6.9 | Medium |
| tut_011 | Practice mode generates cases | R6.10 | High |
| tut_012 | Progress saves to storage | R6.11 | High |
| tut_013 | Progress loads on restart | R6.11 | High |

---

## Test Execution

### Running Unit Tests
```bash
cargo test
```

### Running Tests with Output
```bash
cargo test -- --nocapture
```

### Running Specific Test
```bash
cargo test cube_001
```

### Running Tests by Category
```bash
cargo test cube_      # All cube tests
cargo test solver_    # All solver tests
cargo test valid_     # All validation tests
```

### Running with Coverage
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Running Benchmarks
```bash
cargo bench
```

---

## Platform Testing Checklist

Before each release, manually verify on:

### Web
- [ ] Chrome (latest) - Windows
- [ ] Chrome (latest) - macOS
- [ ] Firefox (latest) - Windows
- [ ] Firefox (latest) - macOS
- [ ] Safari (latest) - macOS
- [ ] Safari - iOS 17
- [ ] Chrome - Android 14
- [ ] Edge (latest) - Windows

### Desktop
- [ ] macOS 12 Monterey (Intel)
- [ ] macOS 14 Sonoma (Apple Silicon)
- [ ] Windows 10
- [ ] Windows 11
- [ ] Ubuntu 22.04
- [ ] Ubuntu 24.04

### Mobile
- [ ] Android 10 device
- [ ] Android 14 device
- [ ] iOS 15 device
- [ ] iOS 17 device
- [ ] iPad (any iOS)

---

## Test Data

### Standard Scrambles for Testing

```
2x2 Easy:    R U R' U R U2 R'
2x2 Medium:  R2 U F2 U' R U' R U F2 R
3x3 Easy:    R U R' U'
3x3 Medium:  R U R' U' R' F R2 U' R' U' R U R' F'
3x3 Hard:    B2 L' F2 D2 R' D2 R2 F2 R' B2 U2 F' D' L U B' F U2 R' D2
4x4:         Rw U2 Rw' U2 Rw U2 Rw' U2 Uw2 Rw U2 Rw'
```

### Known Solution Cases

| Scramble | Expected Solution |
|----------|-------------------|
| R | R' |
| R U R' U' | (R U R' U')5 or optimal |
| U | U' |
| R2 | R2 |

### Superflip (20 moves optimal)
```
U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2
```

---

## CI/CD Integration

### GitHub Actions Workflow

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install cargo-tarpaulin
      - run: cargo tarpaulin --out Xml
      - uses: codecov/codecov-action@v3
```

---

## Bug Tracking

All test failures should be logged as GitHub issues with:
- Test ID that failed
- Platform/browser
- Steps to reproduce
- Expected vs actual result
- Screenshots/logs if applicable

---

## Glossary

| Term | Definition |
|------|------------|
| OLL | Orient Last Layer - step where last layer pieces are rotated to correct orientation |
| PLL | Permute Last Layer - step where last layer pieces are moved to correct positions |
| F2L | First Two Layers - method of solving first two layers simultaneously |
| Parity | Special case in 4x4+ cubes requiring extra algorithms |
| Sexy Move | R U R' U' - common beginner algorithm |
| Kociemba | Two-phase algorithm for optimal 3x3 solving |
