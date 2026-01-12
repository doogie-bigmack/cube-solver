//! Integration tests for R6.10 Interactive Practice Mode
//!
//! Tests the following acceptance criteria:
//! - Generate specific practice cases
//! - Hints available
//! - Check if solution is correct

use rubiks_cube_solver::cube::Move;
use rubiks_cube_solver::tutorial::practice::{
    PracticeCase, PracticeSession, PracticeGenerator, PracticeType, Difficulty
};

#[test]
fn test_practice_001_generate_specific_cases() {
    // Should be able to generate practice cases
    let case = PracticeCase::new(
        "test_case_1".to_string(),
        PracticeType::Random,
        Difficulty::Easy,
        3,
        vec![Move::R, Move::U, Move::RPrime],
        "Solve this cube".to_string(),
        "Start with the white cross".to_string(),
    );

    assert_eq!(case.id, "test_case_1");
    assert_eq!(case.cube_size, 3);
    assert_eq!(case.difficulty, Difficulty::Easy);
    assert_eq!(case.scramble.len(), 3);
    assert!(!case.description.is_empty());
    assert!(!case.hint.is_empty());
}

#[test]
fn test_practice_002_generate_pattern_case() {
    // Should support pattern-specific practice
    let case = PracticeCase::new(
        "cross_1".to_string(),
        PracticeType::Pattern("Cross".to_string()),
        Difficulty::Medium,
        3,
        vec![Move::F, Move::R, Move::U],
        "Solve the white cross".to_string(),
        "Look for white edge pieces".to_string(),
    );

    assert_eq!(case.practice_type, PracticeType::Pattern("Cross".to_string()));
}

#[test]
fn test_practice_003_generate_algorithm_case() {
    // Should support algorithm-specific practice
    let case = PracticeCase::new(
        "sune_1".to_string(),
        PracticeType::Algorithm("Sune".to_string()),
        Difficulty::Hard,
        3,
        vec![Move::R, Move::U, Move::RPrime],
        "Practice the Sune algorithm".to_string(),
        "R U R' U R U2 R'".to_string(),
    );

    assert_eq!(case.practice_type, PracticeType::Algorithm("Sune".to_string()));
}

#[test]
fn test_practice_004_hints_available() {
    // Should provide hints to students
    let cases = vec![
        PracticeCase::new(
            "1".to_string(),
            PracticeType::Random,
            Difficulty::Easy,
            2,
            vec![Move::R, Move::U],
            "Case 1".to_string(),
            "This is a helpful hint!".to_string(),
        ),
    ];

    let mut session = PracticeSession::new(cases);

    // Initially no hints used
    assert_eq!(session.hints_used, 0);

    // Get a hint
    let hint = session.get_hint();
    assert!(hint.is_some());
    assert_eq!(hint.unwrap(), "This is a helpful hint!".to_string());

    // Hints used should increment
    assert_eq!(session.hints_used, 1);
}

#[test]
fn test_practice_005_check_solution_correct() {
    // Should validate correct solutions
    let case = PracticeCase::new(
        "test".to_string(),
        PracticeType::Random,
        Difficulty::Beginner,
        2,
        vec![Move::R],
        "Undo this move".to_string(),
        "Try R'".to_string(),
    );

    // R' is the inverse of R, so it should solve the cube
    assert!(case.check_solution(&[Move::RPrime]));
}

#[test]
fn test_practice_006_check_solution_incorrect() {
    // Should detect incorrect solutions
    let case = PracticeCase::new(
        "test".to_string(),
        PracticeType::Random,
        Difficulty::Beginner,
        2,
        vec![Move::R],
        "Undo this move".to_string(),
        "Hint".to_string(),
    );

    // U doesn't solve a cube scrambled with R
    assert!(!case.check_solution(&[Move::U]));
}

#[test]
fn test_practice_007_session_tracks_progress() {
    // Should track how many cases are completed
    let cases = vec![
        PracticeCase::new(
            "1".to_string(),
            PracticeType::Random,
            Difficulty::Easy,
            2,
            vec![Move::R],
            "Case 1".to_string(),
            "Hint 1".to_string(),
        ),
        PracticeCase::new(
            "2".to_string(),
            PracticeType::Random,
            Difficulty::Easy,
            2,
            vec![Move::U],
            "Case 2".to_string(),
            "Hint 2".to_string(),
        ),
    ];

    let mut session = PracticeSession::new(cases);

    // Initially no cases completed
    assert_eq!(session.cases_completed, 0);
    assert_eq!(session.completion_percentage(), 0.0);

    // Solve the first case
    session.check_solution(&[Move::RPrime]);
    assert_eq!(session.cases_completed, 1);
    assert_eq!(session.completion_percentage(), 50.0);

    // Move to next case
    session.next_case();
    session.check_solution(&[Move::UPrime]);
    assert_eq!(session.cases_completed, 2);
    assert_eq!(session.completion_percentage(), 100.0);
}

#[test]
fn test_practice_008_session_navigation() {
    // Should allow navigation between cases
    let cases = vec![
        PracticeCase::new("1".to_string(), PracticeType::Random, Difficulty::Easy, 2, vec![Move::R], "Case 1".to_string(), "Hint 1".to_string()),
        PracticeCase::new("2".to_string(), PracticeType::Random, Difficulty::Easy, 2, vec![Move::U], "Case 2".to_string(), "Hint 2".to_string()),
        PracticeCase::new("3".to_string(), PracticeType::Random, Difficulty::Easy, 2, vec![Move::F], "Case 3".to_string(), "Hint 3".to_string()),
    ];

    let mut session = PracticeSession::new(cases);

    assert_eq!(session.current_index, 0);
    assert!(session.current_case().is_some());
    assert_eq!(session.current_case().unwrap().id, "1");

    // Move to next
    assert!(session.next_case());
    assert_eq!(session.current_index, 1);
    assert_eq!(session.current_case().unwrap().id, "2");

    // Move to next
    assert!(session.next_case());
    assert_eq!(session.current_index, 2);
    assert_eq!(session.current_case().unwrap().id, "3");

    // No more cases
    assert!(!session.next_case());
}

#[test]
fn test_practice_009_difficulty_levels() {
    // Should support multiple difficulty levels
    assert_eq!(Difficulty::Beginner.name(), "Beginner");
    assert_eq!(Difficulty::Easy.name(), "Easy");
    assert_eq!(Difficulty::Medium.name(), "Medium");
    assert_eq!(Difficulty::Hard.name(), "Hard");
    assert_eq!(Difficulty::Expert.name(), "Expert");
}

#[test]
fn test_practice_010_scramble_length_varies_by_difficulty() {
    // Scramble length should correspond to difficulty
    let (_min_b, max_b) = Difficulty::Beginner.scramble_length_range();
    let (min_e, max_e) = Difficulty::Easy.scramble_length_range();
    let (min_m, max_m) = Difficulty::Medium.scramble_length_range();
    let (min_h, max_h) = Difficulty::Hard.scramble_length_range();
    let (min_x, _max_x) = Difficulty::Expert.scramble_length_range();

    // Each difficulty should have increasing ranges (overlaps are allowed)
    assert!(max_b <= min_e);
    assert!(max_e <= min_m);
    assert!(max_m <= min_h);
    assert!(max_h <= min_x);
}

#[test]
fn test_practice_011_generate_random_cases() {
    // Should generate random practice cases
    let case = PracticeGenerator::generate_random_case(2, Difficulty::Medium);

    assert_eq!(case.cube_size, 2);
    assert_eq!(case.difficulty, Difficulty::Medium);
    assert!(!case.scramble.is_empty());
    assert!(!case.description.is_empty());
    assert!(!case.hint.is_empty());

    // Scramble should be in medium range
    let (min, max) = Difficulty::Medium.scramble_length_range();
    assert!(case.scramble.len() >= min && case.scramble.len() <= max);
}

#[test]
fn test_practice_012_generate_session() {
    // Should generate a practice session with multiple cases
    let session = PracticeGenerator::generate_session(2, Difficulty::Easy, 5);

    assert_eq!(session.total_cases(), 5);
    assert_eq!(session.cases.len(), 5);

    for case in &session.cases {
        assert_eq!(case.cube_size, 2);
        assert_eq!(case.difficulty, Difficulty::Easy);
    }
}

#[test]
fn test_practice_013_session_tracks_attempts() {
    // Should track total attempts made
    let cases = vec![
        PracticeCase::new("1".to_string(), PracticeType::Random, Difficulty::Easy, 2, vec![Move::R], "Case 1".to_string(), "Hint".to_string()),
    ];

    let mut session = PracticeSession::new(cases);

    assert_eq!(session.total_attempts, 0);

    // Make an incorrect attempt
    session.check_solution(&[Move::U]);
    assert_eq!(session.total_attempts, 1);

    // Make a correct attempt
    session.check_solution(&[Move::RPrime]);
    assert_eq!(session.total_attempts, 2);
}

#[test]
fn test_practice_014_case_has_scrambled_cube() {
    // Should provide the scrambled cube state
    let case = PracticeCase::new(
        "test".to_string(),
        PracticeType::Random,
        Difficulty::Easy,
        2,
        vec![Move::R, Move::U],
        "Test".to_string(),
        "Hint".to_string(),
    );

    let cube = case.get_scrambled_cube();
    assert_eq!(cube.size(), 2);
    assert!(!cube.is_solved()); // Should be scrambled
}

#[test]
fn test_practice_015_case_with_expected_solution() {
    // Should support cases with expected solutions
    let case = PracticeCase::new(
        "test".to_string(),
        PracticeType::Random,
        Difficulty::Easy,
        2,
        vec![Move::R],
        "Test".to_string(),
        "Hint".to_string(),
    ).with_solution(vec![Move::RPrime]);

    assert_eq!(case.expected_solution, vec![Move::RPrime]);
}

#[test]
fn test_practice_016_case_with_target_description() {
    // Should support target state descriptions
    let case = PracticeCase::new(
        "test".to_string(),
        PracticeType::Pattern("Cross".to_string()),
        Difficulty::Medium,
        3,
        vec![Move::R, Move::U],
        "Test".to_string(),
        "Hint".to_string(),
    ).with_target("Solve the white cross".to_string());

    assert!(case.target_description.is_some());
    assert_eq!(case.target_description.unwrap(), "Solve the white cross");
}

#[test]
fn test_practice_017_session_is_complete() {
    // Should detect when all cases are completed
    let cases = vec![
        PracticeCase::new("1".to_string(), PracticeType::Random, Difficulty::Easy, 2, vec![Move::R], "Case 1".to_string(), "Hint".to_string()),
        PracticeCase::new("2".to_string(), PracticeType::Random, Difficulty::Easy, 2, vec![Move::U], "Case 2".to_string(), "Hint".to_string()),
    ];

    let mut session = PracticeSession::new(cases);

    assert!(!session.is_complete());

    session.next_case();
    assert!(session.is_complete()); // At last case
}

#[test]
fn test_practice_018_generate_solution_2x2() {
    // Should generate solutions for 2x2 cases
    let case = PracticeCase::new(
        "test".to_string(),
        PracticeType::Random,
        Difficulty::Easy,
        2,
        vec![Move::R, Move::U],
        "Test".to_string(),
        "Hint".to_string(),
    );

    let result = case.generate_solution();
    assert!(result.is_ok());

    let solution = result.unwrap();
    assert!(!solution.is_empty());

    // Verify the solution actually works
    assert!(case.check_solution(&solution));
}

#[test]
fn test_practice_019_generate_solution_3x3() {
    // Should generate solutions for 3x3 cases (simple scrambles)
    let case = PracticeCase::new(
        "test".to_string(),
        PracticeType::Random,
        Difficulty::Beginner,
        3,
        vec![Move::R, Move::U],
        "Test".to_string(),
        "Hint".to_string(),
    );

    let result = case.generate_solution();
    assert!(result.is_ok());

    let solution = result.unwrap();
    assert!(!solution.is_empty());

    // Verify the solution actually works
    assert!(case.check_solution(&solution));
}

#[test]
fn test_practice_020_multiple_hint_requests() {
    // Should allow multiple hint requests
    let cases = vec![
        PracticeCase::new("1".to_string(), PracticeType::Random, Difficulty::Easy, 2, vec![Move::R], "Case".to_string(), "Helpful hint".to_string()),
    ];

    let mut session = PracticeSession::new(cases);

    // Request hint multiple times
    session.get_hint();
    session.get_hint();
    session.get_hint();

    assert_eq!(session.hints_used, 3);
}
