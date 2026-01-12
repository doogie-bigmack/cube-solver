//! Integration tests for 4x4 tutorial lesson
//!
//! Tests R6.9 requirements:
//! - Center solving strategy
//! - Edge pairing strategy
//! - Parity algorithm explanation

use rubiks_cube_solver::cube::Cube;
use rubiks_cube_solver::tutorial::lessons::FourByFourLesson;

#[test]
fn test_4x4_lesson_001_lesson_creation() {
    let lesson = FourByFourLesson::new();
    assert!(lesson.step_count() > 0, "Lesson should have steps");
    assert!(lesson.case_count() > 0, "Lesson should have cases");
    assert!(lesson.exercise_count() > 0, "Lesson should have practice exercises");
}

#[test]
fn test_4x4_lesson_002_has_reduction_method() {
    let lesson = FourByFourLesson::new();

    // Should have steps explaining the reduction method
    let has_reduction_explanation = lesson.steps.iter().any(|s| {
        s.title.contains("Reduction") || s.description.contains("reduction") || s.description.contains("Reduction")
    });
    assert!(has_reduction_explanation, "Lesson should explain the reduction method");
}

#[test]
fn test_4x4_lesson_003_step_by_step_instructions() {
    let lesson = FourByFourLesson::new();

    // Should have at least 6 steps (intro, differences, method overview, centers, edges, parity, practice)
    assert!(lesson.step_count() >= 6, "Should have multiple instructional steps");

    // Each step should have required fields
    for step in &lesson.steps {
        assert!(!step.title.is_empty(), "Step should have a title");
        assert!(!step.description.is_empty(), "Step should have a description");
        assert!(!step.kid_friendly_text.is_empty(), "Step should have kid-friendly text");
        assert!(!step.visual_hint.is_empty(), "Step should have a visual hint");
    }
}

#[test]
fn test_4x4_lesson_004_has_practice_mode() {
    let lesson = FourByFourLesson::new();

    // Should have practice exercises
    assert_eq!(lesson.exercise_count(), 3, "Should have 3 practice exercises");

    // Each exercise should have required fields
    for exercise in &lesson.practice_exercises {
        assert!(!exercise.description.is_empty(), "Exercise should have a description");
        assert!(!exercise.scramble.is_empty(), "Exercise should have a scramble");
        assert!(!exercise.hint.is_empty(), "Exercise should have a hint");
        assert!(!exercise.phase.is_empty(), "Exercise should have a phase");
    }
}

#[test]
fn test_4x4_lesson_005_centers_solving_step() {
    let lesson = FourByFourLesson::new();

    // Should have a step about solving centers
    let has_centers = lesson.steps.iter().any(|s| {
        s.title.contains("Center") || s.description.contains("center")
    });
    assert!(has_centers, "Lesson should teach center solving");
}

#[test]
fn test_4x4_lesson_006_edges_pairing_step() {
    let lesson = FourByFourLesson::new();

    // Should have a step about pairing edges
    let has_edges = lesson.steps.iter().any(|s| {
        s.title.contains("Edge") || s.title.contains("Pair") || s.description.contains("edge") || s.description.contains("pair")
    });
    assert!(has_edges, "Lesson should teach edge pairing");
}

#[test]
fn test_4x4_lesson_007_parity_step() {
    let lesson = FourByFourLesson::new();

    // Should have a step about parity
    let has_parity = lesson.steps.iter().any(|s| {
        s.title.contains("Parity") || s.title.contains("parity") || s.description.contains("parity")
    });
    assert!(has_parity, "Lesson should teach parity handling");
}

#[test]
fn test_4x4_lesson_008_center_algorithms() {
    let lesson = FourByFourLesson::new();

    // Should have center-solving algorithms
    let center_cases = lesson.get_cases_by_category("centers");
    assert!(center_cases.len() >= 2, "Should have at least 2 center algorithms");

    // Each case should have a valid algorithm
    for case in center_cases {
        assert!(!case.algorithm.is_empty(), "Center case '{}' should have an algorithm", case.name);
        assert!(!case.recognition.is_empty(), "Center case '{}' should have recognition tips", case.name);
    }
}

#[test]
fn test_4x4_lesson_009_edge_algorithms() {
    let lesson = FourByFourLesson::new();

    // Should have edge-pairing algorithms
    let edge_cases = lesson.get_cases_by_category("edges");
    assert!(edge_cases.len() >= 2, "Should have at least 2 edge pairing algorithms");

    // Each case should have a valid algorithm
    for case in edge_cases {
        assert!(!case.algorithm.is_empty(), "Edge case '{}' should have an algorithm", case.name);
        assert!(!case.recognition.is_empty(), "Edge case '{}' should have recognition tips", case.name);
    }
}

#[test]
fn test_4x4_lesson_010_parity_algorithms() {
    let lesson = FourByFourLesson::new();

    // Should have parity algorithms
    let parity_cases = lesson.get_cases_by_category("parity");
    assert_eq!(parity_cases.len(), 2, "Should have exactly 2 parity algorithms (OLL and PLL)");

    // Check for OLL parity
    let has_oll_parity = parity_cases.iter().any(|c| c.name.contains("OLL"));
    assert!(has_oll_parity, "Should have OLL parity algorithm");

    // Check for PLL parity
    let has_pll_parity = parity_cases.iter().any(|c| c.name.contains("PLL"));
    assert!(has_pll_parity, "Should have PLL parity algorithm");

    // Each case should have a valid algorithm
    for case in parity_cases {
        assert!(!case.algorithm.is_empty(), "Parity case '{}' should have an algorithm", case.name);
        assert!(!case.recognition.is_empty(), "Parity case '{}' should have recognition tips", case.name);
    }
}

#[test]
fn test_4x4_lesson_011_practice_phases() {
    let lesson = FourByFourLesson::new();

    // Should have practice for different phases
    let has_centers_practice = lesson.practice_exercises.iter().any(|e| e.phase == "centers");
    let has_edges_practice = lesson.practice_exercises.iter().any(|e| e.phase == "edges");
    let has_full_practice = lesson.practice_exercises.iter().any(|e| e.phase == "full");

    assert!(has_centers_practice, "Should have centers-only practice");
    assert!(has_edges_practice, "Should have edges-only practice");
    assert!(has_full_practice, "Should have full solve practice");
}

#[test]
fn test_4x4_lesson_012_practice_scrambles_valid() {
    let lesson = FourByFourLesson::new();

    for (i, exercise) in lesson.practice_exercises.iter().enumerate() {
        // Create a 4x4 cube and apply the scramble
        let mut cube = Cube::new(4);

        for mv in &exercise.scramble {
            cube.apply_move(*mv);
        }

        // Cube should be scrambled (not solved) for most exercises
        let is_solved = cube.is_solved();

        // The full solve should definitely be scrambled
        if exercise.phase == "full" {
            assert!(!is_solved, "Exercise {} (full) scramble should result in a non-solved cube", i);
        }
    }
}

#[test]
fn test_4x4_lesson_013_apply_scramble_centers() {
    let lesson = FourByFourLesson::new();

    // Should be able to apply the centers practice scramble
    let result = lesson.apply_scramble(0);
    assert!(result.is_ok(), "Should be able to apply centers practice scramble");

    let cube = result.unwrap();
    assert_eq!(cube.size(), 4, "Scrambled cube should be 4x4");
}

#[test]
fn test_4x4_lesson_014_apply_scramble_edges() {
    let lesson = FourByFourLesson::new();

    // Should be able to apply the edges practice scramble
    let result = lesson.apply_scramble(1);
    assert!(result.is_ok(), "Should be able to apply edges practice scramble");

    let cube = result.unwrap();
    assert_eq!(cube.size(), 4, "Scrambled cube should be 4x4");
}

#[test]
fn test_4x4_lesson_015_apply_scramble_full() {
    let lesson = FourByFourLesson::new();

    // Should be able to apply the full practice scramble
    let result = lesson.apply_scramble(2);
    assert!(result.is_ok(), "Should be able to apply full practice scramble");

    let cube = result.unwrap();
    assert_eq!(cube.size(), 4, "Scrambled cube should be 4x4");
    assert!(!cube.is_solved(), "Full scramble should result in unsolved cube");
}

#[test]
fn test_4x4_lesson_016_invalid_exercise_index() {
    let lesson = FourByFourLesson::new();

    // Should return error for invalid index
    let result = lesson.apply_scramble(99);
    assert!(result.is_err(), "Should return error for invalid exercise index");
}

#[test]
fn test_4x4_lesson_017_kid_friendly_language() {
    let lesson = FourByFourLesson::new();

    // All steps should use kid-friendly language
    for step in &lesson.steps {
        let text = step.kid_friendly_text.to_lowercase();

        // Should avoid overly technical terms
        let is_kid_friendly = !text.contains("commutator") && !text.contains("conjugate");

        // Should use simple, encouraging language
        let is_encouraging = text.contains("!") || text.contains("easy") ||
                           text.contains("simple") || text.contains("practice") ||
                           text.contains("fun");

        assert!(is_kid_friendly || is_encouraging,
            "Step '{}' should use kid-friendly language", step.title);
    }
}

#[test]
fn test_4x4_lesson_018_cases_complete() {
    let lesson = FourByFourLesson::new();

    // Should have all required case categories
    let center_cases = lesson.get_cases_by_category("centers");
    let edge_cases = lesson.get_cases_by_category("edges");
    let parity_cases = lesson.get_cases_by_category("parity");

    assert!(center_cases.len() >= 2, "Should have at least 2 center cases");
    assert!(edge_cases.len() >= 2, "Should have at least 2 edge cases");
    assert_eq!(parity_cases.len(), 2, "Should have exactly 2 parity cases");

    // Total should be at least 6 cases
    assert!(lesson.case_count() >= 6, "Should have at least 6 total cases");
}

#[test]
fn test_4x4_lesson_019_algorithms_executable() {
    let lesson = FourByFourLesson::new();

    // All algorithms should be executable on a 4x4 cube
    let mut cube = Cube::new(4);

    for case in &lesson.cases {
        let _initial_state = cube.clone();

        // Execute the algorithm - should not panic
        for mv in &case.algorithm {
            cube.apply_move(*mv);
        }

        // Algorithm executed successfully (no panics)
    }
}

#[test]
fn test_4x4_lesson_020_lesson_progression() {
    let lesson = FourByFourLesson::new();

    // Steps should follow a logical progression
    let titles: Vec<String> = lesson.steps.iter().map(|s| s.title.clone()).collect();

    // Should start with introduction
    assert!(titles[0].contains("Welcome") || titles[0].contains("Introduction") || titles[0].contains("4x4"),
        "First step should be an introduction");

    // Should explain the method before diving into specifics
    let early_steps_explain_method = titles[0..3].iter().any(|t|
        t.contains("Reduction") || t.contains("Method") || t.contains("Different")
    );
    assert!(early_steps_explain_method, "Early steps should explain the method");

    // Should end with practice or summary
    let last_step = &titles[titles.len() - 1];
    assert!(last_step.contains("Practice") || last_step.contains("Perfect"),
        "Last step should encourage practice");
}

#[test]
fn test_4x4_lesson_021_visual_hints_present() {
    let lesson = FourByFourLesson::new();

    // All steps should have visual hints
    for step in &lesson.steps {
        assert!(!step.visual_hint.is_empty(),
            "Step '{}' should have a visual hint", step.title);
    }
}

#[test]
fn test_4x4_lesson_022_all_cases_have_categories() {
    let lesson = FourByFourLesson::new();

    // All cases should have a valid category
    for case in &lesson.cases {
        assert!(
            case.category == "centers" || case.category == "edges" || case.category == "parity",
            "Case '{}' should have a valid category", case.name
        );
    }
}

#[test]
fn test_4x4_lesson_023_parity_explanation() {
    let lesson = FourByFourLesson::new();

    // Should explain both types of parity - look for a step specifically about parity fixing
    let parity_step = lesson.steps.iter().find(|s| {
        s.title.contains("Parity") || s.title.contains("parity")
    });

    assert!(parity_step.is_some(), "Should have a parity explanation step");

    let parity_step = parity_step.unwrap();
    let desc = parity_step.description.to_lowercase();

    // Should mention both OLL and PLL parity
    assert!(desc.contains("oll"), "Should explain OLL parity");
    assert!(desc.contains("pll"), "Should explain PLL parity");
}

#[test]
fn test_4x4_lesson_024_reduction_method_steps() {
    let lesson = FourByFourLesson::new();

    // Should have steps for all 4 phases of reduction method
    let has_centers_step = lesson.steps.iter().any(|s|
        s.title.contains("Center") || s.description.contains("centers")
    );
    let has_edges_step = lesson.steps.iter().any(|s|
        s.title.contains("Edge") || s.title.contains("Pair") || s.description.contains("pair")
    );
    let has_3x3_step = lesson.steps.iter().any(|s|
        s.title.contains("3x3") || s.description.contains("3x3")
    );
    let has_parity_step = lesson.steps.iter().any(|s|
        s.title.contains("Parity") || s.description.contains("parity")
    );

    assert!(has_centers_step, "Should have centers solving step");
    assert!(has_edges_step, "Should have edge pairing step");
    assert!(has_3x3_step, "Should have 3x3 solving step");
    assert!(has_parity_step, "Should have parity handling step");
}

#[test]
fn test_4x4_lesson_025_default_impl() {
    // Test the Default implementation
    let lesson1 = FourByFourLesson::new();
    let lesson2 = FourByFourLesson::default();

    assert_eq!(lesson1.step_count(), lesson2.step_count());
    assert_eq!(lesson1.case_count(), lesson2.case_count());
    assert_eq!(lesson1.exercise_count(), lesson2.exercise_count());
}
