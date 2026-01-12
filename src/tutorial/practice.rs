//! Interactive Practice Mode
//!
//! This module implements R6.10 from the PRD:
//! - Generate specific practice cases
//! - Hints available
//! - Check if solution is correct
//!
//! The practice mode provides a comprehensive system for students to practice
//! solving various cube scenarios with feedback, hints, and solution validation.

use crate::cube::{Cube, Move};
use crate::solver::{solve_2x2, solve_3x3};

/// Difficulty level for practice cases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    /// Very easy cases, 1-3 moves to solve
    Beginner,
    /// Easy cases, 3-5 moves to solve
    Easy,
    /// Medium cases, 5-10 moves to solve
    Medium,
    /// Hard cases, 10-15 moves to solve
    Hard,
    /// Expert cases, 15+ moves to solve
    Expert,
}

impl Difficulty {
    /// Get the scramble length range for this difficulty
    pub fn scramble_length_range(&self) -> (usize, usize) {
        match self {
            Difficulty::Beginner => (1, 3),
            Difficulty::Easy => (3, 5),
            Difficulty::Medium => (5, 10),
            Difficulty::Hard => (10, 15),
            Difficulty::Expert => (15, 25),
        }
    }

    /// Get a human-readable name for this difficulty
    pub fn name(&self) -> &'static str {
        match self {
            Difficulty::Beginner => "Beginner",
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Expert => "Expert",
        }
    }
}

/// Type of practice case
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticeType {
    /// Practice solving a specific pattern (e.g., cross, OLL, PLL)
    Pattern(String),
    /// Practice solving from a random scramble
    Random,
    /// Practice a specific algorithm
    Algorithm(String),
}

/// A practice case for the student to solve
#[derive(Debug, Clone)]
pub struct PracticeCase {
    /// Unique identifier for this case
    pub id: String,
    /// Type of practice
    pub practice_type: PracticeType,
    /// Difficulty level
    pub difficulty: Difficulty,
    /// The cube size (2 for 2x2, 3 for 3x3, etc.)
    pub cube_size: usize,
    /// Initial scramble to apply
    pub scramble: Vec<Move>,
    /// Description of the case
    pub description: String,
    /// Hint for the student (initially hidden)
    pub hint: String,
    /// Optional target state description
    pub target_description: Option<String>,
    /// Expected solution (one possible solution)
    pub expected_solution: Vec<Move>,
}

impl PracticeCase {
    /// Create a new practice case
    pub fn new(
        id: String,
        practice_type: PracticeType,
        difficulty: Difficulty,
        cube_size: usize,
        scramble: Vec<Move>,
        description: String,
        hint: String,
    ) -> Self {
        Self {
            id,
            practice_type,
            difficulty,
            cube_size,
            scramble,
            description,
            hint,
            target_description: None,
            expected_solution: Vec::new(),
        }
    }

    /// Create a practice case with an expected solution
    pub fn with_solution(mut self, solution: Vec<Move>) -> Self {
        self.expected_solution = solution;
        self
    }

    /// Create a practice case with a target description
    pub fn with_target(mut self, target_description: String) -> Self {
        self.target_description = Some(target_description);
        self
    }

    /// Get the scrambled cube for this practice case
    pub fn get_scrambled_cube(&self) -> Cube {
        let mut cube = Cube::new(self.cube_size);
        for mv in &self.scramble {
            cube.apply_move(*mv);
        }
        cube
    }

    /// Check if a sequence of moves solves the practice case
    pub fn check_solution(&self, moves: &[Move]) -> bool {
        let mut cube = self.get_scrambled_cube();

        // Apply the student's moves
        for mv in moves {
            cube.apply_move(*mv);
        }

        // Check if the cube is solved
        cube.is_solved()
    }

    /// Generate a solution for this practice case
    pub fn generate_solution(&self) -> Result<Vec<Move>, String> {
        let cube = self.get_scrambled_cube();

        match self.cube_size {
            2 => {
                let solution = solve_2x2(&cube)?;
                Ok(solution.moves)
            }
            3 => {
                let solution = solve_3x3(&cube)?;
                Ok(solution.moves)
            }
            _ => Err(format!("Solver not implemented for {}x{} cubes", self.cube_size, self.cube_size)),
        }
    }
}

/// Practice session to track student progress
#[derive(Debug, Clone)]
pub struct PracticeSession {
    /// Cases in this session
    pub cases: Vec<PracticeCase>,
    /// Current case index
    pub current_index: usize,
    /// Number of hints used
    pub hints_used: usize,
    /// Cases completed successfully
    pub cases_completed: usize,
    /// Total attempts made
    pub total_attempts: usize,
}

impl PracticeSession {
    /// Create a new practice session
    pub fn new(cases: Vec<PracticeCase>) -> Self {
        Self {
            cases,
            current_index: 0,
            hints_used: 0,
            cases_completed: 0,
            total_attempts: 0,
        }
    }

    /// Get the current practice case
    pub fn current_case(&self) -> Option<&PracticeCase> {
        self.cases.get(self.current_index)
    }

    /// Get a hint for the current case
    pub fn get_hint(&mut self) -> Option<String> {
        if let Some(case) = self.current_case() {
            let hint = case.hint.clone();
            self.hints_used += 1;
            Some(hint)
        } else {
            None
        }
    }

    /// Check if the student's solution is correct
    pub fn check_solution(&mut self, moves: &[Move]) -> bool {
        self.total_attempts += 1;

        if let Some(case) = self.current_case() {
            if case.check_solution(moves) {
                self.cases_completed += 1;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Move to the next case
    pub fn next_case(&mut self) -> bool {
        if self.current_index + 1 < self.cases.len() {
            self.current_index += 1;
            true
        } else {
            false
        }
    }

    /// Check if all cases are completed
    pub fn is_complete(&self) -> bool {
        self.current_index >= self.cases.len() - 1
    }

    /// Get the completion percentage
    pub fn completion_percentage(&self) -> f32 {
        if self.cases.is_empty() {
            100.0
        } else {
            (self.cases_completed as f32 / self.cases.len() as f32) * 100.0
        }
    }

    /// Get the total number of cases
    pub fn total_cases(&self) -> usize {
        self.cases.len()
    }
}

/// Practice case generator
pub struct PracticeGenerator;

impl PracticeGenerator {
    /// Generate a random practice case for the given difficulty and cube size
    pub fn generate_random_case(
        cube_size: usize,
        difficulty: Difficulty,
    ) -> PracticeCase {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let (min_len, max_len) = difficulty.scramble_length_range();
        let scramble_length = rng.gen_range(min_len..=max_len);

        // Generate random scramble
        let available_moves = Self::get_available_moves(cube_size);
        let mut scramble = Vec::new();
        let mut last_face = None;

        for _ in 0..scramble_length {
            let mut next_move = available_moves[rng.gen_range(0..available_moves.len())];

            // Avoid same face repetition
            if let Some(last) = last_face {
                while Self::same_face(&next_move, &last) {
                    next_move = available_moves[rng.gen_range(0..available_moves.len())];
                }
            }

            scramble.push(next_move);
            last_face = Some(next_move);
        }

        let id = format!("random_{}x{}_{}_{}", cube_size, cube_size, difficulty.name().to_lowercase(), scramble_length);
        let description = format!(
            "Solve this {}x{} cube from a {} scramble ({} moves)",
            cube_size, cube_size, difficulty.name(), scramble_length
        );
        let hint = match difficulty {
            Difficulty::Beginner => "Start with solving one face. Look for easy pieces to place first.",
            Difficulty::Easy => "Try to solve the first layer completely before moving to the next layer.",
            Difficulty::Medium => "Use the algorithms you've learned. Break it down into steps.",
            Difficulty::Hard => "Look for patterns you recognize. Use advanced techniques if you know them.",
            Difficulty::Expert => "This is a challenging scramble. Take your time and think through each step.",
        }.to_string();

        PracticeCase::new(id, PracticeType::Random, difficulty, cube_size, scramble, description, hint)
    }

    /// Generate a set of practice cases for a session
    pub fn generate_session(
        cube_size: usize,
        difficulty: Difficulty,
        count: usize,
    ) -> PracticeSession {
        let cases: Vec<PracticeCase> = (0..count)
            .map(|_| Self::generate_random_case(cube_size, difficulty))
            .collect();

        PracticeSession::new(cases)
    }

    /// Get available moves for a cube size
    fn get_available_moves(cube_size: usize) -> Vec<Move> {
        use Move::*;
        match cube_size {
            2 => vec![R, RPrime, R2, U, UPrime, U2, F, FPrime, F2],
            _ => vec![R, RPrime, R2, L, LPrime, L2, U, UPrime, U2, D, DPrime, D2, F, FPrime, F2, B, BPrime, B2],
        }
    }

    /// Check if two moves affect the same face
    fn same_face(move1: &Move, move2: &Move) -> bool {
        use Move::*;
        matches!(
            (move1, move2),
            (R, R) | (R, RPrime) | (R, R2) | (RPrime, R) | (RPrime, RPrime) | (RPrime, R2) | (R2, R) | (R2, RPrime) | (R2, R2) |
            (L, L) | (L, LPrime) | (L, L2) | (LPrime, L) | (LPrime, LPrime) | (LPrime, L2) | (L2, L) | (L2, LPrime) | (L2, L2) |
            (U, U) | (U, UPrime) | (U, U2) | (UPrime, U) | (UPrime, UPrime) | (UPrime, U2) | (U2, U) | (U2, UPrime) | (U2, U2) |
            (D, D) | (D, DPrime) | (D, D2) | (DPrime, D) | (DPrime, DPrime) | (DPrime, D2) | (D2, D) | (D2, DPrime) | (D2, D2) |
            (F, F) | (F, FPrime) | (F, F2) | (FPrime, F) | (FPrime, FPrime) | (FPrime, F2) | (F2, F) | (F2, FPrime) | (F2, F2) |
            (B, B) | (B, BPrime) | (B, B2) | (BPrime, B) | (BPrime, BPrime) | (BPrime, B2) | (B2, B) | (B2, BPrime) | (B2, B2)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_levels() {
        assert_eq!(Difficulty::Beginner.name(), "Beginner");
        assert_eq!(Difficulty::Easy.name(), "Easy");
        assert_eq!(Difficulty::Medium.name(), "Medium");
        assert_eq!(Difficulty::Hard.name(), "Hard");
        assert_eq!(Difficulty::Expert.name(), "Expert");
    }

    #[test]
    fn test_difficulty_scramble_ranges() {
        assert_eq!(Difficulty::Beginner.scramble_length_range(), (1, 3));
        assert_eq!(Difficulty::Easy.scramble_length_range(), (3, 5));
        assert_eq!(Difficulty::Medium.scramble_length_range(), (5, 10));
        assert_eq!(Difficulty::Hard.scramble_length_range(), (10, 15));
        assert_eq!(Difficulty::Expert.scramble_length_range(), (15, 25));
    }

    #[test]
    fn test_practice_case_creation() {
        let case = PracticeCase::new(
            "test_1".to_string(),
            PracticeType::Random,
            Difficulty::Easy,
            3,
            vec![Move::R, Move::U],
            "Test case".to_string(),
            "Test hint".to_string(),
        );

        assert_eq!(case.id, "test_1");
        assert_eq!(case.difficulty, Difficulty::Easy);
        assert_eq!(case.cube_size, 3);
        assert_eq!(case.scramble.len(), 2);
    }

    #[test]
    fn test_practice_case_scrambled_cube() {
        let case = PracticeCase::new(
            "test_2".to_string(),
            PracticeType::Random,
            Difficulty::Beginner,
            2,
            vec![Move::R, Move::U, Move::R],
            "Test case".to_string(),
            "Test hint".to_string(),
        );

        let cube = case.get_scrambled_cube();
        assert_eq!(cube.size(), 2);
        assert!(!cube.is_solved());
    }

    #[test]
    fn test_check_solution_correct() {
        let case = PracticeCase::new(
            "test_3".to_string(),
            PracticeType::Random,
            Difficulty::Beginner,
            2,
            vec![Move::R],
            "Test case".to_string(),
            "Test hint".to_string(),
        );

        // Reverse of R is R'
        assert!(case.check_solution(&[Move::RPrime]));
    }

    #[test]
    fn test_check_solution_incorrect() {
        let case = PracticeCase::new(
            "test_4".to_string(),
            PracticeType::Random,
            Difficulty::Beginner,
            2,
            vec![Move::R],
            "Test case".to_string(),
            "Test hint".to_string(),
        );

        // U doesn't solve R scramble
        assert!(!case.check_solution(&[Move::U]));
    }

    #[test]
    fn test_practice_session_creation() {
        let cases = vec![
            PracticeCase::new(
                "1".to_string(),
                PracticeType::Random,
                Difficulty::Easy,
                2,
                vec![Move::R],
                "Case 1".to_string(),
                "Hint 1".to_string(),
            ),
            PracticeCase::new(
                "2".to_string(),
                PracticeType::Random,
                Difficulty::Easy,
                2,
                vec![Move::U],
                "Case 2".to_string(),
                "Hint 2".to_string(),
            ),
        ];

        let session = PracticeSession::new(cases);
        assert_eq!(session.total_cases(), 2);
        assert_eq!(session.current_index, 0);
        assert_eq!(session.cases_completed, 0);
    }

    #[test]
    fn test_practice_session_navigation() {
        let cases = vec![
            PracticeCase::new(
                "1".to_string(),
                PracticeType::Random,
                Difficulty::Easy,
                2,
                vec![Move::R],
                "Case 1".to_string(),
                "Hint 1".to_string(),
            ),
            PracticeCase::new(
                "2".to_string(),
                PracticeType::Random,
                Difficulty::Easy,
                2,
                vec![Move::U],
                "Case 2".to_string(),
                "Hint 2".to_string(),
            ),
        ];

        let mut session = PracticeSession::new(cases);
        assert!(session.next_case());
        assert_eq!(session.current_index, 1);
        assert!(!session.next_case()); // Already at last case
    }

    #[test]
    fn test_practice_session_hints() {
        let cases = vec![
            PracticeCase::new(
                "1".to_string(),
                PracticeType::Random,
                Difficulty::Easy,
                2,
                vec![Move::R],
                "Case 1".to_string(),
                "Test hint".to_string(),
            ),
        ];

        let mut session = PracticeSession::new(cases);
        assert_eq!(session.hints_used, 0);

        let hint = session.get_hint();
        assert!(hint.is_some());
        assert_eq!(hint.unwrap(), "Test hint".to_string());
        assert_eq!(session.hints_used, 1);
    }

    #[test]
    fn test_practice_session_completion() {
        let cases = vec![
            PracticeCase::new(
                "1".to_string(),
                PracticeType::Random,
                Difficulty::Easy,
                2,
                vec![Move::R],
                "Case 1".to_string(),
                "Hint 1".to_string(),
            ),
            PracticeCase::new(
                "2".to_string(),
                PracticeType::Random,
                Difficulty::Easy,
                2,
                vec![Move::U],
                "Case 2".to_string(),
                "Hint 2".to_string(),
            ),
        ];

        let mut session = PracticeSession::new(cases);
        assert!(!session.is_complete());

        // Move to last case
        session.next_case();
        assert!(session.is_complete());

        // Solve the case
        session.check_solution(&[Move::UPrime]);
        assert_eq!(session.cases_completed, 1);
        assert_eq!(session.completion_percentage(), 50.0);
    }

    #[test]
    fn test_generate_random_case() {
        let case = PracticeGenerator::generate_random_case(2, Difficulty::Easy);
        assert_eq!(case.cube_size, 2);
        assert_eq!(case.difficulty, Difficulty::Easy);
        assert!(!case.scramble.is_empty());

        // Scramble length should be in the easy range
        let (min, max) = Difficulty::Easy.scramble_length_range();
        assert!(case.scramble.len() >= min && case.scramble.len() <= max);
    }

    #[test]
    fn test_generate_session() {
        let session = PracticeGenerator::generate_session(2, Difficulty::Medium, 5);
        assert_eq!(session.total_cases(), 5);

        for case in &session.cases {
            assert_eq!(case.cube_size, 2);
            assert_eq!(case.difficulty, Difficulty::Medium);
        }
    }

    #[test]
    fn test_case_with_solution() {
        let case = PracticeCase::new(
            "test".to_string(),
            PracticeType::Random,
            Difficulty::Easy,
            2,
            vec![Move::R],
            "Test".to_string(),
            "Hint".to_string(),
        ).with_solution(vec![Move::RPrime]);

        assert_eq!(case.expected_solution, vec![Move::RPrime]);
    }

    #[test]
    fn test_case_with_target() {
        let case = PracticeCase::new(
            "test".to_string(),
            PracticeType::Pattern("Cross".to_string()),
            Difficulty::Medium,
            3,
            vec![Move::R, Move::U],
            "Test".to_string(),
            "Hint".to_string(),
        ).with_target("Solve the white cross".to_string());

        assert_eq!(case.target_description, Some("Solve the white cross".to_string()));
    }
}
