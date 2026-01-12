//! 3x3 tutorial: PLL (Permute Last Layer)
//!
//! This module implements R6.7 from the PRD:
//! - 2-look PLL algorithms
//! - Pattern recognition
//! - Practice mode

use crate::cube::Move;

/// Represents different PLL patterns/cases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PllPattern {
    /// All corners in correct positions (edges only need permuting)
    CornersCorrect,
    /// Headlights pattern (two adjacent corners need swapping)
    Headlights,
    /// Diagonal corners swap (opposite corners need swapping)
    DiagonalSwap,
    /// All edges in correct positions (corners only need permuting)
    EdgesCorrect,
    /// Adjacent edge swap (Ua perm)
    UaPerm,
    /// Adjacent edge swap opposite direction (Ub perm)
    UbPerm,
    /// Opposite edge swap (H perm)
    HPerm,
    /// Z perm pattern
    ZPerm,
}

/// Represents a PLL solving case/algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct PllCase {
    /// Name of the case
    pub name: String,
    /// Description of the pattern
    pub description: String,
    /// The algorithm (sequence of moves)
    pub algorithm: Vec<Move>,
    /// Kid-friendly explanation
    pub explanation: String,
    /// Which PLL pattern this case addresses
    pub pattern: PllPattern,
    /// Visual representation hint
    pub visual_hint: String,
}

/// Represents a single lesson step for the PLL tutorial
#[derive(Debug, Clone, PartialEq)]
pub struct PllLessonStep {
    /// Title of the step
    pub title: String,
    /// Description/explanation
    pub description: String,
    /// Optional example moves
    pub example_moves: Option<Vec<Move>>,
    /// Kid-friendly explanation
    pub kid_friendly_text: String,
    /// Optional visual cue or tip
    pub tip: Option<String>,
}

/// Practice exercise for PLL
#[derive(Debug, Clone, PartialEq)]
pub struct PllPracticeExercise {
    /// Title of the exercise
    pub title: String,
    /// Description of what to do
    pub description: String,
    /// Scramble to set up the exercise
    pub setup_moves: Vec<Move>,
    /// Expected solution (one possible solution)
    pub solution: Vec<Move>,
    /// Hint for the student
    pub hint: String,
    /// Which pattern this creates
    pub pattern: PllPattern,
}

/// The complete PLL tutorial lesson
#[derive(Debug, Clone)]
pub struct PllLesson {
    /// Lesson steps
    pub steps: Vec<PllLessonStep>,
    /// Common PLL cases/algorithms (2-look PLL)
    pub cases: Vec<PllCase>,
    /// Practice exercises
    pub practice_exercises: Vec<PllPracticeExercise>,
}

impl PllLesson {
    /// Creates a new PLL lesson with all steps
    pub fn new() -> Self {
        Self {
            steps: vec![
                Self::intro_step(),
                Self::what_is_pll_step(),
                Self::two_look_pll_explanation_step(),
                Self::corner_permutation_step(),
                Self::edge_permutation_step(),
                Self::pattern_recognition_step(),
                Self::practice_step(),
            ],
            cases: vec![
                // Step 1: Corner permutation
                Self::aa_perm(),
                Self::e_perm(),
                // Step 2: Edge permutation
                Self::ua_perm(),
                Self::ub_perm(),
                Self::h_perm(),
                Self::z_perm(),
            ],
            practice_exercises: vec![
                Self::practice_aa_perm(),
                Self::practice_ua_perm(),
                Self::practice_ub_perm(),
                Self::practice_h_perm(),
            ],
        }
    }

    // Lesson steps

    fn intro_step() -> PllLessonStep {
        PllLessonStep {
            title: "Welcome to PLL!".to_string(),
            description: "PLL stands for 'Permute Last Layer'. This is the final step where we move the pieces on the top layer into their correct positions. We'll use a beginner-friendly method called '2-look PLL' that breaks it into two easy steps.".to_string(),
            example_moves: None,
            kid_friendly_text: "Think of PLL like a sliding puzzle! All the yellow stickers are facing up already, now we just need to slide the pieces to their correct spots.".to_string(),
            tip: Some("After PLL, your cube will be completely solved! This is the final step!".to_string()),
        }
    }

    fn what_is_pll_step() -> PllLessonStep {
        PllLessonStep {
            title: "What is PLL?".to_string(),
            description: "After OLL, all the yellow stickers face up, but the pieces are in the wrong positions. PLL is the step where we permute (move) the pieces to their correct spots without messing up the yellow face.".to_string(),
            example_moves: None,
            kid_friendly_text: "Right now, all the yellows face up, but the colors on the sides don't match. PLL fixes that!".to_string(),
            tip: Some("Look at the side colors, not the yellow - that's the key to recognizing PLL cases!".to_string()),
        }
    }

    fn two_look_pll_explanation_step() -> PllLessonStep {
        PllLessonStep {
            title: "2-Look PLL Method".to_string(),
            description: "Instead of learning 21 algorithms, beginners use 2-look PLL which only needs about 6 algorithms. Step 1: Position the corners correctly. Step 2: Position the edges correctly.".to_string(),
            example_moves: None,
            kid_friendly_text: "We split PLL into two parts: First put the corners in the right spots, then fix the edges!".to_string(),
            tip: Some("Advanced solvers use 1-look PLL with 21 algorithms, but 2-look is much easier to learn!".to_string()),
        }
    }

    fn corner_permutation_step() -> PllLessonStep {
        PllLessonStep {
            title: "Step 1: Corner Permutation".to_string(),
            description: "The first step is to get all four corner pieces into their correct positions. Look at the side colors to identify if corners need to swap.".to_string(),
            example_moves: Some(vec![
                Move::X, Move::RPrime, Move::U, Move::RPrime, Move::D2,
                Move::R, Move::UPrime, Move::RPrime, Move::D2, Move::R2, Move::XPrime
            ]),
            kid_friendly_text: "Look at the corners from the side - do any two match colors? We'll use special algorithms to swap them into place!".to_string(),
            tip: Some("Look for 'headlights' - two corners with matching colors on one side!".to_string()),
        }
    }

    fn edge_permutation_step() -> PllLessonStep {
        PllLessonStep {
            title: "Step 2: Edge Permutation".to_string(),
            description: "Once corners are correct, permute the edge pieces so all side colors match. This completes PLL and solves the cube!".to_string(),
            example_moves: Some(vec![
                Move::R, Move::UPrime, Move::R, Move::U, Move::R, Move::U,
                Move::R, Move::UPrime, Move::RPrime, Move::UPrime, Move::R2
            ]),
            kid_friendly_text: "Now swap the edges! The most common ones are called Ua and Ub - they cycle three edges around.".to_string(),
            tip: Some("After this step, your cube is solved! Time to celebrate!".to_string()),
        }
    }

    fn pattern_recognition_step() -> PllLessonStep {
        PllLessonStep {
            title: "Pattern Recognition".to_string(),
            description: "Learning to quickly recognize which PLL case you have is crucial. Look at the side colors (not yellow) to identify matching pieces and what needs to swap.".to_string(),
            example_moves: None,
            kid_friendly_text: "With practice, you'll spot patterns super fast! Look for headlights, matching edges, or if pieces are diagonal.".to_string(),
            tip: Some("Start by checking corners: Are they all correct? Do two need to swap? Then check edges!".to_string()),
        }
    }

    fn practice_step() -> PllLessonStep {
        PllLessonStep {
            title: "Practice Time!".to_string(),
            description: "Now it's time to practice! Try the exercises below to master PLL. Remember, it's okay to look up the algorithms - with practice, you'll memorize them naturally.".to_string(),
            example_moves: None,
            kid_friendly_text: "Let's practice! Start with the basic algorithms and work your way up. You're almost a cuber!".to_string(),
            tip: Some("Practice one algorithm until it feels natural, then add the next. Muscle memory is key!".to_string()),
        }
    }

    // PLL Cases

    // Step 1: Corner Permutation

    fn aa_perm() -> PllCase {
        use Move::*;
        PllCase {
            name: "Aa Perm (Adjacent Corner Swap)".to_string(),
            description: "Swaps two adjacent corners - one of the most common PLL cases".to_string(),
            algorithm: vec![
                XPrime, R, UPrime, R, D2, RPrime, U, R, D2, R2, X
            ],
            explanation: "This swaps two corners next to each other. Hold the headlights on the left and execute!".to_string(),
            pattern: PllPattern::Headlights,
            visual_hint: "Two corners match on one side (headlights), two corners need to swap".to_string(),
        }
    }

    fn e_perm() -> PllCase {
        use Move::*;
        PllCase {
            name: "E Perm (Diagonal Corner Swap)".to_string(),
            description: "Swaps two diagonal corners and also swaps all four edges".to_string(),
            algorithm: vec![
                XPrime, RPrime, U, RPrime, D2, R, UPrime, RPrime, D2, R2, X
            ],
            explanation: "This is for when diagonal corners need to swap. Trickier to recognize but super useful!".to_string(),
            pattern: PllPattern::DiagonalSwap,
            visual_hint: "No headlights - opposite corners need to swap diagonally".to_string(),
        }
    }

    // Step 2: Edge Permutation

    fn ua_perm() -> PllCase {
        use Move::*;
        PllCase {
            name: "Ua Perm".to_string(),
            description: "Cycles three edges clockwise - one of the two most common edge permutations".to_string(),
            algorithm: vec![
                R, UPrime, R, U, R, U, R, UPrime, RPrime, UPrime, R2
            ],
            explanation: "Ua cycles three edges around. Hold the solved edge in back and go!".to_string(),
            pattern: PllPattern::UaPerm,
            visual_hint: "One edge correct, three edges cycle clockwise".to_string(),
        }
    }

    fn ub_perm() -> PllCase {
        use Move::*;
        PllCase {
            name: "Ub Perm".to_string(),
            description: "Cycles three edges counter-clockwise - mirror of Ua perm".to_string(),
            algorithm: vec![
                R2, U, R, U, RPrime, UPrime, RPrime, UPrime, RPrime, U, RPrime
            ],
            explanation: "Ub is the opposite of Ua - cycles edges the other way. Hold solved edge in back!".to_string(),
            pattern: PllPattern::UbPerm,
            visual_hint: "One edge correct, three edges cycle counter-clockwise".to_string(),
        }
    }

    fn h_perm() -> PllCase {
        use Move::*;
        PllCase {
            name: "H Perm".to_string(),
            description: "Swaps opposite edges on two sides - makes an H pattern".to_string(),
            algorithm: vec![
                M2, U, M2, U2, M2, U, M2
            ],
            explanation: "H perm is super easy to remember - just M2 U M2 U2 M2 U M2!".to_string(),
            pattern: PllPattern::HPerm,
            visual_hint: "Opposite edges swap on front/back and left/right (H shape)".to_string(),
        }
    }

    fn z_perm() -> PllCase {
        use Move::*;
        PllCase {
            name: "Z Perm".to_string(),
            description: "Swaps adjacent edges on two opposite sides - makes a Z pattern".to_string(),
            algorithm: vec![
                M, UPrime, M2, UPrime, M2, UPrime, MPrime, U2, M2
            ],
            explanation: "Z perm swaps edges diagonally. Hold the correct edges on left/right!".to_string(),
            pattern: PllPattern::ZPerm,
            visual_hint: "Adjacent edges swap diagonally (Z shape when viewing sides)".to_string(),
        }
    }

    // Practice exercises

    fn practice_aa_perm() -> PllPracticeExercise {
        use Move::*;
        PllPracticeExercise {
            title: "Practice: Aa Perm".to_string(),
            description: "Practice swapping adjacent corners with the Aa perm".to_string(),
            setup_moves: vec![X, RPrime, UPrime, R, D2, RPrime, U, R, D2, R2, XPrime],
            solution: vec![XPrime, R, UPrime, R, D2, RPrime, U, R, D2, R2, X],
            hint: "Look for headlights on the left side, then execute: x' R U' R D2 R' U R D2 R2 x".to_string(),
            pattern: PllPattern::Headlights,
        }
    }

    fn practice_ua_perm() -> PllPracticeExercise {
        use Move::*;
        PllPracticeExercise {
            title: "Practice: Ua Perm".to_string(),
            description: "Practice cycling three edges clockwise".to_string(),
            setup_moves: vec![R2, U, R, U, RPrime, UPrime, RPrime, UPrime, RPrime, U, RPrime],
            solution: vec![R, UPrime, R, U, R, U, R, UPrime, RPrime, UPrime, R2],
            hint: "Put the solved edge in back, then: R U' R U R U R U' R' U' R2".to_string(),
            pattern: PllPattern::UaPerm,
        }
    }

    fn practice_ub_perm() -> PllPracticeExercise {
        use Move::*;
        PllPracticeExercise {
            title: "Practice: Ub Perm".to_string(),
            description: "Practice cycling three edges counter-clockwise".to_string(),
            setup_moves: vec![R, UPrime, R, U, R, U, R, UPrime, RPrime, UPrime, R2],
            solution: vec![R2, U, R, U, RPrime, UPrime, RPrime, UPrime, RPrime, U, RPrime],
            hint: "Put the solved edge in back, then: R2 U R U R' U' R' U' R' U R'".to_string(),
            pattern: PllPattern::UbPerm,
        }
    }

    fn practice_h_perm() -> PllPracticeExercise {
        use Move::*;
        PllPracticeExercise {
            title: "Practice: H Perm".to_string(),
            description: "Practice swapping opposite edges with H perm".to_string(),
            setup_moves: vec![M2, U, M2, U2, M2, U, M2],
            solution: vec![M2, U, M2, U2, M2, U, M2],
            hint: "Easy one! Just: M2 U M2 U2 M2 U M2".to_string(),
            pattern: PllPattern::HPerm,
        }
    }

    /// Returns all lesson steps
    pub fn get_steps(&self) -> &[PllLessonStep] {
        &self.steps
    }

    /// Returns all PLL cases
    pub fn get_cases(&self) -> &[PllCase] {
        &self.cases
    }

    /// Returns all practice exercises
    pub fn get_practice_exercises(&self) -> &[PllPracticeExercise] {
        &self.practice_exercises
    }

    /// Returns a specific case by pattern
    pub fn get_case_by_pattern(&self, pattern: PllPattern) -> Option<&PllCase> {
        self.cases.iter().find(|c| c.pattern == pattern)
    }
}

impl Default for PllLesson {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pll_lesson_creation() {
        let lesson = PllLesson::new();
        assert!(!lesson.steps.is_empty());
        assert!(!lesson.cases.is_empty());
        assert!(!lesson.practice_exercises.is_empty());
    }

    #[test]
    fn test_pll_lesson_has_all_steps() {
        let lesson = PllLesson::new();
        assert_eq!(lesson.steps.len(), 7);
        assert_eq!(lesson.steps[0].title, "Welcome to PLL!");
    }

    #[test]
    fn test_pll_lesson_has_2look_cases() {
        let lesson = PllLesson::new();
        // 2-look PLL should have corner + edge cases
        assert!(lesson.cases.len() >= 6); // At least the common cases

        // Check we have corner permutation cases
        let has_corner_cases = lesson.cases.iter().any(|c|
            c.pattern == PllPattern::Headlights ||
            c.pattern == PllPattern::DiagonalSwap
        );
        assert!(has_corner_cases);

        // Check we have edge permutation cases
        let has_edge_cases = lesson.cases.iter().any(|c|
            c.pattern == PllPattern::UaPerm ||
            c.pattern == PllPattern::UbPerm
        );
        assert!(has_edge_cases);
    }

    #[test]
    fn test_ua_perm_algorithm() {
        use Move::*;
        let lesson = PllLesson::new();
        let ua = lesson.get_case_by_pattern(PllPattern::UaPerm).unwrap();

        assert_eq!(ua.name, "Ua Perm");
        assert_eq!(ua.algorithm, vec![R, UPrime, R, U, R, U, R, UPrime, RPrime, UPrime, R2]);
    }

    #[test]
    fn test_ub_perm_algorithm() {
        use Move::*;
        let lesson = PllLesson::new();
        let ub = lesson.get_case_by_pattern(PllPattern::UbPerm).unwrap();

        assert_eq!(ub.name, "Ub Perm");
        assert_eq!(ub.algorithm, vec![R2, U, R, U, RPrime, UPrime, RPrime, UPrime, RPrime, U, RPrime]);
    }

    #[test]
    fn test_h_perm_algorithm() {
        use Move::*;
        let lesson = PllLesson::new();
        let h = lesson.get_case_by_pattern(PllPattern::HPerm).unwrap();

        assert_eq!(h.name, "H Perm");
        assert_eq!(h.algorithm, vec![M2, U, M2, U2, M2, U, M2]);
    }

    #[test]
    fn test_practice_exercises() {
        let lesson = PllLesson::new();
        assert_eq!(lesson.practice_exercises.len(), 4);

        // Each exercise should have setup, solution, and hint
        for exercise in &lesson.practice_exercises {
            assert!(!exercise.setup_moves.is_empty());
            assert!(!exercise.solution.is_empty());
            assert!(!exercise.hint.is_empty());
        }
    }

    #[test]
    fn test_get_case_by_pattern() {
        let lesson = PllLesson::new();

        let ua = lesson.get_case_by_pattern(PllPattern::UaPerm);
        assert!(ua.is_some());
        assert_eq!(ua.unwrap().name, "Ua Perm");

        let h = lesson.get_case_by_pattern(PllPattern::HPerm);
        assert!(h.is_some());
    }

    #[test]
    fn test_all_steps_have_content() {
        let lesson = PllLesson::new();

        for step in &lesson.steps {
            assert!(!step.title.is_empty());
            assert!(!step.description.is_empty());
            assert!(!step.kid_friendly_text.is_empty());
        }
    }

    #[test]
    fn test_all_cases_have_visual_hints() {
        let lesson = PllLesson::new();

        for case in &lesson.cases {
            assert!(!case.visual_hint.is_empty());
            assert!(!case.explanation.is_empty());
        }
    }

    #[test]
    fn test_corner_permutation_algorithms() {
        let lesson = PllLesson::new();

        // Should have algorithms for corner swapping
        let corner_cases: Vec<_> = lesson.cases.iter()
            .filter(|c| c.pattern == PllPattern::Headlights || c.pattern == PllPattern::DiagonalSwap)
            .collect();
        assert!(!corner_cases.is_empty());
    }

    #[test]
    fn test_edge_permutation_algorithms() {
        let lesson = PllLesson::new();

        // Should have algorithms for edge swapping
        let edge_cases: Vec<_> = lesson.cases.iter()
            .filter(|c|
                c.pattern == PllPattern::UaPerm ||
                c.pattern == PllPattern::UbPerm ||
                c.pattern == PllPattern::HPerm ||
                c.pattern == PllPattern::ZPerm
            )
            .collect();
        assert!(!edge_cases.is_empty());
    }

    #[test]
    fn test_aa_perm_exists() {
        let lesson = PllLesson::new();
        let aa = lesson.get_case_by_pattern(PllPattern::Headlights);
        assert!(aa.is_some());
        assert!(aa.unwrap().name.contains("Aa Perm"));
    }

    #[test]
    fn test_all_algorithms_not_empty() {
        let lesson = PllLesson::new();

        for case in &lesson.cases {
            assert!(!case.algorithm.is_empty(), "Algorithm for {} is empty", case.name);
        }
    }
}
