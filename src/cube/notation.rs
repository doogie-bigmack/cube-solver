//! Move notation parser for Rubik's cube
//!
//! Parses standard Rubik's cube notation into Move and WideMove enums.
//!
//! Supported notation:
//! - Basic moves: R, L, U, D, F, B
//! - Prime (counter-clockwise): R', L', U', D', F', B'
//! - Double (180 degrees): R2, L2, U2, D2, F2, B2
//! - Wide moves: Rw, Lw, Uw, Dw, Fw, Bw
//! - Wide with depth: 3Rw, 2Uw, etc.
//! - Slice moves: M, E, S (with ', 2)
//! - Rotations: x, y, z (with ', 2)
//! - Algorithms: "R U R' U'" (space-separated sequences)

use crate::cube::{Move, WideMove, WideFace, Direction};

/// Represents a parsed move that could be either a basic Move or a WideMove
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedMove {
    Basic(Move),
    Wide(WideMove),
}

/// Error type for notation parsing
#[derive(Debug, Clone, PartialEq)]
pub enum NotationError {
    /// Invalid move notation (e.g., "X" which isn't a valid move)
    InvalidMove(String),
    /// Invalid depth value (e.g., "0Rw" or negative depth)
    InvalidDepth(String),
    /// Empty input string
    EmptyInput,
}

impl std::fmt::Display for NotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotationError::InvalidMove(m) => write!(f, "Invalid move notation: {}", m),
            NotationError::InvalidDepth(d) => write!(f, "Invalid depth value: {}", d),
            NotationError::EmptyInput => write!(f, "Empty input string"),
        }
    }
}

impl std::error::Error for NotationError {}

/// Parse a single move from a string
///
/// Examples:
/// - "R" -> Move::R
/// - "R'" -> Move::RPrime
/// - "R2" -> Move::R2
/// - "Rw" -> WideMove { face: R, direction: Clockwise, depth: 2 }
/// - "3Rw" -> WideMove { face: R, direction: Clockwise, depth: 3 }
/// - "x" -> Move::X
pub fn parse_move(input: &str) -> Result<ParsedMove, NotationError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(NotationError::EmptyInput);
    }

    // Check for wide moves (ending with 'w')
    if trimmed.ends_with('w') || trimmed.ends_with("w'") || trimmed.ends_with("w2") {
        return parse_wide_move(trimmed);
    }

    // Parse basic moves, slice moves, and rotations
    parse_basic_move(trimmed)
}

/// Parse a wide move (e.g., "Rw", "Rw'", "Rw2", "3Rw", "3Rw2")
fn parse_wide_move(input: &str) -> Result<ParsedMove, NotationError> {
    let mut chars = input.chars().peekable();
    let mut depth_str = String::new();

    // Extract optional depth prefix (e.g., "3" in "3Rw")
    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            depth_str.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    // Parse depth or default to 2
    let depth = if depth_str.is_empty() {
        2 // Default depth for wide moves (Rw means 2 layers)
    } else {
        match depth_str.parse::<usize>() {
            Ok(0) => return Err(NotationError::InvalidDepth(depth_str)),
            Ok(d) => d,
            Err(_) => return Err(NotationError::InvalidDepth(depth_str)),
        }
    };

    // Get the face letter (should be uppercase)
    let face_char = match chars.next() {
        Some(ch) => ch.to_ascii_uppercase(),
        None => return Err(NotationError::InvalidMove(input.to_string())),
    };

    // Parse face
    let face = match face_char {
        'R' => WideFace::R,
        'L' => WideFace::L,
        'U' => WideFace::U,
        'D' => WideFace::D,
        'F' => WideFace::F,
        'B' => WideFace::B,
        _ => return Err(NotationError::InvalidMove(input.to_string())),
    };

    // Expect 'w'
    if chars.next() != Some('w') {
        return Err(NotationError::InvalidMove(input.to_string()));
    }

    // Parse direction modifier (' or 2)
    let direction = match chars.next() {
        Some('\'') => Direction::CounterClockwise,
        Some('2') => Direction::Double,
        None => Direction::Clockwise,
        _ => return Err(NotationError::InvalidMove(input.to_string())),
    };

    Ok(ParsedMove::Wide(WideMove { face, direction, depth }))
}

/// Parse a basic move, slice move, or rotation
fn parse_basic_move(input: &str) -> Result<ParsedMove, NotationError> {
    // Handle lowercase x, y, z rotations
    let normalized = if input.len() >= 1 {
        let first = input.chars().next().unwrap();
        if first == 'x' || first == 'y' || first == 'z' {
            // Keep x, y, z lowercase for now, will uppercase later
            input.to_string()
        } else {
            input.to_uppercase()
        }
    } else {
        input.to_uppercase()
    };

    let mov = match normalized.as_str() {
        // R face
        "R" => Move::R,
        "R'" => Move::RPrime,
        "R2" => Move::R2,

        // L face
        "L" => Move::L,
        "L'" => Move::LPrime,
        "L2" => Move::L2,

        // U face
        "U" => Move::U,
        "U'" => Move::UPrime,
        "U2" => Move::U2,

        // D face
        "D" => Move::D,
        "D'" => Move::DPrime,
        "D2" => Move::D2,

        // F face
        "F" => Move::F,
        "F'" => Move::FPrime,
        "F2" => Move::F2,

        // B face
        "B" => Move::B,
        "B'" => Move::BPrime,
        "B2" => Move::B2,

        // M slice
        "M" => Move::M,
        "M'" => Move::MPrime,
        "M2" => Move::M2,

        // E slice
        "E" => Move::E,
        "E'" => Move::EPrime,
        "E2" => Move::E2,

        // S slice
        "S" => Move::S,
        "S'" => Move::SPrime,
        "S2" => Move::S2,

        // X rotation (case-insensitive)
        "x" | "X" => Move::X,
        "x'" | "X'" => Move::XPrime,
        "x2" | "X2" => Move::X2,

        // Y rotation (case-insensitive)
        "y" | "Y" => Move::Y,
        "y'" | "Y'" => Move::YPrime,
        "y2" | "Y2" => Move::Y2,

        // Z rotation (case-insensitive)
        "z" | "Z" => Move::Z,
        "z'" | "Z'" => Move::ZPrime,
        "z2" | "Z2" => Move::Z2,

        _ => return Err(NotationError::InvalidMove(input.to_string())),
    };

    Ok(ParsedMove::Basic(mov))
}

/// Parse an algorithm string (space-separated moves)
///
/// Example: "R U R' U'" -> [Move::R, Move::U, Move::RPrime, Move::UPrime]
pub fn parse_algorithm(input: &str) -> Result<Vec<ParsedMove>, NotationError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    // Split by whitespace and parse each token
    let tokens: Vec<&str> = trimmed.split_whitespace().collect();
    let mut moves = Vec::new();

    for token in tokens {
        let parsed = parse_move(token)?;
        moves.push(parsed);
    }

    Ok(moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_moves() {
        // Basic face moves
        assert_eq!(parse_move("R").unwrap(), ParsedMove::Basic(Move::R));
        assert_eq!(parse_move("L").unwrap(), ParsedMove::Basic(Move::L));
        assert_eq!(parse_move("U").unwrap(), ParsedMove::Basic(Move::U));
        assert_eq!(parse_move("D").unwrap(), ParsedMove::Basic(Move::D));
        assert_eq!(parse_move("F").unwrap(), ParsedMove::Basic(Move::F));
        assert_eq!(parse_move("B").unwrap(), ParsedMove::Basic(Move::B));
    }

    #[test]
    fn test_parse_prime_moves() {
        assert_eq!(parse_move("R'").unwrap(), ParsedMove::Basic(Move::RPrime));
        assert_eq!(parse_move("L'").unwrap(), ParsedMove::Basic(Move::LPrime));
        assert_eq!(parse_move("U'").unwrap(), ParsedMove::Basic(Move::UPrime));
    }

    #[test]
    fn test_parse_double_moves() {
        assert_eq!(parse_move("R2").unwrap(), ParsedMove::Basic(Move::R2));
        assert_eq!(parse_move("L2").unwrap(), ParsedMove::Basic(Move::L2));
        assert_eq!(parse_move("U2").unwrap(), ParsedMove::Basic(Move::U2));
    }

    #[test]
    fn test_parse_slice_moves() {
        assert_eq!(parse_move("M").unwrap(), ParsedMove::Basic(Move::M));
        assert_eq!(parse_move("M'").unwrap(), ParsedMove::Basic(Move::MPrime));
        assert_eq!(parse_move("M2").unwrap(), ParsedMove::Basic(Move::M2));
        assert_eq!(parse_move("E").unwrap(), ParsedMove::Basic(Move::E));
        assert_eq!(parse_move("E'").unwrap(), ParsedMove::Basic(Move::EPrime));
        assert_eq!(parse_move("E2").unwrap(), ParsedMove::Basic(Move::E2));
        assert_eq!(parse_move("S").unwrap(), ParsedMove::Basic(Move::S));
        assert_eq!(parse_move("S'").unwrap(), ParsedMove::Basic(Move::SPrime));
        assert_eq!(parse_move("S2").unwrap(), ParsedMove::Basic(Move::S2));
    }

    #[test]
    fn test_parse_rotations() {
        // Uppercase
        assert_eq!(parse_move("X").unwrap(), ParsedMove::Basic(Move::X));
        assert_eq!(parse_move("Y").unwrap(), ParsedMove::Basic(Move::Y));
        assert_eq!(parse_move("Z").unwrap(), ParsedMove::Basic(Move::Z));

        // Lowercase (should also work)
        assert_eq!(parse_move("x").unwrap(), ParsedMove::Basic(Move::X));
        assert_eq!(parse_move("y").unwrap(), ParsedMove::Basic(Move::Y));
        assert_eq!(parse_move("z").unwrap(), ParsedMove::Basic(Move::Z));

        // Prime
        assert_eq!(parse_move("x'").unwrap(), ParsedMove::Basic(Move::XPrime));
        assert_eq!(parse_move("y'").unwrap(), ParsedMove::Basic(Move::YPrime));
        assert_eq!(parse_move("z'").unwrap(), ParsedMove::Basic(Move::ZPrime));

        // Double
        assert_eq!(parse_move("x2").unwrap(), ParsedMove::Basic(Move::X2));
        assert_eq!(parse_move("y2").unwrap(), ParsedMove::Basic(Move::Y2));
        assert_eq!(parse_move("z2").unwrap(), ParsedMove::Basic(Move::Z2));
    }

    #[test]
    fn test_parse_wide_moves() {
        // Basic wide moves (default depth 2)
        match parse_move("Rw").unwrap() {
            ParsedMove::Wide(w) => {
                assert_eq!(w.face, WideFace::R);
                assert_eq!(w.direction, Direction::Clockwise);
                assert_eq!(w.depth, 2);
            },
            _ => panic!("Expected wide move"),
        }

        match parse_move("Lw").unwrap() {
            ParsedMove::Wide(w) => {
                assert_eq!(w.face, WideFace::L);
                assert_eq!(w.direction, Direction::Clockwise);
                assert_eq!(w.depth, 2);
            },
            _ => panic!("Expected wide move"),
        }
    }

    #[test]
    fn test_parse_wide_with_depth() {
        // 3Rw
        match parse_move("3Rw").unwrap() {
            ParsedMove::Wide(w) => {
                assert_eq!(w.face, WideFace::R);
                assert_eq!(w.direction, Direction::Clockwise);
                assert_eq!(w.depth, 3);
            },
            _ => panic!("Expected wide move"),
        }

        // 2Uw
        match parse_move("2Uw").unwrap() {
            ParsedMove::Wide(w) => {
                assert_eq!(w.face, WideFace::U);
                assert_eq!(w.direction, Direction::Clockwise);
                assert_eq!(w.depth, 2);
            },
            _ => panic!("Expected wide move"),
        }
    }

    #[test]
    fn test_parse_wide_prime() {
        match parse_move("Rw'").unwrap() {
            ParsedMove::Wide(w) => {
                assert_eq!(w.face, WideFace::R);
                assert_eq!(w.direction, Direction::CounterClockwise);
                assert_eq!(w.depth, 2);
            },
            _ => panic!("Expected wide move"),
        }
    }

    #[test]
    fn test_parse_wide_double() {
        match parse_move("Rw2").unwrap() {
            ParsedMove::Wide(w) => {
                assert_eq!(w.face, WideFace::R);
                assert_eq!(w.direction, Direction::Double);
                assert_eq!(w.depth, 2);
            },
            _ => panic!("Expected wide move"),
        }

        match parse_move("3Rw2").unwrap() {
            ParsedMove::Wide(w) => {
                assert_eq!(w.face, WideFace::R);
                assert_eq!(w.direction, Direction::Double);
                assert_eq!(w.depth, 3);
            },
            _ => panic!("Expected wide move"),
        }
    }

    #[test]
    fn test_parse_algorithm() {
        let result = parse_algorithm("R U R' U'").unwrap();
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], ParsedMove::Basic(Move::R));
        assert_eq!(result[1], ParsedMove::Basic(Move::U));
        assert_eq!(result[2], ParsedMove::Basic(Move::RPrime));
        assert_eq!(result[3], ParsedMove::Basic(Move::UPrime));
    }

    #[test]
    fn test_parse_algorithm_with_extra_spaces() {
        let result = parse_algorithm("R  U  R'   U'").unwrap();
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], ParsedMove::Basic(Move::R));
        assert_eq!(result[1], ParsedMove::Basic(Move::U));
    }

    #[test]
    fn test_parse_algorithm_lowercase() {
        let result = parse_algorithm("r u r' u'").unwrap();
        assert_eq!(result.len(), 4);
        // Note: lowercase r, u become R, U (not wide moves without 'w')
        assert_eq!(result[0], ParsedMove::Basic(Move::R));
        assert_eq!(result[1], ParsedMove::Basic(Move::U));
    }

    #[test]
    fn test_parse_empty_string() {
        let result = parse_algorithm("").unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_whitespace_only() {
        let result = parse_algorithm("   ").unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_invalid_move() {
        assert!(parse_move("Q").is_err());
        assert!(parse_move("R3").is_err());
        assert!(parse_move("RR").is_err());
    }

    #[test]
    fn test_invalid_depth() {
        assert!(parse_move("0Rw").is_err());
    }

    #[test]
    fn test_mixed_algorithm() {
        let result = parse_algorithm("R Rw M x' U2").unwrap();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], ParsedMove::Basic(Move::R));
        match &result[1] {
            ParsedMove::Wide(w) => assert_eq!(w.face, WideFace::R),
            _ => panic!("Expected wide move"),
        }
        assert_eq!(result[2], ParsedMove::Basic(Move::M));
        assert_eq!(result[3], ParsedMove::Basic(Move::XPrime));
        assert_eq!(result[4], ParsedMove::Basic(Move::U2));
    }
}
