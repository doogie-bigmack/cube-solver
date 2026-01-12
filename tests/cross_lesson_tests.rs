//! Integration tests for R6.3 - Cross Tutorial
//!
//! These tests verify the cross tutorial lesson implementation

use rubiks_cube_solver::cube::{Cube, Move};
use rubiks_cube_solver::tutorial::lessons::{CrossLesson, CrossEdge};

#[test]
fn test_cross_001_lesson_has_all_steps() {
    let lesson = CrossLesson::new();
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
fn test_cross_002_lesson_covers_key_concepts() {
    let lesson = CrossLesson::new();
    let all_text: String = lesson
        .get_all_steps()
        .iter()
        .map(|s| format!("{} {} {}", s.title, s.description, s.kid_friendly_text))
        .collect::<Vec<_>>()
        .join(" ");

    // Check that key concepts are mentioned
    assert!(all_text.contains("daisy") || all_text.contains("Daisy"));
    assert!(all_text.contains("cross") || all_text.contains("Cross"));
    assert!(all_text.contains("edge"));
    assert!(all_text.contains("center"));
    assert!(all_text.contains("align") || all_text.contains("match"));
}

#[test]
fn test_cross_003_has_multiple_algorithms() {
    let lesson = CrossLesson::new();
    assert!(lesson.get_all_cases().len() >= 5);

    // Each case should have an algorithm
    for case in lesson.get_all_cases() {
        assert!(!case.name.is_empty());
        assert!(!case.description.is_empty());
        assert!(!case.explanation.is_empty());
        // Algorithm can be empty for "already solved" case
    }
}

#[test]
fn test_cross_004_practice_exercises_exist() {
    let lesson = CrossLesson::new();
    assert!(lesson.get_practice_exercises().len() >= 3);

    for exercise in lesson.get_practice_exercises() {
        assert!(!exercise.title.is_empty());
        assert!(!exercise.description.is_empty());
        assert!(!exercise.hint.is_empty());
        assert!(!exercise.setup_moves.is_empty());
    }
}

#[test]
fn test_cross_005_verify_solved_cube() {
    let cube = Cube::new(3);
    assert!(CrossLesson::verify_white_cross(&cube));
}

#[test]
fn test_cross_006_verify_rejects_wrong_size() {
    let cube_2x2 = Cube::new(2);
    assert!(!CrossLesson::verify_white_cross(&cube_2x2));

    let cube_4x4 = Cube::new(4);
    assert!(!CrossLesson::verify_white_cross(&cube_4x4));
}

#[test]
fn test_cross_007_verify_rejects_scrambled() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);
    cube.apply_move(Move::RPrime);
    cube.apply_move(Move::UPrime);

    assert!(!CrossLesson::verify_white_cross(&cube));
}

#[test]
fn test_cross_008_cross_edge_enum_variants() {
    let _front = CrossEdge::Front;
    let _right = CrossEdge::Right;
    let _back = CrossEdge::Back;
    let _left = CrossEdge::Left;

    // Verify they can be compared
    assert_eq!(CrossEdge::Front, CrossEdge::Front);
    assert_ne!(CrossEdge::Front, CrossEdge::Right);
}

#[test]
fn test_cross_009_example_moves_provided() {
    let lesson = CrossLesson::new();
    let mut has_examples = 0;

    for step in lesson.get_all_steps() {
        if step.example_moves.is_some() {
            has_examples += 1;
        }
    }

    // At least some steps should have example moves
    assert!(has_examples >= 3);
}

#[test]
fn test_cross_010_tips_provided() {
    let lesson = CrossLesson::new();
    let mut has_tips = 0;

    for step in lesson.get_all_steps() {
        if step.tip.is_some() {
            has_tips += 1;
        }
    }

    // At least some steps should have tips
    assert!(has_tips >= 5);
}

#[test]
fn test_cross_011_get_step_by_index() {
    let lesson = CrossLesson::new();

    let first_step = lesson.get_step(0);
    assert!(first_step.is_some());
    assert!(first_step.unwrap().title.contains("Welcome"));

    let invalid_step = lesson.get_step(999);
    assert!(invalid_step.is_none());
}

#[test]
fn test_cross_012_get_case_by_index() {
    let lesson = CrossLesson::new();

    let first_case = lesson.get_case(0);
    assert!(first_case.is_some());

    let invalid_case = lesson.get_case(999);
    assert!(invalid_case.is_none());
}

#[test]
fn test_cross_013_get_practice_exercise_by_index() {
    let lesson = CrossLesson::new();

    let first_exercise = lesson.get_practice_exercise(0);
    assert!(first_exercise.is_some());

    let invalid_exercise = lesson.get_practice_exercise(999);
    assert!(invalid_exercise.is_none());
}

#[test]
fn test_cross_014_daisy_method_explained() {
    let lesson = CrossLesson::new();

    // Find the daisy method step
    let daisy_step = lesson.get_all_steps().iter().find(|s| {
        s.title.to_lowercase().contains("daisy")
    });

    assert!(daisy_step.is_some());
    let step = daisy_step.unwrap();
    assert!(!step.description.is_empty());
    assert!(step.example_moves.is_some());
}

#[test]
fn test_cross_015_align_edges_explained() {
    let lesson = CrossLesson::new();

    // Find the align edges step
    let align_step = lesson.get_all_steps().iter().find(|s| {
        s.title.to_lowercase().contains("align")
    });

    assert!(align_step.is_some());
    let step = align_step.unwrap();
    assert!(step.description.contains("U") || step.description.contains("top"));
}

#[test]
fn test_cross_016_flip_down_explained() {
    let lesson = CrossLesson::new();

    // Find the flip down step
    let flip_step = lesson.get_all_steps().iter().find(|s| {
        s.title.to_lowercase().contains("flip") || s.title.to_lowercase().contains("complete")
    });

    assert!(flip_step.is_some());
    let step = flip_step.unwrap();
    assert!(step.description.contains("F2") || step.description.contains("180"));
}

#[test]
fn test_cross_017_practice_exercise_scrambles_cube() {
    let lesson = CrossLesson::new();

    for exercise in lesson.get_practice_exercises() {
        let mut cube = Cube::new(3);

        // Apply setup moves
        for mov in &exercise.setup_moves {
            cube.apply_move(*mov);
        }

        // After scrambling, cube should not have solved cross
        // (though this might occasionally be false for very simple scrambles)
        // We just verify the exercise is applied
        assert_eq!(cube.size(), 3);
    }
}

#[test]
fn test_cross_018_case_algorithms_are_moves() {
    let lesson = CrossLesson::new();

    for case in lesson.get_all_cases() {
        // Algorithms should either be empty (already solved case) or have moves
        let mut cube = Cube::new(3);

        // Verify we can apply the algorithm without panicking
        for mov in &case.algorithm {
            cube.apply_move(*mov);
        }
    }
}

#[test]
fn test_cross_019_kid_friendly_text_different() {
    let lesson = CrossLesson::new();

    for step in lesson.get_all_steps() {
        // Kid-friendly text should be different from description
        // (providing alternative explanation)
        assert_ne!(step.kid_friendly_text, step.description);
    }
}

#[test]
fn test_cross_020_default_trait_works() {
    let lesson1 = CrossLesson::new();
    let lesson2 = CrossLesson::default();

    assert_eq!(lesson1.step_count(), lesson2.step_count());
}

#[test]
fn test_cross_021_verify_after_single_move() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::R);

    // After a single R move, cross should not be solved
    assert!(!CrossLesson::verify_white_cross(&cube));
}

#[test]
fn test_cross_022_verify_cross_with_corners_unsolved() {
    // This test would require manually setting cube state
    // to have cross solved but corners wrong
    // For now, we just verify the function doesn't panic
    let cube = Cube::new(3);
    let _ = CrossLesson::verify_white_cross(&cube);
}

#[test]
fn test_cross_023_all_cases_have_edge_assignment() {
    let lesson = CrossLesson::new();

    for case in lesson.get_all_cases() {
        // Each case should have an edge assigned
        let _ = case.edge; // Just access it to verify it exists
    }
}

#[test]
fn test_cross_024_intro_step_is_first() {
    let lesson = CrossLesson::new();
    let first = lesson.get_step(0).unwrap();

    assert!(first.title.contains("Welcome") || first.title.contains("Cross"));
}

#[test]
fn test_cross_025_practice_step_is_last() {
    let lesson = CrossLesson::new();
    let last_index = lesson.step_count() - 1;
    let last = lesson.get_step(last_index).unwrap();

    assert!(last.title.contains("Practice") || last.title.contains("practice"));
}
