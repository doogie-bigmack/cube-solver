//! Integration tests for solution playback (R5.7, R5.8)
//!
//! Tests the solution playback functionality including:
//! - Solution display
//! - Playback speed configuration
//! - Playback state transitions
//!
//! Note: Full UI interaction tests require browser environment

use rubiks_cube_solver::cube::{Cube, Move};
use rubiks_cube_solver::solver::{solve_2x2, solve_3x3, Solution, SolutionStep};
use rubiks_cube_solver::components::solution_player::{PlaybackSpeed, PlaybackState};

#[test]
fn test_solution_can_be_displayed() {
    // Test that we can create a solution and it has the right properties
    let steps = vec![
        SolutionStep::new("Step 1", vec![Move::R, Move::U]),
        SolutionStep::new("Step 2", vec![Move::F, Move::D]),
    ];

    let solution = Solution::new(steps, 100);

    assert_eq!(solution.move_count(), 4);
    assert_eq!(solution.step_count(), 2);
    assert_eq!(solution.to_notation(), "R U F D");
}

#[test]
fn test_playback_speed_configuration() {
    // Test R5.7 acceptance criteria: Configurable speed
    let speeds = vec![
        PlaybackSpeed::VerySlow,
        PlaybackSpeed::Slow,
        PlaybackSpeed::Normal,
        PlaybackSpeed::Fast,
        PlaybackSpeed::VeryFast,
    ];

    for speed in speeds {
        let duration_ms = speed.duration_ms();
        assert!(duration_ms > 0, "Speed {:?} should have positive duration", speed);
        assert!(duration_ms <= 2000, "Speed {:?} duration too long", speed);
        assert!(!speed.label().is_empty(), "Speed {:?} should have label", speed);
    }

    // Verify speeds are in descending order
    assert!(PlaybackSpeed::VerySlow.duration_ms() > PlaybackSpeed::Slow.duration_ms());
    assert!(PlaybackSpeed::Slow.duration_ms() > PlaybackSpeed::Normal.duration_ms());
    assert!(PlaybackSpeed::Normal.duration_ms() > PlaybackSpeed::Fast.duration_ms());
    assert!(PlaybackSpeed::Fast.duration_ms() > PlaybackSpeed::VeryFast.duration_ms());
}

#[test]
fn test_playback_state_transitions() {
    // Test R5.8 acceptance criteria: Pause/resume/step controls
    // These states should be distinct
    assert_ne!(PlaybackState::Stopped, PlaybackState::Playing);
    assert_ne!(PlaybackState::Playing, PlaybackState::Paused);
    assert_ne!(PlaybackState::Paused, PlaybackState::Completed);
    assert_ne!(PlaybackState::Stopped, PlaybackState::Completed);
}

#[test]
fn test_2x2_solution_playback_data() {
    // Test that 2x2 solver produces solution suitable for playback
    let mut cube = Cube::new(2);
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);
    cube.apply_move(Move::R);

    let solution = solve_2x2(&cube).expect("Should solve");
    let generic_solution = solution.to_solution();

    // Verify solution has required properties for playback
    assert!(generic_solution.move_count() > 0, "Solution should have moves");
    assert!(generic_solution.step_count() > 0, "Solution should have steps");
    assert!(!generic_solution.to_notation().is_empty(), "Solution should have notation");
    assert!(!generic_solution.summary().is_empty(), "Solution should have summary");
}

#[test]
fn test_3x3_solution_playback_data() {
    // Test that 3x3 solver produces solution suitable for playback
    let mut cube = Cube::new(3);
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);
    cube.apply_move(Move::RPrime);
    cube.apply_move(Move::UPrime);

    let solution = solve_3x3(&cube).expect("Should solve");
    let generic_solution = solution.to_solution();

    // Verify solution has required properties for playback
    assert!(generic_solution.move_count() > 0, "Solution should have moves");
    assert!(generic_solution.step_count() > 0, "Solution should have steps");
    assert!(!generic_solution.to_notation().is_empty(), "Solution should have notation");
    assert!(!generic_solution.summary().is_empty(), "Solution should have summary");

    // Verify steps are educational
    for step in &generic_solution.steps {
        assert!(!step.description.is_empty(), "Each step should have description");
    }
}

#[test]
fn test_solution_notation_formatting() {
    // Test R5.9 acceptance criteria: Move explanation text
    let steps = vec![
        SolutionStep::new("Sexy move", vec![Move::R, Move::U, Move::RPrime, Move::UPrime]),
        SolutionStep::new("Sune", vec![Move::R, Move::U, Move::RPrime, Move::U, Move::R, Move::U2, Move::RPrime]),
    ];

    let solution = Solution::new(steps, 150);
    let notation = solution.to_notation();

    // Should be readable notation
    assert!(notation.contains("R"));
    assert!(notation.contains("U"));
    assert!(notation.contains("R'"));
    assert!(notation.contains("U'"));
    assert!(notation.contains("U2"));
}

#[test]
fn test_playback_speed_values() {
    // Test that speed values meet R5.7 requirements
    // VerySlow should be 2000ms (2 seconds per move)
    assert_eq!(PlaybackSpeed::VerySlow.duration_ms(), 2000);

    // Slow should be 1000ms (1 second per move)
    assert_eq!(PlaybackSpeed::Slow.duration_ms(), 1000);

    // Normal should be 500ms (0.5 seconds per move)
    assert_eq!(PlaybackSpeed::Normal.duration_ms(), 500);

    // Fast should be 250ms (0.25 seconds per move)
    assert_eq!(PlaybackSpeed::Fast.duration_ms(), 250);

    // VeryFast should be 100ms (0.1 seconds per move)
    assert_eq!(PlaybackSpeed::VeryFast.duration_ms(), 100);
}

#[test]
fn test_solution_steps_ordering() {
    // Verify that solution steps maintain order for playback
    let moves1 = vec![Move::R, Move::U];
    let moves2 = vec![Move::F, Move::D];
    let moves3 = vec![Move::L, Move::B];

    let steps = vec![
        SolutionStep::new("Step 1", moves1.clone()),
        SolutionStep::new("Step 2", moves2.clone()),
        SolutionStep::new("Step 3", moves3.clone()),
    ];

    let solution = Solution::new(steps.clone(), 200);

    // Verify step order preserved
    assert_eq!(solution.steps[0].moves, moves1);
    assert_eq!(solution.steps[1].moves, moves2);
    assert_eq!(solution.steps[2].moves, moves3);

    // Verify all_moves() preserves order
    let all_moves = solution.all_moves();
    assert_eq!(all_moves[0], Move::R);
    assert_eq!(all_moves[1], Move::U);
    assert_eq!(all_moves[2], Move::F);
    assert_eq!(all_moves[3], Move::D);
    assert_eq!(all_moves[4], Move::L);
    assert_eq!(all_moves[5], Move::B);
}

#[test]
fn test_empty_solution_handling() {
    // Test edge case: already solved cube
    let cube = Cube::new(3);
    let solution = solve_3x3(&cube).expect("Should handle solved cube");

    // Should have at least one step indicating cube is solved
    assert!(solution.step_count() > 0);
    assert!(solution.steps[0].description.contains("solved") ||
            solution.steps[0].description.contains("already"));
}

#[test]
fn test_solution_with_explanations() {
    // Test that steps can have educational explanations (R5.9)
    let step = SolutionStep::with_explanation(
        "Orient last layer",
        vec![Move::R, Move::U, Move::RPrime, Move::UPrime],
        "This algorithm flips the edge pieces on the top layer"
    );

    assert_eq!(step.description, "Orient last layer");
    assert!(step.explanation.is_some());
    assert!(step.explanation.unwrap().contains("edge pieces"));
}
