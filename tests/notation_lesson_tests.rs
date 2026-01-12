use rubiks_cube_solver::cube::Move;
use rubiks_cube_solver::tutorial::{LessonStep, NotationLesson};

// Test that notation lesson has correct number of steps
#[test]
fn test_notation_lesson_001_has_all_steps() {
    let lesson = NotationLesson::new();
    assert_eq!(lesson.step_count(), 11);
}

// Test that intro step exists and has content
#[test]
fn test_notation_lesson_002_intro_step() {
    let lesson = NotationLesson::new();
    let step = lesson.get_step(0).expect("Intro step should exist");

    assert!(step.title.contains("Welcome"));
    assert!(!step.description.is_empty());
    assert!(!step.kid_friendly_text.is_empty());
    assert_eq!(step.example_move, None);
}

// Test that all six face steps exist
#[test]
fn test_notation_lesson_003_all_faces_have_steps() {
    let lesson = NotationLesson::new();
    let steps = lesson.get_all_steps();

    let titles: Vec<String> = steps.iter().map(|s| s.title.clone()).collect();
    let all_titles = titles.join(" ");

    assert!(all_titles.contains("Right"));
    assert!(all_titles.contains("Left"));
    assert!(all_titles.contains("Up"));
    assert!(all_titles.contains("Down"));
    assert!(all_titles.contains("Front"));
    assert!(all_titles.contains("Back"));
}

// Test that R face step has correct move example
#[test]
fn test_notation_lesson_004_r_face_example() {
    let lesson = NotationLesson::new();
    let r_step = lesson.get_step(2).expect("R face step should exist");

    assert!(r_step.title.contains("Right"));
    assert_eq!(r_step.example_move, Some(Move::R));
    assert!(r_step.description.contains("clockwise"));
}

// Test that L face step has correct move example
#[test]
fn test_notation_lesson_005_l_face_example() {
    let lesson = NotationLesson::new();
    let l_step = lesson.get_step(3).expect("L face step should exist");

    assert!(l_step.title.contains("Left"));
    assert_eq!(l_step.example_move, Some(Move::L));
}

// Test that U face step has correct move example
#[test]
fn test_notation_lesson_006_u_face_example() {
    let lesson = NotationLesson::new();
    let u_step = lesson.get_step(4).expect("U face step should exist");

    assert!(u_step.title.contains("Up"));
    assert_eq!(u_step.example_move, Some(Move::U));
}

// Test that D face step has correct move example
#[test]
fn test_notation_lesson_007_d_face_example() {
    let lesson = NotationLesson::new();
    let d_step = lesson.get_step(5).expect("D face step should exist");

    assert!(d_step.title.contains("Down"));
    assert_eq!(d_step.example_move, Some(Move::D));
}

// Test that F face step has correct move example
#[test]
fn test_notation_lesson_008_f_face_example() {
    let lesson = NotationLesson::new();
    let f_step = lesson.get_step(6).expect("F face step should exist");

    assert!(f_step.title.contains("Front"));
    assert_eq!(f_step.example_move, Some(Move::F));
}

// Test that B face step has correct move example
#[test]
fn test_notation_lesson_009_b_face_example() {
    let lesson = NotationLesson::new();
    let b_step = lesson.get_step(7).expect("B face step should exist");

    assert!(b_step.title.contains("Back"));
    assert_eq!(b_step.example_move, Some(Move::B));
}

// Test that prime moves are explained
#[test]
fn test_notation_lesson_010_prime_moves() {
    let lesson = NotationLesson::new();
    let prime_step = lesson.get_step(8).expect("Prime moves step should exist");

    assert!(prime_step.title.contains("Prime"));
    assert_eq!(prime_step.example_move, Some(Move::RPrime));
    assert!(prime_step.description.contains("counter-clockwise") || prime_step.description.contains("opposite"));
}

// Test that double moves are explained
#[test]
fn test_notation_lesson_011_double_moves() {
    let lesson = NotationLesson::new();
    let double_step = lesson.get_step(9).expect("Double moves step should exist");

    assert!(double_step.title.contains("Double"));
    assert_eq!(double_step.example_move, Some(Move::R2));
    assert!(double_step.description.contains("180") || double_step.description.contains("twice"));
}

// Test that practice step exists
#[test]
fn test_notation_lesson_012_practice_step() {
    let lesson = NotationLesson::new();
    let practice_step = lesson.get_step(10).expect("Practice step should exist");

    assert!(practice_step.title.contains("Practice"));
    assert!(!practice_step.description.is_empty());
}

// Test that all steps have kid-friendly text
#[test]
fn test_notation_lesson_013_all_kid_friendly() {
    let lesson = NotationLesson::new();

    for (i, step) in lesson.get_all_steps().iter().enumerate() {
        assert!(
            !step.kid_friendly_text.is_empty(),
            "Step {} should have kid-friendly text",
            i
        );
    }
}

// Test that kid-friendly text differs from description
#[test]
fn test_notation_lesson_014_kid_friendly_differs() {
    let lesson = NotationLesson::new();

    for (i, step) in lesson.get_all_steps().iter().enumerate() {
        assert_ne!(
            step.kid_friendly_text, step.description,
            "Step {} kid-friendly text should differ from description",
            i
        );
    }
}

// Test lesson step cloning
#[test]
fn test_notation_lesson_015_step_cloning() {
    let lesson = NotationLesson::new();
    let step = lesson.get_step(0).expect("First step should exist");
    let cloned_step = step.clone();

    assert_eq!(step.title, cloned_step.title);
    assert_eq!(step.description, cloned_step.description);
    assert_eq!(step.kid_friendly_text, cloned_step.kid_friendly_text);
    assert_eq!(step.example_move, cloned_step.example_move);
}

// Test lesson cloning
#[test]
fn test_notation_lesson_016_lesson_cloning() {
    let lesson = NotationLesson::new();
    let cloned_lesson = lesson.clone();

    assert_eq!(lesson.step_count(), cloned_lesson.step_count());
}

// Test default trait implementation
#[test]
fn test_notation_lesson_017_default_trait() {
    let lesson1 = NotationLesson::new();
    let lesson2 = NotationLesson::default();

    assert_eq!(lesson1.step_count(), lesson2.step_count());
}

// Test that steps are in logical order
#[test]
fn test_notation_lesson_018_logical_order() {
    let lesson = NotationLesson::new();
    let steps = lesson.get_all_steps();

    // First step should be intro
    assert!(steps[0].title.contains("Welcome"));

    // Face names should come before individual faces
    assert!(steps[1].title.contains("Six Faces"));

    // Individual faces (steps 2-7)
    assert!(steps[2].title.contains("Right"));
    assert!(steps[3].title.contains("Left"));
    assert!(steps[4].title.contains("Up"));
    assert!(steps[5].title.contains("Down"));
    assert!(steps[6].title.contains("Front"));
    assert!(steps[7].title.contains("Back"));

    // Prime and double moves after basic faces
    assert!(steps[8].title.contains("Prime"));
    assert!(steps[9].title.contains("Double"));

    // Practice at the end
    assert!(steps[10].title.contains("Practice"));
}

// Test boundary conditions for get_step
#[test]
fn test_notation_lesson_019_get_step_bounds() {
    let lesson = NotationLesson::new();

    // Valid indices
    assert!(lesson.get_step(0).is_some());
    assert!(lesson.get_step(10).is_some());

    // Invalid indices
    assert!(lesson.get_step(11).is_none());
    assert!(lesson.get_step(100).is_none());
}

// Test that example moves are valid Move enum variants
#[test]
fn test_notation_lesson_020_valid_moves() {
    let lesson = NotationLesson::new();

    for step in lesson.get_all_steps() {
        if let Some(mv) = step.example_move {
            // If a move exists, it should be a valid Move variant
            // Just checking that it's one of the expected moves
            assert!(
                matches!(
                    mv,
                    Move::R
                        | Move::L
                        | Move::U
                        | Move::D
                        | Move::F
                        | Move::B
                        | Move::RPrime
                        | Move::R2
                ),
                "Move should be a basic face move"
            );
        }
    }
}
