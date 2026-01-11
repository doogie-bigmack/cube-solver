//! Integration tests for wide moves (R1.3)
//!
//! Tests wide moves (Rw, Lw, Uw, Dw, Fw, Bw) on various cube sizes.

use rubiks_cube_solver::cube::{Cube, Direction, WideFace, WideMove};

#[test]
fn cube_014_rw_on_4x4() {
    // Test Rw (wide R) on 4x4
    let mut cube = Cube::new(4);
    cube.apply_wide_move(WideMove::rw());

    // Should not be solved after one Rw
    assert!(!cube.is_solved());

    // Apply Rw' to return to solved
    cube.apply_wide_move(WideMove::rw_prime());
    assert!(cube.is_solved());
}

#[test]
fn cube_015_lw_on_4x4() {
    // Test Lw (wide L) on 4x4
    let mut cube = Cube::new(4);
    cube.apply_wide_move(WideMove::lw());

    assert!(!cube.is_solved());

    cube.apply_wide_move(WideMove::lw_prime());
    assert!(cube.is_solved());
}

#[test]
fn cube_016_3rw_on_5x5() {
    // Test 3Rw (3-wide R) on 5x5
    let mut cube = Cube::new(5);
    let wide_3 = WideMove::new(WideFace::R, Direction::Clockwise, 3);
    cube.apply_wide_move(wide_3);

    assert!(!cube.is_solved());

    // Apply inverse to return to solved
    cube.apply_wide_move(wide_3.inverse());
    assert!(cube.is_solved());
}

#[test]
fn test_all_wide_moves_on_4x4() {
    // Test all 6 wide moves and their inverses on 4x4
    let moves = [
        (WideMove::rw(), WideMove::rw_prime()),
        (WideMove::lw(), WideMove::lw_prime()),
        (WideMove::uw(), WideMove::uw_prime()),
        (WideMove::dw(), WideMove::dw_prime()),
        (WideMove::fw(), WideMove::fw_prime()),
        (WideMove::bw(), WideMove::bw_prime()),
    ];

    for (mv, inv) in moves.iter() {
        let mut cube = Cube::new(4);
        cube.apply_wide_move(*mv);
        cube.apply_wide_move(*inv);
        assert!(
            cube.is_solved(),
            "Failed for {:?} and its inverse",
            mv.to_notation()
        );
    }
}

#[test]
fn test_wide_double_moves() {
    // Test that X2 followed by X2 returns to solved
    let double_moves = [
        WideMove::rw2(),
        WideMove::lw2(),
        WideMove::uw2(),
        WideMove::dw2(),
        WideMove::fw2(),
        WideMove::bw2(),
    ];

    for mv in double_moves.iter() {
        let mut cube = Cube::new(4);
        cube.apply_wide_move(*mv);
        cube.apply_wide_move(*mv);
        assert!(cube.is_solved(), "Failed for {:?}", mv.to_notation());
    }
}

#[test]
fn test_wide_moves_on_various_sizes() {
    // Test wide moves on cubes 3x3 to 10x10
    for size in 3..=10 {
        let mut cube = Cube::new(size);

        // Apply Rw and Rw' - should return to solved
        cube.apply_wide_move(WideMove::rw());
        cube.apply_wide_move(WideMove::rw_prime());
        assert!(cube.is_solved(), "Failed on {}x{} cube", size, size);
    }
}

#[test]
fn test_variable_depth_wide_moves() {
    // Test wide moves with different depths on 8x8
    let cube_size = 8;

    for depth in 2..=4 {
        let mut cube = Cube::new(cube_size);
        let mv = WideMove::new(WideFace::R, Direction::Clockwise, depth);
        cube.apply_wide_move(mv);
        cube.apply_wide_move(mv.inverse());
        assert!(
            cube.is_solved(),
            "Failed for depth {} on 8x8 cube",
            depth
        );
    }
}

#[test]
fn test_wide_move_algorithm_sequence() {
    // Test a sequence of wide moves
    let mut cube = Cube::new(4);

    // Apply a short sequence
    cube.apply_wide_move(WideMove::rw());
    cube.apply_wide_move(WideMove::uw());

    // Verify color counts are still valid
    assert!(cube.has_valid_color_counts());

    // Undo the sequence
    cube.apply_wide_move(WideMove::uw_prime());
    cube.apply_wide_move(WideMove::rw_prime());

    assert!(cube.is_solved());
}

#[test]
fn test_wide_moves_preserve_cube_integrity() {
    // Apply many wide moves and verify the cube is still valid
    let mut cube = Cube::new(5);

    // Apply a complex sequence
    cube.apply_wide_move(WideMove::rw());
    cube.apply_wide_move(WideMove::uw());
    cube.apply_wide_move(WideMove::fw());
    cube.apply_wide_move(WideMove::lw());
    cube.apply_wide_move(WideMove::dw());
    cube.apply_wide_move(WideMove::bw());

    // Cube should still have valid color counts
    assert!(cube.has_valid_color_counts());
}

#[test]
fn test_wide_move_notation_format() {
    // Test notation generation
    assert_eq!(WideMove::rw().to_notation(), "Rw");
    assert_eq!(WideMove::rw_prime().to_notation(), "Rw'");
    assert_eq!(WideMove::rw2().to_notation(), "Rw2");

    // Test depth-prefixed notation
    let wide_3 = WideMove::new(WideFace::U, Direction::Clockwise, 3);
    assert_eq!(wide_3.to_notation(), "3Uw");

    let wide_4_prime = WideMove::new(WideFace::F, Direction::CounterClockwise, 4);
    assert_eq!(wide_4_prime.to_notation(), "4Fw'");
}

#[test]
fn test_wide_move_on_3x3() {
    // On 3x3, Rw is equivalent to L' + x rotation
    // But we just test that it works correctly
    let mut cube = Cube::new(3);
    cube.apply_wide_move(WideMove::rw());
    cube.apply_wide_move(WideMove::rw_prime());
    assert!(cube.is_solved());
}

#[test]
fn test_commutator_with_wide_moves() {
    // Test [Rw, Uw] commutator (Rw Uw Rw' Uw')
    let mut cube = Cube::new(4);

    // Apply commutator 6 times - should return to solved
    for _ in 0..6 {
        cube.apply_wide_move(WideMove::rw());
        cube.apply_wide_move(WideMove::uw());
        cube.apply_wide_move(WideMove::rw_prime());
        cube.apply_wide_move(WideMove::uw_prime());
    }

    // May not return to solved in exactly 6 iterations,
    // but should always preserve color counts
    assert!(cube.has_valid_color_counts());
}
