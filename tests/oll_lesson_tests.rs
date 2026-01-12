//! Integration tests for OLL (Orient Last Layer) lesson
//!
//! Tests for R6.6 acceptance criteria:
//! - 2-look OLL algorithms
//! - Pattern recognition
//! - Practice mode

use rubiks_cube_solver::tutorial::lessons::{
    OllLesson, OllPattern
};
use rubiks_cube_solver::cube::{Cube, Move};

#[test]
fn test_oll_lesson_001_creation() {
    let lesson = OllLesson::new();
    assert!(!lesson.get_steps().is_empty(), "Lesson should have steps");
    assert!(!lesson.get_cases().is_empty(), "Lesson should have OLL cases");
    assert!(!lesson.get_practice_exercises().is_empty(), "Lesson should have practice exercises");
}

#[test]
fn test_oll_lesson_002_has_intro_step() {
    let lesson = OllLesson::new();
    let steps = lesson.get_steps();

    assert!(steps[0].title.contains("Welcome") || steps[0].title.contains("OLL"));
    assert!(!steps[0].description.is_empty());
    assert!(!steps[0].kid_friendly_text.is_empty());
}

#[test]
fn test_oll_lesson_003_has_all_required_steps() {
    let lesson = OllLesson::new();
    let steps = lesson.get_steps();

    // Should have at least: intro, what is OLL, 2-look explanation, edge orientation, corner orientation, pattern recognition, practice
    assert!(steps.len() >= 7, "Should have at least 7 lesson steps");

    let step_titles: Vec<String> = steps.iter().map(|s| s.title.to_lowercase()).collect();
    let titles_str = step_titles.join(" ");

    assert!(titles_str.contains("oll"), "Should explain what OLL is");
    assert!(titles_str.contains("2-look") || titles_str.contains("two"), "Should explain 2-look method");
    assert!(titles_str.contains("edge") || titles_str.contains("cross"), "Should cover edge orientation");
    assert!(titles_str.contains("corner"), "Should cover corner orientation");
    assert!(titles_str.contains("pattern") || titles_str.contains("recognition"), "Should cover pattern recognition");
    assert!(titles_str.contains("practice"), "Should have practice section");
}

#[test]
fn test_oll_lesson_004_2look_oll_cases() {
    let lesson = OllLesson::new();
    let cases = lesson.get_cases();

    // 2-look OLL needs edge orientation (4 cases) + corner orientation (7 common cases)
    assert!(cases.len() >= 10, "Should have at least 10 OLL cases for 2-look method");

    // Verify we have edge orientation cases (dot, L, line)
    let edge_patterns = [OllPattern::Dot, OllPattern::LShape, OllPattern::Line];
    for pattern in &edge_patterns {
        assert!(
            cases.iter().any(|c| c.pattern == *pattern),
            "Should have {:?} pattern for edge orientation",
            pattern
        );
    }

    // Verify we have key corner orientation cases (Sune, AntiSune)
    let corner_patterns = [OllPattern::Sune, OllPattern::AntiSune];
    for pattern in &corner_patterns {
        assert!(
            cases.iter().any(|c| c.pattern == *pattern),
            "Should have {:?} pattern for corner orientation",
            pattern
        );
    }
}

#[test]
fn test_oll_lesson_005_sune_algorithm() {
    let lesson = OllLesson::new();
    let sune = lesson.get_case_by_pattern(OllPattern::Sune)
        .expect("Should have Sune case");

    assert_eq!(sune.name, "Sune");
    assert!(!sune.description.is_empty());
    assert!(!sune.algorithm.is_empty());
    assert!(!sune.explanation.is_empty());
    assert!(!sune.visual_hint.is_empty());

    // Sune algorithm: R U R' U R U2 R'
    use Move::*;
    let expected = vec![R, U, RPrime, U, R, U2, RPrime];
    assert_eq!(sune.algorithm, expected, "Sune algorithm should be R U R' U R U2 R'");
}

#[test]
fn test_oll_lesson_006_antisune_algorithm() {
    let lesson = OllLesson::new();
    let antisune = lesson.get_case_by_pattern(OllPattern::AntiSune)
        .expect("Should have AntiSune case");

    assert_eq!(antisune.name, "AntiSune");

    // AntiSune: R' U' R U' R' U2 R
    use Move::*;
    let expected = vec![RPrime, UPrime, R, UPrime, RPrime, U2, R];
    assert_eq!(antisune.algorithm, expected, "AntiSune should be R' U' R U' R' U2 R");
}

#[test]
fn test_oll_lesson_007_edge_orientation_algorithms() {
    let lesson = OllLesson::new();

    // Test dot to line/L
    let dot_cases: Vec<_> = lesson.get_cases().iter()
        .filter(|c| c.pattern == OllPattern::Dot)
        .collect();
    assert!(!dot_cases.is_empty(), "Should have dot pattern case(s)");

    // Test L to cross
    let l_cases: Vec<_> = lesson.get_cases().iter()
        .filter(|c| c.pattern == OllPattern::LShape)
        .collect();
    assert!(!l_cases.is_empty(), "Should have L-shape case(s)");

    // Test line to cross
    let line_cases: Vec<_> = lesson.get_cases().iter()
        .filter(|c| c.pattern == OllPattern::Line)
        .collect();
    assert!(!line_cases.is_empty(), "Should have line case(s)");
}

#[test]
fn test_oll_lesson_008_all_cases_have_algorithms() {
    let lesson = OllLesson::new();

    for case in lesson.get_cases() {
        assert!(!case.name.is_empty(), "Case should have a name");
        assert!(!case.description.is_empty(), "Case should have description");
        assert!(!case.algorithm.is_empty(), "Case should have algorithm");
        assert!(!case.explanation.is_empty(), "Case should have explanation");
        assert!(!case.visual_hint.is_empty(), "Case should have visual hint");
    }
}

#[test]
fn test_oll_lesson_009_kid_friendly_explanations() {
    let lesson = OllLesson::new();

    for step in lesson.get_steps() {
        assert!(!step.kid_friendly_text.is_empty(), "Step '{}' should have kid-friendly text", step.title);

        // Kid-friendly text should avoid technical jargon
        let text = step.kid_friendly_text.to_lowercase();
        // Should use simple, encouraging language
        assert!(
            text.len() > 20,
            "Kid-friendly text for '{}' should be substantial",
            step.title
        );
    }

    for case in lesson.get_cases() {
        assert!(!case.explanation.is_empty(), "Case '{}' should have explanation", case.name);
    }
}

#[test]
fn test_oll_lesson_010_practice_exercises_exist() {
    let lesson = OllLesson::new();
    let exercises = lesson.get_practice_exercises();

    assert!(exercises.len() >= 5, "Should have at least 5 practice exercises");

    // Should have exercises for key patterns
    let patterns_covered: Vec<OllPattern> = exercises.iter().map(|e| e.pattern).collect();
    assert!(patterns_covered.contains(&OllPattern::Dot), "Should have dot pattern practice");
    assert!(patterns_covered.contains(&OllPattern::Sune), "Should have Sune practice");
}

#[test]
fn test_oll_lesson_011_practice_exercises_complete() {
    let lesson = OllLesson::new();

    for exercise in lesson.get_practice_exercises() {
        assert!(!exercise.title.is_empty(), "Exercise should have title");
        assert!(!exercise.description.is_empty(), "Exercise should have description");
        assert!(!exercise.setup_moves.is_empty(), "Exercise should have setup moves");
        assert!(!exercise.solution.is_empty(), "Exercise should have solution");
        assert!(!exercise.hint.is_empty(), "Exercise should have hint");
    }
}

#[test]
fn test_oll_lesson_012_practice_dot_pattern() {
    let lesson = OllLesson::new();
    let exercises = lesson.get_practice_exercises();

    let dot_exercise = exercises.iter()
        .find(|e| e.pattern == OllPattern::Dot)
        .expect("Should have dot pattern exercise");

    // Verify exercise has setup and solution
    assert!(!dot_exercise.setup_moves.is_empty());
    assert!(!dot_exercise.solution.is_empty());
    assert!(dot_exercise.hint.to_lowercase().contains("dot") ||
            dot_exercise.hint.to_lowercase().contains("f r"));
}

#[test]
fn test_oll_lesson_013_practice_sune() {
    let lesson = OllLesson::new();
    let exercises = lesson.get_practice_exercises();

    let sune_exercise = exercises.iter()
        .find(|e| e.pattern == OllPattern::Sune)
        .expect("Should have Sune exercise");

    assert!(sune_exercise.title.to_lowercase().contains("sune"));
    assert!(sune_exercise.hint.to_lowercase().contains("sune") ||
            sune_exercise.hint.to_lowercase().contains("r u r"));
}

#[test]
fn test_oll_lesson_014_pattern_recognition_step() {
    let lesson = OllLesson::new();
    let steps = lesson.get_steps();

    // Should have a step about pattern recognition
    let has_pattern_step = steps.iter().any(|s| {
        s.title.to_lowercase().contains("pattern") ||
        s.title.to_lowercase().contains("recognition")
    });

    assert!(has_pattern_step, "Should have pattern recognition step");
}

#[test]
fn test_oll_lesson_015_visual_hints() {
    let lesson = OllLesson::new();

    for case in lesson.get_cases() {
        assert!(!case.visual_hint.is_empty(), "Case '{}' should have visual hint", case.name);

        // Visual hints should describe what to look for
        let hint = case.visual_hint.to_lowercase();
        assert!(hint.len() > 10, "Visual hint for '{}' should be descriptive", case.name);
    }
}

#[test]
fn test_oll_lesson_016_algorithms_are_valid() {
    let lesson = OllLesson::new();

    // Test that algorithms can be applied to a cube without errors
    for case in lesson.get_cases() {
        let mut cube = Cube::new(3);

        // Apply the algorithm
        for mv in &case.algorithm {
            cube.apply_move(*mv);
        }

        // If we got here, the algorithm is valid (no panics)
        assert_eq!(cube.size(), 3);
    }
}

#[test]
fn test_oll_lesson_017_practice_setup_creates_valid_state() {
    let lesson = OllLesson::new();

    // Test that practice setup moves create valid cube states
    for exercise in lesson.get_practice_exercises() {
        let mut cube = Cube::new(3);

        // Apply setup moves
        for mv in &exercise.setup_moves {
            cube.apply_move(*mv);
        }

        // Cube should still be valid
        assert!(cube.validate().is_ok(),
                "Practice setup for '{}' should create valid cube state",
                exercise.title);
    }
}

#[test]
fn test_oll_lesson_018_corner_orientation_cases() {
    let lesson = OllLesson::new();
    let cases = lesson.get_cases();

    // Should have multiple corner orientation cases beyond just Sune/AntiSune
    let corner_cases: Vec<_> = cases.iter()
        .filter(|c| matches!(c.pattern,
            OllPattern::Sune | OllPattern::AntiSune | OllPattern::H |
            OllPattern::Pi | OllPattern::T | OllPattern::LPattern | OllPattern::U
        ))
        .collect();

    assert!(corner_cases.len() >= 5, "Should have at least 5 corner orientation cases");
}

#[test]
fn test_oll_lesson_019_all_patterns_represented() {
    let lesson = OllLesson::new();
    let cases = lesson.get_cases();

    // Verify key patterns from 2-look OLL are present
    let patterns = vec![
        OllPattern::Dot,
        OllPattern::Line,
        OllPattern::LShape,
        OllPattern::Sune,
        OllPattern::AntiSune,
    ];

    for pattern in patterns {
        assert!(
            cases.iter().any(|c| c.pattern == pattern),
            "Should have case for {:?} pattern",
            pattern
        );
    }
}

#[test]
fn test_oll_lesson_020_default_implementation() {
    let lesson1 = OllLesson::new();
    let lesson2 = OllLesson::default();

    assert_eq!(lesson1.get_steps().len(), lesson2.get_steps().len());
    assert_eq!(lesson1.get_cases().len(), lesson2.get_cases().len());
    assert_eq!(lesson1.get_practice_exercises().len(), lesson2.get_practice_exercises().len());
}

#[test]
fn test_oll_lesson_021_case_lookup_by_pattern() {
    let lesson = OllLesson::new();

    // Test get_case_by_pattern works for all patterns we have
    let sune = lesson.get_case_by_pattern(OllPattern::Sune);
    assert!(sune.is_some());
    assert_eq!(sune.unwrap().pattern, OllPattern::Sune);

    let antisune = lesson.get_case_by_pattern(OllPattern::AntiSune);
    assert!(antisune.is_some());
    assert_eq!(antisune.unwrap().pattern, OllPattern::AntiSune);
}

#[test]
fn test_oll_lesson_022_two_look_method_coverage() {
    let lesson = OllLesson::new();
    let steps = lesson.get_steps();

    // Should explicitly mention 2-look method
    let mentions_2look = steps.iter().any(|s| {
        s.title.to_lowercase().contains("2-look") ||
        s.title.to_lowercase().contains("two-look") ||
        s.description.to_lowercase().contains("2-look") ||
        s.description.to_lowercase().contains("two-look")
    });

    assert!(mentions_2look, "Should explain the 2-look OLL method");
}

#[test]
fn test_oll_lesson_023_step_progression() {
    let lesson = OllLesson::new();
    let steps = lesson.get_steps();

    // Steps should progress logically
    // First should be intro/welcome
    assert!(steps[0].title.to_lowercase().contains("welcome") ||
            steps[0].title.to_lowercase().contains("oll"));

    // Last should be practice
    assert!(steps[steps.len() - 1].title.to_lowercase().contains("practice"));
}

#[test]
fn test_oll_lesson_024_tips_provided() {
    let lesson = OllLesson::new();
    let steps = lesson.get_steps();

    // At least some steps should have tips
    let steps_with_tips = steps.iter().filter(|s| s.tip.is_some()).count();
    assert!(steps_with_tips >= 3, "Should have tips in multiple steps");
}

#[test]
fn test_oll_lesson_025_algorithms_have_moves() {
    let lesson = OllLesson::new();

    for case in lesson.get_cases() {
        assert!(
            case.algorithm.len() >= 3,
            "Case '{}' should have at least 3 moves",
            case.name
        );
        assert!(
            case.algorithm.len() <= 20,
            "Case '{}' algorithm should be reasonable length for 2-look OLL",
            case.name
        );
    }
}
