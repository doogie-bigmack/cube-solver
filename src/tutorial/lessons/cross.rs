//! 3x3 tutorial: Cross
//!
//! This module implements R6.3 from the PRD:
//! - Step-by-step cross solving
//! - Multiple algorithms for different cases
//! - Practice mode

use crate::cube::{Color, Cube, Move, FaceName};

/// Represents a cross edge piece position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossEdge {
    /// Front edge (white-red, white-blue, etc.)
    Front,
    /// Right edge
    Right,
    /// Back edge
    Back,
    /// Left edge
    Left,
}

/// Represents a cross solving case/algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct CrossCase {
    /// Name of the case
    pub name: String,
    /// Description of when to use this case
    pub description: String,
    /// The algorithm (sequence of moves)
    pub algorithm: Vec<Move>,
    /// Kid-friendly explanation
    pub explanation: String,
    /// Which edge this case solves
    pub edge: CrossEdge,
}

/// Represents a single lesson step for the cross tutorial
#[derive(Debug, Clone, PartialEq)]
pub struct CrossLessonStep {
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

/// Practice exercise for cross solving
#[derive(Debug, Clone, PartialEq)]
pub struct CrossPracticeExercise {
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
}

/// The complete cross tutorial lesson
#[derive(Debug, Clone)]
pub struct CrossLesson {
    /// Lesson steps
    pub steps: Vec<CrossLessonStep>,
    /// Common cross cases/algorithms
    pub cases: Vec<CrossCase>,
    /// Practice exercises
    pub practice_exercises: Vec<CrossPracticeExercise>,
}

impl CrossLesson {
    /// Creates a new cross lesson with all steps
    pub fn new() -> Self {
        Self {
            steps: vec![
                Self::intro_step(),
                Self::what_is_cross_step(),
                Self::choose_color_step(),
                Self::find_edges_step(),
                Self::daisy_method_step(),
                Self::align_edges_step(),
                Self::flip_down_step(),
                Self::tips_and_tricks_step(),
                Self::practice_step(),
            ],
            cases: vec![
                Self::edge_on_top_aligned(),
                Self::edge_on_top_misaligned(),
                Self::edge_on_middle(),
                Self::edge_on_bottom_correct(),
                Self::edge_on_bottom_flipped(),
            ],
            practice_exercises: vec![
                Self::simple_cross_exercise(),
                Self::medium_cross_exercise(),
                Self::advanced_cross_exercise(),
            ],
        }
    }

    // ==================== Lesson Steps ====================

    /// Introduction to solving the cross
    fn intro_step() -> CrossLessonStep {
        CrossLessonStep {
            title: "Welcome to Cross Solving!".to_string(),
            description: "The cross is the very first step in solving a Rubik's Cube using the beginner's method. You'll create a plus sign (+) on one face with matching edge colors.".to_string(),
            example_moves: None,
            kid_friendly_text: "Think of the cross like building the foundation of a house - it's the most important first step! Once you master this, the rest gets easier.".to_string(),
            tip: Some("Don't worry about the corners yet - we're only working with edges!".to_string()),
        }
    }

    /// Explain what the cross is
    fn what_is_cross_step() -> CrossLessonStep {
        CrossLessonStep {
            title: "What is the Cross?".to_string(),
            description: "The cross consists of 4 edge pieces around a center piece. If you're solving white first, you need 4 white edge pieces forming a + shape, with each edge's side color matching the center below it.".to_string(),
            example_moves: None,
            kid_friendly_text: "Imagine a white flower with 4 petals! Each petal (edge) needs to match the color of the face it's touching.".to_string(),
            tip: Some("The cross has 5 pieces total: 1 center + 4 edges".to_string()),
        }
    }

    /// Choose which color to start with
    fn choose_color_step() -> CrossLessonStep {
        CrossLessonStep {
            title: "Choose Your Color".to_string(),
            description: "Most people start with white or yellow on top. We'll use white in this tutorial. Hold your cube with white on top (U face).".to_string(),
            example_moves: None,
            kid_friendly_text: "White is easier to see, like snow on a mountain! You can pick yellow later when you're more comfortable.".to_string(),
            tip: Some("Always solve the cross on the same color until you're confident".to_string()),
        }
    }

    /// Find the edge pieces
    fn find_edges_step() -> CrossLessonStep {
        CrossLessonStep {
            title: "Find Your Edge Pieces".to_string(),
            description: "Look for the 4 edge pieces that have white on them. They are: white-red, white-blue, white-orange, and white-green. They might be anywhere on the cube!".to_string(),
            example_moves: None,
            kid_friendly_text: "It's like a treasure hunt! Look all around the cube to find your white edges. There are exactly 4 of them hiding somewhere.".to_string(),
            tip: Some("Edge pieces have exactly 2 colors, never 3".to_string()),
        }
    }

    /// The daisy method
    fn daisy_method_step() -> CrossLessonStep {
        CrossLessonStep {
            title: "The Daisy Method".to_string(),
            description: "First, get all 4 white edges around the yellow center (on top). Don't worry about matching the sides yet - just make a white 'daisy' flower around the yellow center. Use simple moves like F, R, L, B to bring white edges to the top.".to_string(),
            example_moves: Some(vec![Move::F, Move::F]),
            kid_friendly_text: "Make a white flower around the yellow center! The petals don't need to match yet - just get all the white edges pointing up. It's okay if they're messy!".to_string(),
            tip: Some("If a white edge is on top but facing the wrong way, move it away and bring it back correctly".to_string()),
        }
    }

    /// Align the edges
    fn align_edges_step() -> CrossLessonStep {
        CrossLessonStep {
            title: "Align Your Edges".to_string(),
            description: "Now rotate the top (U face) to align each white edge with its matching center color. For example, if an edge is white-red, turn U until the red part is above the red center.".to_string(),
            example_moves: Some(vec![Move::U]),
            kid_friendly_text: "Spin the top like a merry-go-round! Stop when each petal's color matches the center face below it. Red petal above red center, blue above blue, etc.".to_string(),
            tip: Some("Only turn the U face to align - don't mess up your daisy!".to_string()),
        }
    }

    /// Flip edges down to complete cross
    fn flip_down_step() -> CrossLessonStep {
        CrossLessonStep {
            title: "Flip Down to Complete".to_string(),
            description: "Once an edge is aligned, flip it down to the bottom with a 180-degree turn. If white-red is aligned above the red center, do F2 to flip it to the bottom. Repeat for all 4 edges: align with U, then flip with F2, R2, B2, or L2.".to_string(),
            example_moves: Some(vec![Move::F2]),
            kid_friendly_text: "Now flip each petal down to make a cross on the bottom! Turn the front face twice (F2) and watch the white edge flip down. Do this for all 4 edges one by one.".to_string(),
            tip: Some("After flipping, that edge is done! Don't touch it again while doing the others".to_string()),
        }
    }

    /// Tips and tricks
    fn tips_and_tricks_step() -> CrossLessonStep {
        CrossLessonStep {
            title: "Tips for Faster Cross".to_string(),
            description: "With practice, you can skip the daisy and solve edges directly. Look for edges on the bottom and use moves like F R' F' to solve them without the daisy method. But the daisy is perfect for learning!".to_string(),
            example_moves: Some(vec![Move::F, Move::RPrime, Move::FPrime]),
            kid_friendly_text: "As you get better, you'll find shortcuts! But there's no rush - the daisy method always works and is super reliable.".to_string(),
            tip: Some("Advanced solvers can do the cross in 5-8 moves, but 15-20 moves is great for beginners!".to_string()),
        }
    }

    /// Practice step
    fn practice_step() -> CrossLessonStep {
        CrossLessonStep {
            title: "Let's Practice!".to_string(),
            description: "Scramble your cube and try solving the white cross using the daisy method. Take your time! Speed comes with practice. Try to get all 4 edges matching their centers.".to_string(),
            example_moves: None,
            kid_friendly_text: "Practice makes perfect! Do the cross 10 times and you'll start to feel like a pro. Don't give up if it's tricky at first - everyone struggles with the cross when they start!".to_string(),
            tip: Some("Set a goal: solve the cross 5 times today!".to_string()),
        }
    }

    // ==================== Cross Cases ====================

    /// Edge on top layer, already aligned
    fn edge_on_top_aligned() -> CrossCase {
        CrossCase {
            name: "Edge on Top - Aligned".to_string(),
            description: "White edge is on the top layer with white facing up, and the side color matches the center below.".to_string(),
            algorithm: vec![Move::F2],
            explanation: "The edge is already in the right spot! Just flip it down with a 180-degree turn of that face.".to_string(),
            edge: CrossEdge::Front,
        }
    }

    /// Edge on top layer, misaligned
    fn edge_on_top_misaligned() -> CrossCase {
        CrossCase {
            name: "Edge on Top - Misaligned".to_string(),
            description: "White edge is on the top layer with white facing up, but the side color doesn't match the center below.".to_string(),
            algorithm: vec![Move::U, Move::F2],
            explanation: "Turn the top layer (U) until the edge aligns with its matching center, then flip it down with F2 (or R2, L2, B2).".to_string(),
            edge: CrossEdge::Front,
        }
    }

    /// Edge in middle layer
    fn edge_on_middle() -> CrossCase {
        CrossCase {
            name: "Edge in Middle Layer".to_string(),
            description: "White edge is stuck in the middle layer between top and bottom.".to_string(),
            algorithm: vec![Move::F],
            explanation: "Turn the front face once to pop the edge up to the top layer, then solve it normally using the daisy method.".to_string(),
            edge: CrossEdge::Front,
        }
    }

    /// Edge on bottom, correct orientation
    fn edge_on_bottom_correct() -> CrossCase {
        CrossCase {
            name: "Edge on Bottom - Correct".to_string(),
            description: "White edge is already on the bottom with white facing down and side colors matching.".to_string(),
            algorithm: vec![],
            explanation: "This edge is already solved! Leave it alone and work on the other edges.".to_string(),
            edge: CrossEdge::Front,
        }
    }

    /// Edge on bottom, flipped (wrong orientation)
    fn edge_on_bottom_flipped() -> CrossCase {
        CrossCase {
            name: "Edge on Bottom - Flipped".to_string(),
            description: "White edge is on the bottom but white is facing outward instead of down, or colors don't match.".to_string(),
            algorithm: vec![Move::F2, Move::U, Move::F2],
            explanation: "Pop it up to the top (F2), turn top to get it out of the way (U), then bring it back aligned and flip down (F2).".to_string(),
            edge: CrossEdge::Front,
        }
    }

    // ==================== Practice Exercises ====================

    /// Simple cross practice
    fn simple_cross_exercise() -> CrossPracticeExercise {
        CrossPracticeExercise {
            title: "Simple Cross Practice".to_string(),
            description: "This exercise has 2 white edges close to their correct positions.".to_string(),
            setup_moves: vec![Move::F, Move::R, Move::U2, Move::RPrime, Move::FPrime],
            solution: vec![Move::F2, Move::R2],
            hint: "Look for white edges on the top layer - they're easy to spot!".to_string(),
        }
    }

    /// Medium difficulty cross
    fn medium_cross_exercise() -> CrossPracticeExercise {
        CrossPracticeExercise {
            title: "Medium Cross Challenge".to_string(),
            description: "This exercise has edges in various positions around the cube.".to_string(),
            setup_moves: vec![Move::F, Move::R, Move::U, Move::RPrime, Move::FPrime, Move::U2, Move::F, Move::R, Move::U, Move::RPrime, Move::FPrime],
            solution: vec![Move::F, Move::F, Move::U, Move::R2, Move::U, Move::F2, Move::U, Move::B2, Move::U, Move::L2],
            hint: "Use the daisy method - get all white edges around the yellow center first!".to_string(),
        }
    }

    /// Advanced cross exercise
    fn advanced_cross_exercise() -> CrossPracticeExercise {
        CrossPracticeExercise {
            title: "Advanced Cross Challenge".to_string(),
            description: "This is a more complex scramble requiring good technique.".to_string(),
            setup_moves: vec![
                Move::R, Move::U, Move::RPrime, Move::U, Move::R, Move::U2, Move::RPrime,
                Move::F, Move::U, Move::FPrime, Move::U, Move::F, Move::U2, Move::FPrime,
            ],
            solution: vec![
                Move::F, Move::U, Move::F2, Move::U2, Move::R2,
                Move::U, Move::B2, Move::U2, Move::L2,
            ],
            hint: "Take your time! This one is tricky. Focus on one edge at a time.".to_string(),
        }
    }

    // ==================== Helper Methods ====================

    /// Get the total number of steps
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Get a specific step by index
    pub fn get_step(&self, index: usize) -> Option<&CrossLessonStep> {
        self.steps.get(index)
    }

    /// Get all steps
    pub fn get_all_steps(&self) -> &[CrossLessonStep] {
        &self.steps
    }

    /// Get all cross cases
    pub fn get_all_cases(&self) -> &[CrossCase] {
        &self.cases
    }

    /// Get a specific case by index
    pub fn get_case(&self, index: usize) -> Option<&CrossCase> {
        self.cases.get(index)
    }

    /// Get all practice exercises
    pub fn get_practice_exercises(&self) -> &[CrossPracticeExercise] {
        &self.practice_exercises
    }

    /// Get a specific practice exercise
    pub fn get_practice_exercise(&self, index: usize) -> Option<&CrossPracticeExercise> {
        self.practice_exercises.get(index)
    }

    /// Verify if a cube has a correct white cross on the bottom (Down face)
    /// Note: In standard solving, white is typically solved on Down face first
    pub fn verify_white_cross(cube: &Cube) -> bool {
        if cube.size() != 3 {
            return false; // Cross tutorial is for 3x3 only
        }

        // Check that the down face center is white
        // Default cube has White on Up, Yellow on Down, so we check Down face
        // Actually, let me check if the down face has white cross OR up face has white cross
        // For flexibility, let's check the actual down face

        // Try down face first (after solving, white cross is on bottom)
        let down_face = cube.get_face(FaceName::D);
        let down_center = down_face.get(1, 1);

        // Also try up face (default solved cube has white on top)
        let up_face = cube.get_face(FaceName::U);
        let up_center = up_face.get(1, 1);

        // Determine which face to check based on where white center is
        let (target_face, check_top_or_bottom): (&crate::cube::Face, bool) =
            if down_center == Color::White {
                (down_face, false) // white on bottom, check bottom row of sides
            } else if up_center == Color::White {
                (up_face, true) // white on top, check top row of sides
            } else {
                return false; // No white center on up or down
            };

        // Check all 4 edge pieces on the target face
        let edges = [
            (1, 0), // top edge
            (1, 2), // bottom edge
            (0, 1), // left edge
            (2, 1), // right edge
        ];

        for &(row, col) in &edges {
            let color = target_face.get(row, col);
            if color != Color::White {
                return false;
            }
        }

        // Check that side colors match
        // If white is on bottom (D), check bottom row (row=2) of side faces
        // If white is on top (U), check top row (row=0) of side faces
        let side_row = if check_top_or_bottom { 0 } else { 2 };

        let front_face = cube.get_face(FaceName::F);
        let front_edge_color = front_face.get(side_row, 1);
        let front_center_color = front_face.get(1, 1);
        if front_edge_color != front_center_color {
            return false;
        }

        let right_face = cube.get_face(FaceName::R);
        let right_edge_color = right_face.get(side_row, 1);
        let right_center_color = right_face.get(1, 1);
        if right_edge_color != right_center_color {
            return false;
        }

        let back_face = cube.get_face(FaceName::B);
        let back_edge_color = back_face.get(side_row, 1);
        let back_center_color = back_face.get(1, 1);
        if back_edge_color != back_center_color {
            return false;
        }

        let left_face = cube.get_face(FaceName::L);
        let left_edge_color = left_face.get(side_row, 1);
        let left_center_color = left_face.get(1, 1);
        if left_edge_color != left_center_color {
            return false;
        }

        true
    }
}

impl Default for CrossLesson {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_lesson_creation() {
        let lesson = CrossLesson::new();
        assert!(lesson.step_count() > 0);
        assert_eq!(lesson.step_count(), 9);
    }

    #[test]
    fn test_lesson_steps_have_content() {
        let lesson = CrossLesson::new();
        for step in lesson.get_all_steps() {
            assert!(!step.title.is_empty());
            assert!(!step.description.is_empty());
            assert!(!step.kid_friendly_text.is_empty());
        }
    }

    #[test]
    fn test_has_cross_cases() {
        let lesson = CrossLesson::new();
        assert_eq!(lesson.cases.len(), 5);

        for case in lesson.get_all_cases() {
            assert!(!case.name.is_empty());
            assert!(!case.description.is_empty());
            assert!(!case.explanation.is_empty());
        }
    }

    #[test]
    fn test_has_practice_exercises() {
        let lesson = CrossLesson::new();
        assert_eq!(lesson.practice_exercises.len(), 3);

        for exercise in lesson.get_practice_exercises() {
            assert!(!exercise.title.is_empty());
            assert!(!exercise.description.is_empty());
            assert!(!exercise.hint.is_empty());
            assert!(!exercise.setup_moves.is_empty());
        }
    }

    #[test]
    fn test_verify_solved_cube_cross() {
        let cube = Cube::new(3);
        assert!(CrossLesson::verify_white_cross(&cube));
    }

    #[test]
    fn test_verify_scrambled_cube_no_cross() {
        let mut cube = Cube::new(3);
        cube.apply_move(Move::R);
        cube.apply_move(Move::U);
        cube.apply_move(Move::RPrime);

        // After scrambling, cross should not be solved
        assert!(!CrossLesson::verify_white_cross(&cube));
    }

    #[test]
    fn test_daisy_method_step_has_example() {
        let lesson = CrossLesson::new();
        let daisy_step = lesson.get_step(4); // Daisy method is step 4
        assert!(daisy_step.is_some());
        assert!(daisy_step.unwrap().example_moves.is_some());
    }

    #[test]
    fn test_all_steps_have_tips_or_not() {
        let lesson = CrossLesson::new();
        // Just verify we can access the tip field
        for step in lesson.get_all_steps() {
            // Tip is optional, so just check we can read it
            let _ = &step.tip;
        }
    }

    #[test]
    fn test_cross_edge_enum() {
        // Just verify the enum variants exist
        let _front = CrossEdge::Front;
        let _right = CrossEdge::Right;
        let _back = CrossEdge::Back;
        let _left = CrossEdge::Left;
    }

    #[test]
    fn test_get_specific_step() {
        let lesson = CrossLesson::new();
        let first_step = lesson.get_step(0);
        assert!(first_step.is_some());
        assert_eq!(first_step.unwrap().title, "Welcome to Cross Solving!");
    }

    #[test]
    fn test_get_specific_case() {
        let lesson = CrossLesson::new();
        let first_case = lesson.get_case(0);
        assert!(first_case.is_some());
        assert!(first_case.unwrap().name.contains("Aligned"));
    }

    #[test]
    fn test_simple_exercise_setup_scrambles() {
        let lesson = CrossLesson::new();
        let exercise = lesson.get_practice_exercise(0).unwrap();

        let mut cube = Cube::new(3);
        // Apply setup
        for mov in &exercise.setup_moves {
            cube.apply_move(*mov);
        }

        // Verify it's scrambled (cross is not solved)
        assert!(!CrossLesson::verify_white_cross(&cube));
    }

    #[test]
    fn test_exercise_has_solution() {
        let lesson = CrossLesson::new();
        let exercise = lesson.get_practice_exercise(0).unwrap();

        // Just verify the exercise has a solution defined
        assert!(!exercise.solution.is_empty());
    }
}
