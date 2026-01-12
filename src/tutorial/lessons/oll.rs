//! 3x3 tutorial: OLL (Orient Last Layer)
//!
//! This module implements R6.6 from the PRD:
//! - 2-look OLL algorithms
//! - Pattern recognition
//! - Practice mode

use crate::cube::Move;

/// Represents different OLL patterns/cases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OllPattern {
    /// No edges oriented (dot pattern)
    Dot,
    /// Two adjacent edges oriented (L shape)
    LShape,
    /// Two opposite edges oriented (line)
    Line,
    /// All edges oriented (cross)
    Cross,
    /// Sune pattern
    Sune,
    /// AntiSune pattern
    AntiSune,
    /// Pi pattern
    Pi,
    /// H pattern
    H,
    /// T pattern
    T,
    /// L pattern (corners)
    LPattern,
    /// U pattern
    U,
}

/// Represents an OLL solving case/algorithm
#[derive(Debug, Clone, PartialEq)]
pub struct OllCase {
    /// Name of the case
    pub name: String,
    /// Description of the pattern
    pub description: String,
    /// The algorithm (sequence of moves)
    pub algorithm: Vec<Move>,
    /// Kid-friendly explanation
    pub explanation: String,
    /// Which OLL pattern this case addresses
    pub pattern: OllPattern,
    /// Visual representation hint
    pub visual_hint: String,
}

/// Represents a single lesson step for the OLL tutorial
#[derive(Debug, Clone, PartialEq)]
pub struct OllLessonStep {
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

/// Practice exercise for OLL
#[derive(Debug, Clone, PartialEq)]
pub struct OllPracticeExercise {
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
    pub pattern: OllPattern,
}

/// The complete OLL tutorial lesson
#[derive(Debug, Clone)]
pub struct OllLesson {
    /// Lesson steps
    pub steps: Vec<OllLessonStep>,
    /// Common OLL cases/algorithms (2-look OLL)
    pub cases: Vec<OllCase>,
    /// Practice exercises
    pub practice_exercises: Vec<OllPracticeExercise>,
}

impl OllLesson {
    /// Creates a new OLL lesson with all steps
    pub fn new() -> Self {
        Self {
            steps: vec![
                Self::intro_step(),
                Self::what_is_oll_step(),
                Self::two_look_oll_explanation_step(),
                Self::edge_orientation_step(),
                Self::corner_orientation_step(),
                Self::pattern_recognition_step(),
                Self::practice_step(),
            ],
            cases: vec![
                // Step 1: Edge orientation (make yellow cross on top)
                Self::dot_to_line(),
                Self::dot_to_l_shape(),
                Self::l_shape_to_cross(),
                Self::line_to_cross(),
                // Step 2: Corner orientation (orient all corners)
                Self::sune(),
                Self::antisune(),
                Self::h_pattern(),
                Self::pi_pattern(),
                Self::t_pattern(),
                Self::l_pattern(),
                Self::u_pattern(),
            ],
            practice_exercises: vec![
                Self::practice_dot_pattern(),
                Self::practice_l_shape(),
                Self::practice_line(),
                Self::practice_sune(),
                Self::practice_antisune(),
            ],
        }
    }

    // Lesson steps

    fn intro_step() -> OllLessonStep {
        OllLessonStep {
            title: "Welcome to OLL!".to_string(),
            description: "OLL stands for 'Orient Last Layer'. This is the step where we get all the yellow stickers facing up on the top layer. We'll use a beginner-friendly method called '2-look OLL' that breaks it into two easy steps.".to_string(),
            example_moves: None,
            kid_friendly_text: "Think of OLL like flipping all the yellow stickers so they face the sky! First we make a yellow cross, then we flip the corners.".to_string(),
            tip: Some("Don't worry about where the pieces are - we only care about making them face up!".to_string()),
        }
    }

    fn what_is_oll_step() -> OllLessonStep {
        OllLessonStep {
            title: "What is OLL?".to_string(),
            description: "After solving the first two layers, the top layer will be scrambled. OLL is the step where we orient (flip) all the pieces on the top layer so the yellow stickers all face up.".to_string(),
            example_moves: None,
            kid_friendly_text: "Right now, some yellow stickers might be on the sides. OLL flips them all to the top!".to_string(),
            tip: Some("The pieces might be in the wrong positions, but that's okay - we'll fix that in the next step (PLL)!".to_string()),
        }
    }

    fn two_look_oll_explanation_step() -> OllLessonStep {
        OllLessonStep {
            title: "2-Look OLL Method".to_string(),
            description: "Instead of learning 57 algorithms, beginners use 2-look OLL which only needs about 10 algorithms. Step 1: Make a yellow cross. Step 2: Orient the corners.".to_string(),
            example_moves: None,
            kid_friendly_text: "We split OLL into two parts: First make a yellow + (plus sign) on top, then flip all the corners yellow side up!".to_string(),
            tip: Some("Advanced solvers use 1-look OLL with 57 algorithms, but 2-look is much easier to learn!".to_string()),
        }
    }

    fn edge_orientation_step() -> OllLessonStep {
        OllLessonStep {
            title: "Step 1: Edge Orientation (Yellow Cross)".to_string(),
            description: "The first step is to get all four edge pieces oriented so their yellow stickers face up, making a yellow cross pattern on the top.".to_string(),
            example_moves: Some(vec![
                Move::F, Move::RPrime, Move::U, Move::R, Move::UPrime, Move::FPrime
            ]),
            kid_friendly_text: "Look at the top of your cube. You might see a dot, L shape, line, or already have a cross. We'll use special moves to make it into a + shape!".to_string(),
            tip: Some("The algorithm 'F R U R' U' F'' works for multiple cases - just repeat and turn the cube!".to_string()),
        }
    }

    fn corner_orientation_step() -> OllLessonStep {
        OllLessonStep {
            title: "Step 2: Corner Orientation".to_string(),
            description: "Once you have the yellow cross, orient the corner pieces so all their yellow stickers face up too. This completes OLL!".to_string(),
            example_moves: Some(vec![
                Move::R, Move::U, Move::RPrime, Move::U, Move::R, Move::U2, Move::RPrime
            ]),
            kid_friendly_text: "Now flip the corners! The most common algorithm is called 'Sune' - it's super useful and flips corners in a special way.".to_string(),
            tip: Some("You might need to repeat the algorithm or turn the top layer between uses!".to_string()),
        }
    }

    fn pattern_recognition_step() -> OllLessonStep {
        OllLessonStep {
            title: "Pattern Recognition".to_string(),
            description: "Learning to quickly recognize which OLL case you have is an important skill. Look at the yellow stickers on top and match them to the patterns you learned.".to_string(),
            example_moves: None,
            kid_friendly_text: "With practice, you'll see patterns super fast! Start by counting how many yellow stickers are on top, then look at their shape.".to_string(),
            tip: Some("For edges: Dot → L → Line → Cross. For corners: Look for familiar shapes like Sune, T, or H!".to_string()),
        }
    }

    fn practice_step() -> OllLessonStep {
        OllLessonStep {
            title: "Practice Time!".to_string(),
            description: "Now it's time to practice! Try the exercises below to master OLL. Remember, it's okay to look up the algorithms - with practice, you'll memorize them naturally.".to_string(),
            example_moves: None,
            kid_friendly_text: "Let's practice! Start with easy cases and work your way up. You've got this!".to_string(),
            tip: Some("Practice one algorithm at a time until it feels natural, then move to the next one.".to_string()),
        }
    }

    // OLL Cases

    // Step 1: Edge Orientation

    fn dot_to_line() -> OllCase {
        use Move::*;
        OllCase {
            name: "Dot to Line".to_string(),
            description: "When you have no yellow edges oriented (dot pattern), use this to make a line".to_string(),
            algorithm: vec![F, RPrime, U, R, UPrime, FPrime],
            explanation: "Hold the cube any way, do F R' U R U' F', and you'll get a line!".to_string(),
            pattern: OllPattern::Dot,
            visual_hint: "No yellow edges on top = dot in the middle".to_string(),
        }
    }

    fn dot_to_l_shape() -> OllCase {
        use Move::*;
        OllCase {
            name: "Dot to L-Shape (Alternative)".to_string(),
            description: "Another option from dot pattern - makes an L shape instead".to_string(),
            algorithm: vec![F, U, R, UPrime, RPrime, FPrime],
            explanation: "Similar to the line algorithm but turns U instead of U' - makes an L!".to_string(),
            pattern: OllPattern::Dot,
            visual_hint: "No yellow edges on top = dot in the middle".to_string(),
        }
    }

    fn l_shape_to_cross() -> OllCase {
        use Move::*;
        OllCase {
            name: "L-Shape to Cross".to_string(),
            description: "When you have an L shape (two adjacent edges oriented), make it a cross".to_string(),
            algorithm: vec![F, U, R, UPrime, RPrime, FPrime],
            explanation: "Hold the L in the back-left corner, then do F U R U' R' F' to finish the cross!".to_string(),
            pattern: OllPattern::LShape,
            visual_hint: "Two yellow edges forming an L shape".to_string(),
        }
    }

    fn line_to_cross() -> OllCase {
        use Move::*;
        OllCase {
            name: "Line to Cross".to_string(),
            description: "When you have a line (two opposite edges oriented), make it a cross".to_string(),
            algorithm: vec![F, R, U, RPrime, UPrime, FPrime],
            explanation: "Hold the line horizontal (left-right), do F R U R' U' F', and boom - yellow cross!".to_string(),
            pattern: OllPattern::Line,
            visual_hint: "Yellow line going left to right or front to back".to_string(),
        }
    }

    // Step 2: Corner Orientation

    fn sune() -> OllCase {
        use Move::*;
        OllCase {
            name: "Sune".to_string(),
            description: "The most famous OLL algorithm - orients corners when you see the Sune pattern".to_string(),
            algorithm: vec![R, U, RPrime, U, R, U2, RPrime],
            explanation: "Sune is your best friend! Hold the yellow corners correctly and do R U R' U R U2 R'.".to_string(),
            pattern: OllPattern::Sune,
            visual_hint: "Yellow cross already done, one corner correct, headlights on right".to_string(),
        }
    }

    fn antisune() -> OllCase {
        use Move::*;
        OllCase {
            name: "AntiSune".to_string(),
            description: "Mirror of Sune - used when the pattern is flipped".to_string(),
            algorithm: vec![RPrime, UPrime, R, UPrime, RPrime, U2, R],
            explanation: "It's Sune backwards! Hold it right and do R' U' R U' R' U2 R.".to_string(),
            pattern: OllPattern::AntiSune,
            visual_hint: "Yellow cross done, headlights on left".to_string(),
        }
    }

    fn h_pattern() -> OllCase {
        use Move::*;
        OllCase {
            name: "H Pattern".to_string(),
            description: "All corners need to flip - looks like an H".to_string(),
            algorithm: vec![R, U2, RPrime, UPrime, R, U, RPrime, UPrime, R, UPrime, RPrime],
            explanation: "Big algorithm but easy to remember - it's like two Sunes together!".to_string(),
            pattern: OllPattern::H,
            visual_hint: "Yellow cross done, corners make an H shape".to_string(),
        }
    }

    fn pi_pattern() -> OllCase {
        use Move::*;
        OllCase {
            name: "Pi Pattern".to_string(),
            description: "Two corners correct, two need flipping - looks like π".to_string(),
            algorithm: vec![R, U2, R2, UPrime, R2, UPrime, R2, U2, R],
            explanation: "Hold the correct corners in back, then do this algorithm to fix the front!".to_string(),
            pattern: OllPattern::Pi,
            visual_hint: "Yellow cross done, two corners solved in back".to_string(),
        }
    }

    fn t_pattern() -> OllCase {
        use Move::*;
        OllCase {
            name: "T Pattern".to_string(),
            description: "Makes a T shape - one corner correct, three need work".to_string(),
            algorithm: vec![R, U, RPrime, UPrime, RPrime, F, R, FPrime],
            explanation: "This one is called 'Sexy Sledge' - R U R' U' then do a sledgehammer!".to_string(),
            pattern: OllPattern::T,
            visual_hint: "Yellow cross done, T shape on top".to_string(),
        }
    }

    fn l_pattern() -> OllCase {
        use Move::*;
        OllCase {
            name: "L Pattern (Bowtie)".to_string(),
            description: "Two corners diagonally opposite are correct".to_string(),
            algorithm: vec![F, R, UPrime, RPrime, UPrime, R, U, RPrime, FPrime],
            explanation: "Hold the correct corners on the right side, then do this algorithm!".to_string(),
            pattern: OllPattern::LPattern,
            visual_hint: "Yellow cross done, diagonal corners solved".to_string(),
        }
    }

    fn u_pattern() -> OllCase {
        use Move::*;
        OllCase {
            name: "U Pattern (Headlights)".to_string(),
            description: "Two adjacent corners are correct - looks like headlights".to_string(),
            algorithm: vec![R, U2, RPrime, UPrime, R, UPrime, RPrime],
            explanation: "This is like half a Sune! Keep the headlights in back and solve it.".to_string(),
            pattern: OllPattern::U,
            visual_hint: "Yellow cross done, two corners solved in back".to_string(),
        }
    }

    // Practice exercises

    fn practice_dot_pattern() -> OllPracticeExercise {
        use Move::*;
        OllPracticeExercise {
            title: "Practice: Dot Pattern".to_string(),
            description: "Practice making a yellow cross from a dot pattern".to_string(),
            setup_moves: vec![R, U, RPrime, UPrime, RPrime, F, R, FPrime],
            solution: vec![F, RPrime, U, R, UPrime, FPrime],
            hint: "Use the Dot to Line algorithm: F R' U R U' F'".to_string(),
            pattern: OllPattern::Dot,
        }
    }

    fn practice_l_shape() -> OllPracticeExercise {
        use Move::*;
        OllPracticeExercise {
            title: "Practice: L-Shape Pattern".to_string(),
            description: "Practice converting an L-shape to a cross".to_string(),
            setup_moves: vec![F, R, U, RPrime, UPrime, FPrime, U2],
            solution: vec![F, U, R, UPrime, RPrime, FPrime],
            hint: "Hold the L in back-left, use: F U R U' R' F'".to_string(),
            pattern: OllPattern::LShape,
        }
    }

    fn practice_line() -> OllPracticeExercise {
        use Move::*;
        OllPracticeExercise {
            title: "Practice: Line Pattern".to_string(),
            description: "Practice making a cross from a line".to_string(),
            setup_moves: vec![F, R, U, RPrime, UPrime, FPrime],
            solution: vec![F, R, U, RPrime, UPrime, FPrime],
            hint: "Hold line horizontal, use: F R U R' U' F'".to_string(),
            pattern: OllPattern::Line,
        }
    }

    fn practice_sune() -> OllPracticeExercise {
        use Move::*;
        OllPracticeExercise {
            title: "Practice: Sune".to_string(),
            description: "Practice the famous Sune algorithm".to_string(),
            setup_moves: vec![R, U, RPrime, U, R, U2, RPrime, U2],
            solution: vec![R, U, RPrime, U, R, U2, RPrime],
            hint: "The most important OLL algorithm: R U R' U R U2 R'".to_string(),
            pattern: OllPattern::Sune,
        }
    }

    fn practice_antisune() -> OllPracticeExercise {
        use Move::*;
        OllPracticeExercise {
            title: "Practice: AntiSune".to_string(),
            description: "Practice the AntiSune algorithm".to_string(),
            setup_moves: vec![RPrime, UPrime, R, UPrime, RPrime, U2, R, U2],
            solution: vec![RPrime, UPrime, R, UPrime, RPrime, U2, R],
            hint: "Sune's mirror: R' U' R U' R' U2 R".to_string(),
            pattern: OllPattern::AntiSune,
        }
    }

    /// Returns all lesson steps
    pub fn get_steps(&self) -> &[OllLessonStep] {
        &self.steps
    }

    /// Returns all OLL cases
    pub fn get_cases(&self) -> &[OllCase] {
        &self.cases
    }

    /// Returns all practice exercises
    pub fn get_practice_exercises(&self) -> &[OllPracticeExercise] {
        &self.practice_exercises
    }

    /// Returns a specific case by pattern
    pub fn get_case_by_pattern(&self, pattern: OllPattern) -> Option<&OllCase> {
        self.cases.iter().find(|c| c.pattern == pattern)
    }
}

impl Default for OllLesson {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oll_lesson_creation() {
        let lesson = OllLesson::new();
        assert!(!lesson.steps.is_empty());
        assert!(!lesson.cases.is_empty());
        assert!(!lesson.practice_exercises.is_empty());
    }

    #[test]
    fn test_oll_lesson_has_all_steps() {
        let lesson = OllLesson::new();
        assert_eq!(lesson.steps.len(), 7);
        assert_eq!(lesson.steps[0].title, "Welcome to OLL!");
    }

    #[test]
    fn test_oll_lesson_has_2look_cases() {
        let lesson = OllLesson::new();
        // 2-look OLL should have edge + corner cases
        assert!(lesson.cases.len() >= 10); // At least the common cases

        // Check we have edge orientation cases
        let has_edge_cases = lesson.cases.iter().any(|c|
            c.pattern == OllPattern::Dot ||
            c.pattern == OllPattern::Line ||
            c.pattern == OllPattern::LShape
        );
        assert!(has_edge_cases);

        // Check we have corner orientation cases
        let has_corner_cases = lesson.cases.iter().any(|c|
            c.pattern == OllPattern::Sune ||
            c.pattern == OllPattern::AntiSune
        );
        assert!(has_corner_cases);
    }

    #[test]
    fn test_sune_algorithm() {
        use Move::*;
        let lesson = OllLesson::new();
        let sune = lesson.get_case_by_pattern(OllPattern::Sune).unwrap();

        assert_eq!(sune.name, "Sune");
        assert_eq!(sune.algorithm, vec![R, U, RPrime, U, R, U2, RPrime]);
    }

    #[test]
    fn test_antisune_algorithm() {
        use Move::*;
        let lesson = OllLesson::new();
        let antisune = lesson.get_case_by_pattern(OllPattern::AntiSune).unwrap();

        assert_eq!(antisune.name, "AntiSune");
        assert_eq!(antisune.algorithm, vec![RPrime, UPrime, R, UPrime, RPrime, U2, R]);
    }

    #[test]
    fn test_practice_exercises() {
        let lesson = OllLesson::new();
        assert_eq!(lesson.practice_exercises.len(), 5);

        // Each exercise should have setup, solution, and hint
        for exercise in &lesson.practice_exercises {
            assert!(!exercise.setup_moves.is_empty());
            assert!(!exercise.solution.is_empty());
            assert!(!exercise.hint.is_empty());
        }
    }

    #[test]
    fn test_get_case_by_pattern() {
        let lesson = OllLesson::new();

        let sune = lesson.get_case_by_pattern(OllPattern::Sune);
        assert!(sune.is_some());
        assert_eq!(sune.unwrap().name, "Sune");

        let line = lesson.get_case_by_pattern(OllPattern::Line);
        assert!(line.is_some());
    }

    #[test]
    fn test_all_steps_have_content() {
        let lesson = OllLesson::new();

        for step in &lesson.steps {
            assert!(!step.title.is_empty());
            assert!(!step.description.is_empty());
            assert!(!step.kid_friendly_text.is_empty());
        }
    }

    #[test]
    fn test_all_cases_have_visual_hints() {
        let lesson = OllLesson::new();

        for case in &lesson.cases {
            assert!(!case.visual_hint.is_empty());
            assert!(!case.explanation.is_empty());
        }
    }

    #[test]
    fn test_edge_orientation_algorithms() {
        let lesson = OllLesson::new();

        // Should have algorithms for dot, L, and line patterns
        let dot_cases: Vec<_> = lesson.cases.iter()
            .filter(|c| c.pattern == OllPattern::Dot)
            .collect();
        assert!(!dot_cases.is_empty());

        let l_cases: Vec<_> = lesson.cases.iter()
            .filter(|c| c.pattern == OllPattern::LShape)
            .collect();
        assert!(!l_cases.is_empty());

        let line_cases: Vec<_> = lesson.cases.iter()
            .filter(|c| c.pattern == OllPattern::Line)
            .collect();
        assert!(!line_cases.is_empty());
    }
}
