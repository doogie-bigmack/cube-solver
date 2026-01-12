//! Integration tests for MoveDisplay component
//!
//! Tests the move explanation and display functionality including:
//! - Basic move explanations
//! - Wide move explanations
//! - Slice move explanations
//! - Rotation explanations
//! - Notation display
//! - Progress tracking

use rubiks_cube_solver::components::{get_move_explanation, get_move_notation};
use rubiks_cube_solver::cube::moves::{Move, WideMove, WideFace, Direction};
use rubiks_cube_solver::cube::ParsedMove;

#[test]
fn test_display_001_r_move_notation() {
    let r_move = ParsedMove::Basic(Move::R);
    let notation = get_move_notation(&r_move);
    assert_eq!(notation, "R");
}

#[test]
fn test_display_002_r_prime_move_notation() {
    let r_prime = ParsedMove::Basic(Move::RPrime);
    let notation = get_move_notation(&r_prime);
    assert_eq!(notation, "R'");
}

#[test]
fn test_display_003_r2_move_notation() {
    let r2 = ParsedMove::Basic(Move::R2);
    let notation = get_move_notation(&r2);
    assert_eq!(notation, "R2");
}

#[test]
fn test_display_004_rw_move_notation() {
    let rw = WideMove {
        face: WideFace::R,
        direction: Direction::Clockwise,
        depth: 2,
    };
    let wide_move = ParsedMove::Wide(rw);
    let notation = get_move_notation(&wide_move);
    assert_eq!(notation, "Rw");
}

#[test]
fn test_display_005_r_move_explanation() {
    let r_move = ParsedMove::Basic(Move::R);
    let explanation = get_move_explanation(&r_move);
    assert_eq!(explanation, "Turn the right face clockwise");
}

#[test]
fn test_display_006_r_prime_explanation() {
    let r_prime = ParsedMove::Basic(Move::RPrime);
    let explanation = get_move_explanation(&r_prime);
    assert_eq!(explanation, "Turn the right face counter-clockwise");
}

#[test]
fn test_display_007_u_move_explanation() {
    let u_move = ParsedMove::Basic(Move::U);
    let explanation = get_move_explanation(&u_move);
    assert_eq!(explanation, "Turn the top face clockwise");
}

#[test]
fn test_display_008_d_move_explanation() {
    let d_move = ParsedMove::Basic(Move::D);
    let explanation = get_move_explanation(&d_move);
    assert_eq!(explanation, "Turn the bottom face clockwise");
}

#[test]
fn test_display_009_f_move_explanation() {
    let f_move = ParsedMove::Basic(Move::F);
    let explanation = get_move_explanation(&f_move);
    assert_eq!(explanation, "Turn the front face clockwise");
}

#[test]
fn test_display_010_b_move_explanation() {
    let b_move = ParsedMove::Basic(Move::B);
    let explanation = get_move_explanation(&b_move);
    assert_eq!(explanation, "Turn the back face clockwise");
}

#[test]
fn test_display_011_l_move_explanation() {
    let l_move = ParsedMove::Basic(Move::L);
    let explanation = get_move_explanation(&l_move);
    assert_eq!(explanation, "Turn the left face clockwise");
}

#[test]
fn test_display_012_m_slice_explanation() {
    let m_move = ParsedMove::Basic(Move::M);
    let explanation = get_move_explanation(&m_move);
    assert!(explanation.contains("middle"));
    assert!(explanation.contains("slice"));
}

#[test]
fn test_display_013_e_slice_explanation() {
    let e_move = ParsedMove::Basic(Move::E);
    let explanation = get_move_explanation(&e_move);
    assert!(explanation.contains("middle"));
    assert!(explanation.contains("horizontal"));
}

#[test]
fn test_display_014_s_slice_explanation() {
    let s_move = ParsedMove::Basic(Move::S);
    let explanation = get_move_explanation(&s_move);
    assert!(explanation.contains("middle"));
    assert!(explanation.contains("front-back"));
}

#[test]
fn test_display_015_x_rotation_explanation() {
    let x_move = ParsedMove::Basic(Move::X);
    let explanation = get_move_explanation(&x_move);
    assert!(explanation.contains("whole cube"));
    assert!(explanation.contains("R"));
}

#[test]
fn test_display_016_y_rotation_explanation() {
    let y_move = ParsedMove::Basic(Move::Y);
    let explanation = get_move_explanation(&y_move);
    assert!(explanation.contains("whole cube"));
    assert!(explanation.contains("U"));
}

#[test]
fn test_display_017_z_rotation_explanation() {
    let z_move = ParsedMove::Basic(Move::Z);
    let explanation = get_move_explanation(&z_move);
    assert!(explanation.contains("whole cube"));
    assert!(explanation.contains("F"));
}

#[test]
fn test_display_018_rw_wide_move_explanation() {
    let rw = WideMove {
        face: WideFace::R,
        direction: Direction::Clockwise,
        depth: 2,
    };
    let wide_move = ParsedMove::Wide(rw);
    let explanation = get_move_explanation(&wide_move);
    assert_eq!(explanation, "Turn two layers of the right side clockwise");
}

#[test]
fn test_display_019_uw_prime_explanation() {
    let uw_prime = WideMove {
        face: WideFace::U,
        direction: Direction::CounterClockwise,
        depth: 2,
    };
    let wide_move = ParsedMove::Wide(uw_prime);
    let explanation = get_move_explanation(&wide_move);
    assert_eq!(explanation, "Turn two layers of the top side counter-clockwise");
}

#[test]
fn test_display_020_3rw_explanation() {
    let three_rw = WideMove {
        face: WideFace::R,
        direction: Direction::Clockwise,
        depth: 3,
    };
    let wide_move = ParsedMove::Wide(three_rw);
    let explanation = get_move_explanation(&wide_move);
    assert_eq!(explanation, "Turn 3 layers of the right side clockwise");
}

#[test]
fn test_display_021_wide_double_explanation() {
    let rw2 = WideMove {
        face: WideFace::R,
        direction: Direction::Double,
        depth: 2,
    };
    let wide_move = ParsedMove::Wide(rw2);
    let explanation = get_move_explanation(&wide_move);
    assert_eq!(explanation, "Turn two layers of the right side 180 degrees");
}

#[test]
fn test_display_022_all_basic_faces() {
    // Test that all 6 basic faces have explanations
    let faces = vec![Move::R, Move::L, Move::U, Move::D, Move::F, Move::B];
    for face_move in faces {
        let parsed = ParsedMove::Basic(face_move);
        let explanation = get_move_explanation(&parsed);
        assert!(!explanation.is_empty());
        assert!(explanation.len() > 10); // Should be a meaningful explanation
    }
}

#[test]
fn test_display_023_all_prime_moves() {
    // Test that all prime moves have explanations
    let primes = vec![Move::RPrime, Move::LPrime, Move::UPrime, Move::DPrime, Move::FPrime, Move::BPrime];
    for prime_move in primes {
        let parsed = ParsedMove::Basic(prime_move);
        let explanation = get_move_explanation(&parsed);
        assert!(explanation.contains("counter-clockwise"));
    }
}

#[test]
fn test_display_024_all_double_moves() {
    // Test that all double moves have explanations
    let doubles = vec![Move::R2, Move::L2, Move::U2, Move::D2, Move::F2, Move::B2];
    for double_move in doubles {
        let parsed = ParsedMove::Basic(double_move);
        let explanation = get_move_explanation(&parsed);
        assert!(explanation.contains("180 degrees"));
    }
}

#[test]
fn test_display_025_notation_matches_move() {
    // Test that notation matches the move type
    let test_cases = vec![
        (Move::R, "R"),
        (Move::RPrime, "R'"),
        (Move::R2, "R2"),
        (Move::L, "L"),
        (Move::UPrime, "U'"),
        (Move::F2, "F2"),
    ];

    for (m, expected) in test_cases {
        let parsed = ParsedMove::Basic(m);
        let notation = get_move_notation(&parsed);
        assert_eq!(notation, expected);
    }
}

#[test]
fn test_display_026_kid_friendly_language() {
    // Verify explanations use kid-friendly language
    let r_move = ParsedMove::Basic(Move::R);
    let explanation = get_move_explanation(&r_move);

    // Should not use technical jargon
    assert!(!explanation.contains("algorithm"));
    assert!(!explanation.contains("permutation"));

    // Should use simple, clear language
    assert!(explanation.contains("Turn") || explanation.contains("Rotate"));
}

#[test]
fn test_display_027_slice_moves_comprehensive() {
    // Test all slice moves have proper explanations
    let slices = vec![
        Move::M, Move::MPrime, Move::M2,
        Move::E, Move::EPrime, Move::E2,
        Move::S, Move::SPrime, Move::S2,
    ];

    for slice_move in slices {
        let parsed = ParsedMove::Basic(slice_move);
        let explanation = get_move_explanation(&parsed);
        assert!(!explanation.is_empty());
        // All slice moves should mention they're middle/slice moves
        assert!(explanation.contains("middle") || explanation.contains("slice"));
    }
}

#[test]
fn test_display_028_rotations_comprehensive() {
    // Test all rotations have proper explanations
    let rotations = vec![
        Move::X, Move::XPrime, Move::X2,
        Move::Y, Move::YPrime, Move::Y2,
        Move::Z, Move::ZPrime, Move::Z2,
    ];

    for rotation_move in rotations {
        let parsed = ParsedMove::Basic(rotation_move);
        let explanation = get_move_explanation(&parsed);
        assert!(!explanation.is_empty());
        // All rotations should mention whole cube
        assert!(explanation.contains("whole cube"));
    }
}

#[test]
fn test_display_029_wide_faces_all() {
    // Test all wide face types
    let wide_faces = vec![
        WideFace::R, WideFace::L, WideFace::U,
        WideFace::D, WideFace::F, WideFace::B,
    ];

    for wide_face in wide_faces {
        let wide_move = WideMove {
            face: wide_face,
            direction: Direction::Clockwise,
            depth: 2,
        };
        let parsed = ParsedMove::Wide(wide_move);
        let explanation = get_move_explanation(&parsed);
        assert!(!explanation.is_empty());
        assert!(explanation.contains("two layers"));
    }
}

#[test]
fn test_display_030_explanations_are_unique() {
    // Verify different moves have different explanations
    let r = get_move_explanation(&ParsedMove::Basic(Move::R));
    let l = get_move_explanation(&ParsedMove::Basic(Move::L));
    let u = get_move_explanation(&ParsedMove::Basic(Move::U));

    assert_ne!(r, l);
    assert_ne!(r, u);
    assert_ne!(l, u);
}
