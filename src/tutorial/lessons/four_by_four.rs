//! 4x4 Tutorial Lesson
//!
//! This module implements R6.9 from the PRD:
//! - Center solving strategy
//! - Edge pairing strategy
//! - Parity algorithm explanation

use crate::cube::{Cube, Move};

/// Represents a single step in the 4x4 tutorial
#[derive(Debug, Clone, PartialEq)]
pub struct FourByFourLessonStep {
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

/// Represents a specific case or algorithm in the 4x4 reduction method
#[derive(Debug, Clone, PartialEq)]
pub struct FourByFourCase {
    /// Name of the case
    pub name: String,
    /// Description of when to use it
    pub description: String,
    /// Algorithm to solve this case
    pub algorithm: Vec<Move>,
    /// How to recognize this case
    pub recognition: String,
    /// Category: "centers", "edges", or "parity"
    pub category: String,
}

/// Represents a practice exercise for the 4x4 tutorial
#[derive(Debug, Clone, PartialEq)]
pub struct FourByFourPracticeExercise {
    /// Description of the practice task
    pub description: String,
    /// Initial cube state (scrambled)
    pub scramble: Vec<Move>,
    /// Hint for solving
    pub hint: String,
    /// Which phase to practice: "centers", "edges", or "full"
    pub phase: String,
}

/// The complete 4x4 tutorial lesson using the reduction method
#[derive(Debug, Clone)]
pub struct FourByFourLesson {
    /// Lesson steps
    pub steps: Vec<FourByFourLessonStep>,
    /// 4x4 method cases (centers, edges, parity)
    pub cases: Vec<FourByFourCase>,
    /// Practice exercises
    pub practice_exercises: Vec<FourByFourPracticeExercise>,
}

impl FourByFourLesson {
    /// Creates a new 4x4 tutorial lesson
    pub fn new() -> Self {
        Self {
            steps: vec![
                Self::intro_step(),
                Self::differences_from_3x3_step(),
                Self::reduction_method_overview_step(),
                Self::step1_centers_step(),
                Self::step2_edges_step(),
                Self::step3_solve_as_3x3_step(),
                Self::step4_parity_step(),
                Self::practice_step(),
            ],
            cases: vec![
                Self::center_slice_move(),
                Self::center_3_2_3_algorithm(),
                Self::center_flip_algorithm(),
                Self::edge_pairing_basic(),
                Self::edge_pairing_slice(),
                Self::oll_parity_algorithm(),
                Self::pll_parity_algorithm(),
            ],
            practice_exercises: vec![
                Self::practice_centers(),
                Self::practice_edges(),
                Self::practice_full(),
            ],
        }
    }

    /// Introduction to 4x4 solving
    fn intro_step() -> FourByFourLessonStep {
        FourByFourLessonStep {
            title: "Welcome to 4x4 Solving!".to_string(),
            description: "The 4x4 cube (also called the Rubik's Revenge) is more challenging than the 3x3 because it has no fixed center pieces and additional edge pieces that need to be paired.".to_string(),
            algorithm: None,
            visual_hint: "A 4x4 cube has 56 movable pieces compared to the 3x3's 20 pieces.".to_string(),
            kid_friendly_text: "The 4x4 is like a bigger, more exciting puzzle! It has more pieces to solve, but once you know the tricks, it's a fun challenge!".to_string(),
        }
    }

    /// Explain differences from 3x3
    fn differences_from_3x3_step() -> FourByFourLessonStep {
        FourByFourLessonStep {
            title: "How 4x4 is Different from 3x3".to_string(),
            description: "Unlike the 3x3, the 4x4 has no fixed center pieces (the centers can move), and it has 'wing' edges that must be paired together. You also might encounter special 'parity' cases that don't exist on a 3x3.".to_string(),
            algorithm: None,
            visual_hint: "The 4x4 has 24 center pieces (4 per face) and 24 edge pieces (pairs of wings) that need pairing.".to_string(),
            kid_friendly_text: "On a 3x3, the centers stay in place. On a 4x4, the centers can move around! You need to put them in the right spots before solving the rest.".to_string(),
        }
    }

    /// Explain the reduction method
    fn reduction_method_overview_step() -> FourByFourLessonStep {
        FourByFourLessonStep {
            title: "The Reduction Method".to_string(),
            description: "We'll use the reduction method, which has 4 main steps: 1) Solve all center pieces (make 6 solid-colored center blocks), 2) Pair up all edge pieces, 3) Solve like a 3x3 cube, 4) Handle any parity cases if they appear.".to_string(),
            algorithm: None,
            visual_hint: "The reduction method 'reduces' the 4x4 to a 3x3 by fixing the centers and pairing the edges first.".to_string(),
            kid_friendly_text: "Think of it like organizing your toys: first group similar toys together (centers), then match pairs (edges), then solve the whole thing like a regular puzzle!".to_string(),
        }
    }

    /// Step 1: Centers
    fn step1_centers_step() -> FourByFourLessonStep {
        FourByFourLessonStep {
            title: "Step 1: Solve the Centers".to_string(),
            description: "Start by solving all 6 centers. Pick one center (usually white) and make a solid 2x2 block of that color. Then do the same for the opposite face (yellow), and finally the remaining 4 faces. Use wide moves and special algorithms to move center pieces without disturbing solved centers.".to_string(),
            algorithm: Some(vec![Move::R, Move::U, Move::RPrime, Move::UPrime]), // Example center algorithm
            visual_hint: "Solve opposite colors first (white/yellow, then red/orange, then blue/green) to avoid undoing your work.".to_string(),
            kid_friendly_text: "Centers are like the foundation of a house. Get these right first, and everything else becomes easier! Start with white, then yellow, then work on the other colors.".to_string(),
        }
    }

    /// Step 2: Edges
    fn step2_edges_step() -> FourByFourLessonStep {
        FourByFourLessonStep {
            title: "Step 2: Pair the Edges".to_string(),
            description: "Now pair up all 12 edges. Each 'edge' on a 4x4 is made of two wing pieces that need to be brought together. Use special moves to pair edges without breaking the solved centers. Work systematically - pair all edges for one layer, then move to the next.".to_string(),
            algorithm: Some(vec![Move::U, Move::RPrime, Move::UPrime, Move::R]), // Edge pairing algorithm
            visual_hint: "Always pair edges using the top layer. Keep your solved centers on the left and right faces.".to_string(),
            kid_friendly_text: "Edge pairing is like matching socks! Find two wing pieces that belong together and bring them to the top layer to pair them up. Do this 12 times, one for each edge.".to_string(),
        }
    }

    /// Step 3: Solve as 3x3
    fn step3_solve_as_3x3_step() -> FourByFourLessonStep {
        FourByFourLessonStep {
            title: "Step 3: Solve Like a 3x3".to_string(),
            description: "With centers solved and edges paired, your 4x4 now looks like a 3x3! Use all your 3x3 solving knowledge - make a cross, solve F2L (first two layers), orient the last layer (OLL), and permute the last layer (PLL). Treat the paired edges as single pieces.".to_string(),
            algorithm: None,
            visual_hint: "Use only outer layer moves (R, L, U, D, F, B) from now on. Don't use slice moves or you'll unpair edges!".to_string(),
            kid_friendly_text: "This is the fun part! Now you can use everything you learned from solving the 3x3. Just remember to only turn the outer layers, not the inner ones.".to_string(),
        }
    }

    /// Step 4: Parity
    fn step4_parity_step() -> FourByFourLessonStep {
        FourByFourLessonStep {
            title: "Step 4: Fix Parity (If Needed)".to_string(),
            description: "Sometimes you'll encounter a 'parity' - a case that can't happen on a regular 3x3. There are two types: OLL parity (one edge flipped) and PLL parity (two edges swapped). Don't panic! Just learn these special algorithms to fix them. OLL parity needs fixing before OLL, and PLL parity needs fixing after PLL if it appears.".to_string(),
            algorithm: Some(vec![Move::R2, Move::B2, Move::U2, Move::L, Move::U2, Move::RPrime, Move::U2, Move::R, Move::U2, Move::F2, Move::R, Move::F2, Move::LPrime, Move::B2, Move::R2]), // OLL parity
            visual_hint: "OLL parity looks like one edge is flipped. PLL parity looks like two edges are swapped after you've done PLL.".to_string(),
            kid_friendly_text: "Parity is like a special puzzle twist! It's rare, but when it happens, use these magic algorithms to fix it. Think of it as a secret level in a video game!".to_string(),
        }
    }

    /// Practice recommendations
    fn practice_step() -> FourByFourLessonStep {
        FourByFourLessonStep {
            title: "Practice Makes Perfect!".to_string(),
            description: "Try solving the practice exercises below. Start with centers-only practice, then edges-only, and finally full solves. The 4x4 takes more time than a 3x3 at first, but with practice, you'll get faster and more confident!".to_string(),
            algorithm: None,
            visual_hint: "Practice each phase separately until you're comfortable, then put it all together for a full solve.".to_string(),
            kid_friendly_text: "Learning the 4x4 is like learning to ride a bike with training wheels first. Practice each step until you feel confident, then put it all together!".to_string(),
        }
    }

    // Center Solving Cases

    /// Center slice move technique
    fn center_slice_move() -> FourByFourCase {
        FourByFourCase {
            name: "Center Slice Move".to_string(),
            description: "Basic technique to move center pieces using special moves".to_string(),
            algorithm: vec![Move::R, Move::U, Move::RPrime, Move::UPrime],
            recognition: "Use this to move center pieces from one face to another".to_string(),
            category: "centers".to_string(),
        }
    }

    /// 3-2-3 algorithm for centers
    fn center_3_2_3_algorithm() -> FourByFourCase {
        FourByFourCase {
            name: "3-2-3 Algorithm".to_string(),
            description: "Swaps two center pieces without disturbing other centers".to_string(),
            algorithm: vec![Move::R, Move::U2, Move::R, Move::U2, Move::R, Move::U2],
            recognition: "Use when you need to swap two specific center pieces".to_string(),
            category: "centers".to_string(),
        }
    }

    /// Center flip algorithm
    fn center_flip_algorithm() -> FourByFourCase {
        FourByFourCase {
            name: "Center Flip".to_string(),
            description: "Rotates a 2x2 center block without affecting other centers".to_string(),
            algorithm: vec![Move::R2, Move::U2, Move::R2, Move::U2, Move::R2, Move::U2],
            recognition: "Use when center colors are correct but rotated wrong".to_string(),
            category: "centers".to_string(),
        }
    }

    // Edge Pairing Cases

    /// Basic edge pairing
    fn edge_pairing_basic() -> FourByFourCase {
        FourByFourCase {
            name: "Basic Edge Pairing".to_string(),
            description: "Pairs two edge wings together in the top layer".to_string(),
            algorithm: vec![Move::U, Move::RPrime, Move::UPrime, Move::R],
            recognition: "Use when both wing pieces are visible in the top layer".to_string(),
            category: "edges".to_string(),
        }
    }

    /// Edge pairing with slice move
    fn edge_pairing_slice() -> FourByFourCase {
        FourByFourCase {
            name: "Slice Edge Pairing".to_string(),
            description: "Pairs edges using special moves when one wing is hidden".to_string(),
            algorithm: vec![Move::U2, Move::R, Move::U, Move::RPrime, Move::U2],
            recognition: "Use when one wing is in a middle layer".to_string(),
            category: "edges".to_string(),
        }
    }

    // Parity Cases

    /// OLL parity algorithm
    fn oll_parity_algorithm() -> FourByFourCase {
        FourByFourCase {
            name: "OLL Parity".to_string(),
            description: "Fixes a single flipped edge before OLL".to_string(),
            algorithm: vec![
                Move::R2, Move::B2, Move::U2, Move::L, Move::U2, Move::RPrime,
                Move::U2, Move::R, Move::U2, Move::F2, Move::R, Move::F2,
                Move::LPrime, Move::B2, Move::R2
            ],
            recognition: "One edge appears to be flipped after pairing all edges".to_string(),
            category: "parity".to_string(),
        }
    }

    /// PLL parity algorithm
    fn pll_parity_algorithm() -> FourByFourCase {
        FourByFourCase {
            name: "PLL Parity".to_string(),
            description: "Swaps two opposite edges after you've done PLL".to_string(),
            algorithm: vec![
                Move::R2, Move::U2, Move::R2, Move::U2, Move::R2, Move::U2
            ],
            recognition: "Two edges are swapped but everything else is solved".to_string(),
            category: "parity".to_string(),
        }
    }

    // Practice Exercises

    /// Centers-only practice
    fn practice_centers() -> FourByFourPracticeExercise {
        FourByFourPracticeExercise {
            description: "Centers Only: Practice solving just the center pieces".to_string(),
            scramble: vec![
                Move::R, Move::U, Move::R2, Move::U2, Move::R, Move::U,
                Move::F, Move::U, Move::F2, Move::U2
            ],
            hint: "Start with white centers, then yellow, then work on the remaining four faces.".to_string(),
            phase: "centers".to_string(),
        }
    }

    /// Edges-only practice
    fn practice_edges() -> FourByFourPracticeExercise {
        FourByFourPracticeExercise {
            description: "Edges Only: Practice pairing edge pieces (assume centers are solved)".to_string(),
            scramble: vec![
                Move::R, Move::U, Move::R2, Move::U2, Move::R, Move::U,
                Move::F, Move::U, Move::F2, Move::U2, Move::F, Move::U
            ],
            hint: "Use the basic edge pairing technique. Pair all 12 edges systematically.".to_string(),
            phase: "edges".to_string(),
        }
    }

    /// Full solve practice
    fn practice_full() -> FourByFourPracticeExercise {
        FourByFourPracticeExercise {
            description: "Full Solve: Complete 4x4 solve from scrambled state".to_string(),
            scramble: vec![
                Move::R, Move::U, Move::R2, Move::U2, Move::R, Move::U,
                Move::F, Move::U, Move::F2, Move::U2, Move::F, Move::U,
                Move::R, Move::U, Move::R2, Move::U2, Move::R, Move::U,
                Move::F, Move::U, Move::F2, Move::U2
            ],
            hint: "Follow all four steps: centers, edges, solve as 3x3, handle parity if needed.".to_string(),
            phase: "full".to_string(),
        }
    }

    /// Get the number of steps in the lesson
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Get the number of cases
    pub fn case_count(&self) -> usize {
        self.cases.len()
    }

    /// Get the number of practice exercises
    pub fn exercise_count(&self) -> usize {
        self.practice_exercises.len()
    }

    /// Get cases by category
    pub fn get_cases_by_category(&self, category: &str) -> Vec<&FourByFourCase> {
        self.cases.iter().filter(|c| c.category == category).collect()
    }

    /// Apply a scramble to a 4x4 cube
    pub fn apply_scramble(&self, exercise_index: usize) -> Result<Cube, String> {
        if exercise_index >= self.practice_exercises.len() {
            return Err("Invalid exercise index".to_string());
        }

        let exercise = &self.practice_exercises[exercise_index];
        let mut cube = Cube::new(4);

        // Apply scramble
        for mv in &exercise.scramble {
            cube.apply_move(*mv);
        }

        Ok(cube)
    }
}

impl Default for FourByFourLesson {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_lesson() {
        let lesson = FourByFourLesson::new();
        assert!(lesson.step_count() > 0);
        assert!(lesson.case_count() > 0);
        assert!(lesson.exercise_count() > 0);
    }

    #[test]
    fn test_steps_have_required_fields() {
        let lesson = FourByFourLesson::new();
        for step in &lesson.steps {
            assert!(!step.title.is_empty());
            assert!(!step.description.is_empty());
            assert!(!step.kid_friendly_text.is_empty());
            assert!(!step.visual_hint.is_empty());
        }
    }

    #[test]
    fn test_cases_by_category() {
        let lesson = FourByFourLesson::new();
        let center_cases = lesson.get_cases_by_category("centers");
        let edge_cases = lesson.get_cases_by_category("edges");
        let parity_cases = lesson.get_cases_by_category("parity");

        assert!(center_cases.len() >= 2, "Should have at least 2 center cases");
        assert!(edge_cases.len() >= 2, "Should have at least 2 edge cases");
        assert_eq!(parity_cases.len(), 2, "Should have exactly 2 parity cases");
    }

    #[test]
    fn test_practice_exercises_have_phases() {
        let lesson = FourByFourLesson::new();
        for exercise in &lesson.practice_exercises {
            assert!(!exercise.phase.is_empty());
            assert!(
                exercise.phase == "centers" ||
                exercise.phase == "edges" ||
                exercise.phase == "full"
            );
        }
    }

    #[test]
    fn test_apply_scramble() {
        let lesson = FourByFourLesson::new();
        let result = lesson.apply_scramble(0);
        assert!(result.is_ok());

        let cube = result.unwrap();
        assert_eq!(cube.size(), 4);
    }

    #[test]
    fn test_invalid_exercise_index() {
        let lesson = FourByFourLesson::new();
        let result = lesson.apply_scramble(99);
        assert!(result.is_err());
    }

    #[test]
    fn test_all_cases_have_algorithms() {
        let lesson = FourByFourLesson::new();
        for case in &lesson.cases {
            assert!(!case.algorithm.is_empty(), "Case '{}' should have an algorithm", case.name);
            assert!(!case.recognition.is_empty(), "Case '{}' should have recognition tips", case.name);
        }
    }
}
