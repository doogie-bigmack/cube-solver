//! 2x2 Tutorial Lesson
//!
//! This module implements R6.8 from the PRD:
//! - Ortega method (beginner-friendly 2x2 solving method)
//! - Step-by-step instructions
//! - Practice mode

use crate::cube::{Cube, Move};
use crate::solver::solve_2x2;

/// Represents a single step in the 2x2 tutorial
#[derive(Debug, Clone, PartialEq)]
pub struct TwoByTwoLessonStep {
    /// Title of the step
    pub title: String,
    /// Description/explanation
    pub description: String,
    /// Algorithm to demonstrate (if applicable)
    pub algorithm: Option<Vec<Move>>,
    /// Visual aid description
    pub visual_hint: String,
    /// Kid-friendly explanation
    pub kid_friendly_text: String,
}

/// Represents a specific case in the Ortega method
#[derive(Debug, Clone, PartialEq)]
pub struct OrtegaCase {
    /// Name of the case
    pub name: String,
    /// Description of when to use it
    pub description: String,
    /// Algorithm to solve this case
    pub algorithm: Vec<Move>,
    /// How to recognize this case
    pub recognition: String,
}

/// Represents a practice exercise for the 2x2 tutorial
#[derive(Debug, Clone, PartialEq)]
pub struct TwoByTwoPracticeExercise {
    /// Description of the practice task
    pub description: String,
    /// Initial cube state (scrambled)
    pub scramble: Vec<Move>,
    /// Hint for solving
    pub hint: String,
    /// Expected solution (one possible solution)
    pub solution: Vec<Move>,
}

/// The complete 2x2 tutorial lesson using the Ortega method
#[derive(Debug, Clone)]
pub struct TwoByTwoLesson {
    /// Lesson steps
    pub steps: Vec<TwoByTwoLessonStep>,
    /// Ortega method cases
    pub ortega_cases: Vec<OrtegaCase>,
    /// Practice exercises
    pub practice_exercises: Vec<TwoByTwoPracticeExercise>,
}

impl TwoByTwoLesson {
    /// Creates a new 2x2 tutorial lesson
    pub fn new() -> Self {
        Self {
            steps: vec![
                Self::intro_step(),
                Self::differences_from_3x3_step(),
                Self::ortega_method_overview_step(),
                Self::step1_first_face_step(),
                Self::step2_orient_last_layer_step(),
                Self::step3_permute_last_layer_step(),
                Self::practice_step(),
            ],
            ortega_cases: vec![
                Self::oll_case_h(),
                Self::oll_case_pi(),
                Self::oll_case_sune(),
                Self::oll_case_antisune(),
                Self::pll_case_adjacent_swap(),
                Self::pll_case_diagonal_swap(),
            ],
            practice_exercises: vec![
                Self::practice_easy(),
                Self::practice_medium(),
                Self::practice_hard(),
            ],
        }
    }

    /// Introduction to 2x2 solving
    fn intro_step() -> TwoByTwoLessonStep {
        TwoByTwoLessonStep {
            title: "Welcome to 2x2 Solving!".to_string(),
            description: "The 2x2 cube (also called the Pocket Cube) is easier to solve than the 3x3 because it only has corners - no edges or centers!".to_string(),
            algorithm: None,
            visual_hint: "A 2x2 cube has 8 corner pieces, compared to the 3x3's 26 pieces.".to_string(),
            kid_friendly_text: "Think of the 2x2 as a 3x3 with just the corners. It's like solving only the corner pieces of a 3x3!".to_string(),
        }
    }

    /// Explain differences from 3x3
    fn differences_from_3x3_step() -> TwoByTwoLessonStep {
        TwoByTwoLessonStep {
            title: "How 2x2 is Different".to_string(),
            description: "Unlike the 3x3, the 2x2 has no fixed center pieces, so you can start with any color as the first face. There's also no concept of 'edges' - only corners!".to_string(),
            algorithm: None,
            visual_hint: "Any color can be the 'first face' since there are no center pieces to tell you which color should be on top.".to_string(),
            kid_friendly_text: "On a 3x3, you always know which color goes where because of the center pieces. On a 2x2, you get to pick any color to start with!".to_string(),
        }
    }

    /// Explain the Ortega method
    fn ortega_method_overview_step() -> TwoByTwoLessonStep {
        TwoByTwoLessonStep {
            title: "The Ortega Method".to_string(),
            description: "We'll use the Ortega method, which has 3 simple steps: 1) Solve one face completely, 2) Orient the opposite face (all same color facing up), 3) Permute the corners into their correct positions.".to_string(),
            algorithm: None,
            visual_hint: "Ortega is one of the fastest beginner methods and uses only a few easy algorithms.".to_string(),
            kid_friendly_text: "The Ortega method is like building a sandwich: first layer (bottom bread), then orientation (the filling), then permutation (top bread in the right place)!".to_string(),
        }
    }

    /// Step 1: First face
    fn step1_first_face_step() -> TwoByTwoLessonStep {
        TwoByTwoLessonStep {
            title: "Step 1: Solve One Face".to_string(),
            description: "Pick any color and solve that entire face. This is done intuitively - just turn the cube until all 4 corners on one face are the same color. The opposite face doesn't need to match yet.".to_string(),
            algorithm: None,
            visual_hint: "Try to solve the white face first. Look for white corners and move them to the white face using R, U, and F moves.".to_string(),
            kid_friendly_text: "This is the easiest part! Just play around with the cube until you get all 4 white pieces on one face. Don't worry about the other side yet.".to_string(),
        }
    }

    /// Step 2: Orient last layer
    fn step2_orient_last_layer_step() -> TwoByTwoLessonStep {
        TwoByTwoLessonStep {
            title: "Step 2: Orient Last Layer (OLL)".to_string(),
            description: "Now make the opposite face all one color (usually yellow if you started with white). You might need to use one of the OLL algorithms to flip corners so they all face the same way.".to_string(),
            algorithm: Some(vec![Move::R, Move::U, Move::RPrime, Move::UPrime, Move::RPrime, Move::F, Move::R, Move::FPrime]), // Sune algorithm
            visual_hint: "Look at the top face. If not all pieces are the same color, use an OLL algorithm like Sune to orient them.".to_string(),
            kid_friendly_text: "OLL stands for 'Orient Last Layer'. We want all yellow (or your chosen color) pieces to face up, even if they're not in the right positions yet.".to_string(),
        }
    }

    /// Step 3: Permute last layer
    fn step3_permute_last_layer_step() -> TwoByTwoLessonStep {
        TwoByTwoLessonStep {
            title: "Step 3: Permute Last Layer (PLL)".to_string(),
            description: "Finally, move the top corners to their correct positions. If two corners need to swap, use a PLL algorithm. You might need to do this twice.".to_string(),
            algorithm: Some(vec![Move::R, Move::U, Move::RPrime, Move::UPrime, Move::RPrime, Move::F, Move::R2, Move::UPrime, Move::RPrime, Move::UPrime, Move::R, Move::U, Move::RPrime, Move::FPrime]), // Y-perm
            visual_hint: "Look at the sides of the cube. If two adjacent corners need to swap, use the Y-perm algorithm shown above.".to_string(),
            kid_friendly_text: "PLL stands for 'Permute Last Layer'. Now we're moving pieces to their correct homes, like solving a puzzle!".to_string(),
        }
    }

    /// Practice recommendations
    fn practice_step() -> TwoByTwoLessonStep {
        TwoByTwoLessonStep {
            title: "Practice Makes Perfect!".to_string(),
            description: "Try solving the practice exercises below. Start with easy scrambles and work your way up to harder ones. Soon you'll be able to solve any 2x2 cube!".to_string(),
            algorithm: None,
            visual_hint: "The more you practice, the faster you'll recognize patterns and know which algorithm to use.".to_string(),
            kid_friendly_text: "Just like learning to ride a bike, solving a 2x2 gets easier with practice. Try the exercises and challenge yourself!".to_string(),
        }
    }

    // OLL Cases

    /// OLL case: H pattern
    fn oll_case_h() -> OrtegaCase {
        OrtegaCase {
            name: "H Pattern".to_string(),
            description: "Two opposite corners need to be flipped".to_string(),
            algorithm: vec![Move::F, Move::R, Move::U, Move::RPrime, Move::UPrime, Move::FPrime],
            recognition: "Look for two corners diagonally opposite that need flipping".to_string(),
        }
    }

    /// OLL case: Pi pattern
    fn oll_case_pi() -> OrtegaCase {
        OrtegaCase {
            name: "Pi Pattern".to_string(),
            description: "Two adjacent corners on same side need to be flipped".to_string(),
            algorithm: vec![Move::F, Move::R, Move::U, Move::RPrime, Move::UPrime, Move::R, Move::U, Move::RPrime, Move::UPrime, Move::FPrime],
            recognition: "Look for two corners next to each other that need flipping".to_string(),
        }
    }

    /// OLL case: Sune
    fn oll_case_sune() -> OrtegaCase {
        OrtegaCase {
            name: "Sune".to_string(),
            description: "One corner in front-right needs orienting".to_string(),
            algorithm: vec![Move::R, Move::U, Move::RPrime, Move::UPrime, Move::RPrime, Move::F, Move::R, Move::FPrime],
            recognition: "One corner piece sticks out differently from the others".to_string(),
        }
    }

    /// OLL case: Antisune
    fn oll_case_antisune() -> OrtegaCase {
        OrtegaCase {
            name: "Antisune".to_string(),
            description: "Mirror of Sune - one corner in front-left needs orienting".to_string(),
            algorithm: vec![Move::FPrime, Move::UPrime, Move::F, Move::U, Move::F, Move::RPrime, Move::FPrime, Move::R],
            recognition: "Like Sune but mirrored - the odd corner is on the left".to_string(),
        }
    }

    // PLL Cases

    /// PLL case: Adjacent swap
    fn pll_case_adjacent_swap() -> OrtegaCase {
        OrtegaCase {
            name: "Adjacent Swap (Y-Perm)".to_string(),
            description: "Two adjacent corners need to swap positions".to_string(),
            algorithm: vec![Move::R, Move::U, Move::RPrime, Move::UPrime, Move::RPrime, Move::F, Move::R2, Move::UPrime, Move::RPrime, Move::UPrime, Move::R, Move::U, Move::RPrime, Move::FPrime],
            recognition: "Two corners next to each other are in the wrong positions".to_string(),
        }
    }

    /// PLL case: Diagonal swap
    fn pll_case_diagonal_swap() -> OrtegaCase {
        OrtegaCase {
            name: "Diagonal Swap".to_string(),
            description: "Two diagonal corners need to swap positions".to_string(),
            algorithm: vec![Move::F, Move::R, Move::UPrime, Move::RPrime, Move::UPrime, Move::R, Move::U, Move::RPrime, Move::FPrime, Move::R, Move::U, Move::RPrime, Move::UPrime, Move::RPrime, Move::F, Move::R, Move::FPrime],
            recognition: "Two corners diagonally opposite are in the wrong positions".to_string(),
        }
    }

    // Practice Exercises

    /// Easy practice exercise
    fn practice_easy() -> TwoByTwoPracticeExercise {
        TwoByTwoPracticeExercise {
            description: "Easy: Simple 3-move scramble".to_string(),
            scramble: vec![Move::R, Move::U, Move::R],
            hint: "Try to solve the white face first, then work on orienting the yellow face.".to_string(),
            solution: vec![Move::RPrime, Move::UPrime, Move::RPrime], // One possible solution
        }
    }

    /// Medium practice exercise
    fn practice_medium() -> TwoByTwoPracticeExercise {
        TwoByTwoPracticeExercise {
            description: "Medium: 6-move scramble with rotation".to_string(),
            scramble: vec![Move::R, Move::U, Move::R2, Move::U2, Move::RPrime, Move::U],
            hint: "After solving the first face, you'll likely need an OLL algorithm.".to_string(),
            solution: vec![Move::UPrime, Move::R, Move::U2, Move::R2, Move::UPrime, Move::RPrime], // One possible solution
        }
    }

    /// Hard practice exercise
    fn practice_hard() -> TwoByTwoPracticeExercise {
        TwoByTwoPracticeExercise {
            description: "Hard: Complex 9-move scramble".to_string(),
            scramble: vec![Move::R, Move::U2, Move::R, Move::U, Move::R, Move::UPrime, Move::R2, Move::U2, Move::R2],
            hint: "This will require all three steps: first face, OLL, and PLL. Take your time!".to_string(),
            solution: vec![], // Let the solver figure it out
        }
    }

    /// Get the number of steps in the lesson
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Get the number of Ortega cases
    pub fn case_count(&self) -> usize {
        self.ortega_cases.len()
    }

    /// Get the number of practice exercises
    pub fn exercise_count(&self) -> usize {
        self.practice_exercises.len()
    }

    /// Solve a practice exercise using the 2x2 solver
    pub fn solve_practice(&self, exercise_index: usize) -> Result<Vec<Move>, String> {
        if exercise_index >= self.practice_exercises.len() {
            return Err("Invalid exercise index".to_string());
        }

        let exercise = &self.practice_exercises[exercise_index];
        let mut cube = Cube::new(2);

        // Apply scramble
        for mv in &exercise.scramble {
            cube.apply_move(*mv);
        }

        // Solve using the 2x2 solver
        let solution = solve_2x2(&cube)?;
        Ok(solution.moves)
    }
}

impl Default for TwoByTwoLesson {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lesson_creation() {
        let lesson = TwoByTwoLesson::new();
        assert_eq!(lesson.step_count(), 7);
        assert!(lesson.case_count() > 0);
        assert!(lesson.exercise_count() > 0);
    }

    #[test]
    fn test_has_all_steps() {
        let lesson = TwoByTwoLesson::new();
        assert!(lesson.steps.iter().any(|s| s.title.contains("Welcome")));
        assert!(lesson.steps.iter().any(|s| s.title.contains("Ortega")));
        assert!(lesson.steps.iter().any(|s| s.title.contains("Solve One Face") || s.title.contains("First Face")));
        assert!(lesson.steps.iter().any(|s| s.title.contains("Orient")));
        assert!(lesson.steps.iter().any(|s| s.title.contains("Permute")));
    }

    #[test]
    fn test_has_oll_cases() {
        let lesson = TwoByTwoLesson::new();
        assert!(lesson.ortega_cases.iter().any(|c| c.name.contains("Sune")));
        assert!(lesson.ortega_cases.iter().any(|c| c.name.contains("Antisune")));
        assert!(lesson.ortega_cases.iter().any(|c| c.name.contains("Pattern")));
    }

    #[test]
    fn test_has_pll_cases() {
        let lesson = TwoByTwoLesson::new();
        assert!(lesson.ortega_cases.iter().any(|c| c.name.contains("Adjacent")));
        assert!(lesson.ortega_cases.iter().any(|c| c.name.contains("Diagonal")));
    }

    #[test]
    fn test_practice_exercises() {
        let lesson = TwoByTwoLesson::new();
        assert_eq!(lesson.exercise_count(), 3);

        // Check easy exercise
        assert!(lesson.practice_exercises[0].description.contains("Easy"));
        assert!(!lesson.practice_exercises[0].scramble.is_empty());

        // Check medium exercise
        assert!(lesson.practice_exercises[1].description.contains("Medium"));

        // Check hard exercise
        assert!(lesson.practice_exercises[2].description.contains("Hard"));
    }

    #[test]
    fn test_solve_easy_practice() {
        let lesson = TwoByTwoLesson::new();
        let result = lesson.solve_practice(0);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_solve_medium_practice() {
        let lesson = TwoByTwoLesson::new();
        let result = lesson.solve_practice(1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_solve_hard_practice() {
        let lesson = TwoByTwoLesson::new();
        let result = lesson.solve_practice(2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_exercise_index() {
        let lesson = TwoByTwoLesson::new();
        let result = lesson.solve_practice(99);
        assert!(result.is_err());
    }

    #[test]
    fn test_all_algorithms_valid() {
        let lesson = TwoByTwoLesson::new();

        // Check that all case algorithms are non-empty
        for case in &lesson.ortega_cases {
            assert!(!case.algorithm.is_empty(), "Case '{}' has empty algorithm", case.name);
        }

        // Check that step algorithms (if present) are non-empty
        for step in &lesson.steps {
            if let Some(ref alg) = step.algorithm {
                assert!(!alg.is_empty(), "Step '{}' has empty algorithm", step.title);
            }
        }
    }

    #[test]
    fn test_kid_friendly_text_present() {
        let lesson = TwoByTwoLesson::new();

        // All steps should have kid-friendly text
        for step in &lesson.steps {
            assert!(!step.kid_friendly_text.is_empty(), "Step '{}' missing kid-friendly text", step.title);
        }
    }
}
