//! Move Tests for R1.2
//!
//! Integration tests for face rotation operations as specified in test-plan.md

use rubiks_cube_solver::cube::{Color, Cube, Move};

/// cube_006: R move rotates face correctly
#[test]
fn cube_006_r_move_rotates_correctly() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::R);

    // After R move, the right face should still be all red (just rotated)
    assert!(cube.right.is_solved());

    // Front right column should now have white (from up)
    assert_eq!(cube.front.get_col(2), vec![Color::White, Color::White, Color::White]);

    // Down right column should now have green (from front)
    assert_eq!(cube.down.get_col(2), vec![Color::Green, Color::Green, Color::Green]);

    // Up right column should now have blue (from back, reversed)
    assert_eq!(cube.up.get_col(2), vec![Color::Blue, Color::Blue, Color::Blue]);
}

/// cube_007: L move rotates face correctly
#[test]
fn cube_007_l_move_rotates_correctly() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::L);

    // Left face should still be all orange (just rotated)
    assert!(cube.left.is_solved());

    // After L move: Up left -> Back right (reversed), Back right -> Down left,
    // Down left -> Front left, Front left -> Up left
    assert_eq!(cube.front.get_col(0), vec![Color::Yellow, Color::Yellow, Color::Yellow]);
    assert_eq!(cube.up.get_col(0), vec![Color::Green, Color::Green, Color::Green]);
}

/// cube_008: U move rotates face correctly
#[test]
fn cube_008_u_move_rotates_correctly() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::U);

    // Up face should still be all white (just rotated)
    assert!(cube.up.is_solved());

    // After U move: Front top -> Right top -> Back top -> Left top -> Front top
    assert_eq!(cube.right.get_row(0), vec![Color::Green, Color::Green, Color::Green]);
    assert_eq!(cube.back.get_row(0), vec![Color::Red, Color::Red, Color::Red]);
    assert_eq!(cube.left.get_row(0), vec![Color::Blue, Color::Blue, Color::Blue]);
    assert_eq!(cube.front.get_row(0), vec![Color::Orange, Color::Orange, Color::Orange]);
}

/// cube_009: D move rotates face correctly
#[test]
fn cube_009_d_move_rotates_correctly() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::D);

    // Down face should still be all yellow (just rotated)
    assert!(cube.down.is_solved());

    // After D move: Front bottom -> Left bottom -> Back bottom -> Right bottom -> Front bottom
    assert_eq!(cube.left.get_row(2), vec![Color::Green, Color::Green, Color::Green]);
    assert_eq!(cube.back.get_row(2), vec![Color::Orange, Color::Orange, Color::Orange]);
    assert_eq!(cube.right.get_row(2), vec![Color::Blue, Color::Blue, Color::Blue]);
    assert_eq!(cube.front.get_row(2), vec![Color::Red, Color::Red, Color::Red]);
}

/// cube_010: F move rotates face correctly
#[test]
fn cube_010_f_move_rotates_correctly() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::F);

    // Front face should still be all green (just rotated)
    assert!(cube.front.is_solved());
}

/// cube_011: B move rotates face correctly
#[test]
fn cube_011_b_move_rotates_correctly() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::B);

    // Back face should still be all blue (just rotated)
    assert!(cube.back.is_solved());
}

/// cube_012: R' (inverse) move works
#[test]
fn cube_012_r_prime_is_inverse() {
    let mut cube = Cube::new(3);

    // Apply R then R'
    cube.apply_move(Move::R);
    cube.apply_move(Move::RPrime);

    // Should be back to solved
    assert!(cube.is_solved());
}

/// cube_013: R2 (double) move works
#[test]
fn cube_013_r2_double_move() {
    let mut cube = Cube::new(3);

    // R2 twice should return to solved (R2 has order 2)
    cube.apply_move(Move::R2);
    cube.apply_move(Move::R2);

    assert!(cube.is_solved());
}

/// cube_023: Sexy move (R U R' U') 6x returns to solved
#[test]
fn cube_023_sexy_move_order() {
    let mut cube = Cube::new(3);

    // The sexy move has order 6
    for _ in 0..6 {
        cube.apply_move(Move::R);
        cube.apply_move(Move::U);
        cube.apply_move(Move::RPrime);
        cube.apply_move(Move::UPrime);
    }

    assert!(cube.is_solved());
}

/// cube_025: Adjacent face edges update on R move
#[test]
fn cube_025_adjacent_edges_r_move() {
    let mut cube = Cube::new(3);

    // Mark the front right column with distinctive colors
    cube.front.set(0, 2, Color::Red);
    cube.front.set(1, 2, Color::Blue);
    cube.front.set(2, 2, Color::Orange);

    cube.apply_move(Move::R);

    // These should now be on the down face's right column
    assert_eq!(cube.down.get(0, 2), Color::Red);
    assert_eq!(cube.down.get(1, 2), Color::Blue);
    assert_eq!(cube.down.get(2, 2), Color::Orange);
}

/// cube_026: Adjacent face edges update on U move
#[test]
fn cube_026_adjacent_edges_u_move() {
    let mut cube = Cube::new(3);

    // Mark the front top row with distinctive colors
    cube.front.set(0, 0, Color::Red);
    cube.front.set(0, 1, Color::Blue);
    cube.front.set(0, 2, Color::Orange);

    cube.apply_move(Move::U);

    // These should now be on the right face's top row
    assert_eq!(cube.right.get(0, 0), Color::Red);
    assert_eq!(cube.right.get(0, 1), Color::Blue);
    assert_eq!(cube.right.get(0, 2), Color::Orange);
}

/// Test all inverse moves return cube to solved
#[test]
fn test_all_inverse_pairs() {
    let move_pairs = [
        (Move::R, Move::RPrime),
        (Move::L, Move::LPrime),
        (Move::U, Move::UPrime),
        (Move::D, Move::DPrime),
        (Move::F, Move::FPrime),
        (Move::B, Move::BPrime),
    ];

    for (mv, inv) in move_pairs.iter() {
        let mut cube = Cube::new(3);
        cube.apply_move(*mv);
        cube.apply_move(*inv);
        assert!(cube.is_solved(), "Move {:?} and {:?} should cancel", mv, inv);
    }
}

/// Test all double moves have order 2
#[test]
fn test_double_move_order() {
    let double_moves = [Move::R2, Move::L2, Move::U2, Move::D2, Move::F2, Move::B2];

    for mv in double_moves.iter() {
        let mut cube = Cube::new(3);
        cube.apply_move(*mv);
        cube.apply_move(*mv);
        assert!(cube.is_solved(), "{:?} applied twice should solve", mv);
    }
}

/// Test moves work on 4x4 cube
#[test]
fn test_4x4_moves() {
    let mut cube = Cube::new(4);

    // Apply all basic moves
    let moves = [Move::R, Move::L, Move::U, Move::D, Move::F, Move::B];
    for mv in moves.iter() {
        cube.apply_move(*mv);
    }

    // Apply all inverse moves (in reverse order for proper cancellation)
    let inverse_moves = [
        Move::BPrime,
        Move::FPrime,
        Move::DPrime,
        Move::UPrime,
        Move::LPrime,
        Move::RPrime,
    ];
    for mv in inverse_moves.iter() {
        cube.apply_move(*mv);
    }

    assert!(cube.is_solved());
}

/// Test moves work on 2x2 cube
#[test]
fn test_2x2_moves() {
    let mut cube = Cube::new(2);

    // Sexy move should still work
    for _ in 0..6 {
        cube.apply_move(Move::R);
        cube.apply_move(Move::U);
        cube.apply_move(Move::RPrime);
        cube.apply_move(Move::UPrime);
    }

    assert!(cube.is_solved());
}

/// Test moves preserve color counts
#[test]
fn test_moves_preserve_color_counts() {
    let mut cube = Cube::new(3);

    // Apply random-ish sequence of moves
    let moves = [
        Move::R,
        Move::U,
        Move::F2,
        Move::LPrime,
        Move::D,
        Move::B2,
        Move::R2,
        Move::UPrime,
    ];

    for mv in moves.iter() {
        cube.apply_move(*mv);
    }

    // Color counts should still be valid
    assert!(cube.has_valid_color_counts());
}

/// Test Sune algorithm has order 6
#[test]
fn test_sune_order() {
    let mut cube = Cube::new(3);

    // Sune: R U R' U R U2 R'
    for _ in 0..6 {
        cube.apply_move(Move::R);
        cube.apply_move(Move::U);
        cube.apply_move(Move::RPrime);
        cube.apply_move(Move::U);
        cube.apply_move(Move::R);
        cube.apply_move(Move::U2);
        cube.apply_move(Move::RPrime);
    }

    assert!(cube.is_solved());
}

/// Test that applying a move sequence and its reverse returns to solved
#[test]
fn test_sequence_and_reverse() {
    let mut cube = Cube::new(3);

    let moves = vec![Move::R, Move::U, Move::RPrime, Move::F, Move::D2, Move::B];

    // Apply moves
    cube.apply_moves(&moves);

    // Apply reverse (inverse of each move in reverse order)
    let reverse: Vec<Move> = moves.iter().rev().map(|m| m.inverse()).collect();
    cube.apply_moves(&reverse);

    assert!(cube.is_solved());
}
