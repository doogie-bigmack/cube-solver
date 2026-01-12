//! Integration tests for PLL (Permute Last Layer) tutorial lesson
//!
//! Tests R6.7 requirements:
//! - 2-look PLL algorithms
//! - Pattern recognition
//! - Practice mode

use rubiks_cube_solver::tutorial::lessons::pll::{PllLesson, PllPattern};
use rubiks_cube_solver::cube::{Cube, Move};

#[test]
fn test_pll_lesson_loads_successfully() {
    let lesson = PllLesson::new();
    assert!(!lesson.get_steps().is_empty(), "PLL lesson should have steps");
    assert!(!lesson.get_cases().is_empty(), "PLL lesson should have cases");
    assert!(!lesson.get_practice_exercises().is_empty(), "PLL lesson should have practice exercises");
}

#[test]
fn test_pll_lesson_has_correct_number_of_steps() {
    let lesson = PllLesson::new();
    // Should have: intro, what is PLL, 2-look explanation, corner perm, edge perm, pattern recognition, practice
    assert_eq!(lesson.get_steps().len(), 7, "PLL lesson should have 7 steps");
}

#[test]
fn test_pll_lesson_intro_step() {
    let lesson = PllLesson::new();
    let intro = &lesson.get_steps()[0];

    assert_eq!(intro.title, "Welcome to PLL!");
    assert!(intro.description.contains("Permute Last Layer"));
    assert!(intro.kid_friendly_text.contains("sliding puzzle"));
}

#[test]
fn test_pll_lesson_has_2look_algorithms() {
    let lesson = PllLesson::new();

    // 2-look PLL should have at least 6 common algorithms
    assert!(lesson.get_cases().len() >= 6);

    // Should have corner permutation cases
    let has_corner_perms = lesson.get_cases().iter().any(|c|
        c.pattern == PllPattern::Headlights || c.pattern == PllPattern::DiagonalSwap
    );
    assert!(has_corner_perms, "Should have corner permutation algorithms");

    // Should have edge permutation cases
    let has_edge_perms = lesson.get_cases().iter().any(|c|
        c.pattern == PllPattern::UaPerm || c.pattern == PllPattern::UbPerm
    );
    assert!(has_edge_perms, "Should have edge permutation algorithms");
}

#[test]
fn test_ua_perm_case() {
    let lesson = PllLesson::new();
    let ua = lesson.get_case_by_pattern(PllPattern::UaPerm);

    assert!(ua.is_some(), "Should have Ua perm case");
    let ua = ua.unwrap();

    assert_eq!(ua.name, "Ua Perm");
    assert!(!ua.algorithm.is_empty());
    assert!(!ua.explanation.is_empty());
    assert!(!ua.visual_hint.is_empty());
}

#[test]
fn test_ub_perm_case() {
    let lesson = PllLesson::new();
    let ub = lesson.get_case_by_pattern(PllPattern::UbPerm);

    assert!(ub.is_some(), "Should have Ub perm case");
    let ub = ub.unwrap();

    assert_eq!(ub.name, "Ub Perm");
    assert!(!ub.algorithm.is_empty());
    assert!(ub.explanation.contains("opposite"));
}

#[test]
fn test_h_perm_case() {
    let lesson = PllLesson::new();
    let h = lesson.get_case_by_pattern(PllPattern::HPerm);

    assert!(h.is_some(), "Should have H perm case");
    let h = h.unwrap();

    assert_eq!(h.name, "H Perm");
    assert!(!h.algorithm.is_empty());
    assert!(h.algorithm.iter().any(|m| matches!(m, Move::M2)));
}

#[test]
fn test_z_perm_case() {
    let lesson = PllLesson::new();
    let z = lesson.get_case_by_pattern(PllPattern::ZPerm);

    assert!(z.is_some(), "Should have Z perm case");
    let z = z.unwrap();

    assert_eq!(z.name, "Z Perm");
    assert!(!z.algorithm.is_empty());
}

#[test]
fn test_aa_perm_case() {
    let lesson = PllLesson::new();
    let aa = lesson.get_case_by_pattern(PllPattern::Headlights);

    assert!(aa.is_some(), "Should have Aa perm (headlights) case");
    let aa = aa.unwrap();

    assert!(aa.name.contains("Aa Perm"));
    assert!(!aa.algorithm.is_empty());
    assert!(aa.explanation.contains("adjacent") || aa.explanation.contains("headlights"));
}

#[test]
fn test_e_perm_case() {
    let lesson = PllLesson::new();
    let e = lesson.get_case_by_pattern(PllPattern::DiagonalSwap);

    assert!(e.is_some(), "Should have E perm (diagonal swap) case");
    let e = e.unwrap();

    assert!(e.name.contains("E Perm"));
    assert!(!e.algorithm.is_empty());
    assert!(e.explanation.contains("diagonal"));
}

#[test]
fn test_all_algorithms_are_valid_moves() {
    let lesson = PllLesson::new();

    for case in lesson.get_cases() {
        assert!(!case.algorithm.is_empty(), "Case {} has empty algorithm", case.name);

        // Verify algorithms can be applied to a cube
        let mut cube = Cube::new(3);
        for mv in &case.algorithm {
            cube.apply_move(*mv);
        }
    }
}

#[test]
fn test_practice_exercises_exist() {
    let lesson = PllLesson::new();
    let exercises = lesson.get_practice_exercises();

    assert!(!exercises.is_empty(), "Should have practice exercises");
    assert!(exercises.len() >= 4, "Should have at least 4 practice exercises");
}

#[test]
fn test_practice_exercises_have_valid_moves() {
    let lesson = PllLesson::new();

    for exercise in lesson.get_practice_exercises() {
        assert!(!exercise.setup_moves.is_empty(), "Exercise {} has no setup", exercise.title);
        assert!(!exercise.solution.is_empty(), "Exercise {} has no solution", exercise.title);
        assert!(!exercise.hint.is_empty(), "Exercise {} has no hint", exercise.title);

        // Verify setup and solution can be applied
        let mut cube = Cube::new(3);
        for mv in &exercise.setup_moves {
            cube.apply_move(*mv);
        }
        for mv in &exercise.solution {
            cube.apply_move(*mv);
        }
    }
}

#[test]
fn test_ua_perm_practice_exercise() {
    let lesson = PllLesson::new();
    let exercises = lesson.get_practice_exercises();

    let ua_exercise = exercises.iter().find(|e| e.pattern == PllPattern::UaPerm);
    assert!(ua_exercise.is_some(), "Should have Ua perm practice exercise");

    let exercise = ua_exercise.unwrap();
    assert!(exercise.title.contains("Ua Perm"));
    assert!(!exercise.setup_moves.is_empty());
    assert!(!exercise.solution.is_empty());
}

#[test]
fn test_ub_perm_practice_exercise() {
    let lesson = PllLesson::new();
    let exercises = lesson.get_practice_exercises();

    let ub_exercise = exercises.iter().find(|e| e.pattern == PllPattern::UbPerm);
    assert!(ub_exercise.is_some(), "Should have Ub perm practice exercise");

    let exercise = ub_exercise.unwrap();
    assert!(exercise.title.contains("Ub Perm"));
}

#[test]
fn test_h_perm_practice_exercise() {
    let lesson = PllLesson::new();
    let exercises = lesson.get_practice_exercises();

    let h_exercise = exercises.iter().find(|e| e.pattern == PllPattern::HPerm);
    assert!(h_exercise.is_some(), "Should have H perm practice exercise");
}

#[test]
fn test_all_steps_have_kid_friendly_text() {
    let lesson = PllLesson::new();

    for step in lesson.get_steps() {
        assert!(!step.kid_friendly_text.is_empty(),
            "Step '{}' missing kid-friendly text", step.title);
        assert!(!step.description.is_empty(),
            "Step '{}' missing description", step.title);
    }
}

#[test]
fn test_all_cases_have_visual_hints() {
    let lesson = PllLesson::new();

    for case in lesson.get_cases() {
        assert!(!case.visual_hint.is_empty(),
            "Case '{}' missing visual hint", case.name);
    }
}

#[test]
fn test_pattern_recognition_step_exists() {
    let lesson = PllLesson::new();
    let steps = lesson.get_steps();

    let has_pattern_recognition = steps.iter().any(|s|
        s.title.contains("Pattern Recognition")
    );
    assert!(has_pattern_recognition, "Should have pattern recognition step");
}

#[test]
fn test_corner_permutation_step_exists() {
    let lesson = PllLesson::new();
    let steps = lesson.get_steps();

    let has_corner_perm = steps.iter().any(|s|
        s.title.contains("Corner Permutation")
    );
    assert!(has_corner_perm, "Should have corner permutation step");
}

#[test]
fn test_edge_permutation_step_exists() {
    let lesson = PllLesson::new();
    let steps = lesson.get_steps();

    let has_edge_perm = steps.iter().any(|s|
        s.title.contains("Edge Permutation")
    );
    assert!(has_edge_perm, "Should have edge permutation step");
}

#[test]
fn test_two_look_pll_explanation_exists() {
    let lesson = PllLesson::new();
    let steps = lesson.get_steps();

    let has_explanation = steps.iter().any(|s|
        s.title.contains("2-Look PLL") || s.description.contains("2-look")
    );
    assert!(has_explanation, "Should explain 2-look PLL method");
}

#[test]
fn test_practice_step_exists() {
    let lesson = PllLesson::new();
    let steps = lesson.get_steps();

    let has_practice = steps.iter().any(|s|
        s.title.contains("Practice")
    );
    assert!(has_practice, "Should have practice step");
}

#[test]
fn test_all_pll_patterns_covered() {
    let lesson = PllLesson::new();
    let cases = lesson.get_cases();

    // Check that we have variety in patterns
    let patterns: Vec<_> = cases.iter().map(|c| c.pattern).collect();

    // Should have both corner and edge permutations
    let has_ua = patterns.contains(&PllPattern::UaPerm);
    let has_ub = patterns.contains(&PllPattern::UbPerm);
    let has_h = patterns.contains(&PllPattern::HPerm);

    assert!(has_ua, "Should have Ua perm pattern");
    assert!(has_ub, "Should have Ub perm pattern");
    assert!(has_h, "Should have H perm pattern");
}

#[test]
fn test_lesson_completeness() {
    let lesson = PllLesson::new();

    // Verify lesson has all required components for R6.7
    assert!(lesson.get_steps().len() >= 5, "Should have at least 5 steps");
    assert!(lesson.get_cases().len() >= 6, "Should have at least 6 algorithms (2-look PLL)");
    assert!(lesson.get_practice_exercises().len() >= 3, "Should have at least 3 practice exercises");

    // All cases should have complete information
    for case in lesson.get_cases() {
        assert!(!case.name.is_empty());
        assert!(!case.description.is_empty());
        assert!(!case.algorithm.is_empty());
        assert!(!case.explanation.is_empty());
        assert!(!case.visual_hint.is_empty());
    }
}
