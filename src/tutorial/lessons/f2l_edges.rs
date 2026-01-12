//! 3x3 tutorial: Second Layer (F2L Edges)
//!
//! This module implements R6.5 from the PRD:
//! - Edge insertion algorithms
//! - Left and right cases
//! - Practice mode

use crate::cube::{Color, Cube, Move, FaceName};
use crate::cube::state::Face;

/// Represents an edge piece position in the second layer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgePosition {
    /// Front-Right edge
    FrontRight,
    /// Front-Left edge
    FrontLeft,
    /// Back-Right edge
    BackRight,
    /// Back-Left edge
    BackLeft,
}

/// Represents an edge solving case/algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeCase {
    /// Name of the case
    pub name: String,
    /// Description of when to use this case
    pub description: String,
    /// The algorithm (sequence of moves)
    pub algorithm: Vec<Move>,
    /// Kid-friendly explanation
    pub explanation: String,
    /// Which edge position this case addresses
    pub position: EdgePosition,
}

/// Represents a single lesson step for the second layer tutorial
#[derive(Debug, Clone, PartialEq)]
pub struct SecondLayerLessonStep {
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

/// Practice exercise for edge insertion
#[derive(Debug, Clone, PartialEq)]
pub struct SecondLayerPracticeExercise {
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

/// The complete second layer tutorial lesson
#[derive(Debug, Clone)]
pub struct SecondLayerLesson {
    /// Lesson steps
    pub steps: Vec<SecondLayerLessonStep>,
    /// Common edge cases/algorithms
    pub cases: Vec<EdgeCase>,
    /// Practice exercises
    pub practice_exercises: Vec<SecondLayerPracticeExercise>,
}

impl SecondLayerLesson {
    /// Creates a new second layer lesson with all steps
    pub fn new() -> Self {
        Self {
            steps: vec![
                Self::intro_step(),
                Self::what_are_second_layer_edges_step(),
                Self::find_edges_step(),
                Self::identify_position_step(),
                Self::right_algorithm_step(),
                Self::left_algorithm_step(),
                Self::wrong_edge_in_slot_step(),
                Self::tips_and_tricks_step(),
                Self::practice_step(),
            ],
            cases: vec![
                Self::edge_goes_right(),
                Self::edge_goes_left(),
                Self::edge_in_slot_wrong_orientation(),
                Self::edge_in_slot_wrong_position(),
                Self::edge_in_top_with_yellow(),
            ],
            practice_exercises: vec![
                Self::simple_edge_exercise(),
                Self::medium_edge_exercise(),
                Self::advanced_edge_exercise(),
            ],
        }
    }

    // ==================== Lesson Steps ====================

    /// Introduction to solving second layer edges
    fn intro_step() -> SecondLayerLessonStep {
        SecondLayerLessonStep {
            title: "Welcome to Second Layer Edges!".to_string(),
            description: "After solving the entire first layer (white cross + white corners), the next step is to solve the 4 edge pieces in the middle layer. This completes the first two layers, which is called F2L (First Two Layers).".to_string(),
            example_moves: None,
            kid_friendly_text: "Great job on the first layer! Now let's build the middle belt of the cube. We'll add 4 edge pieces between the white layer and yellow layer. Like building the second floor of a house!".to_string(),
            tip: Some("Make sure your entire first layer is solved before starting!".to_string()),
        }
    }

    /// Explain what second layer edges are
    fn what_are_second_layer_edges_step() -> SecondLayerLessonStep {
        SecondLayerLessonStep {
            title: "What are Second Layer Edges?".to_string(),
            description: "The second layer has 4 edge pieces. These edges have 2 colors (NO yellow or white). For example: red-blue, red-green, orange-blue, or orange-green. They go between the white layer and yellow layer.".to_string(),
            example_moves: None,
            kid_friendly_text: "Second layer edges are the 4 pieces in the middle belt of the cube. Each one has 2 colors, but NO yellow or white! They connect the side faces. Look for edges like red-blue or green-orange.".to_string(),
            tip: Some("If an edge has yellow or white, it belongs in a different layer!".to_string()),
        }
    }

    /// Find the edge pieces
    fn find_edges_step() -> SecondLayerLessonStep {
        SecondLayerLessonStep {
            title: "Find Your Edge Pieces".to_string(),
            description: "Look in the top layer (yellow face) for edges that have NO yellow. These are your second layer edges. Each edge's 2 colors tell you where it belongs - match them to the center colors on the side faces.".to_string(),
            example_moves: None,
            kid_friendly_text: "Hunt for edges in the top layer that don't have any yellow on them! Check both sides of each edge piece. The colors tell you where the edge belongs - like an address!".to_string(),
            tip: Some("There are only 4 edges in the second layer, so you need to find 4 edges without yellow".to_string()),
        }
    }

    /// Identify where edge belongs
    fn identify_position_step() -> SecondLayerLessonStep {
        SecondLayerLessonStep {
            title: "Position the Edge Above Its Home".to_string(),
            description: "Before inserting an edge, match its front color to a center color. Rotate the top layer (U) until the edge's front sticker matches the center color below it. Now look at the edge's other color - does it match the left center or right center? This tells you which algorithm to use.".to_string(),
            example_moves: Some(vec![Move::U]),
            kid_friendly_text: "First, spin the top layer (U) until one color on the edge matches the center below it. Perfect! Now peek at the edge's other color. Does it match the center on the LEFT or RIGHT? That tells you which way to move it!".to_string(),
            tip: Some("Always match one edge color to the center below it first".to_string()),
        }
    }

    /// Right algorithm
    fn right_algorithm_step() -> SecondLayerLessonStep {
        SecondLayerLessonStep {
            title: "Right Algorithm: U R U' R' U' F' U F".to_string(),
            description: "Use this algorithm when the edge belongs on the RIGHT side. The edge's other color should match the center on the right. This sequence moves the edge from the top down into the right slot in the middle layer.".to_string(),
            example_moves: Some(vec![
                Move::U, Move::R, Move::UPrime, Move::RPrime,
                Move::UPrime, Move::FPrime, Move::U, Move::F
            ]),
            kid_friendly_text: "When the edge needs to go RIGHT, use this: Up, Right, Up-back, Right-back, Up-back, Front-back, Up, Front. It's like: U R U' R' U' F' U F. Practice it a few times!".to_string(),
            tip: Some("Think of it as: setup (U R U' R'), then insert (U' F' U F)".to_string()),
        }
    }

    /// Left algorithm
    fn left_algorithm_step() -> SecondLayerLessonStep {
        SecondLayerLessonStep {
            title: "Left Algorithm: U' L' U L U F U' F'".to_string(),
            description: "Use this algorithm when the edge belongs on the LEFT side. The edge's other color should match the center on the left. This is the mirror image of the right algorithm.".to_string(),
            example_moves: Some(vec![
                Move::UPrime, Move::LPrime, Move::U, Move::L,
                Move::U, Move::F, Move::UPrime, Move::FPrime
            ]),
            kid_friendly_text: "When the edge needs to go LEFT, use this: Up-back, Left-back, Up, Left, Up, Front, Up-back, Front-back. It's like: U' L' U L U F U' F'. Just the opposite of the right algorithm!".to_string(),
            tip: Some("The left and right algorithms are mirror images of each other".to_string()),
        }
    }

    /// When edge is already in second layer but wrong
    fn wrong_edge_in_slot_step() -> SecondLayerLessonStep {
        SecondLayerLessonStep {
            title: "Edge Already in Second Layer (Wrong)".to_string(),
            description: "Sometimes an edge is already in the middle layer but in the wrong position or flipped. To fix this, use either algorithm (right or left) to pop the edge out into the top layer. Then solve it normally.".to_string(),
            example_moves: Some(vec![
                Move::U, Move::R, Move::UPrime, Move::RPrime,
                Move::UPrime, Move::FPrime, Move::U, Move::F
            ]),
            kid_friendly_text: "If an edge is stuck in the wrong spot in the middle, don't worry! Just use the right or left algorithm to pop it up to the top layer like opening a door. Then put it in the correct spot!".to_string(),
            tip: Some("Any algorithm will pop out a wrong edge - just pick the most convenient one".to_string()),
        }
    }

    /// Tips and tricks
    fn tips_and_tricks_step() -> SecondLayerLessonStep {
        SecondLayerLessonStep {
            title: "Tips for Faster Second Layer".to_string(),
            description: "The key is to memorize just TWO algorithms: right (U R U' R' U' F' U F) and left (U' L' U L U F U' F'). They're mirror images! With practice, you'll quickly recognize which one to use by checking where the edge's other color matches.".to_string(),
            example_moves: Some(vec![
                Move::U, Move::R, Move::UPrime, Move::RPrime,
                Move::UPrime, Move::FPrime, Move::U, Move::F
            ]),
            kid_friendly_text: "You only need TWO moves for the whole second layer! Right algorithm and left algorithm. They're opposites of each other, so if you know one, you know both! Practice them 10 times each today.".to_string(),
            tip: Some("Advanced solvers pair edges with corners (F2L pairs), but learn this method first!".to_string()),
        }
    }

    /// Practice step
    fn practice_step() -> SecondLayerLessonStep {
        SecondLayerLessonStep {
            title: "Let's Practice!".to_string(),
            description: "Solve the entire first layer, then find the 4 second layer edges. Insert them one by one: position each edge above its home, check left or right, then use the correct algorithm. Take your time!".to_string(),
            example_moves: None,
            kid_friendly_text: "You're ready to practice! Solve the white layer first, then add the 4 middle edges. Go slow at first - speed comes with practice. Try to solve the second layer 5 times today!".to_string(),
            tip: Some("Goal: Solve first two layers (F2L) in under 2 minutes!".to_string()),
        }
    }

    // ==================== Edge Cases ====================

    /// Edge goes to the right
    fn edge_goes_right() -> EdgeCase {
        EdgeCase {
            name: "Edge Goes Right".to_string(),
            description: "The edge is in the top layer with front color matching the center below, and the other color matching the RIGHT center.".to_string(),
            algorithm: vec![
                Move::U, Move::R, Move::UPrime, Move::RPrime,
                Move::UPrime, Move::FPrime, Move::U, Move::F
            ],
            explanation: "Position edge above its home (front color matches center), then use: U R U' R' U' F' U F to insert it into the right slot.".to_string(),
            position: EdgePosition::FrontRight,
        }
    }

    /// Edge goes to the left
    fn edge_goes_left() -> EdgeCase {
        EdgeCase {
            name: "Edge Goes Left".to_string(),
            description: "The edge is in the top layer with front color matching the center below, and the other color matching the LEFT center.".to_string(),
            algorithm: vec![
                Move::UPrime, Move::LPrime, Move::U, Move::L,
                Move::U, Move::F, Move::UPrime, Move::FPrime
            ],
            explanation: "Position edge above its home (front color matches center), then use: U' L' U L U F U' F' to insert it into the left slot.".to_string(),
            position: EdgePosition::FrontLeft,
        }
    }

    /// Edge in slot but wrong orientation
    fn edge_in_slot_wrong_orientation() -> EdgeCase {
        EdgeCase {
            name: "Edge in Slot - Flipped".to_string(),
            description: "The edge is already in the middle layer in the correct position but flipped (colors reversed).".to_string(),
            algorithm: vec![
                Move::U, Move::R, Move::UPrime, Move::RPrime,
                Move::UPrime, Move::FPrime, Move::U, Move::F,
                Move::U2,
                Move::U, Move::R, Move::UPrime, Move::RPrime,
                Move::UPrime, Move::FPrime, Move::U, Move::F
            ],
            explanation: "Pop the edge out with right algorithm, rotate top to position it correctly, then insert again with right algorithm.".to_string(),
            position: EdgePosition::FrontRight,
        }
    }

    /// Edge in wrong slot
    fn edge_in_slot_wrong_position() -> EdgeCase {
        EdgeCase {
            name: "Edge in Slot - Wrong Position".to_string(),
            description: "The edge is in the middle layer but in the wrong slot entirely.".to_string(),
            algorithm: vec![
                Move::U, Move::R, Move::UPrime, Move::RPrime,
                Move::UPrime, Move::FPrime, Move::U, Move::F
            ],
            explanation: "Use right or left algorithm to pop the edge up to the top layer, then solve it normally by positioning it correctly.".to_string(),
            position: EdgePosition::FrontRight,
        }
    }

    /// Edge has yellow on it (wrong layer)
    fn edge_in_top_with_yellow() -> EdgeCase {
        EdgeCase {
            name: "Edge Has Yellow".to_string(),
            description: "The edge in the top layer has yellow on it - this edge belongs in the top layer, not the second layer.".to_string(),
            algorithm: vec![],
            explanation: "Skip this edge! Edges with yellow belong in the top layer and will be solved in the last layer steps (OLL/PLL). Focus on edges with NO yellow.".to_string(),
            position: EdgePosition::FrontRight,
        }
    }

    // ==================== Practice Exercises ====================

    /// Simple edge practice
    fn simple_edge_exercise() -> SecondLayerPracticeExercise {
        SecondLayerPracticeExercise {
            title: "Simple Edge Practice".to_string(),
            description: "This exercise has 1 edge in the top layer ready to insert into the right slot. First layer is already solved.".to_string(),
            setup_moves: vec![
                Move::U, Move::R, Move::UPrime, Move::RPrime,
                Move::UPrime, Move::FPrime, Move::U, Move::F
            ],
            solution: vec![
                Move::U, Move::R, Move::UPrime, Move::RPrime,
                Move::UPrime, Move::FPrime, Move::U, Move::F
            ],
            hint: "Match the front color to the center, then use the right algorithm!".to_string(),
        }
    }

    /// Medium difficulty edge
    fn medium_edge_exercise() -> SecondLayerPracticeExercise {
        SecondLayerPracticeExercise {
            title: "Medium Edge Challenge".to_string(),
            description: "This exercise has 2 edges to solve - one goes right, one goes left. Practice both algorithms!".to_string(),
            setup_moves: vec![
                Move::U, Move::R, Move::UPrime, Move::RPrime,
                Move::UPrime, Move::FPrime, Move::U, Move::F,
                Move::U2,
                Move::UPrime, Move::LPrime, Move::U, Move::L,
                Move::U, Move::F, Move::UPrime, Move::FPrime,
            ],
            solution: vec![
                Move::U, Move::R, Move::UPrime, Move::RPrime,
                Move::UPrime, Move::FPrime, Move::U, Move::F,
                Move::U,
                Move::UPrime, Move::LPrime, Move::U, Move::L,
                Move::U, Move::F, Move::UPrime, Move::FPrime,
            ],
            hint: "Do one edge at a time. Check if it goes left or right before starting!".to_string(),
        }
    }

    /// Advanced edge exercise
    fn advanced_edge_exercise() -> SecondLayerPracticeExercise {
        SecondLayerPracticeExercise {
            title: "Advanced Second Layer Challenge".to_string(),
            description: "All 4 second layer edges are scrambled! Some in top, some might be in wrong slots. This is a complete second layer practice.".to_string(),
            setup_moves: vec![
                // Pop out all middle edges and scramble them
                Move::U, Move::R, Move::UPrime, Move::RPrime,
                Move::UPrime, Move::FPrime, Move::U, Move::F,
                Move::U,
                Move::U, Move::R, Move::UPrime, Move::RPrime,
                Move::UPrime, Move::FPrime, Move::U, Move::F,
                Move::U2,
                Move::UPrime, Move::LPrime, Move::U, Move::L,
                Move::U, Move::F, Move::UPrime, Move::FPrime,
                Move::U,
                Move::UPrime, Move::LPrime, Move::U, Move::L,
                Move::U, Move::F, Move::UPrime, Move::FPrime,
            ],
            solution: vec![
                // Insert edges one by one (example solution)
                Move::U, Move::R, Move::UPrime, Move::RPrime,
                Move::UPrime, Move::FPrime, Move::U, Move::F,
                Move::U2,
                Move::U, Move::R, Move::UPrime, Move::RPrime,
                Move::UPrime, Move::FPrime, Move::U, Move::F,
                Move::U,
                Move::UPrime, Move::LPrime, Move::U, Move::L,
                Move::U, Move::F, Move::UPrime, Move::FPrime,
                Move::U2,
                Move::UPrime, Move::LPrime, Move::U, Move::L,
                Move::U, Move::F, Move::UPrime, Move::FPrime,
            ],
            hint: "Take your time! Find each edge one at a time. If an edge is stuck in the wrong slot, pop it out first!".to_string(),
        }
    }

    // ==================== Helper Methods ====================

    /// Get the total number of steps
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Get a specific step by index
    pub fn get_step(&self, index: usize) -> Option<&SecondLayerLessonStep> {
        self.steps.get(index)
    }

    /// Get all steps
    pub fn get_all_steps(&self) -> &[SecondLayerLessonStep] {
        &self.steps
    }

    /// Get all edge cases
    pub fn get_all_cases(&self) -> &[EdgeCase] {
        &self.cases
    }

    /// Get a specific case by index
    pub fn get_case(&self, index: usize) -> Option<&EdgeCase> {
        self.cases.get(index)
    }

    /// Get all practice exercises
    pub fn get_practice_exercises(&self) -> &[SecondLayerPracticeExercise] {
        &self.practice_exercises
    }

    /// Get a specific practice exercise
    pub fn get_practice_exercise(&self, index: usize) -> Option<&SecondLayerPracticeExercise> {
        self.practice_exercises.get(index)
    }

    /// Verify if a cube has the first two layers (F2L) solved
    /// Checks if the white face and middle layer are complete
    pub fn verify_f2l(cube: &Cube) -> bool {
        if cube.size() != 3 {
            return false; // F2L is for 3x3 only
        }

        // First, verify the entire first layer is solved
        let down_face = cube.get_face(FaceName::D);
        let down_center = down_face.get(1, 1);

        let up_face = cube.get_face(FaceName::U);
        let up_center = up_face.get(1, 1);

        let (target_face, check_bottom): (&Face, bool) =
            if down_center == Color::White {
                (down_face, true) // white on bottom
            } else if up_center == Color::White {
                (up_face, false) // white on top
            } else {
                return false; // No white center on up or down
            };

        // Check that ALL 9 stickers on the white face are white
        for row in 0..3 {
            for col in 0..3 {
                if target_face.get(row, col) != Color::White {
                    return false;
                }
            }
        }

        // Check the first two rows of each side face (bottom and middle)
        let (row1, row2) = if check_bottom { (2, 1) } else { (0, 1) };

        let faces = [FaceName::F, FaceName::R, FaceName::B, FaceName::L];
        for face_name in &faces {
            let face = cube.get_face(*face_name);
            let center_color = face.get(1, 1);

            // Check both the bottom row (first layer) and middle row (second layer)
            for col in 0..3 {
                if face.get(row1, col) != center_color {
                    return false;
                }
                if face.get(row2, col) != center_color {
                    return false;
                }
            }
        }

        true
    }
}

impl Default for SecondLayerLesson {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_layer_lesson_creation() {
        let lesson = SecondLayerLesson::new();
        assert!(lesson.step_count() > 0);
        assert_eq!(lesson.step_count(), 9);
    }

    #[test]
    fn test_lesson_steps_have_content() {
        let lesson = SecondLayerLesson::new();
        for step in lesson.get_all_steps() {
            assert!(!step.title.is_empty());
            assert!(!step.description.is_empty());
            assert!(!step.kid_friendly_text.is_empty());
        }
    }

    #[test]
    fn test_has_edge_cases() {
        let lesson = SecondLayerLesson::new();
        assert_eq!(lesson.cases.len(), 5);

        for case in lesson.get_all_cases() {
            assert!(!case.name.is_empty());
            assert!(!case.description.is_empty());
            assert!(!case.explanation.is_empty());
        }
    }

    #[test]
    fn test_has_practice_exercises() {
        let lesson = SecondLayerLesson::new();
        assert_eq!(lesson.practice_exercises.len(), 3);

        for exercise in lesson.get_practice_exercises() {
            assert!(!exercise.title.is_empty());
            assert!(!exercise.description.is_empty());
            assert!(!exercise.hint.is_empty());
        }
    }

    #[test]
    fn test_verify_solved_cube_f2l() {
        let cube = Cube::new(3);
        assert!(SecondLayerLesson::verify_f2l(&cube));
    }

    #[test]
    fn test_verify_scrambled_cube_no_f2l() {
        let mut cube = Cube::new(3);
        cube.apply_move(Move::R);
        cube.apply_move(Move::U);
        cube.apply_move(Move::RPrime);

        // After scrambling, F2L should not be complete
        assert!(!SecondLayerLesson::verify_f2l(&cube));
    }

    #[test]
    fn test_right_algorithm_has_moves() {
        let lesson = SecondLayerLesson::new();
        let case = lesson.get_case(0).unwrap(); // Right algorithm
        assert!(!case.algorithm.is_empty());
        assert_eq!(case.algorithm.len(), 8);
    }

    #[test]
    fn test_left_algorithm_has_moves() {
        let lesson = SecondLayerLesson::new();
        let case = lesson.get_case(1).unwrap(); // Left algorithm
        assert!(!case.algorithm.is_empty());
        assert_eq!(case.algorithm.len(), 8);
    }

    #[test]
    fn test_edge_position_enum() {
        // Just verify the enum variants exist
        let _fr = EdgePosition::FrontRight;
        let _fl = EdgePosition::FrontLeft;
        let _br = EdgePosition::BackRight;
        let _bl = EdgePosition::BackLeft;
    }

    #[test]
    fn test_get_specific_step() {
        let lesson = SecondLayerLesson::new();
        let first_step = lesson.get_step(0);
        assert!(first_step.is_some());
        assert!(first_step.unwrap().title.contains("Second Layer"));
    }

    #[test]
    fn test_get_specific_case() {
        let lesson = SecondLayerLesson::new();
        let first_case = lesson.get_case(0);
        assert!(first_case.is_some());
        assert!(first_case.unwrap().name.contains("Right"));
    }

    #[test]
    fn test_simple_exercise_has_solution() {
        let lesson = SecondLayerLesson::new();
        let exercise = lesson.get_practice_exercise(0).unwrap();
        assert!(!exercise.solution.is_empty());
    }

    #[test]
    fn test_all_exercises_have_hints() {
        let lesson = SecondLayerLesson::new();
        for exercise in lesson.get_practice_exercises() {
            assert!(!exercise.hint.is_empty());
        }
    }

    #[test]
    fn test_algorithms_are_different() {
        let lesson = SecondLayerLesson::new();
        let right = &lesson.get_case(0).unwrap().algorithm;
        let left = &lesson.get_case(1).unwrap().algorithm;
        assert_ne!(right, left);
    }

    #[test]
    fn test_right_algorithm_correct() {
        let lesson = SecondLayerLesson::new();
        let case = lesson.get_case(0).unwrap();
        assert_eq!(case.algorithm, vec![
            Move::U, Move::R, Move::UPrime, Move::RPrime,
            Move::UPrime, Move::FPrime, Move::U, Move::F
        ]);
    }

    #[test]
    fn test_left_algorithm_correct() {
        let lesson = SecondLayerLesson::new();
        let case = lesson.get_case(1).unwrap();
        assert_eq!(case.algorithm, vec![
            Move::UPrime, Move::LPrime, Move::U, Move::L,
            Move::U, Move::F, Move::UPrime, Move::FPrime
        ]);
    }
}
