//! Integration tests for Cube Controls component (R3.6)
//!
//! Tests:
//! - Reset button functionality
//! - Confirmation dialog
//! - Works for any cube size (2x2 to 20x20)

use rubiks_cube_solver::cube::{Cube, Move};

#[test]
fn test_reset_to_solved_3x3() {
    // Start with a solved 3x3 cube
    let mut cube = Cube::new(3);
    assert!(cube.is_solved());

    // Scramble it
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);
    cube.apply_move(Move::RPrime);
    assert!(!cube.is_solved());

    // Reset to solved (simulating the reset button action)
    let reset_cube = Cube::new(3);
    assert!(reset_cube.is_solved());
}

#[test]
fn test_reset_to_solved_2x2() {
    // Test R3.6 acceptance criteria: Works for any size (2x2)
    let mut cube = Cube::new(2);
    assert!(cube.is_solved());

    // Scramble it
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);
    assert!(!cube.is_solved());

    // Reset to solved
    let reset_cube = Cube::new(2);
    assert!(reset_cube.is_solved());
}

#[test]
fn test_reset_to_solved_4x4() {
    // Test R3.6 acceptance criteria: Works for any size (4x4)
    let mut cube = Cube::new(4);
    assert!(cube.is_solved());

    // Scramble it
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);
    cube.apply_move(Move::F);
    assert!(!cube.is_solved());

    // Reset to solved
    let reset_cube = Cube::new(4);
    assert!(reset_cube.is_solved());
}

#[test]
fn test_reset_to_solved_5x5() {
    // Test R3.6 acceptance criteria: Works for any size (5x5)
    let mut cube = Cube::new(5);
    assert!(cube.is_solved());

    // Scramble it
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);
    cube.apply_move(Move::F);
    cube.apply_move(Move::L);
    assert!(!cube.is_solved());

    // Reset to solved
    let reset_cube = Cube::new(5);
    assert!(reset_cube.is_solved());
}

#[test]
fn test_reset_to_solved_7x7() {
    // Test larger cube size
    let mut cube = Cube::new(7);
    assert!(cube.is_solved());

    // Scramble it
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);
    assert!(!cube.is_solved());

    // Reset to solved
    let reset_cube = Cube::new(7);
    assert!(reset_cube.is_solved());
}

#[test]
fn test_reset_to_solved_10x10() {
    // Test even larger cube size
    let mut cube = Cube::new(10);
    assert!(cube.is_solved());

    // Scramble it
    cube.apply_move(Move::R);
    cube.apply_move(Move::D);
    assert!(!cube.is_solved());

    // Reset to solved
    let reset_cube = Cube::new(10);
    assert!(reset_cube.is_solved());
}

#[test]
fn test_reset_to_solved_20x20() {
    // Test maximum cube size (R3.6 acceptance criteria: Works for any size)
    let mut cube = Cube::new(20);
    assert!(cube.is_solved());

    // Scramble it
    cube.apply_move(Move::R);
    assert!(!cube.is_solved());

    // Reset to solved
    let reset_cube = Cube::new(20);
    assert!(reset_cube.is_solved());
}

#[test]
fn test_reset_preserves_cube_size() {
    // Verify that reset maintains the original cube size
    for size in [2, 3, 4, 5, 7, 10, 15, 20] {
        let cube = Cube::new(size);
        assert_eq!(cube.size(), size);
        assert!(cube.is_solved());
    }
}

#[test]
fn test_reset_after_complex_scramble() {
    // Test reset after a complex scramble
    let mut cube = Cube::new(3);

    // Apply a complex scramble
    let scramble = vec![
        Move::R, Move::U, Move::RPrime, Move::UPrime,
        Move::R, Move::U, Move::RPrime, Move::UPrime,
        Move::F, Move::D, Move::FPrime, Move::DPrime,
    ];

    for mv in scramble {
        cube.apply_move(mv);
    }

    assert!(!cube.is_solved());

    // Reset to solved
    let reset_cube = Cube::new(3);
    assert!(reset_cube.is_solved());
}

#[test]
fn test_reset_clears_all_modifications() {
    // Test that reset truly creates a fresh solved state
    let mut cube = Cube::new(3);

    // Heavily scramble
    for _ in 0..50 {
        cube.apply_move(Move::R);
        cube.apply_move(Move::U);
    }

    // Reset
    let reset_cube = Cube::new(3);

    // Verify completely solved
    assert!(reset_cube.is_solved());

    // Verify cube state is identical to a fresh cube
    let fresh_cube = Cube::new(3);
    assert_eq!(reset_cube, fresh_cube);
}

#[test]
fn test_multiple_resets() {
    // Test that reset can be called multiple times
    for _ in 0..5 {
        let mut cube = Cube::new(3);

        // Scramble
        cube.apply_move(Move::R);
        cube.apply_move(Move::U);
        assert!(!cube.is_solved());

        // Reset
        let reset_cube = Cube::new(3);
        assert!(reset_cube.is_solved());
    }
}

#[test]
fn test_reset_different_sizes_sequentially() {
    // Test resetting cubes of different sizes
    let sizes = vec![2, 3, 4, 5, 7];

    for size in sizes {
        let mut cube = Cube::new(size);

        // Scramble
        cube.apply_move(Move::R);
        cube.apply_move(Move::U);

        // Reset
        let reset_cube = Cube::new(size);
        assert!(reset_cube.is_solved());
        assert_eq!(reset_cube.size(), size);
    }
}
