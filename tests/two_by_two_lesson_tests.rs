//! Integration tests for 2x2 tutorial lesson
//!
//! Tests R6.8 requirements:
//! - Ortega method or beginner method
//! - Step-by-step instructions
//! - Practice mode

use rubiks_cube_solver::cube::Cube;
use rubiks_cube_solver::tutorial::lessons::{TwoByTwoLesson, OrtegaCase};

#[test]
fn test_2x2_lesson_001_lesson_creation() {
    let lesson = TwoByTwoLesson::new();
    assert!(lesson.step_count() > 0, "Lesson should have steps");
    assert!(lesson.case_count() > 0, "Lesson should have Ortega cases");
    assert!(lesson.exercise_count() > 0, "Lesson should have practice exercises");
}

#[test]
fn test_2x2_lesson_002_has_ortega_method() {
    let lesson = TwoByTwoLesson::new();

    // Should have steps explaining the Ortega method
    let has_ortega_explanation = lesson.steps.iter().any(|s| {
        s.title.contains("Ortega") || s.description.contains("Ortega")
    });
    assert!(has_ortega_explanation, "Lesson should explain the Ortega method");
}

#[test]
fn test_2x2_lesson_003_step_by_step_instructions() {
    let lesson = TwoByTwoLesson::new();

    // Should have at least 5 steps (intro, method overview, step 1, step 2, step 3)
    assert!(lesson.step_count() >= 5, "Should have multiple instructional steps");

    // Each step should have required fields
    for step in &lesson.steps {
        assert!(!step.title.is_empty(), "Step should have a title");
        assert!(!step.description.is_empty(), "Step should have a description");
        assert!(!step.kid_friendly_text.is_empty(), "Step should have kid-friendly text");
        assert!(!step.visual_hint.is_empty(), "Step should have a visual hint");
    }
}

#[test]
fn test_2x2_lesson_004_has_practice_mode() {
    let lesson = TwoByTwoLesson::new();

    // Should have practice exercises
    assert_eq!(lesson.exercise_count(), 3, "Should have 3 practice exercises");

    // Each exercise should have required fields
    for exercise in &lesson.practice_exercises {
        assert!(!exercise.description.is_empty(), "Exercise should have a description");
        assert!(!exercise.scramble.is_empty(), "Exercise should have a scramble");
        assert!(!exercise.hint.is_empty(), "Exercise should have a hint");
    }
}

#[test]
fn test_2x2_lesson_005_first_face_step() {
    let lesson = TwoByTwoLesson::new();

    // Should have a step about solving the first face
    let has_first_face = lesson.steps.iter().any(|s| {
        s.title.contains("First Face") || s.description.contains("first face") || s.description.contains("one face")
    });
    assert!(has_first_face, "Lesson should teach solving the first face");
}

#[test]
fn test_2x2_lesson_006_oll_step() {
    let lesson = TwoByTwoLesson::new();

    // Should have a step about Orient Last Layer (OLL)
    let has_oll = lesson.steps.iter().any(|s| {
        s.title.contains("OLL") || s.title.contains("Orient") || s.description.contains("orient")
    });
    assert!(has_oll, "Lesson should teach OLL (Orient Last Layer)");
}

#[test]
fn test_2x2_lesson_007_pll_step() {
    let lesson = TwoByTwoLesson::new();

    // Should have a step about Permute Last Layer (PLL)
    let has_pll = lesson.steps.iter().any(|s| {
        s.title.contains("PLL") || s.title.contains("Permute") || s.description.contains("permute")
    });
    assert!(has_pll, "Lesson should teach PLL (Permute Last Layer)");
}

#[test]
fn test_2x2_lesson_008_oll_algorithms() {
    let lesson = TwoByTwoLesson::new();

    // Should have OLL case algorithms
    let oll_cases: Vec<&OrtegaCase> = lesson.ortega_cases.iter()
        .filter(|c| c.name.contains("Pattern") || c.name.contains("Sune"))
        .collect();

    assert!(oll_cases.len() >= 2, "Should have at least 2 OLL algorithms");

    // Each case should have a valid algorithm
    for case in oll_cases {
        assert!(!case.algorithm.is_empty(), "OLL case '{}' should have an algorithm", case.name);
        assert!(!case.recognition.is_empty(), "OLL case '{}' should have recognition tips", case.name);
    }
}

#[test]
fn test_2x2_lesson_009_pll_algorithms() {
    let lesson = TwoByTwoLesson::new();

    // Should have PLL case algorithms
    let pll_cases: Vec<&OrtegaCase> = lesson.ortega_cases.iter()
        .filter(|c| c.name.contains("Swap"))
        .collect();

    assert!(pll_cases.len() >= 2, "Should have at least 2 PLL algorithms");

    // Each case should have a valid algorithm
    for case in pll_cases {
        assert!(!case.algorithm.is_empty(), "PLL case '{}' should have an algorithm", case.name);
        assert!(!case.recognition.is_empty(), "PLL case '{}' should have recognition tips", case.name);
    }
}

#[test]
fn test_2x2_lesson_010_practice_difficulty_levels() {
    let lesson = TwoByTwoLesson::new();

    // Should have easy, medium, and hard exercises
    let easy = lesson.practice_exercises.iter().any(|e| e.description.contains("Easy"));
    let medium = lesson.practice_exercises.iter().any(|e| e.description.contains("Medium"));
    let hard = lesson.practice_exercises.iter().any(|e| e.description.contains("Hard"));

    assert!(easy, "Should have an easy practice exercise");
    assert!(medium, "Should have a medium practice exercise");
    assert!(hard, "Should have a hard practice exercise");
}

#[test]
fn test_2x2_lesson_011_practice_scrambles_valid() {
    let lesson = TwoByTwoLesson::new();

    for (i, exercise) in lesson.practice_exercises.iter().enumerate() {
        // Create a 2x2 cube and apply the scramble
        let mut cube = Cube::new(2);

        for mv in &exercise.scramble {
            cube.apply_move(*mv);
        }

        // Cube should be scrambled (not solved)
        let is_solved = cube.is_solved();

        // At least the harder exercises should result in a scrambled cube
        if exercise.description.contains("Medium") || exercise.description.contains("Hard") {
            assert!(!is_solved, "Exercise {} scramble should result in a non-solved cube", i);
        }
    }
}

#[test]
fn test_2x2_lesson_012_solve_practice_easy() {
    let lesson = TwoByTwoLesson::new();

    // Should be able to solve the easy practice exercise
    let result = lesson.solve_practice(0);
    assert!(result.is_ok(), "Should be able to solve easy practice exercise");

    let solution = result.unwrap();
    assert!(!solution.is_empty(), "Solution should not be empty");

    // Verify the solution actually works
    let mut cube = Cube::new(2);
    for mv in &lesson.practice_exercises[0].scramble {
        cube.apply_move(*mv);
    }
    for mv in &solution {
        cube.apply_move(*mv);
    }
    assert!(cube.is_solved(), "Solution should solve the scrambled cube");
}

#[test]
fn test_2x2_lesson_013_solve_practice_medium() {
    let lesson = TwoByTwoLesson::new();

    // Should be able to solve the medium practice exercise
    let result = lesson.solve_practice(1);
    assert!(result.is_ok(), "Should be able to solve medium practice exercise");

    let solution = result.unwrap();
    assert!(!solution.is_empty(), "Solution should not be empty");

    // Verify the solution actually works
    let mut cube = Cube::new(2);
    for mv in &lesson.practice_exercises[1].scramble {
        cube.apply_move(*mv);
    }
    for mv in &solution {
        cube.apply_move(*mv);
    }
    assert!(cube.is_solved(), "Solution should solve the scrambled cube");
}

#[test]
fn test_2x2_lesson_014_solve_practice_hard() {
    let lesson = TwoByTwoLesson::new();

    // Should be able to solve the hard practice exercise
    let result = lesson.solve_practice(2);
    assert!(result.is_ok(), "Should be able to solve hard practice exercise");

    let solution = result.unwrap();
    assert!(!solution.is_empty(), "Solution should not be empty");

    // Verify the solution actually works
    let mut cube = Cube::new(2);
    for mv in &lesson.practice_exercises[2].scramble {
        cube.apply_move(*mv);
    }
    for mv in &solution {
        cube.apply_move(*mv);
    }
    assert!(cube.is_solved(), "Solution should solve the scrambled cube");
}

#[test]
fn test_2x2_lesson_015_invalid_exercise_index() {
    let lesson = TwoByTwoLesson::new();

    // Should return error for invalid index
    let result = lesson.solve_practice(99);
    assert!(result.is_err(), "Should return error for invalid exercise index");
}

#[test]
fn test_2x2_lesson_016_kid_friendly_language() {
    let lesson = TwoByTwoLesson::new();

    // All steps should use kid-friendly language
    for step in &lesson.steps {
        let text = step.kid_friendly_text.to_lowercase();

        // Should avoid overly technical terms
        let is_kid_friendly = !text.contains("commutator") && !text.contains("conjugate");

        // Should use simple, encouraging language
        let is_encouraging = text.contains("!") || text.contains("easy") || text.contains("simple") || text.contains("practice");

        assert!(is_kid_friendly || is_encouraging,
            "Step '{}' should use kid-friendly language", step.title);
    }
}

#[test]
fn test_2x2_lesson_017_ortega_cases_complete() {
    let lesson = TwoByTwoLesson::new();

    // Should have key Ortega method cases
    let has_h_pattern = lesson.ortega_cases.iter().any(|c| c.name.contains("H"));
    let has_pi_pattern = lesson.ortega_cases.iter().any(|c| c.name.contains("Pi"));
    let has_sune = lesson.ortega_cases.iter().any(|c| c.name.contains("Sune"));
    let has_adjacent_swap = lesson.ortega_cases.iter().any(|c| c.name.contains("Adjacent"));
    let has_diagonal_swap = lesson.ortega_cases.iter().any(|c| c.name.contains("Diagonal"));

    assert!(has_h_pattern, "Should have H pattern OLL case");
    assert!(has_pi_pattern, "Should have Pi pattern OLL case");
    assert!(has_sune, "Should have Sune OLL case");
    assert!(has_adjacent_swap, "Should have Adjacent Swap PLL case");
    assert!(has_diagonal_swap, "Should have Diagonal Swap PLL case");
}

#[test]
fn test_2x2_lesson_018_algorithms_executable() {
    let lesson = TwoByTwoLesson::new();

    // All algorithms should be executable on a 2x2 cube
    let mut cube = Cube::new(2);

    for case in &lesson.ortega_cases {
        let _initial_state = cube.clone();

        // Execute the algorithm
        for mv in &case.algorithm {
            cube.apply_move(*mv);
        }

        // Cube should have changed (unless it was a do-nothing algorithm)
        // Most algorithms should change the cube state
        // (We're just verifying no panics/errors occur)
    }
}

#[test]
fn test_2x2_lesson_019_lesson_progression() {
    let lesson = TwoByTwoLesson::new();

    // Steps should follow a logical progression
    let titles: Vec<String> = lesson.steps.iter().map(|s| s.title.clone()).collect();

    // Should start with introduction
    assert!(titles[0].contains("Welcome") || titles[0].contains("Introduction") || titles[0].contains("2x2"),
        "First step should be an introduction");

    // Should explain the method before diving into specifics
    let early_steps_explain_method = titles[0..3].iter().any(|t|
        t.contains("Ortega") || t.contains("Method") || t.contains("Different")
    );
    assert!(early_steps_explain_method, "Early steps should explain the method");

    // Should end with practice or summary
    let last_step = &titles[titles.len() - 1];
    assert!(last_step.contains("Practice") || last_step.contains("Perfect"),
        "Last step should encourage practice");
}

#[test]
fn test_2x2_lesson_020_visual_hints_present() {
    let lesson = TwoByTwoLesson::new();

    // All steps should have visual hints
    for step in &lesson.steps {
        assert!(!step.visual_hint.is_empty(),
            "Step '{}' should have a visual hint", step.title);
    }
}
