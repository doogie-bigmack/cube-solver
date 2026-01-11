//! Scramble generator for Rubik's cube
//!
//! This module implements R1.8 from the PRD:
//! - Generate random scramble of configurable length
//! - Avoid redundant moves (no R R or R R')
//! - Return both move list and scrambled cube state

use super::moves::Move;
use super::state::Cube;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Represents a generated scramble with moves and resulting state
#[derive(Debug, Clone)]
pub struct Scramble {
    /// The sequence of moves that make up the scramble
    pub moves: Vec<Move>,
    /// The resulting cube state after applying the scramble
    pub cube: Cube,
}

impl Scramble {
    /// Creates a new scramble with the given moves and cube state
    pub fn new(moves: Vec<Move>, cube: Cube) -> Self {
        Self { moves, cube }
    }

    /// Returns the scramble as a notation string (e.g., "R U R' U'")
    pub fn to_notation(&self) -> String {
        self.moves
            .iter()
            .map(|m| m.to_notation())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Configuration for scramble generation
#[derive(Debug, Clone)]
pub struct ScrambleConfig {
    /// Number of moves in the scramble
    pub length: usize,
    /// Cube size
    pub size: usize,
}

impl Default for ScrambleConfig {
    fn default() -> Self {
        Self {
            length: 20,
            size: 3,
        }
    }
}

impl ScrambleConfig {
    /// Creates a new scramble configuration
    pub fn new(length: usize, size: usize) -> Self {
        Self { length, size }
    }
}

/// Generates a random scramble for a cube
///
/// # Arguments
/// * `config` - Configuration for the scramble
///
/// # Returns
/// A Scramble containing the move sequence and resulting cube state
///
/// # Example
/// ```
/// use rubiks_cube_solver::cube::{Cube, scramble::{generate_scramble, ScrambleConfig}};
///
/// let config = ScrambleConfig::new(20, 3);
/// let scramble = generate_scramble(&config);
/// assert_eq!(scramble.moves.len(), 20);
/// ```
pub fn generate_scramble(config: &ScrambleConfig) -> Scramble {
    let mut cube = Cube::new(config.size);
    let mut moves = Vec::new();
    let mut rng = thread_rng();

    // All basic moves (excluding wide moves, slice moves on even cubes, and rotations for scrambles)
    let all_moves = get_available_moves(config.size);

    for _ in 0..config.length {
        let next_move = select_next_move(&moves, &all_moves, &mut rng);
        cube.apply_move(next_move);
        moves.push(next_move);
    }

    Scramble::new(moves, cube)
}

/// Gets all available moves for a given cube size
fn get_available_moves(size: usize) -> Vec<Move> {
    // Basic face moves available for all cube sizes
    let mut moves = vec![
        Move::R,
        Move::RPrime,
        Move::R2,
        Move::L,
        Move::LPrime,
        Move::L2,
        Move::U,
        Move::UPrime,
        Move::U2,
        Move::D,
        Move::DPrime,
        Move::D2,
        Move::F,
        Move::FPrime,
        Move::F2,
        Move::B,
        Move::BPrime,
        Move::B2,
    ];

    // Add slice moves only for odd-sized cubes 3x3 and larger
    if size >= 3 && size % 2 == 1 {
        moves.extend_from_slice(&[
            Move::M,
            Move::MPrime,
            Move::M2,
            Move::E,
            Move::EPrime,
            Move::E2,
            Move::S,
            Move::SPrime,
            Move::S2,
        ]);
    }

    moves
}

/// Selects the next move avoiding redundancies
///
/// Avoids:
/// - Same face in succession (R R)
/// - Same face with inverse (R R')
/// - Same face with double (R R2)
fn select_next_move<R: rand::Rng>(
    previous_moves: &[Move],
    available_moves: &[Move],
    rng: &mut R,
) -> Move {
    if previous_moves.is_empty() {
        return *available_moves.choose(rng).unwrap();
    }

    let last_move = previous_moves.last().unwrap();
    let last_face = get_move_face(last_move);

    // Filter out moves on the same face as the last move
    let valid_moves: Vec<Move> = available_moves
        .iter()
        .filter(|&&m| get_move_face(&m) != last_face)
        .copied()
        .collect();

    // Additional check: avoid opposite faces in succession for better randomization
    // (e.g., R followed by L doesn't help much in scrambling)
    if previous_moves.len() >= 2 {
        let second_last_move = previous_moves[previous_moves.len() - 2];
        let second_last_face = get_move_face(&second_last_move);

        // If last two moves were on opposite faces, avoid both for this move
        if are_opposite_faces(last_face, second_last_face) {
            let filtered: Vec<Move> = valid_moves
                .iter()
                .filter(|&&m| {
                    let face = get_move_face(&m);
                    face != last_face && face != second_last_face
                })
                .copied()
                .collect();

            if !filtered.is_empty() {
                return *filtered.choose(rng).unwrap();
            }
        }
    }

    *valid_moves.choose(rng).unwrap()
}

/// Face identifier for grouping moves
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Face {
    R,
    L,
    U,
    D,
    F,
    B,
    M,
    E,
    S,
}

/// Gets the face for a given move
fn get_move_face(m: &Move) -> Face {
    match m {
        Move::R | Move::RPrime | Move::R2 => Face::R,
        Move::L | Move::LPrime | Move::L2 => Face::L,
        Move::U | Move::UPrime | Move::U2 => Face::U,
        Move::D | Move::DPrime | Move::D2 => Face::D,
        Move::F | Move::FPrime | Move::F2 => Face::F,
        Move::B | Move::BPrime | Move::B2 => Face::B,
        Move::M | Move::MPrime | Move::M2 => Face::M,
        Move::E | Move::EPrime | Move::E2 => Face::E,
        Move::S | Move::SPrime | Move::S2 => Face::S,
        Move::X | Move::XPrime | Move::X2 => Face::R, // Treat rotations as R for face purposes
        Move::Y | Move::YPrime | Move::Y2 => Face::U,
        Move::Z | Move::ZPrime | Move::Z2 => Face::F,
    }
}

/// Checks if two faces are opposite
fn are_opposite_faces(f1: Face, f2: Face) -> bool {
    matches!(
        (f1, f2),
        (Face::R, Face::L)
            | (Face::L, Face::R)
            | (Face::U, Face::D)
            | (Face::D, Face::U)
            | (Face::F, Face::B)
            | (Face::B, Face::F)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_scramble_correct_length() {
        let config = ScrambleConfig::new(20, 3);
        let scramble = generate_scramble(&config);
        assert_eq!(scramble.moves.len(), 20);
    }

    #[test]
    fn test_generate_scramble_different_lengths() {
        for length in [5, 10, 15, 20, 25] {
            let config = ScrambleConfig::new(length, 3);
            let scramble = generate_scramble(&config);
            assert_eq!(scramble.moves.len(), length);
        }
    }

    #[test]
    fn test_scramble_avoids_same_face_succession() {
        // Generate many scrambles and verify no same face in succession
        for _ in 0..100 {
            let config = ScrambleConfig::new(20, 3);
            let scramble = generate_scramble(&config);

            for i in 1..scramble.moves.len() {
                let prev_face = get_move_face(&scramble.moves[i - 1]);
                let curr_face = get_move_face(&scramble.moves[i]);
                assert_ne!(
                    prev_face, curr_face,
                    "Found same face in succession: {:?} followed by {:?}",
                    scramble.moves[i - 1], scramble.moves[i]
                );
            }
        }
    }

    #[test]
    fn test_scramble_produces_scrambled_state() {
        let config = ScrambleConfig::new(20, 3);
        let scramble = generate_scramble(&config);
        let solved = Cube::new(3);

        // Scrambled cube should not equal solved cube
        assert_ne!(scramble.cube, solved);
    }

    #[test]
    fn test_scramble_to_notation() {
        let config = ScrambleConfig::new(5, 3);
        let scramble = generate_scramble(&config);
        let notation = scramble.to_notation();

        // Should have 5 moves separated by spaces (4 spaces total)
        assert_eq!(notation.split_whitespace().count(), 5);
    }

    #[test]
    fn test_scramble_is_valid() {
        let config = ScrambleConfig::new(20, 3);
        let scramble = generate_scramble(&config);

        // Scrambled cube should be valid
        assert!(scramble.cube.validate().is_ok());
    }

    #[test]
    fn test_scramble_2x2() {
        let config = ScrambleConfig::new(15, 2);
        let scramble = generate_scramble(&config);
        assert_eq!(scramble.moves.len(), 15);
        assert!(scramble.cube.validate().is_ok());
    }

    #[test]
    fn test_scramble_4x4() {
        let config = ScrambleConfig::new(40, 4);
        let scramble = generate_scramble(&config);
        assert_eq!(scramble.moves.len(), 40);
        assert!(scramble.cube.validate().is_ok());
    }

    #[test]
    fn test_scramble_5x5_includes_slice_moves() {
        // For 5x5, slice moves should be available
        let config = ScrambleConfig::new(50, 5);
        let scramble = generate_scramble(&config);
        assert_eq!(scramble.moves.len(), 50);

        // Verify the move set includes slice moves
        let available_moves = get_available_moves(5);
        assert!(available_moves.contains(&Move::M));
        assert!(available_moves.contains(&Move::E));
        assert!(available_moves.contains(&Move::S));
    }

    #[test]
    fn test_scramble_randomness() {
        // Generate multiple scrambles and verify they're different
        let config = ScrambleConfig::new(20, 3);
        let scramble1 = generate_scramble(&config);
        let scramble2 = generate_scramble(&config);
        let scramble3 = generate_scramble(&config);

        // Very unlikely all three would be identical
        assert!(
            scramble1.moves != scramble2.moves || scramble2.moves != scramble3.moves,
            "All three scrambles were identical - extremely unlikely"
        );
    }

    #[test]
    fn test_default_config() {
        let config = ScrambleConfig::default();
        assert_eq!(config.length, 20);
        assert_eq!(config.size, 3);
    }
}
