//! Solution representation with step-by-step breakdown
//!
//! This module provides a structured way to represent cube solutions
//! with logical steps and descriptions for educational purposes.

use crate::cube::Move;

/// A single step in a solution with description and moves
#[derive(Debug, Clone, PartialEq)]
pub struct SolutionStep {
    /// Human-readable description of what this step accomplishes
    pub description: String,
    /// The moves that accomplish this step
    pub moves: Vec<Move>,
    /// Optional detailed explanation for educational purposes
    pub explanation: Option<String>,
}

impl SolutionStep {
    /// Create a new solution step
    pub fn new(description: impl Into<String>, moves: Vec<Move>) -> Self {
        Self {
            description: description.into(),
            moves,
            explanation: None,
        }
    }

    /// Create a new solution step with explanation
    pub fn with_explanation(
        description: impl Into<String>,
        moves: Vec<Move>,
        explanation: impl Into<String>,
    ) -> Self {
        Self {
            description: description.into(),
            moves,
            explanation: Some(explanation.into()),
        }
    }

    /// Get the number of moves in this step
    pub fn move_count(&self) -> usize {
        self.moves.len()
    }

    /// Convert moves to notation string
    pub fn to_notation(&self) -> String {
        self.moves
            .iter()
            .map(|m| m.to_notation())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// A complete solution with step-by-step breakdown
#[derive(Debug, Clone, PartialEq)]
pub struct Solution {
    /// The steps that make up the solution
    pub steps: Vec<SolutionStep>,
    /// Time taken to find the solution (in milliseconds)
    pub time_ms: u128,
    /// Optional name/description of the solving method used
    pub method: Option<String>,
}

impl Solution {
    /// Create a new solution
    pub fn new(steps: Vec<SolutionStep>, time_ms: u128) -> Self {
        Self {
            steps,
            time_ms,
            method: None,
        }
    }

    /// Create a new solution with method name
    pub fn with_method(
        steps: Vec<SolutionStep>,
        time_ms: u128,
        method: impl Into<String>,
    ) -> Self {
        Self {
            steps,
            time_ms,
            method: Some(method.into()),
        }
    }

    /// Get all moves from all steps as a flat list
    pub fn all_moves(&self) -> Vec<Move> {
        self.steps
            .iter()
            .flat_map(|step| step.moves.iter())
            .copied()
            .collect()
    }

    /// Get the total number of moves in the solution
    pub fn move_count(&self) -> usize {
        self.all_moves().len()
    }

    /// Get the number of steps in the solution
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Convert the entire solution to notation string
    pub fn to_notation(&self) -> String {
        self.all_moves()
            .iter()
            .map(|m| m.to_notation())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Get a summary of the solution
    pub fn summary(&self) -> String {
        let method_str = self
            .method
            .as_ref()
            .map(|m| format!(" using {}", m))
            .unwrap_or_default();

        format!(
            "Solution{} with {} steps and {} moves (found in {}ms)",
            method_str,
            self.step_count(),
            self.move_count(),
            self.time_ms
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_step_new() {
        let step = SolutionStep::new("Solve bottom layer", vec![Move::R, Move::U]);
        assert_eq!(step.description, "Solve bottom layer");
        assert_eq!(step.moves, vec![Move::R, Move::U]);
        assert_eq!(step.move_count(), 2);
        assert!(step.explanation.is_none());
    }

    #[test]
    fn test_solution_step_with_explanation() {
        let step = SolutionStep::with_explanation(
            "Position corners",
            vec![Move::R, Move::UPrime],
            "This positions the bottom corners correctly",
        );
        assert_eq!(step.description, "Position corners");
        assert_eq!(step.explanation.as_deref(), Some("This positions the bottom corners correctly"));
        assert_eq!(step.move_count(), 2);
    }

    #[test]
    fn test_solution_step_to_notation() {
        let step = SolutionStep::new("Test step", vec![Move::R, Move::UPrime, Move::R2]);
        assert_eq!(step.to_notation(), "R U' R2");
    }

    #[test]
    fn test_solution_new() {
        let steps = vec![
            SolutionStep::new("Step 1", vec![Move::R]),
            SolutionStep::new("Step 2", vec![Move::U, Move::F]),
        ];
        let solution = Solution::new(steps, 100);

        assert_eq!(solution.step_count(), 2);
        assert_eq!(solution.move_count(), 3);
        assert_eq!(solution.time_ms, 100);
        assert!(solution.method.is_none());
    }

    #[test]
    fn test_solution_with_method() {
        let steps = vec![SolutionStep::new("Solve", vec![Move::R])];
        let solution = Solution::with_method(steps, 50, "Beginner's Method");

        assert_eq!(solution.method.as_deref(), Some("Beginner's Method"));
    }

    #[test]
    fn test_solution_all_moves() {
        let steps = vec![
            SolutionStep::new("Step 1", vec![Move::R, Move::U]),
            SolutionStep::new("Step 2", vec![Move::F]),
            SolutionStep::new("Step 3", vec![Move::D, Move::L]),
        ];
        let solution = Solution::new(steps, 100);

        let all_moves = solution.all_moves();
        assert_eq!(all_moves, vec![Move::R, Move::U, Move::F, Move::D, Move::L]);
        assert_eq!(solution.move_count(), 5);
    }

    #[test]
    fn test_solution_to_notation() {
        let steps = vec![
            SolutionStep::new("Step 1", vec![Move::R, Move::U]),
            SolutionStep::new("Step 2", vec![Move::RPrime, Move::UPrime]),
        ];
        let solution = Solution::new(steps, 100);

        assert_eq!(solution.to_notation(), "R U R' U'");
    }

    #[test]
    fn test_solution_summary() {
        let steps = vec![
            SolutionStep::new("Step 1", vec![Move::R]),
            SolutionStep::new("Step 2", vec![Move::U, Move::F]),
        ];
        let solution = Solution::with_method(steps, 150, "Test Method");

        let summary = solution.summary();
        assert!(summary.contains("Test Method"));
        assert!(summary.contains("2 steps"));
        assert!(summary.contains("3 moves"));
        assert!(summary.contains("150ms"));
    }

    #[test]
    fn test_empty_solution() {
        let solution = Solution::new(vec![], 0);
        assert_eq!(solution.step_count(), 0);
        assert_eq!(solution.move_count(), 0);
        assert_eq!(solution.to_notation(), "");
    }

    #[test]
    fn test_solution_step_empty_moves() {
        let step = SolutionStep::new("Already solved", vec![]);
        assert_eq!(step.move_count(), 0);
        assert_eq!(step.to_notation(), "");
    }
}
