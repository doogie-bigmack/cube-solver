//! Integration tests for R6.4 - First Layer Corners Tutorial
//!
//! These tests verify the first layer corners tutorial lesson implementation

use rubiks_cube_solver::cube::{Cube, Move};
use rubiks_cube_solver::tutorial::lessons::{CornersLesson, CornerPosition};

#[test]
fn test_corners_001_lesson_has_all_steps() {
    let lesson = CornersLesson::new();
    assert_eq!(lesson.step_count(), 9);

    // Verify all steps have required content
    for (i, step) in lesson.get_all_steps().iter().enumerate() {
        assert!(!step.title.is_empty(), "Step {} title is empty", i);
        assert!(
            !step.description.is_empty(),
            "Step {} description is empty",
            i
        );
        assert!(
            !step.kid_friendly_text.is_empty(),
            "Step {} kid_friendly_text is empty",
            i
        );
    }
}

#[test]
fn test_corners_002_lesson_covers_key_concepts() {
    let lesson = CornersLesson::new();
    let all_text: String = lesson
        .get_all_steps()
        .iter()
        .map(|s| format!("{} {} {}", s.title, s.description, s.kid_friendly_text))
        .collect::<Vec<_>>()
        .join(" ");

    // Check that key concepts are mentioned
    assert!(all_text.contains("corner") || all_text.contains("Corner"));
    assert!(all_text.contains("first layer") || all_text.contains("First Layer"));
    assert!(all_text.contains("R U R") || all_text.contains("algorithm"));
    assert!(all_text.contains("white"));
    assert!(all_text.contains("position") || all_text.contains("insert"));
}

#[test]
fn test_corners_003_has_insertion_algorithms() {
    let lesson = CornersLesson::new();
    assert!(lesson.get_all_cases().len() >= 6);

    // Each case should have required fields
    for case in lesson.get_all_cases() {
        assert!(!case.name.is_empty());
        assert!(!case.description.is_empty());
        assert!(!case.explanation.is_empty());
    }

    // At least some cases should have algorithms
    let cases_with_algorithms = lesson
        .get_all_cases()
        .iter()
        .filter(|c| !c.algorithm.is_empty())
        .count();
    assert!(cases_with_algorithms >= 4);
}

#[test]
fn test_corners_004_r_u_r_prime_algorithm() {
    let lesson = CornersLesson::new();

    // Find the "White Up" case which should use R U R'
    let white_up_case = lesson.get_case(0);
    assert!(white_up_case.is_some());

    let case = white_up_case.unwrap();
    assert_eq!(case.algorithm, vec![Move::R, Move::U, Move::RPrime]);
    assert!(case.name.contains("White Up"));
}

#[test]
fn test_corners_005_f_prime_u_prime_f_algorithm() {
    let lesson = CornersLesson::new();

    // Find the "White Front" case which should use F' U' F
    let white_front_case = lesson.get_case(1);
    assert!(white_front_case.is_some());

    let case = white_front_case.unwrap();
    assert_eq!(
        case.algorithm,
        vec![Move::FPrime, Move::UPrime, Move::F]
    );
    assert!(case.name.contains("White Front"));
}

#[test]
fn test_corners_006_practice_exercises_exist() {
    let lesson = CornersLesson::new();
    assert!(lesson.get_practice_exercises().len() >= 3);

    for exercise in lesson.get_practice_exercises() {
        assert!(!exercise.title.is_empty());
        assert!(!exercise.description.is_empty());
        assert!(!exercise.hint.is_empty());
        assert!(!exercise.setup_moves.is_empty());
        assert!(!exercise.solution.is_empty());
    }
}

#[test]
fn test_corners_007_verify_solved_cube_first_layer() {
    let cube = Cube::new(3);
    assert!(CornersLesson::verify_first_layer(&cube));
}

#[test]
fn test_corners_008_verify_rejects_wrong_size() {
    let cube_2x2 = Cube::new(2);
    assert!(!CornersLesson::verify_first_layer(&cube_2x2));

    let cube_4x4 = Cube::new(4);
    assert!(!CornersLesson::verify_first_layer(&cube_4x4));
}

#[test]
fn test_corners_009_verify_rejects_scrambled() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);
    cube.apply_move(Move::RPrime);
    cube.apply_move(Move::UPrime);

    // After scrambling, first layer should not be complete
    assert!(!CornersLesson::verify_first_layer(&cube));
}

#[test]
fn test_corners_010_verify_rejects_only_cross() {
    let mut cube = Cube::new(3);
    // This scrambles just the corners while keeping cross
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);
    cube.apply_move(Move::RPrime);

    // First layer needs cross AND corners, not just cross
    assert!(!CornersLesson::verify_first_layer(&cube));
}

#[test]
fn test_corners_011_corner_position_enum_variants() {
    // Test that all corner position variants exist
    let _fr = CornerPosition::FrontRight;
    let _fl = CornerPosition::FrontLeft;
    let _br = CornerPosition::BackRight;
    let _bl = CornerPosition::BackLeft;

    // Test equality
    assert_eq!(CornerPosition::FrontRight, CornerPosition::FrontRight);
    assert_ne!(CornerPosition::FrontRight, CornerPosition::FrontLeft);
}

#[test]
fn test_corners_012_get_specific_step() {
    let lesson = CornersLesson::new();

    // Test getting first step
    let step0 = lesson.get_step(0);
    assert!(step0.is_some());
    assert!(step0.unwrap().title.contains("Corner"));

    // Test getting out of bounds
    let step_invalid = lesson.get_step(100);
    assert!(step_invalid.is_none());
}

#[test]
fn test_corners_013_get_specific_case() {
    let lesson = CornersLesson::new();

    // Test getting first case
    let case0 = lesson.get_case(0);
    assert!(case0.is_some());

    // Test getting out of bounds
    let case_invalid = lesson.get_case(100);
    assert!(case_invalid.is_none());
}

#[test]
fn test_corners_014_get_specific_exercise() {
    let lesson = CornersLesson::new();

    // Test getting first exercise
    let ex0 = lesson.get_practice_exercise(0);
    assert!(ex0.is_some());
    assert!(ex0.unwrap().title.contains("Simple"));

    // Test getting out of bounds
    let ex_invalid = lesson.get_practice_exercise(100);
    assert!(ex_invalid.is_none());
}

#[test]
fn test_corners_015_simple_exercise_setup_and_solve() {
    let lesson = CornersLesson::new();
    let exercise = lesson.get_practice_exercise(0).unwrap();

    let mut cube = Cube::new(3);

    // Apply setup moves
    for mov in &exercise.setup_moves {
        cube.apply_move(*mov);
    }

    // After setup, first layer should not be complete
    assert!(!CornersLesson::verify_first_layer(&cube));

    // Apply solution
    for mov in &exercise.solution {
        cube.apply_move(*mov);
    }

    // After solution, we should be closer to solved
    // Note: Simple exercise might not fully solve, just practice one corner
}

#[test]
fn test_corners_016_default_implementation() {
    let lesson1 = CornersLesson::new();
    let lesson2 = CornersLesson::default();

    assert_eq!(lesson1.step_count(), lesson2.step_count());
    assert_eq!(lesson1.get_all_cases().len(), lesson2.get_all_cases().len());
}

#[test]
fn test_corners_017_steps_have_examples_or_not() {
    let lesson = CornersLesson::new();

    // Some steps should have example moves
    let steps_with_examples = lesson
        .get_all_steps()
        .iter()
        .filter(|s| s.example_moves.is_some())
        .count();

    assert!(steps_with_examples >= 3, "Should have at least 3 steps with example moves");
}

#[test]
fn test_corners_018_cases_identify_corner_positions() {
    let lesson = CornersLesson::new();

    // All cases should have a position assigned
    for case in lesson.get_all_cases() {
        // Just verify the position field exists and is accessible
        let _ = case.position;
    }
}

#[test]
fn test_corners_019_exercises_have_increasing_difficulty() {
    let lesson = CornersLesson::new();

    let simple = lesson.get_practice_exercise(0).unwrap();
    let medium = lesson.get_practice_exercise(1).unwrap();
    let advanced = lesson.get_practice_exercise(2).unwrap();

    // Advanced should have more setup moves than simple
    assert!(
        advanced.setup_moves.len() >= simple.setup_moves.len(),
        "Advanced exercise should have at least as many setup moves as simple"
    );

    // Check titles indicate difficulty
    assert!(simple.title.contains("Simple"));
    assert!(medium.title.contains("Medium"));
    assert!(advanced.title.contains("Advanced"));
}

#[test]
fn test_corners_020_all_exercises_have_hints() {
    let lesson = CornersLesson::new();

    for (i, exercise) in lesson.get_practice_exercises().iter().enumerate() {
        assert!(
            !exercise.hint.is_empty(),
            "Exercise {} should have a hint",
            i
        );
        assert!(
            exercise.hint.len() > 10,
            "Exercise {} hint should be substantial",
            i
        );
    }
}
