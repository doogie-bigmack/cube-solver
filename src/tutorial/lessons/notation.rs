//! Beginner lesson: Cube notation
//!
//! This module implements R6.1 from the PRD:
//! - Explain R, L, U, D, F, B
//! - Explain prime and double moves
//! - Interactive examples

use crate::cube::Move;

/// Represents a single lesson step
#[derive(Debug, Clone, PartialEq)]
pub struct LessonStep {
    /// Title of the step
    pub title: String,
    /// Description/explanation
    pub description: String,
    /// Example move (if applicable)
    pub example_move: Option<Move>,
    /// Kid-friendly explanation
    pub kid_friendly_text: String,
}

/// The complete notation lesson
#[derive(Debug, Clone)]
pub struct NotationLesson {
    /// Lesson steps
    pub steps: Vec<LessonStep>,
}

impl NotationLesson {
    /// Creates a new notation lesson with all steps
    pub fn new() -> Self {
        Self {
            steps: vec![
                Self::intro_step(),
                Self::face_names_step(),
                Self::r_face_step(),
                Self::l_face_step(),
                Self::u_face_step(),
                Self::d_face_step(),
                Self::f_face_step(),
                Self::b_face_step(),
                Self::prime_moves_step(),
                Self::double_moves_step(),
                Self::practice_step(),
            ],
        }
    }

    /// Introduction to notation
    fn intro_step() -> LessonStep {
        LessonStep {
            title: "Welcome to Cube Notation!".to_string(),
            description: "Every move on a Rubik's Cube has a special letter. Learning these letters helps you solve the cube faster!".to_string(),
            example_move: None,
            kid_friendly_text: "Think of it like learning the alphabet for your Rubik's Cube! Each face has its own letter name.".to_string(),
        }
    }

    /// Explain the six face names
    fn face_names_step() -> LessonStep {
        LessonStep {
            title: "The Six Faces".to_string(),
            description: "A Rubik's Cube has 6 faces. Each face is named by its position when you hold the cube: Right (R), Left (L), Up (U), Down (D), Front (F), and Back (B).".to_string(),
            example_move: None,
            kid_friendly_text: "Hold your cube in front of you. The face pointing at you is Front (F). The top is Up (U). Try to find all six faces!".to_string(),
        }
    }

    /// Explain R (Right) face
    fn r_face_step() -> LessonStep {
        LessonStep {
            title: "R - Right Face".to_string(),
            description: "R means turn the right face clockwise (like turning a doorknob to open a door).".to_string(),
            example_move: Some(Move::R),
            kid_friendly_text: "Imagine a clock on the right side. R means turn it the way clock hands go!".to_string(),
        }
    }

    /// Explain L (Left) face
    fn l_face_step() -> LessonStep {
        LessonStep {
            title: "L - Left Face".to_string(),
            description: "L means turn the left face clockwise when you're looking at it from the left side.".to_string(),
            example_move: Some(Move::L),
            kid_friendly_text: "Point the left face toward you. Now turn it clockwise, just like you did with R!".to_string(),
        }
    }

    /// Explain U (Up) face
    fn u_face_step() -> LessonStep {
        LessonStep {
            title: "U - Up Face".to_string(),
            description: "U means turn the top face clockwise (to the right) when you're looking down at it.".to_string(),
            example_move: Some(Move::U),
            kid_friendly_text: "Look down at the top of your cube. U means spin it to the right like a spinning top!".to_string(),
        }
    }

    /// Explain D (Down) face
    fn d_face_step() -> LessonStep {
        LessonStep {
            title: "D - Down Face".to_string(),
            description: "D means turn the bottom face clockwise when you're looking at it from below.".to_string(),
            example_move: Some(Move::D),
            kid_friendly_text: "Turn your cube upside down to see the bottom. Now turn it clockwise!".to_string(),
        }
    }

    /// Explain F (Front) face
    fn f_face_step() -> LessonStep {
        LessonStep {
            title: "F - Front Face".to_string(),
            description: "F means turn the front face clockwise. This is usually the easiest one to remember!".to_string(),
            example_move: Some(Move::F),
            kid_friendly_text: "Look at the face pointing at you. Turn it clockwise like you're turning a steering wheel to the right!".to_string(),
        }
    }

    /// Explain B (Back) face
    fn b_face_step() -> LessonStep {
        LessonStep {
            title: "B - Back Face".to_string(),
            description: "B means turn the back face clockwise when you're looking at it from behind.".to_string(),
            example_move: Some(Move::B),
            kid_friendly_text: "This one is tricky! Turn the cube around and look at the back. Now turn it clockwise.".to_string(),
        }
    }

    /// Explain prime (counter-clockwise) moves
    fn prime_moves_step() -> LessonStep {
        LessonStep {
            title: "Prime Moves (')".to_string(),
            description: "When you see a letter with an apostrophe (like R'), it means turn that face counter-clockwise (the opposite way).".to_string(),
            example_move: Some(Move::RPrime),
            kid_friendly_text: "The apostrophe is like a backwards arrow. R' means turn R backwards instead of forwards!".to_string(),
        }
    }

    /// Explain double (180-degree) moves
    fn double_moves_step() -> LessonStep {
        LessonStep {
            title: "Double Moves (2)".to_string(),
            description: "When you see a letter with a 2 (like R2), it means turn that face twice, or 180 degrees. You can turn it either way!".to_string(),
            example_move: Some(Move::R2),
            kid_friendly_text: "R2 is like doing R two times in a row. Turn it halfway around - it doesn't matter which way!".to_string(),
        }
    }

    /// Practice step
    fn practice_step() -> LessonStep {
        LessonStep {
            title: "Let's Practice!".to_string(),
            description: "Now try these moves on your cube: R U R' U'. This is a famous pattern called the 'Sexy Move'!".to_string(),
            example_move: None,
            kid_friendly_text: "Don't worry if you mess up! You can always reset your cube and try again. Practice makes perfect!".to_string(),
        }
    }

    /// Get the total number of steps
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Get a specific step by index
    pub fn get_step(&self, index: usize) -> Option<&LessonStep> {
        self.steps.get(index)
    }

    /// Get all steps
    pub fn get_all_steps(&self) -> &[LessonStep] {
        &self.steps
    }
}

impl Default for NotationLesson {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notation_lesson_creation() {
        let lesson = NotationLesson::new();
        assert!(lesson.step_count() > 0);
        assert_eq!(lesson.step_count(), 11);
    }

    #[test]
    fn test_lesson_steps_have_content() {
        let lesson = NotationLesson::new();
        for step in lesson.get_all_steps() {
            assert!(!step.title.is_empty());
            assert!(!step.description.is_empty());
            assert!(!step.kid_friendly_text.is_empty());
        }
    }

    #[test]
    fn test_get_specific_step() {
        let lesson = NotationLesson::new();
        let first_step = lesson.get_step(0);
        assert!(first_step.is_some());
        assert_eq!(first_step.unwrap().title, "Welcome to Cube Notation!");
    }

    #[test]
    fn test_r_face_has_example() {
        let lesson = NotationLesson::new();
        let r_step = lesson.get_step(2); // R face is step 2
        assert!(r_step.is_some());
        assert_eq!(r_step.unwrap().example_move, Some(Move::R));
    }

    #[test]
    fn test_prime_move_example() {
        let lesson = NotationLesson::new();
        let prime_step = lesson.get_step(8); // Prime moves step
        assert!(prime_step.is_some());
        assert_eq!(prime_step.unwrap().example_move, Some(Move::RPrime));
    }

    #[test]
    fn test_double_move_example() {
        let lesson = NotationLesson::new();
        let double_step = lesson.get_step(9); // Double moves step
        assert!(double_step.is_some());
        assert_eq!(double_step.unwrap().example_move, Some(Move::R2));
    }

    #[test]
    fn test_all_six_faces_covered() {
        let lesson = NotationLesson::new();
        let steps = lesson.get_all_steps();

        // Check that R, L, U, D, F, B are all mentioned
        let titles: Vec<String> = steps.iter().map(|s| s.title.clone()).collect();
        let all_titles = titles.join(" ");

        assert!(all_titles.contains("Right"));
        assert!(all_titles.contains("Left"));
        assert!(all_titles.contains("Up"));
        assert!(all_titles.contains("Down"));
        assert!(all_titles.contains("Front"));
        assert!(all_titles.contains("Back"));
    }

    #[test]
    fn test_kid_friendly_text_different_from_description() {
        let lesson = NotationLesson::new();
        for step in lesson.get_all_steps() {
            // Kid-friendly text should be different from description
            // (providing alternate explanation)
            assert_ne!(step.kid_friendly_text, step.description);
        }
    }
}
