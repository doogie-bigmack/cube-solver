//! Integration tests for move notation parser
//!
//! Tests cover all notation parsing requirements from the test plan.

use rubiks_cube_solver::cube::{Move, ParsedMove, parse_move, parse_algorithm, WideFace, Direction};

/// nota_001: Parse single move "R"
#[test]
fn nota_001_parse_single_move_r() {
    let result = parse_move("R").unwrap();
    assert_eq!(result, ParsedMove::Basic(Move::R));
}

/// nota_002: Parse inverse "R'"
#[test]
fn nota_002_parse_inverse() {
    let result = parse_move("R'").unwrap();
    assert_eq!(result, ParsedMove::Basic(Move::RPrime));
}

/// nota_003: Parse double "R2"
#[test]
fn nota_003_parse_double() {
    let result = parse_move("R2").unwrap();
    assert_eq!(result, ParsedMove::Basic(Move::R2));
}

/// nota_004: Parse wide "Rw"
#[test]
fn nota_004_parse_wide() {
    match parse_move("Rw").unwrap() {
        ParsedMove::Wide(w) => {
            assert_eq!(w.face, WideFace::R);
            assert_eq!(w.direction, Direction::Clockwise);
            assert_eq!(w.depth, 2); // Default depth for wide moves
        },
        _ => panic!("Expected wide move"),
    }
}

/// nota_005: Parse wide with depth "3Rw"
#[test]
fn nota_005_parse_wide_with_depth() {
    match parse_move("3Rw").unwrap() {
        ParsedMove::Wide(w) => {
            assert_eq!(w.face, WideFace::R);
            assert_eq!(w.direction, Direction::Clockwise);
            assert_eq!(w.depth, 3);
        },
        _ => panic!("Expected wide move with depth 3"),
    }
}

/// nota_006: Parse algorithm "R U R' U'"
#[test]
fn nota_006_parse_algorithm() {
    let result = parse_algorithm("R U R' U'").unwrap();
    assert_eq!(result.len(), 4);
    assert_eq!(result[0], ParsedMove::Basic(Move::R));
    assert_eq!(result[1], ParsedMove::Basic(Move::U));
    assert_eq!(result[2], ParsedMove::Basic(Move::RPrime));
    assert_eq!(result[3], ParsedMove::Basic(Move::UPrime));
}

/// nota_007: Parse with extra spaces "R  U  R'"
#[test]
fn nota_007_parse_with_extra_spaces() {
    let result = parse_algorithm("R  U  R'").unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], ParsedMove::Basic(Move::R));
    assert_eq!(result[1], ParsedMove::Basic(Move::U));
    assert_eq!(result[2], ParsedMove::Basic(Move::RPrime));
}

/// nota_008: Parse lowercase "r u r' u'"
#[test]
fn nota_008_parse_lowercase() {
    let result = parse_algorithm("r u r' u'").unwrap();
    assert_eq!(result.len(), 4);
    // Lowercase without 'w' should be treated as uppercase basic moves
    assert_eq!(result[0], ParsedMove::Basic(Move::R));
    assert_eq!(result[1], ParsedMove::Basic(Move::U));
    assert_eq!(result[2], ParsedMove::Basic(Move::RPrime));
    assert_eq!(result[3], ParsedMove::Basic(Move::UPrime));
}

/// nota_009: Invalid notation "X" returns error
/// Note: "X" is valid (rotation), so we test "Q" instead
#[test]
fn nota_009_invalid_notation_returns_error() {
    assert!(parse_move("Q").is_err());
    assert!(parse_move("R3").is_err());
    assert!(parse_move("RR").is_err());
    assert!(parse_move("Invalid").is_err());
}

/// nota_010: Empty string returns empty vec
#[test]
fn nota_010_empty_string_returns_empty_vec() {
    let result = parse_algorithm("").unwrap();
    assert_eq!(result.len(), 0);

    let result = parse_algorithm("   ").unwrap();
    assert_eq!(result.len(), 0);
}

/// nota_011: Parse slice moves "M E S"
#[test]
fn nota_011_parse_slice_moves() {
    let result = parse_algorithm("M E S").unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], ParsedMove::Basic(Move::M));
    assert_eq!(result[1], ParsedMove::Basic(Move::E));
    assert_eq!(result[2], ParsedMove::Basic(Move::S));

    // Test slice moves with modifiers
    let result = parse_algorithm("M' E2 S'").unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], ParsedMove::Basic(Move::MPrime));
    assert_eq!(result[1], ParsedMove::Basic(Move::E2));
    assert_eq!(result[2], ParsedMove::Basic(Move::SPrime));
}

/// nota_012: Parse rotations "x y z"
#[test]
fn nota_012_parse_rotations() {
    let result = parse_algorithm("x y z").unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], ParsedMove::Basic(Move::X));
    assert_eq!(result[1], ParsedMove::Basic(Move::Y));
    assert_eq!(result[2], ParsedMove::Basic(Move::Z));

    // Test rotations with modifiers
    let result = parse_algorithm("x' y2 z'").unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], ParsedMove::Basic(Move::XPrime));
    assert_eq!(result[1], ParsedMove::Basic(Move::Y2));
    assert_eq!(result[2], ParsedMove::Basic(Move::ZPrime));

    // Test uppercase rotations
    let result = parse_algorithm("X Y Z").unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], ParsedMove::Basic(Move::X));
    assert_eq!(result[1], ParsedMove::Basic(Move::Y));
    assert_eq!(result[2], ParsedMove::Basic(Move::Z));
}

/// Additional test: Parse all basic face moves
#[test]
fn test_all_basic_face_moves() {
    let moves = ["R", "L", "U", "D", "F", "B"];
    for m in moves {
        let result = parse_move(m).unwrap();
        match result {
            ParsedMove::Basic(_) => {}, // Success
            _ => panic!("Expected basic move for {}", m),
        }
    }
}

/// Additional test: Parse all prime moves
#[test]
fn test_all_prime_moves() {
    let moves = ["R'", "L'", "U'", "D'", "F'", "B'"];
    for m in moves {
        let result = parse_move(m).unwrap();
        match result {
            ParsedMove::Basic(_) => {}, // Success
            _ => panic!("Expected basic move for {}", m),
        }
    }
}

/// Additional test: Parse all double moves
#[test]
fn test_all_double_moves() {
    let moves = ["R2", "L2", "U2", "D2", "F2", "B2"];
    for m in moves {
        let result = parse_move(m).unwrap();
        match result {
            ParsedMove::Basic(_) => {}, // Success
            _ => panic!("Expected basic move for {}", m),
        }
    }
}

/// Additional test: Parse all wide moves
#[test]
fn test_all_wide_moves() {
    let moves = ["Rw", "Lw", "Uw", "Dw", "Fw", "Bw"];
    for m in moves {
        let result = parse_move(m).unwrap();
        match result {
            ParsedMove::Wide(w) => {
                assert_eq!(w.depth, 2); // Default depth
            },
            _ => panic!("Expected wide move for {}", m),
        }
    }
}

/// Additional test: Parse wide moves with depth
#[test]
fn test_wide_moves_with_various_depths() {
    for depth in 2..=5 {
        let notation = format!("{}Rw", depth);
        match parse_move(&notation).unwrap() {
            ParsedMove::Wide(w) => {
                assert_eq!(w.face, WideFace::R);
                assert_eq!(w.depth, depth);
            },
            _ => panic!("Expected wide move with depth {}", depth),
        }
    }
}

/// Additional test: Parse wide moves with prime
#[test]
fn test_wide_moves_prime() {
    let moves = ["Rw'", "Lw'", "Uw'"];
    for m in moves {
        match parse_move(m).unwrap() {
            ParsedMove::Wide(w) => {
                assert_eq!(w.direction, Direction::CounterClockwise);
            },
            _ => panic!("Expected wide move for {}", m),
        }
    }
}

/// Additional test: Parse wide moves with double
#[test]
fn test_wide_moves_double() {
    let moves = ["Rw2", "Lw2", "Uw2"];
    for m in moves {
        match parse_move(m).unwrap() {
            ParsedMove::Wide(w) => {
                assert_eq!(w.direction, Direction::Double);
            },
            _ => panic!("Expected wide move for {}", m),
        }
    }
}

/// Additional test: Complex algorithm with mixed moves
#[test]
fn test_complex_algorithm() {
    let alg = "R U R' U' R' F R2 U' R' U' R U R' F'";
    let result = parse_algorithm(alg).unwrap();
    assert_eq!(result.len(), 14);
}

/// Additional test: Algorithm with wide moves
#[test]
fn test_algorithm_with_wide_moves() {
    let alg = "Rw U Rw' U' 3Rw";
    let result = parse_algorithm(alg).unwrap();
    assert_eq!(result.len(), 5);

    // Check first move is wide
    match &result[0] {
        ParsedMove::Wide(w) => {
            assert_eq!(w.face, WideFace::R);
            assert_eq!(w.depth, 2);
        },
        _ => panic!("Expected wide move"),
    }

    // Check last move has depth 3
    match &result[4] {
        ParsedMove::Wide(w) => {
            assert_eq!(w.face, WideFace::R);
            assert_eq!(w.depth, 3);
        },
        _ => panic!("Expected wide move with depth 3"),
    }
}

/// Additional test: Invalid depth 0
#[test]
fn test_invalid_depth_zero() {
    assert!(parse_move("0Rw").is_err());
}

/// Additional test: Edge case - whitespace handling
#[test]
fn test_whitespace_trimming() {
    let result = parse_move("  R  ").unwrap();
    assert_eq!(result, ParsedMove::Basic(Move::R));

    let result = parse_move("  Rw  ").unwrap();
    match result {
        ParsedMove::Wide(_) => {}, // Success
        _ => panic!("Expected wide move"),
    }
}
