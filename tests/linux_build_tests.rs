// Integration tests for R7.4 - Desktop Linux Build
//
// These tests verify that the Rubik's Cube Solver core library works correctly
// and is compatible with Linux deployment. The core library is platform-agnostic
// and these tests can run on any platform to verify Linux compatibility.
//
// For full desktop UI testing on Linux, use the build-linux.sh script to build
// the desktop application with GTK/WebKit2GTK dependencies.

use rubiks_cube_solver::cube::*;
use rubiks_cube_solver::solver::*;
use std::path::Path;

#[test]
fn test_linux_build_001_core_library_works() {
    // Verify core cube functionality works (platform-agnostic)
    let cube = Cube::new(3);
    assert_eq!(cube.size(), 3);
    assert!(cube.is_solved());
}

#[test]
fn test_linux_build_002_moves_work() {
    // Verify move system works (platform-agnostic)
    let mut cube = Cube::new(3);
    cube.apply_move(Move::R);
    assert!(!cube.is_solved());
    cube.apply_move(Move::RPrime);
    assert!(cube.is_solved());
}

#[test]
fn test_linux_build_003_solver_works() {
    // Verify 2x2 solver works (platform-agnostic)
    let mut cube = Cube::new(2);
    cube.apply_moves(&[Move::R, Move::U, Move::R, Move::U]);

    let solution = solve_2x2(&cube);
    assert!(solution.is_ok());

    let solution = solution.unwrap();
    assert!(!solution.moves.is_empty());
}

#[test]
fn test_linux_build_004_serialization_works() {
    // Verify serialization works (platform-agnostic)
    let cube = Cube::new(3);
    let json = cube.to_json();
    assert!(json.is_ok());

    let json_str = json.unwrap();
    let restored = Cube::from_json(&json_str);
    assert!(restored.is_ok());

    let restored_cube = restored.unwrap();
    assert_eq!(restored_cube.size(), 3);
    assert!(restored_cube.is_solved());
}

#[test]
fn test_linux_build_005_validation_works() {
    // Verify validation works (platform-agnostic)
    let cube = Cube::new(3);
    let result = cube.validate();
    assert!(result.is_ok(), "Solved cube should be valid");
}

#[test]
fn test_linux_build_006_scramble_works() {
    // Verify scramble generation works (platform-agnostic)
    use rubiks_cube_solver::cube::scramble::*;

    let config = ScrambleConfig::new(20);
    let scramble = generate_scramble(&Cube::new(3), config);
    assert_eq!(scramble.moves.len(), 20);
    assert!(!scramble.cube.is_solved());
}

#[test]
fn test_linux_build_007_notation_parser_works() {
    // Verify notation parser works (platform-agnostic)
    use rubiks_cube_solver::cube::notation::*;

    let result = parse_algorithm("R U R' U'");
    assert!(result.is_ok());

    let moves = result.unwrap();
    assert_eq!(moves.len(), 4);
}

#[test]
fn test_linux_build_008_all_cube_sizes_work() {
    // Verify all cube sizes from 2x2 to 20x20 work
    for size in 2..=20 {
        let cube = Cube::new(size);
        assert_eq!(cube.size(), size);
        assert!(cube.is_solved(), "Size {} cube should start solved", size);
    }
}

#[test]
fn test_linux_build_009_moves_preserve_cube_validity() {
    // Verify moves preserve cube validity
    let mut cube = Cube::new(3);
    let moves = vec![
        Move::R,
        Move::U,
        Move::F,
        Move::L,
        Move::D,
        Move::B,
    ];

    for m in moves {
        cube.apply_move(m);
        assert!(cube.validate().is_ok(), "Cube should remain valid after move {:?}", m);
    }
}

#[test]
fn test_linux_build_010_library_version_info() {
    // Verify we can get library information
    // This tests that the library is properly structured for deployment

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const NAME: &str = env!("CARGO_PKG_NAME");

    assert_eq!(VERSION, "0.1.0");
    assert_eq!(NAME, "rubiks-cube-solver");
}

#[test]
fn test_linux_build_011_build_script_exists() {
    // Verify that the Linux build script exists
    let build_script = Path::new("build-linux.sh");
    assert!(build_script.exists(), "build-linux.sh should exist for Linux builds");
}

#[test]
fn test_linux_build_012_cargo_configuration() {
    // Verify Cargo.toml is configured for cross-platform builds
    let cargo_toml = Path::new("Cargo.toml");
    assert!(cargo_toml.exists(), "Cargo.toml should exist");

    // Read and verify it contains desktop features
    let content = std::fs::read_to_string(cargo_toml).expect("Should read Cargo.toml");
    assert!(content.contains("desktop"), "Cargo.toml should define desktop feature");
    assert!(content.contains("[features]"), "Cargo.toml should have features section");
}

#[test]
fn test_linux_build_013_platform_agnostic_core() {
    // The core library should work on any platform
    // This test verifies no panics occur during basic operations

    let mut cube = Cube::new(3);

    // Apply a sequence of moves (Sexy move algorithm)
    let algorithm = "R U R' U' R' F R2 U' R' U' R U R' F'";
    use rubiks_cube_solver::cube::notation::parse_algorithm;

    let moves = parse_algorithm(algorithm).expect("Should parse algorithm");
    for parsed_move in moves {
        match parsed_move {
            rubiks_cube_solver::cube::notation::ParsedMove::Basic(m) => cube.apply_move(m),
            rubiks_cube_solver::cube::notation::ParsedMove::Wide(_) => {}, // Skip wide moves for 3x3
        }
    }

    // Verify cube is still valid
    assert!(cube.validate().is_ok());
}

#[test]
fn test_linux_build_014_concurrent_operations() {
    // Verify the library works with concurrent operations (important for Linux multi-threading)
    use std::sync::Arc;
    use std::thread;

    let cube = Arc::new(Cube::new(3));
    let handles: Vec<_> = (0..4)
        .map(|i| {
            let cube_clone = Arc::clone(&cube);
            thread::spawn(move || {
                // Each thread validates the cube
                assert!(cube_clone.validate().is_ok(), "Thread {} validation failed", i);
                assert_eq!(cube_clone.size(), 3);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

#[test]
fn test_linux_build_015_large_operations() {
    // Verify the library handles large operations (important for Linux server deployments)
    let mut cube = Cube::new(20); // Largest supported cube

    // Apply a long sequence of moves
    for _ in 0..100 {
        cube.apply_move(Move::R);
        cube.apply_move(Move::U);
    }

    // Verify cube is still valid
    assert!(cube.validate().is_ok());
    assert_eq!(cube.size(), 20);
}

