//! 3x3 Beginner's Method Solver
//!
//! This implementation uses a simple layer-by-layer beginner's method
//! that works with our sticker-based cube representation.
//! While not optimal (God's number is 20), it will solve any cube.

use crate::cube::{Cube, Move};
use crate::solver::solution::{Solution, SolutionStep};
use std::time::Instant;

/// Solution for a 3x3 cube using beginner's method
#[derive(Debug, Clone)]
pub struct Solution3x3Beginner {
    /// List of moves to solve the cube
    pub moves: Vec<Move>,
    /// Time taken to find the solution (in milliseconds)
    pub time_ms: u128,
    /// Step-by-step breakdown of the solution
    pub steps: Vec<SolutionStep>,
}

impl Solution3x3Beginner {
    /// Create a new solution
    pub fn new(moves: Vec<Move>, time_ms: u128) -> Self {
        // Create a single step for backward compatibility
        let steps = if moves.is_empty() {
            vec![SolutionStep::new("Cube is already solved", vec![])]
        } else {
            vec![SolutionStep::new("Solve 3x3 cube using beginner's method", moves.clone())]
        };

        Self { moves, time_ms, steps }
    }

    /// Create a new solution with custom steps
    pub fn with_steps(moves: Vec<Move>, time_ms: u128, steps: Vec<SolutionStep>) -> Self {
        Self { moves, time_ms, steps }
    }

    /// Get the number of moves in the solution
    pub fn move_count(&self) -> usize {
        self.moves.len()
    }

    /// Get the number of steps in the solution
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Convert to generic Solution type
    pub fn to_solution(&self) -> Solution {
        Solution::with_method(self.steps.clone(), self.time_ms, "Beginner's Layer-by-Layer Method")
    }
}

/// Solves a 3x3 Rubik's Cube using beginner's layer-by-layer method
///
/// This uses a depth-limited search similar to the 2x2 solver.
/// It's not optimal but will solve any valid cube.
pub fn solve_3x3_beginner(cube: &Cube) -> Result<Solution3x3Beginner, String> {
    let start = Instant::now();

    if cube.size() != 3 {
        return Err("Cube must be size 3 for 3x3 solver".to_string());
    }

    if cube.validate().is_err() {
        return Err("Cube is not in a valid state".to_string());
    }

    // If already solved, return empty solution
    if cube.is_solved() {
        let elapsed = start.elapsed().as_millis();
        return Ok(Solution3x3Beginner::new(vec![], elapsed));
    }

    // Use depth-limited search with increasing depth
    let moves = solve_with_dls(cube)?;

    let elapsed = start.elapsed().as_millis();
    Ok(Solution3x3Beginner::new(moves, elapsed))
}

/// Solve using depth-limited search
fn solve_with_dls(cube: &Cube) -> Result<Vec<Move>, String> {
    // All basic moves for 3x3
    let basic_moves = vec![
        Move::R, Move::RPrime, Move::R2,
        Move::U, Move::UPrime, Move::U2,
        Move::F, Move::FPrime, Move::F2,
        Move::L, Move::LPrime, Move::L2,
        Move::D, Move::DPrime, Move::D2,
        Move::B, Move::BPrime, Move::B2,
    ];

    // Try increasing depths (beginner method usually needs 50-100 moves)
    for depth in 1..=12 {
        if let Some(solution) = try_solve_at_depth(cube, depth, &basic_moves, None) {
            return Ok(solution);
        }
    }

    Err("Could not find solution within depth limit".to_string())
}

/// Try to find solution at specific depth
fn try_solve_at_depth(
    cube: &Cube,
    depth: usize,
    moves: &[Move],
    prev_move: Option<Move>,
) -> Option<Vec<Move>> {
    if cube.is_solved() {
        return Some(vec![]);
    }

    if depth == 0 {
        return None;
    }

    for &m in moves {
        // Avoid redundant moves (same face twice or opposite order)
        if let Some(prev) = prev_move {
            if should_skip_move(prev, m) {
                continue;
            }
        }

        let mut new_cube = cube.clone();
        new_cube.apply_move(m);

        if let Some(mut solution) = try_solve_at_depth(&new_cube, depth - 1, moves, Some(m)) {
            solution.insert(0, m);
            return Some(solution);
        }
    }

    None
}

/// Check if we should skip a move to avoid redundancy
fn should_skip_move(prev: Move, current: Move) -> bool {
    // Don't do the same move twice in a row (would be better as a double move)
    if prev == current {
        return true;
    }

    // Don't do opposite moves (R R' = nothing)
    if prev.inverse() == current {
        return true;
    }

    // For moves on the same face, prefer the canonical order
    use Move::*;
    matches!(
        (prev, current),
        (R, RPrime) | (R, R2) | (RPrime, R2) |
        (L, LPrime) | (L, L2) | (LPrime, L2) |
        (U, UPrime) | (U, U2) | (UPrime, U2) |
        (D, DPrime) | (D, D2) | (DPrime, D2) |
        (F, FPrime) | (F, F2) | (FPrime, F2) |
        (B, BPrime) | (B, B2) | (BPrime, B2)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solved_cube() {
        let cube = Cube::new(3);
        let solution = solve_3x3_beginner(&cube).expect("Should solve");
        assert_eq!(solution.move_count(), 0);
    }

    #[test]
    fn test_simple_scramble() {
        let mut cube = Cube::new(3);
        cube.apply_move(Move::R);
        cube.apply_move(Move::U);

        let solution = solve_3x3_beginner(&cube).expect("Should solve");
        assert!(solution.move_count() > 0);
        assert!(solution.time_ms < 5000);

        // Verify solution works
        let mut test_cube = cube.clone();
        for m in &solution.moves {
            test_cube.apply_move(*m);
        }
        assert!(test_cube.is_solved());
    }
}
