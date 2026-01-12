//! Integration tests for R6.2: Face colors lesson
//!
//! Tests verify:
//! - Explain standard color scheme
//! - Show opposite colors
//! - Interactive quiz

use rubiks_cube_solver::tutorial::{ColorsLesson, ColorPair};
use rubiks_cube_solver::cube::Color;

#[test]
fn test_colors_lesson_006_standard_color_scheme() {
    // Test that the lesson explains the standard 6 colors
    let lesson = ColorsLesson::new();

    // Should have multiple steps
    assert!(lesson.step_count() >= 5, "Should have at least 5 steps");

    // First step should introduce colors
    let intro = lesson.get_step(0).unwrap();
    assert!(intro.title.contains("Color") || intro.title.contains("color"));

    // Should mention all 6 colors somewhere in the lesson
    let all_text: String = lesson.get_all_steps()
        .iter()
        .map(|s| format!("{} {} {}", s.title, s.description, s.kid_friendly_text))
        .collect::<Vec<_>>()
        .join(" ");

    assert!(all_text.contains("White") || all_text.contains("white"));
    assert!(all_text.contains("Yellow") || all_text.contains("yellow"));
    assert!(all_text.contains("Red") || all_text.contains("red"));
    assert!(all_text.contains("Orange") || all_text.contains("orange"));
    assert!(all_text.contains("Blue") || all_text.contains("blue"));
    assert!(all_text.contains("Green") || all_text.contains("green"));
}

#[test]
fn test_colors_lesson_007_opposite_pairs_explained() {
    // Test that opposite color pairs are explained
    let lesson = ColorsLesson::new();

    let mut white_yellow_found = false;
    let mut red_orange_found = false;
    let mut blue_green_found = false;

    for step in lesson.get_all_steps() {
        if let Some(ref pair) = step.color_pair {
            // Check for white-yellow pair
            if (pair.color1 == Color::White && pair.color2 == Color::Yellow) ||
               (pair.color1 == Color::Yellow && pair.color2 == Color::White) {
                white_yellow_found = true;
                assert!(!pair.explanation.is_empty(), "Pair should have explanation");
            }

            // Check for red-orange pair
            if (pair.color1 == Color::Red && pair.color2 == Color::Orange) ||
               (pair.color1 == Color::Orange && pair.color2 == Color::Red) {
                red_orange_found = true;
                assert!(!pair.explanation.is_empty(), "Pair should have explanation");
            }

            // Check for blue-green pair
            if (pair.color1 == Color::Blue && pair.color2 == Color::Green) ||
               (pair.color1 == Color::Green && pair.color2 == Color::Blue) {
                blue_green_found = true;
                assert!(!pair.explanation.is_empty(), "Pair should have explanation");
            }
        }
    }

    assert!(white_yellow_found, "White-Yellow pair should be explained");
    assert!(red_orange_found, "Red-Orange pair should be explained");
    assert!(blue_green_found, "Blue-Green pair should be explained");
}

#[test]
fn test_colors_lesson_008_interactive_quiz() {
    // Test that the lesson includes an interactive quiz
    let lesson = ColorsLesson::new();

    // Should have quiz questions
    assert!(lesson.quiz_count() > 0, "Should have quiz questions");
    assert!(lesson.quiz_count() >= 3, "Should have at least 3 quiz questions");

    // Quiz questions should have multiple choice options
    for (i, question) in lesson.get_all_quiz_questions().iter().enumerate() {
        assert!(!question.question.is_empty(), "Question {} should have text", i);
        assert!(!question.hint.is_empty(), "Question {} should have a hint", i);
        assert_eq!(question.choices.len(), 4, "Question {} should have 4 choices", i);

        // Correct answer should be in the choices
        assert!(
            question.choices.contains(&question.correct_answer),
            "Question {} correct answer should be in choices",
            i
        );
    }
}

#[test]
fn test_colors_lesson_009_white_yellow_opposite() {
    // Test that white-yellow opposite relationship is taught
    let lesson = ColorsLesson::new();

    // Should have at least one quiz question about white-yellow
    let quiz_questions = lesson.get_all_quiz_questions();
    let mut found_white_yellow_quiz = false;

    for question in quiz_questions {
        if (question.question.contains("White") || question.question.contains("white")) &&
           (question.correct_answer == Color::Yellow || question.correct_answer == Color::White) {
            found_white_yellow_quiz = true;
            break;
        }
        if (question.question.contains("Yellow") || question.question.contains("yellow")) &&
           (question.correct_answer == Color::Yellow || question.correct_answer == Color::White) {
            found_white_yellow_quiz = true;
            break;
        }
    }

    assert!(found_white_yellow_quiz, "Should have quiz about white-yellow relationship");
}

#[test]
fn test_colors_lesson_010_red_orange_opposite() {
    // Test that red-orange opposite relationship is taught
    let lesson = ColorsLesson::new();

    // Check in steps
    let mut found_in_steps = false;
    for step in lesson.get_all_steps() {
        let text = format!("{} {}", step.description, step.kid_friendly_text);
        if (text.contains("Red") || text.contains("red")) &&
           (text.contains("Orange") || text.contains("orange")) {
            if let Some(ref pair) = step.color_pair {
                if (pair.color1 == Color::Red && pair.color2 == Color::Orange) ||
                   (pair.color1 == Color::Orange && pair.color2 == Color::Red) {
                    found_in_steps = true;
                    break;
                }
            }
        }
    }

    assert!(found_in_steps, "Should explain red-orange opposite relationship");
}

#[test]
fn test_colors_lesson_011_blue_green_opposite() {
    // Test that blue-green opposite relationship is taught
    let lesson = ColorsLesson::new();

    // Check in steps
    let mut found_in_steps = false;
    for step in lesson.get_all_steps() {
        let text = format!("{} {}", step.description, step.kid_friendly_text);
        if (text.contains("Blue") || text.contains("blue")) &&
           (text.contains("Green") || text.contains("green")) {
            if let Some(ref pair) = step.color_pair {
                if (pair.color1 == Color::Blue && pair.color2 == Color::Green) ||
                   (pair.color1 == Color::Green && pair.color2 == Color::Blue) {
                    found_in_steps = true;
                    break;
                }
            }
        }
    }

    assert!(found_in_steps, "Should explain blue-green opposite relationship");
}

#[test]
fn test_colors_lesson_012_quiz_answer_checking() {
    // Test that quiz answer checking works correctly
    let lesson = ColorsLesson::new();

    for i in 0..lesson.quiz_count() {
        let question = lesson.get_quiz_question(i).unwrap();

        // Correct answer should pass
        assert!(
            lesson.check_answer(i, question.correct_answer),
            "Question {} should accept correct answer",
            i
        );

        // Find an incorrect answer
        for &choice in &question.choices {
            if choice != question.correct_answer {
                // Wrong answer should fail
                assert!(
                    !lesson.check_answer(i, choice),
                    "Question {} should reject incorrect answer",
                    i
                );
                break;
            }
        }
    }
}

#[test]
fn test_colors_lesson_013_kid_friendly_explanations() {
    // Test that all steps have kid-friendly explanations
    let lesson = ColorsLesson::new();

    for (i, step) in lesson.get_all_steps().iter().enumerate() {
        assert!(!step.kid_friendly_text.is_empty(), "Step {} should have kid-friendly text", i);

        // Kid-friendly text should be different from description
        assert_ne!(
            step.kid_friendly_text,
            step.description,
            "Step {} kid-friendly text should differ from description",
            i
        );

        // Kid-friendly text should be engaging (use words like "like", "think", exclamation marks)
        let text = step.kid_friendly_text.to_lowercase();
        let is_engaging = text.contains("like") ||
                         text.contains("think") ||
                         text.contains("!") ||
                         text.contains("you");

        assert!(is_engaging, "Step {} kid-friendly text should be engaging", i);
    }
}

#[test]
fn test_colors_lesson_014_consistent_with_color_enum() {
    // Test that color pairs match the Color::opposite() method
    let lesson = ColorsLesson::new();

    for step in lesson.get_all_steps() {
        if let Some(ref pair) = step.color_pair {
            // Check that the pair matches the opposite() method
            assert_eq!(
                pair.color1.opposite(),
                pair.color2,
                "Color pair {:?}-{:?} should match opposite() method",
                pair.color1,
                pair.color2
            );
        }
    }
}

#[test]
fn test_colors_lesson_015_step_progression() {
    // Test that steps progress logically
    let lesson = ColorsLesson::new();

    // First step should be introduction
    let first = lesson.get_step(0).unwrap();
    assert!(
        first.title.contains("Welcome") ||
        first.title.contains("Learning") ||
        first.title.contains("Introduction") ||
        first.title.contains("Cube Color"),
        "First step should be introductory"
    );

    // Should have steps covering all three pairs
    assert!(lesson.step_count() >= 5, "Should have enough steps to cover all pairs");

    // Last step should mention practice or quiz
    let last = lesson.get_step(lesson.step_count() - 1).unwrap();
    let last_text = format!("{} {}", last.title, last.description).to_lowercase();
    assert!(
        last_text.contains("practice") ||
        last_text.contains("quiz") ||
        last_text.contains("test"),
        "Last step should introduce practice/quiz"
    );
}

#[test]
fn test_colors_lesson_016_all_quiz_hints() {
    // Test that all quiz questions have helpful hints
    let lesson = ColorsLesson::new();

    for (i, question) in lesson.get_all_quiz_questions().iter().enumerate() {
        assert!(!question.hint.is_empty(), "Question {} should have a hint", i);

        // Hint should mention something relevant
        let hint_lower = question.hint.to_lowercase();

        // Hint should be different from question
        assert_ne!(
            question.hint,
            question.question,
            "Question {} hint should differ from question",
            i
        );

        // Hint should be encouraging or helpful
        let is_helpful = hint_lower.contains("remember") ||
                        hint_lower.contains("think") ||
                        hint_lower.contains("like") ||
                        hint_lower.contains("color") ||
                        hint_lower.contains("warm") ||
                        hint_lower.contains("cool") ||
                        hint_lower.contains("light");

        assert!(is_helpful, "Question {} hint should be helpful", i);
    }
}

#[test]
fn test_colors_lesson_017_color_pair_struct() {
    // Test ColorPair struct functionality
    let pair = ColorPair {
        color1: Color::White,
        color2: Color::Yellow,
        explanation: "Light colors".to_string(),
    };

    assert_eq!(pair.color1, Color::White);
    assert_eq!(pair.color2, Color::Yellow);
    assert_eq!(pair.explanation, "Light colors");

    // Test clone
    let pair2 = pair.clone();
    assert_eq!(pair, pair2);
}

#[test]
fn test_colors_lesson_018_lesson_completeness() {
    // Test that the lesson is complete and comprehensive
    let lesson = ColorsLesson::new();

    // Should have reasonable number of steps (not too few, not too many)
    assert!(lesson.step_count() >= 5, "Should have at least 5 steps");
    assert!(lesson.step_count() <= 12, "Should have at most 12 steps");

    // Should have reasonable number of quiz questions
    assert!(lesson.quiz_count() >= 3, "Should have at least 3 quiz questions");
    assert!(lesson.quiz_count() <= 10, "Should have at most 10 quiz questions");

    // All steps should have content
    for step in lesson.get_all_steps() {
        assert!(!step.title.is_empty());
        assert!(!step.description.is_empty());
        assert!(!step.kid_friendly_text.is_empty());
    }

    // All quiz questions should have content
    for question in lesson.get_all_quiz_questions() {
        assert!(!question.question.is_empty());
        assert!(!question.hint.is_empty());
        assert!(!question.choices.is_empty());
    }
}
