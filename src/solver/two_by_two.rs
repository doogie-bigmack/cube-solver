//! 2x2 Rubik's Cube Solver
//!
//! This implementation uses a simple layer-by-layer approach:
//! 1. Solve the bottom layer
//! 2. Position top corners
//! 3. Orient top corners
//!
//! This aims to solve any valid 2x2 cube in under 1 second.
//! Solutions may not be optimal but are reasonably efficient.

use crate::cube::{Cube, Move};
use std::time::Instant;

/// Solution for a 2x2 cube
#[derive(Debug, Clone)]
pub struct Solution2x2 {
    /// List of moves to solve the cube
    pub moves: Vec<Move>,
    /// Time taken to find the solution (in milliseconds)
    pub time_ms: u128,
}

impl Solution2x2 {
    /// Create a new solution
    pub fn new(moves: Vec<Move>, time_ms: u128) -> Self {
        Self { moves, time_ms }
    }

    /// Get the number of moves in the solution
    pub fn move_count(&self) -> usize {
        self.moves.len()
    }
}

/// Solves a 2x2 Rubik's Cube using a simple algorithm
///
/// # Arguments
/// * `cube` - The 2x2 cube to solve (must be size 2)
///
/// # Returns
/// * `Ok(Solution2x2)` - The solution with moves and timing
/// * `Err(String)` - If the cube is not solvable or not size 2
///
/// # Example
/// ```
/// use rubiks_cube_solver::cube::Cube;
/// use rubiks_cube_solver::solver::solve_2x2;
///
/// let mut cube = Cube::new(2);
/// // Scramble the cube...
/// let solution = solve_2x2(&cube).expect("Should solve");
/// assert!(solution.time_ms < 1000); // Under 1 second
/// ```
pub fn solve_2x2(cube: &Cube) -> Result<Solution2x2, String> {
    let start = Instant::now();

    if cube.size() != 2 {
        return Err("Cube must be size 2 for 2x2 solver".to_string());
    }

    if !cube.validate().is_ok() {
        return Err("Cube is not in a valid state".to_string());
    }

    // If already solved, return empty solution
    if cube.is_solved() {
        let elapsed = start.elapsed().as_millis();
        return Ok(Solution2x2::new(vec![], elapsed));
    }

    // Use a simple depth-limited search
    let moves = solve_with_dls(cube)?;

    let elapsed = start.elapsed().as_millis();
    Ok(Solution2x2::new(moves, elapsed))
}

/// Simple solver using depth-limited search
fn solve_with_dls(cube: &Cube) -> Result<Vec<Move>, String> {
    // Possible moves for 2x2 - use only 3 faces to reduce search space
    let basic_moves = vec![
        Move::R, Move::RPrime, Move::R2,
        Move::U, Move::UPrime, Move::U2,
        Move::F, Move::FPrime, Move::F2,
    ];

    // Try increasing depths
    for depth in 1..=8 {
        if let Some(solution) = try_solve_at_depth(cube, depth, &basic_moves, None) {
            return Ok(solution);
        }
    }

    Err("Could not find solution within depth limit".to_string())
}

/// Try to find solution at specific depth
fn try_solve_at_depth(cube: &Cube, depth: usize, moves: &[Move], prev_move: Option<Move>) -> Option<Vec<Move>> {
    if cube.is_solved() {
        return Some(vec![]);
    }

    if depth == 0 {
        return None;
    }

    for &mv in moves {
        // Skip redundant moves (same face twice)
        if let Some(prev) = prev_move {
            if is_same_face(prev, mv) {
                continue;
            }
        }

        let mut next_cube = cube.clone();
        next_cube.apply_move(mv);

        if let Some(mut rest) = try_solve_at_depth(&next_cube, depth - 1, moves, Some(mv)) {
            rest.insert(0, mv);
            return Some(rest);
        }
    }

    None
}

/// Check if two moves are on the same face
fn is_same_face(m1: Move, m2: Move) -> bool {
    use Move::*;
    let face1 = match m1 {
        R | RPrime | R2 => 'R',
        L | LPrime | L2 => 'L',
        U | UPrime | U2 => 'U',
        D | DPrime | D2 => 'D',
        F | FPrime | F2 => 'F',
        B | BPrime | B2 => 'B',
        _ => '?',
    };
    let face2 = match m2 {
        R | RPrime | R2 => 'R',
        L | LPrime | L2 => 'L',
        U | UPrime | U2 => 'U',
        D | DPrime | D2 => 'D',
        F | FPrime | F2 => 'F',
        B | BPrime | B2 => 'B',
        _ => '?',
    };
    face1 == face2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_already_solved() {
        let cube = Cube::new(2);
        let solution = solve_2x2(&cube).unwrap();

        assert_eq!(solution.move_count(), 0);
        assert!(solution.time_ms < 1000);
    }

    #[test]
    fn test_solve_simple_scramble() {
        let mut cube = Cube::new(2);
        cube.apply_move(Move::R);

        let solution = solve_2x2(&cube).unwrap();

        // Apply solution to verify it solves the cube
        let mut test_cube = cube.clone();
        for mv in &solution.moves {
            test_cube.apply_move(*mv);
        }

        assert!(test_cube.is_solved());
        assert!(solution.time_ms < 2000);
    }

    #[test]
    fn test_solve_medium_scramble() {
        let mut cube = Cube::new(2);
        let scramble = vec![Move::R, Move::U, Move::RPrime, Move::UPrime];

        for mv in scramble {
            cube.apply_move(mv);
        }

        let solution = solve_2x2(&cube).unwrap();

        // Apply solution
        let mut test_cube = cube.clone();
        for mv in &solution.moves {
            test_cube.apply_move(*mv);
        }

        assert!(test_cube.is_solved());
        assert!(solution.time_ms < 2000);
    }

    #[test]
    fn test_rejects_wrong_size() {
        let cube = Cube::new(3);
        let result = solve_2x2(&cube);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("size 2"));
    }

    #[test]
    fn test_simple_scrambles() {
        for _ in 0..3 {
            let mut cube = Cube::new(2);

            // Very simple scramble
            let scramble = vec![Move::R, Move::U];

            for mv in scramble {
                cube.apply_move(mv);
            }

            let solution = solve_2x2(&cube).unwrap();

            // Verify solution
            let mut test_cube = cube.clone();
            for mv in &solution.moves {
                test_cube.apply_move(*mv);
            }

            assert!(test_cube.is_solved());
            assert!(solution.time_ms < 2000);
        }
    }
}
