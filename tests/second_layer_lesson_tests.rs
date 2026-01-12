//! Integration tests for R6.5: Second Layer Tutorial

use rubiks_cube_solver::cube::{Cube, Move};
use rubiks_cube_solver::tutorial::lessons::f2l_edges::{
    SecondLayerLesson, EdgePosition
};

#[test]
fn test_second_layer_lesson_creation() {
    let lesson = SecondLayerLesson::new();
    assert_eq!(lesson.step_count(), 9);
    assert!(lesson.get_all_steps().len() > 0);
    assert!(lesson.get_all_cases().len() > 0);
    assert!(lesson.get_practice_exercises().len() > 0);
}

#[test]
fn test_all_steps_have_titles() {
    let lesson = SecondLayerLesson::new();
    for step in lesson.get_all_steps() {
        assert!(!step.title.is_empty(), "Step should have a title");
        assert!(!step.description.is_empty(), "Step should have a description");
        assert!(!step.kid_friendly_text.is_empty(), "Step should have kid-friendly text");
    }
}

#[test]
fn test_lesson_has_intro_step() {
    let lesson = SecondLayerLesson::new();
    let intro = lesson.get_step(0).unwrap();
    assert!(intro.title.contains("Second Layer"));
    assert!(intro.kid_friendly_text.contains("middle"));
}

#[test]
fn test_lesson_has_right_algorithm_step() {
    let lesson = SecondLayerLesson::new();
    let steps = lesson.get_all_steps();
    let right_step = steps.iter().find(|s| s.title.contains("Right Algorithm"));
    assert!(right_step.is_some());

    let step = right_step.unwrap();
    assert!(step.example_moves.is_some());
    assert!(step.example_moves.as_ref().unwrap().len() > 0);
}

#[test]
fn test_lesson_has_left_algorithm_step() {
    let lesson = SecondLayerLesson::new();
    let steps = lesson.get_all_steps();
    let left_step = steps.iter().find(|s| s.title.contains("Left Algorithm"));
    assert!(left_step.is_some());

    let step = left_step.unwrap();
    assert!(step.example_moves.is_some());
    assert!(step.example_moves.as_ref().unwrap().len() > 0);
}

#[test]
fn test_right_algorithm_case() {
    let lesson = SecondLayerLesson::new();
    let right_case = lesson.get_case(0).unwrap();

    assert!(right_case.name.contains("Right"));
    assert_eq!(right_case.algorithm.len(), 8);
    assert_eq!(right_case.algorithm, vec![
        Move::U, Move::R, Move::UPrime, Move::RPrime,
        Move::UPrime, Move::FPrime, Move::U, Move::F
    ]);
}

#[test]
fn test_left_algorithm_case() {
    let lesson = SecondLayerLesson::new();
    let left_case = lesson.get_case(1).unwrap();

    assert!(left_case.name.contains("Left"));
    assert_eq!(left_case.algorithm.len(), 8);
    assert_eq!(left_case.algorithm, vec![
        Move::UPrime, Move::LPrime, Move::U, Move::L,
        Move::U, Move::F, Move::UPrime, Move::FPrime
    ]);
}

#[test]
fn test_algorithms_are_mirror_images() {
    let lesson = SecondLayerLesson::new();
    let right = lesson.get_case(0).unwrap();
    let left = lesson.get_case(1).unwrap();

    // They should be different
    assert_ne!(right.algorithm, left.algorithm);

    // Both should have the same length
    assert_eq!(right.algorithm.len(), left.algorithm.len());
}

#[test]
fn test_has_five_edge_cases() {
    let lesson = SecondLayerLesson::new();
    assert_eq!(lesson.get_all_cases().len(), 5);

    // Verify we have different cases
    let case_names: Vec<String> = lesson.get_all_cases()
        .iter()
        .map(|c| c.name.clone())
        .collect();

    assert!(case_names.iter().any(|n| n.contains("Right")));
    assert!(case_names.iter().any(|n| n.contains("Left")));
}

#[test]
fn test_practice_exercises_exist() {
    let lesson = SecondLayerLesson::new();
    assert_eq!(lesson.get_practice_exercises().len(), 3);

    for (i, exercise) in lesson.get_practice_exercises().iter().enumerate() {
        assert!(!exercise.title.is_empty(), "Exercise {} should have a title", i);
        assert!(!exercise.description.is_empty(), "Exercise {} should have a description", i);
        assert!(!exercise.hint.is_empty(), "Exercise {} should have a hint", i);
        assert!(!exercise.solution.is_empty(), "Exercise {} should have a solution", i);
    }
}

#[test]
fn test_simple_exercise_structure() {
    let lesson = SecondLayerLesson::new();
    let simple = lesson.get_practice_exercise(0).unwrap();

    assert!(simple.title.contains("Simple"));
    assert!(!simple.setup_moves.is_empty());
    assert!(!simple.solution.is_empty());
}

#[test]
fn test_verify_f2l_on_solved_cube() {
    let cube = Cube::new(3);
    assert!(SecondLayerLesson::verify_f2l(&cube), "Solved cube should have F2L complete");
}

#[test]
fn test_verify_f2l_on_scrambled_cube() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);
    cube.apply_move(Move::RPrime);
    cube.apply_move(Move::UPrime);

    assert!(!SecondLayerLesson::verify_f2l(&cube), "Scrambled cube should not have F2L complete");
}

#[test]
fn test_verify_f2l_only_works_on_3x3() {
    let cube_2x2 = Cube::new(2);
    assert!(!SecondLayerLesson::verify_f2l(&cube_2x2), "F2L verification should only work on 3x3");

    let cube_4x4 = Cube::new(4);
    assert!(!SecondLayerLesson::verify_f2l(&cube_4x4), "F2L verification should only work on 3x3");
}

#[test]
fn test_edge_positions_exist() {
    // Just verify the enum works
    let _fr = EdgePosition::FrontRight;
    let _fl = EdgePosition::FrontLeft;
    let _br = EdgePosition::BackRight;
    let _bl = EdgePosition::BackLeft;
}

#[test]
fn test_all_cases_have_positions() {
    let lesson = SecondLayerLesson::new();
    for case in lesson.get_all_cases() {
        // Each case should have a position assigned
        match case.position {
            EdgePosition::FrontRight | EdgePosition::FrontLeft |
            EdgePosition::BackRight | EdgePosition::BackLeft => {
                // Valid position
            }
        }
    }
}

#[test]
fn test_all_cases_have_explanations() {
    let lesson = SecondLayerLesson::new();
    for case in lesson.get_all_cases() {
        assert!(!case.name.is_empty());
        assert!(!case.description.is_empty());
        assert!(!case.explanation.is_empty());
    }
}

#[test]
fn test_lesson_tips_present() {
    let lesson = SecondLayerLesson::new();
    let steps_with_tips = lesson.get_all_steps()
        .iter()
        .filter(|s| s.tip.is_some())
        .count();

    assert!(steps_with_tips > 0, "At least some steps should have tips");
}

#[test]
fn test_practice_mode_complete() {
    let lesson = SecondLayerLesson::new();

    // Should have practice step
    let practice_step = lesson.get_all_steps()
        .iter()
        .find(|s| s.title.contains("Practice"));
    assert!(practice_step.is_some());

    // Should have exercises
    assert!(lesson.get_practice_exercises().len() >= 3);
}

#[test]
fn test_edge_case_with_yellow_is_special() {
    let lesson = SecondLayerLesson::new();
    let cases = lesson.get_all_cases();

    // Find the "edge has yellow" case
    let yellow_case = cases.iter().find(|c| c.name.contains("Yellow"));

    assert!(yellow_case.is_some());
    let case = yellow_case.unwrap();

    // This case should have no algorithm (skip it)
    assert_eq!(case.algorithm.len(), 0);
    assert!(case.explanation.contains("Skip"));
}

#[test]
fn test_default_trait() {
    let lesson = SecondLayerLesson::default();
    assert_eq!(lesson.step_count(), 9);
}

#[test]
fn test_get_step_out_of_bounds() {
    let lesson = SecondLayerLesson::new();
    assert!(lesson.get_step(999).is_none());
}

#[test]
fn test_get_case_out_of_bounds() {
    let lesson = SecondLayerLesson::new();
    assert!(lesson.get_case(999).is_none());
}

#[test]
fn test_get_exercise_out_of_bounds() {
    let lesson = SecondLayerLesson::new();
    assert!(lesson.get_practice_exercise(999).is_none());
}

#[test]
fn test_lesson_progression() {
    let lesson = SecondLayerLesson::new();
    let steps = lesson.get_all_steps();

    // Verify logical progression
    assert!(steps[0].title.contains("Welcome") || steps[0].title.contains("Second Layer"));

    // Should have algorithm steps
    let has_right_algo = steps.iter().any(|s| s.title.contains("Right Algorithm"));
    let has_left_algo = steps.iter().any(|s| s.title.contains("Left Algorithm"));
    assert!(has_right_algo);
    assert!(has_left_algo);

    // Should end with practice
    let last_step = steps.last().unwrap();
    assert!(last_step.title.contains("Practice"));
}

#[test]
fn test_algorithms_preserve_cube_structure() {
    let lesson = SecondLayerLesson::new();

    for case in lesson.get_all_cases() {
        if case.algorithm.is_empty() {
            continue; // Skip cases with no algorithm
        }

        let mut cube = Cube::new(3);

        // Apply the algorithm
        for mov in &case.algorithm {
            cube.apply_move(*mov);
        }

        // Verify color counts are preserved (cube structure intact)
        // Each color should still have 9 stickers
        let validation = cube.validate();
        assert!(validation.is_ok(), "Algorithm should preserve cube structure");
    }
}
