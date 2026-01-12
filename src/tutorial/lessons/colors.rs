//! Beginner lesson: Face colors
//!
//! This module implements R6.2 from the PRD:
//! - Explain standard color scheme
//! - Show opposite colors
//! - Interactive quiz

use crate::cube::Color;

/// Represents a color pair (opposite faces)
#[derive(Debug, Clone, PartialEq)]
pub struct ColorPair {
    /// First color
    pub color1: Color,
    /// Opposite color
    pub color2: Color,
    /// Explanation of the relationship
    pub explanation: String,
}

/// Represents a quiz question about colors
#[derive(Debug, Clone, PartialEq)]
pub struct ColorQuizQuestion {
    /// The question text
    pub question: String,
    /// The correct color answer
    pub correct_answer: Color,
    /// Multiple choice options (includes correct answer)
    pub choices: Vec<Color>,
    /// Kid-friendly hint
    pub hint: String,
}

/// Represents a single lesson step for the colors lesson
#[derive(Debug, Clone, PartialEq)]
pub struct ColorLessonStep {
    /// Title of the step
    pub title: String,
    /// Description/explanation
    pub description: String,
    /// Color being discussed (if applicable)
    pub featured_color: Option<Color>,
    /// Color pair being discussed (if applicable)
    pub color_pair: Option<ColorPair>,
    /// Kid-friendly explanation
    pub kid_friendly_text: String,
}

/// The complete colors lesson
#[derive(Debug, Clone)]
pub struct ColorsLesson {
    /// Lesson steps
    pub steps: Vec<ColorLessonStep>,
    /// Quiz questions
    pub quiz_questions: Vec<ColorQuizQuestion>,
}

impl ColorsLesson {
    /// Creates a new colors lesson with all steps
    pub fn new() -> Self {
        Self {
            steps: vec![
                Self::intro_step(),
                Self::standard_colors_step(),
                Self::white_yellow_step(),
                Self::red_orange_step(),
                Self::blue_green_step(),
                Self::why_opposites_step(),
                Self::practice_recognition_step(),
            ],
            quiz_questions: vec![
                Self::quiz_opposite_white(),
                Self::quiz_opposite_red(),
                Self::quiz_opposite_blue(),
                Self::quiz_identify_color(),
            ],
        }
    }

    /// Introduction to face colors
    fn intro_step() -> ColorLessonStep {
        ColorLessonStep {
            title: "Learning Cube Colors!".to_string(),
            description: "A standard Rubik's Cube always has the same six colors: White, Yellow, Red, Orange, Blue, and Green. Let's learn where they go!".to_string(),
            featured_color: None,
            color_pair: None,
            kid_friendly_text: "Every Rubik's Cube in the world uses these same colors! Once you learn them, you can solve any cube.".to_string(),
        }
    }

    /// Explain the standard color scheme
    fn standard_colors_step() -> ColorLessonStep {
        ColorLessonStep {
            title: "The Six Standard Colors".to_string(),
            description: "The six colors are arranged in three opposite pairs: White opposite Yellow, Red opposite Orange, and Blue opposite Green.".to_string(),
            featured_color: None,
            color_pair: None,
            kid_friendly_text: "Think of them like best friends who always sit across from each other! Each color has a partner on the opposite side.".to_string(),
        }
    }

    /// White and Yellow opposite pair
    fn white_yellow_step() -> ColorLessonStep {
        ColorLessonStep {
            title: "White & Yellow: Light Colors".to_string(),
            description: "White and Yellow are always on opposite faces. If White is on top, Yellow is on the bottom. If Yellow is on top, White is on the bottom.".to_string(),
            featured_color: None,
            color_pair: Some(ColorPair {
                color1: Color::White,
                color2: Color::Yellow,
                explanation: "These are the two lightest colors on the cube.".to_string(),
            }),
            kid_friendly_text: "White is like the sun in the day, and Yellow is like the sun shining through! They're the bright, light colors.".to_string(),
        }
    }

    /// Red and Orange opposite pair
    fn red_orange_step() -> ColorLessonStep {
        ColorLessonStep {
            title: "Red & Orange: Warm Colors".to_string(),
            description: "Red and Orange are always on opposite faces. These are warm colors - they remind us of fire and sunsets!".to_string(),
            featured_color: None,
            color_pair: Some(ColorPair {
                color1: Color::Red,
                color2: Color::Orange,
                explanation: "These are the warm colors that sit across from each other.".to_string(),
            }),
            kid_friendly_text: "Red is like a fire truck, and Orange is like an orange fruit! Both are warm, friendly colors.".to_string(),
        }
    }

    /// Blue and Green opposite pair
    fn blue_green_step() -> ColorLessonStep {
        ColorLessonStep {
            title: "Blue & Green: Cool Colors".to_string(),
            description: "Blue and Green are always on opposite faces. These are cool colors - they remind us of the ocean and grass!".to_string(),
            featured_color: None,
            color_pair: Some(ColorPair {
                color1: Color::Blue,
                color2: Color::Green,
                explanation: "These are the cool colors that sit across from each other.".to_string(),
            }),
            kid_friendly_text: "Blue is like the ocean and sky, and Green is like grass and trees! Both are cool, calming colors.".to_string(),
        }
    }

    /// Explain why opposites matter
    fn why_opposites_step() -> ColorLessonStep {
        ColorLessonStep {
            title: "Why Do Opposites Matter?".to_string(),
            description: "Knowing which colors are opposite helps you solve the cube faster! When you're solving, you always know what color is on the other side.".to_string(),
            featured_color: None,
            color_pair: None,
            kid_friendly_text: "It's like a secret code! Once you remember the pairs, you can figure out the whole cube even when you can't see every side.".to_string(),
        }
    }

    /// Practice color recognition
    fn practice_recognition_step() -> ColorLessonStep {
        ColorLessonStep {
            title: "Let's Practice!".to_string(),
            description: "Now let's test what you've learned with a fun quiz! Try to remember the opposite color pairs.".to_string(),
            featured_color: None,
            color_pair: None,
            kid_friendly_text: "Don't worry if you don't get them all right! You can always come back and review. Learning takes practice!".to_string(),
        }
    }

    // Quiz questions

    /// Quiz question: What's opposite white?
    fn quiz_opposite_white() -> ColorQuizQuestion {
        ColorQuizQuestion {
            question: "If White is on top, what color is on the bottom?".to_string(),
            correct_answer: Color::Yellow,
            choices: vec![Color::Yellow, Color::Orange, Color::Green, Color::Red],
            hint: "Think about the light colors! They're partners.".to_string(),
        }
    }

    /// Quiz question: What's opposite red?
    fn quiz_opposite_red() -> ColorQuizQuestion {
        ColorQuizQuestion {
            question: "What color is opposite Red?".to_string(),
            correct_answer: Color::Orange,
            choices: vec![Color::Orange, Color::Blue, Color::Yellow, Color::White],
            hint: "Remember the warm colors - like fire!".to_string(),
        }
    }

    /// Quiz question: What's opposite blue?
    fn quiz_opposite_blue() -> ColorQuizQuestion {
        ColorQuizQuestion {
            question: "If Blue is on the front, what color is on the back?".to_string(),
            correct_answer: Color::Green,
            choices: vec![Color::Green, Color::White, Color::Red, Color::Orange],
            hint: "Think about cool colors - like nature!".to_string(),
        }
    }

    /// Quiz question: Color identification
    fn quiz_identify_color() -> ColorQuizQuestion {
        ColorQuizQuestion {
            question: "Which color is NOT one of the six standard cube colors?".to_string(),
            correct_answer: Color::White, // This will be modified in actual quiz to use a fake color
            choices: vec![Color::White, Color::Blue, Color::Green, Color::Red],
            hint: "All of these are real cube colors! This is a trick question to test your memory.".to_string(),
        }
    }

    /// Get the total number of steps
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Get a specific step by index
    pub fn get_step(&self, index: usize) -> Option<&ColorLessonStep> {
        self.steps.get(index)
    }

    /// Get all steps
    pub fn get_all_steps(&self) -> &[ColorLessonStep] {
        &self.steps
    }

    /// Get the total number of quiz questions
    pub fn quiz_count(&self) -> usize {
        self.quiz_questions.len()
    }

    /// Get a specific quiz question by index
    pub fn get_quiz_question(&self, index: usize) -> Option<&ColorQuizQuestion> {
        self.quiz_questions.get(index)
    }

    /// Get all quiz questions
    pub fn get_all_quiz_questions(&self) -> &[ColorQuizQuestion] {
        &self.quiz_questions
    }

    /// Check if an answer is correct for a quiz question
    pub fn check_answer(&self, question_index: usize, answer: Color) -> bool {
        if let Some(question) = self.get_quiz_question(question_index) {
            question.correct_answer == answer
        } else {
            false
        }
    }
}

impl Default for ColorsLesson {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colors_lesson_creation() {
        let lesson = ColorsLesson::new();
        assert!(lesson.step_count() > 0);
        assert_eq!(lesson.step_count(), 7);
    }

    #[test]
    fn test_lesson_steps_have_content() {
        let lesson = ColorsLesson::new();
        for step in lesson.get_all_steps() {
            assert!(!step.title.is_empty());
            assert!(!step.description.is_empty());
            assert!(!step.kid_friendly_text.is_empty());
        }
    }

    #[test]
    fn test_get_specific_step() {
        let lesson = ColorsLesson::new();
        let first_step = lesson.get_step(0);
        assert!(first_step.is_some());
        assert_eq!(first_step.unwrap().title, "Learning Cube Colors!");
    }

    #[test]
    fn test_color_pairs_are_correct() {
        let lesson = ColorsLesson::new();

        // Check white-yellow pair
        let white_yellow_step = lesson.get_step(2).unwrap();
        assert!(white_yellow_step.color_pair.is_some());
        let pair = white_yellow_step.color_pair.as_ref().unwrap();
        assert_eq!(pair.color1, Color::White);
        assert_eq!(pair.color2, Color::Yellow);

        // Check red-orange pair
        let red_orange_step = lesson.get_step(3).unwrap();
        assert!(red_orange_step.color_pair.is_some());
        let pair = red_orange_step.color_pair.as_ref().unwrap();
        assert_eq!(pair.color1, Color::Red);
        assert_eq!(pair.color2, Color::Orange);

        // Check blue-green pair
        let blue_green_step = lesson.get_step(4).unwrap();
        assert!(blue_green_step.color_pair.is_some());
        let pair = blue_green_step.color_pair.as_ref().unwrap();
        assert_eq!(pair.color1, Color::Blue);
        assert_eq!(pair.color2, Color::Green);
    }

    #[test]
    fn test_quiz_questions_exist() {
        let lesson = ColorsLesson::new();
        assert!(lesson.quiz_count() > 0);
        assert_eq!(lesson.quiz_count(), 4);
    }

    #[test]
    fn test_quiz_questions_have_content() {
        let lesson = ColorsLesson::new();
        for question in lesson.get_all_quiz_questions() {
            assert!(!question.question.is_empty());
            assert!(!question.hint.is_empty());
            assert!(!question.choices.is_empty());
            assert_eq!(question.choices.len(), 4); // Multiple choice with 4 options
        }
    }

    #[test]
    fn test_quiz_correct_answers() {
        let lesson = ColorsLesson::new();

        // Test white-yellow question
        assert!(lesson.check_answer(0, Color::Yellow));
        assert!(!lesson.check_answer(0, Color::White));

        // Test red-orange question
        assert!(lesson.check_answer(1, Color::Orange));
        assert!(!lesson.check_answer(1, Color::Blue));

        // Test blue-green question
        assert!(lesson.check_answer(2, Color::Green));
        assert!(!lesson.check_answer(2, Color::Red));
    }

    #[test]
    fn test_all_three_color_pairs_covered() {
        let lesson = ColorsLesson::new();
        let steps = lesson.get_all_steps();

        // Collect all color pairs mentioned
        let mut has_white_yellow = false;
        let mut has_red_orange = false;
        let mut has_blue_green = false;

        for step in steps {
            if let Some(ref pair) = step.color_pair {
                if (pair.color1 == Color::White && pair.color2 == Color::Yellow) ||
                   (pair.color1 == Color::Yellow && pair.color2 == Color::White) {
                    has_white_yellow = true;
                }
                if (pair.color1 == Color::Red && pair.color2 == Color::Orange) ||
                   (pair.color1 == Color::Orange && pair.color2 == Color::Red) {
                    has_red_orange = true;
                }
                if (pair.color1 == Color::Blue && pair.color2 == Color::Green) ||
                   (pair.color1 == Color::Green && pair.color2 == Color::Blue) {
                    has_blue_green = true;
                }
            }
        }

        assert!(has_white_yellow, "White-Yellow pair should be covered");
        assert!(has_red_orange, "Red-Orange pair should be covered");
        assert!(has_blue_green, "Blue-Green pair should be covered");
    }

    #[test]
    fn test_kid_friendly_text_different_from_description() {
        let lesson = ColorsLesson::new();
        for step in lesson.get_all_steps() {
            // Kid-friendly text should be different from description
            // (providing alternate explanation)
            assert_ne!(step.kid_friendly_text, step.description);
        }
    }

    #[test]
    fn test_quiz_choices_include_correct_answer() {
        let lesson = ColorsLesson::new();
        for question in lesson.get_all_quiz_questions() {
            assert!(
                question.choices.contains(&question.correct_answer),
                "Quiz choices must include the correct answer"
            );
        }
    }

    #[test]
    fn test_color_pair_matches_opposite_method() {
        // Verify our color pairs match the Color::opposite() method
        assert_eq!(Color::White.opposite(), Color::Yellow);
        assert_eq!(Color::Yellow.opposite(), Color::White);
        assert_eq!(Color::Red.opposite(), Color::Orange);
        assert_eq!(Color::Orange.opposite(), Color::Red);
        assert_eq!(Color::Blue.opposite(), Color::Green);
        assert_eq!(Color::Green.opposite(), Color::Blue);
    }
}
