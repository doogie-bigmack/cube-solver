//! 3x3 tutorial: First Layer Corners (F2L Corners)
//!
//! This module implements R6.4 from the PRD:
//! - Identify corner positions
//! - Insertion algorithms
//! - Practice mode

use crate::cube::{Color, Cube, Move, FaceName};
use crate::cube::state::Face;

/// Represents a corner piece position in the first layer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CornerPosition {
    /// Front-Right corner
    FrontRight,
    /// Front-Left corner
    FrontLeft,
    /// Back-Right corner
    BackRight,
    /// Back-Left corner
    BackLeft,
}

/// Represents a corner solving case/algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct CornerCase {
    /// Name of the case
    pub name: String,
    /// Description of when to use this case
    pub description: String,
    /// The algorithm (sequence of moves)
    pub algorithm: Vec<Move>,
    /// Kid-friendly explanation
    pub explanation: String,
    /// Which corner position this case addresses
    pub position: CornerPosition,
}

/// Represents a single lesson step for the first layer corners tutorial
#[derive(Debug, Clone, PartialEq)]
pub struct CornersLessonStep {
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

/// Practice exercise for corner insertion
#[derive(Debug, Clone, PartialEq)]
pub struct CornersPracticeExercise {
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

/// The complete first layer corners tutorial lesson
#[derive(Debug, Clone)]
pub struct CornersLesson {
    /// Lesson steps
    pub steps: Vec<CornersLessonStep>,
    /// Common corner cases/algorithms
    pub cases: Vec<CornerCase>,
    /// Practice exercises
    pub practice_exercises: Vec<CornersPracticeExercise>,
}

impl CornersLesson {
    /// Creates a new first layer corners lesson with all steps
    pub fn new() -> Self {
        Self {
            steps: vec![
                Self::intro_step(),
                Self::what_are_corners_step(),
                Self::find_corners_step(),
                Self::identify_position_step(),
                Self::white_on_top_algorithm_step(),
                Self::white_on_side_algorithm_step(),
                Self::white_on_bottom_algorithm_step(),
                Self::tips_and_tricks_step(),
                Self::practice_step(),
            ],
            cases: vec![
                Self::corner_in_top_white_up(),
                Self::corner_in_top_white_front(),
                Self::corner_in_top_white_side(),
                Self::corner_in_bottom_correct(),
                Self::corner_in_bottom_wrong_orientation(),
                Self::corner_in_bottom_wrong_position(),
            ],
            practice_exercises: vec![
                Self::simple_corner_exercise(),
                Self::medium_corner_exercise(),
                Self::advanced_corner_exercise(),
            ],
        }
    }

    // ==================== Lesson Steps ====================

    /// Introduction to solving first layer corners
    fn intro_step() -> CornersLessonStep {
        CornersLessonStep {
            title: "Welcome to First Layer Corners!".to_string(),
            description: "After solving the white cross, the next step is to complete the entire first layer by solving all 4 corner pieces. This is sometimes called 'First Two Layers' or F2L corners.".to_string(),
            example_moves: None,
            kid_friendly_text: "You've built the foundation with the cross - now let's build the walls! We'll put 4 white corners around your white cross to complete the whole bottom layer.".to_string(),
            tip: Some("Make sure your white cross is solved before starting corners!".to_string()),
        }
    }

    /// Explain what corners are
    fn what_are_corners_step() -> CornersLessonStep {
        CornersLessonStep {
            title: "What are Corner Pieces?".to_string(),
            description: "Corner pieces have 3 colors and sit at the intersection of 3 faces. For the white layer, you need 4 corners that each have white plus 2 other colors (like white-red-blue).".to_string(),
            example_moves: None,
            kid_friendly_text: "Corner pieces are the pointy parts at each corner of the cube! Each one touches 3 faces, so it has 3 different colors. Look for the ones with white on them!".to_string(),
            tip: Some("There are exactly 8 corner pieces on the whole cube, and 4 have white".to_string()),
        }
    }

    /// Find the corner pieces
    fn find_corners_step() -> CornersLessonStep {
        CornersLessonStep {
            title: "Find Your Corner Pieces".to_string(),
            description: "Look for the 4 corner pieces that have white on them. Each white corner also has 2 side colors that must match the centers they touch. For example, white-red-blue goes between the white, red, and blue centers.".to_string(),
            example_moves: None,
            kid_friendly_text: "Time for another treasure hunt! Find the 4 white corners. Check what other colors they have - those colors tell you exactly where each corner belongs!".to_string(),
            tip: Some("The corner's 3 colors tell you its home position - match them to the centers".to_string()),
        }
    }

    /// Identify where corner belongs
    fn identify_position_step() -> CornersLessonStep {
        CornersLessonStep {
            title: "Identify the Corner's Home".to_string(),
            description: "Before inserting a corner, you must know where it belongs. If you have a white-red-blue corner, it goes in the spot between the white center (bottom), red center, and blue center. Rotate the top layer (U) to position the corner above its home spot.".to_string(),
            example_moves: Some(vec![Move::U]),
            kid_friendly_text: "Every corner has a home where its 3 colors match the 3 centers! First, find the corner's home. Then turn the top layer (U) to bring the corner right above its home. Like parking a car in the right spot!".to_string(),
            tip: Some("Hold the cube so the corner's home spot is at the front-right position".to_string()),
        }
    }

    /// Algorithm when white is on top
    fn white_on_top_algorithm_step() -> CornersLessonStep {
        CornersLessonStep {
            title: "Case 1: White Sticker Facing Up".to_string(),
            description: "If the corner is in the top layer with white facing up (toward the yellow center), use this algorithm: R U R'. This moves the corner down and rotates it into place.".to_string(),
            example_moves: Some(vec![Move::R, Move::U, Move::RPrime]),
            kid_friendly_text: "When white is looking at the sky (yellow center), we use a simple move: Right, Up, Right-back (R U R'). Do this 1-3 times until the corner drops into place with white on bottom!".to_string(),
            tip: Some("You might need to repeat R U R' up to 3 times for one corner".to_string()),
        }
    }

    /// Algorithm when white is on front/side
    fn white_on_side_algorithm_step() -> CornersLessonStep {
        CornersLessonStep {
            title: "Case 2: White Sticker Facing Front or Side".to_string(),
            description: "If the corner is in the top layer with white facing front or to the side (not up or down), use the algorithm: F' U' F. This scoops the corner into position. Repeat 1-3 times until solved.".to_string(),
            example_moves: Some(vec![Move::FPrime, Move::UPrime, Move::F]),
            kid_friendly_text: "When white is looking at you or to the side, use Front-back, Up-back, Front (F' U' F). Like scooping ice cream into a cone! Repeat until the corner slides into place.".to_string(),
            tip: Some("These two algorithms (R U R' and F' U' F) are mirror images of each other".to_string()),
        }
    }

    /// When corner is already in bottom but wrong
    fn white_on_bottom_algorithm_step() -> CornersLessonStep {
        CornersLessonStep {
            title: "Case 3: Corner in Bottom Layer (Wrong)".to_string(),
            description: "If a corner is already in the bottom layer but in the wrong position or wrong orientation, you need to pop it out first. Use R U R' to pop it up to the top layer, then solve it normally as if it's in the top.".to_string(),
            example_moves: Some(vec![Move::R, Move::U, Move::RPrime]),
            kid_friendly_text: "Sometimes a corner is stuck in the wrong spot on the bottom. No problem! Use R U R' to pop it up like a piece of toast. Then solve it normally with your corner algorithms!".to_string(),
            tip: Some("Always pop wrong corners up to the top layer before solving them".to_string()),
        }
    }

    /// Tips and tricks
    fn tips_and_tricks_step() -> CornersLessonStep {
        CornersLessonStep {
            title: "Tips for Faster Corners".to_string(),
            description: "The key is repetition! The same two algorithms (R U R' and F' U' F) solve all corner cases - just repeat them. With practice, you'll instinctively know which one to use. Don't overthink it!".to_string(),
            example_moves: Some(vec![Move::R, Move::U, Move::RPrime, Move::U, Move::R, Move::U, Move::RPrime]),
            kid_friendly_text: "Here's the secret: you only need TWO simple moves to solve ALL corners! R U R' and F' U' F. Just practice these and you'll be amazing at corners in no time!".to_string(),
            tip: Some("Advanced tip: You can pair corners with edges (F2L) but that's for later!".to_string()),
        }
    }

    /// Practice step
    fn practice_step() -> CornersLessonStep {
        CornersLessonStep {
            title: "Let's Practice!".to_string(),
            description: "Solve your white cross, then try inserting all 4 corners. Take your time with each corner - find it, position it above its home, then use R U R' or F' U' F. Practice makes perfect!".to_string(),
            example_moves: None,
            kid_friendly_text: "You're ready to practice! Do the cross first, then solve corners one by one. It's okay if you mess up - just pop the corner back up and try again. Practice 5 times and you'll feel like a pro!".to_string(),
            tip: Some("Goal: Solve the first layer (cross + corners) 10 times today!".to_string()),
        }
    }

    // ==================== Corner Cases ====================

    /// Corner in top layer with white facing up
    fn corner_in_top_white_up() -> CornerCase {
        CornerCase {
            name: "Corner in Top - White Up".to_string(),
            description: "White corner is in the top layer with the white sticker facing up (toward yellow center).".to_string(),
            algorithm: vec![Move::R, Move::U, Move::RPrime],
            explanation: "Position corner above its home spot, then use R U R'. Repeat 1-3 times until the corner is solved.".to_string(),
            position: CornerPosition::FrontRight,
        }
    }

    /// Corner in top layer with white facing front
    fn corner_in_top_white_front() -> CornerCase {
        CornerCase {
            name: "Corner in Top - White Front".to_string(),
            description: "White corner is in the top layer with white sticker facing the front.".to_string(),
            algorithm: vec![Move::FPrime, Move::UPrime, Move::F],
            explanation: "Position corner above its home spot, then use F' U' F. Repeat 1-3 times until solved.".to_string(),
            position: CornerPosition::FrontRight,
        }
    }

    /// Corner in top layer with white facing side
    fn corner_in_top_white_side() -> CornerCase {
        CornerCase {
            name: "Corner in Top - White Side".to_string(),
            description: "White corner is in the top layer with white sticker facing to the right side.".to_string(),
            algorithm: vec![Move::R, Move::U, Move::RPrime],
            explanation: "Use R U R' multiple times (usually 2-3) to rotate the corner into the correct orientation.".to_string(),
            position: CornerPosition::FrontRight,
        }
    }

    /// Corner already in bottom, correct position and orientation
    fn corner_in_bottom_correct() -> CornerCase {
        CornerCase {
            name: "Corner in Bottom - Correct".to_string(),
            description: "White corner is already in the bottom layer in the correct position with white facing down.".to_string(),
            algorithm: vec![],
            explanation: "This corner is already solved! Leave it alone and work on the other corners.".to_string(),
            position: CornerPosition::FrontRight,
        }
    }

    /// Corner in bottom but wrong orientation
    fn corner_in_bottom_wrong_orientation() -> CornerCase {
        CornerCase {
            name: "Corner in Bottom - Wrong Orientation".to_string(),
            description: "White corner is in the bottom layer in the right position, but white is facing front or side instead of down.".to_string(),
            algorithm: vec![Move::R, Move::U, Move::RPrime, Move::U, Move::R, Move::U, Move::RPrime],
            explanation: "Pop the corner up with R U R', rotate it in the top layer, then insert it again with the correct orientation.".to_string(),
            position: CornerPosition::FrontRight,
        }
    }

    /// Corner in bottom but wrong position
    fn corner_in_bottom_wrong_position() -> CornerCase {
        CornerCase {
            name: "Corner in Bottom - Wrong Position".to_string(),
            description: "White corner is in the bottom layer but in the wrong corner spot.".to_string(),
            algorithm: vec![Move::R, Move::U, Move::RPrime],
            explanation: "Pop the corner up to the top layer with R U R', then solve it normally by positioning it above the correct home spot.".to_string(),
            position: CornerPosition::FrontRight,
        }
    }

    // ==================== Practice Exercises ====================

    /// Simple corner practice
    fn simple_corner_exercise() -> CornersPracticeExercise {
        CornersPracticeExercise {
            title: "Simple Corner Practice".to_string(),
            description: "This exercise has 1 white corner in the top layer ready to insert. The cross is already solved.".to_string(),
            setup_moves: vec![Move::R, Move::U, Move::RPrime],
            solution: vec![Move::R, Move::U, Move::RPrime],
            hint: "Position the corner above its home and use R U R' once!".to_string(),
        }
    }

    /// Medium difficulty corner
    fn medium_corner_exercise() -> CornersPracticeExercise {
        CornersPracticeExercise {
            title: "Medium Corner Challenge".to_string(),
            description: "This exercise has 2 white corners to solve. Practice identifying which algorithm to use.".to_string(),
            setup_moves: vec![
                Move::R, Move::U, Move::RPrime, Move::U,
                Move::FPrime, Move::UPrime, Move::F, Move::U2,
            ],
            solution: vec![
                Move::R, Move::U, Move::RPrime,
                Move::U,
                Move::FPrime, Move::UPrime, Move::F,
            ],
            hint: "Work on one corner at a time. Position it above its home spot first!".to_string(),
        }
    }

    /// Advanced corner exercise
    fn advanced_corner_exercise() -> CornersPracticeExercise {
        CornersPracticeExercise {
            title: "Advanced Corner Challenge".to_string(),
            description: "All 4 corners are scrambled! Some are in the top layer, some might be in the bottom. This is a complete first layer practice.".to_string(),
            setup_moves: vec![
                Move::R, Move::U, Move::RPrime, Move::U,
                Move::R, Move::U2, Move::RPrime, Move::U,
                Move::FPrime, Move::UPrime, Move::F, Move::UPrime,
                Move::FPrime, Move::UPrime, Move::F,
            ],
            solution: vec![
                Move::U2, Move::R, Move::U, Move::RPrime,
                Move::U, Move::R, Move::U, Move::RPrime,
                Move::U2, Move::FPrime, Move::UPrime, Move::F,
                Move::UPrime, Move::FPrime, Move::UPrime, Move::F,
            ],
            hint: "Don't rush! Solve one corner at a time. If a corner is stuck in the bottom, pop it up first!".to_string(),
        }
    }

    // ==================== Helper Methods ====================

    /// Get the total number of steps
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Get a specific step by index
    pub fn get_step(&self, index: usize) -> Option<&CornersLessonStep> {
        self.steps.get(index)
    }

    /// Get all steps
    pub fn get_all_steps(&self) -> &[CornersLessonStep] {
        &self.steps
    }

    /// Get all corner cases
    pub fn get_all_cases(&self) -> &[CornerCase] {
        &self.cases
    }

    /// Get a specific case by index
    pub fn get_case(&self, index: usize) -> Option<&CornerCase> {
        self.cases.get(index)
    }

    /// Get all practice exercises
    pub fn get_practice_exercises(&self) -> &[CornersPracticeExercise] {
        &self.practice_exercises
    }

    /// Get a specific practice exercise
    pub fn get_practice_exercise(&self, index: usize) -> Option<&CornersPracticeExercise> {
        self.practice_exercises.get(index)
    }

    /// Verify if a cube has the complete first layer solved (cross + corners)
    /// Checks if the white face (or whichever face is being solved) is complete
    pub fn verify_first_layer(cube: &Cube) -> bool {
        if cube.size() != 3 {
            return false; // First layer tutorial is for 3x3 only
        }

        // Determine which face has white center
        let down_face = cube.get_face(FaceName::D);
        let down_center = down_face.get(1, 1);

        let up_face = cube.get_face(FaceName::U);
        let up_center = up_face.get(1, 1);

        let (target_face, check_top_or_bottom): (&Face, bool) =
            if down_center == Color::White {
                (down_face, false) // white on bottom
            } else if up_center == Color::White {
                (up_face, true) // white on top
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

        // Check that the top/bottom row of each side face matches its center
        let side_row = if check_top_or_bottom { 0 } else { 2 };

        let faces = [FaceName::F, FaceName::R, FaceName::B, FaceName::L];
        for face_name in &faces {
            let face = cube.get_face(*face_name);
            let center_color = face.get(1, 1);

            // Check all 3 stickers in the row
            for col in 0..3 {
                if face.get(side_row, col) != center_color {
                    return false;
                }
            }
        }

        true
    }
}

impl Default for CornersLesson {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corners_lesson_creation() {
        let lesson = CornersLesson::new();
        assert!(lesson.step_count() > 0);
        assert_eq!(lesson.step_count(), 9);
    }

    #[test]
    fn test_lesson_steps_have_content() {
        let lesson = CornersLesson::new();
        for step in lesson.get_all_steps() {
            assert!(!step.title.is_empty());
            assert!(!step.description.is_empty());
            assert!(!step.kid_friendly_text.is_empty());
        }
    }

    #[test]
    fn test_has_corner_cases() {
        let lesson = CornersLesson::new();
        assert_eq!(lesson.cases.len(), 6);

        for case in lesson.get_all_cases() {
            assert!(!case.name.is_empty());
            assert!(!case.description.is_empty());
            assert!(!case.explanation.is_empty());
        }
    }

    #[test]
    fn test_has_practice_exercises() {
        let lesson = CornersLesson::new();
        assert_eq!(lesson.practice_exercises.len(), 3);

        for exercise in lesson.get_practice_exercises() {
            assert!(!exercise.title.is_empty());
            assert!(!exercise.description.is_empty());
            assert!(!exercise.hint.is_empty());
            assert!(!exercise.setup_moves.is_empty());
        }
    }

    #[test]
    fn test_verify_solved_cube_first_layer() {
        let cube = Cube::new(3);
        assert!(CornersLesson::verify_first_layer(&cube));
    }

    #[test]
    fn test_verify_scrambled_cube_no_first_layer() {
        let mut cube = Cube::new(3);
        cube.apply_move(Move::R);
        cube.apply_move(Move::U);
        cube.apply_move(Move::RPrime);

        // After scrambling, first layer should not be complete
        assert!(!CornersLesson::verify_first_layer(&cube));
    }

    #[test]
    fn test_algorithms_have_moves() {
        let lesson = CornersLesson::new();
        // Check that most cases have algorithms (some might be empty for "already correct")
        let cases_with_algorithms = lesson.cases.iter()
            .filter(|c| !c.algorithm.is_empty())
            .count();
        assert!(cases_with_algorithms >= 4);
    }

    #[test]
    fn test_white_up_algorithm() {
        let lesson = CornersLesson::new();
        let case = lesson.get_case(0).unwrap(); // First case: white facing up
        assert!(case.algorithm.len() > 0);
        assert_eq!(case.algorithm, vec![Move::R, Move::U, Move::RPrime]);
    }

    #[test]
    fn test_corner_position_enum() {
        // Just verify the enum variants exist
        let _fr = CornerPosition::FrontRight;
        let _fl = CornerPosition::FrontLeft;
        let _br = CornerPosition::BackRight;
        let _bl = CornerPosition::BackLeft;
    }

    #[test]
    fn test_get_specific_step() {
        let lesson = CornersLesson::new();
        let first_step = lesson.get_step(0);
        assert!(first_step.is_some());
        assert!(first_step.unwrap().title.contains("First Layer Corners"));
    }

    #[test]
    fn test_get_specific_case() {
        let lesson = CornersLesson::new();
        let first_case = lesson.get_case(0);
        assert!(first_case.is_some());
        assert!(first_case.unwrap().name.contains("White Up"));
    }

    #[test]
    fn test_simple_exercise_has_solution() {
        let lesson = CornersLesson::new();
        let exercise = lesson.get_practice_exercise(0).unwrap();
        assert!(!exercise.solution.is_empty());
    }

    #[test]
    fn test_all_exercises_have_hints() {
        let lesson = CornersLesson::new();
        for exercise in lesson.get_practice_exercises() {
            assert!(!exercise.hint.is_empty());
        }
    }
}
