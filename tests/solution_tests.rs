//! Integration tests for step-by-step solution generation (R5.6)
//!
//! Tests the Solution and SolutionStep structures to ensure they meet
//! the acceptance criteria:
//! - Break solution into logical steps
//! - Each step has description
//! - Steps correspond to solving phases

use rubiks_cube_solver::cube::{Cube, Move};
use rubiks_cube_solver::solver::{
    solve_2x2, solve_3x3, Solution, SolutionStep,
};

#[test]
fn test_solution_step_has_description() {
    let step = SolutionStep::new("Solve bottom layer", vec![Move::R, Move::U]);

    assert!(!step.description.is_empty(), "Step should have a description");
    assert_eq!(step.description, "Solve bottom layer");
}

#[test]
fn test_solution_step_has_moves() {
    let step = SolutionStep::new("Solve bottom layer", vec![Move::R, Move::U]);

    assert_eq!(step.moves.len(), 2, "Step should contain moves");
    assert_eq!(step.moves[0], Move::R);
    assert_eq!(step.moves[1], Move::U);
}

#[test]
fn test_solution_step_with_explanation() {
    let step = SolutionStep::with_explanation(
        "Position corners",
        vec![Move::R, Move::UPrime],
        "This algorithm positions the bottom corners in their correct locations"
    );

    assert_eq!(step.description, "Position corners");
    assert!(step.explanation.is_some());
    assert_eq!(
        step.explanation.unwrap(),
        "This algorithm positions the bottom corners in their correct locations"
    );
}

#[test]
fn test_solution_has_steps() {
    let steps = vec![
        SolutionStep::new("Step 1: White cross", vec![Move::R, Move::U]),
        SolutionStep::new("Step 2: White corners", vec![Move::F, Move::D]),
        SolutionStep::new("Step 3: Second layer", vec![Move::R, Move::UPrime]),
    ];

    let solution = Solution::with_method(steps.clone(), 100, "Beginner's Method");

    assert_eq!(solution.step_count(), 3, "Solution should have 3 steps");
    assert_eq!(solution.steps[0].description, "Step 1: White cross");
    assert_eq!(solution.steps[1].description, "Step 2: White corners");
    assert_eq!(solution.steps[2].description, "Step 3: Second layer");
}

#[test]
fn test_solution_all_moves() {
    let steps = vec![
        SolutionStep::new("Step 1", vec![Move::R, Move::U]),
        SolutionStep::new("Step 2", vec![Move::F]),
        SolutionStep::new("Step 3", vec![Move::D, Move::L]),
    ];

    let solution = Solution::new(steps, 100);
    let all_moves = solution.all_moves();

    assert_eq!(all_moves.len(), 5, "All moves should be flattened");
    assert_eq!(all_moves, vec![Move::R, Move::U, Move::F, Move::D, Move::L]);
}

#[test]
fn test_solution_move_count() {
    let steps = vec![
        SolutionStep::new("Step 1", vec![Move::R, Move::U]),
        SolutionStep::new("Step 2", vec![Move::F, Move::D, Move::L]),
    ];

    let solution = Solution::new(steps, 100);

    assert_eq!(solution.move_count(), 5);
}

#[test]
fn test_solution_to_notation() {
    let steps = vec![
        SolutionStep::new("Step 1", vec![Move::R, Move::U]),
        SolutionStep::new("Step 2", vec![Move::RPrime, Move::UPrime]),
    ];

    let solution = Solution::new(steps, 100);

    assert_eq!(solution.to_notation(), "R U R' U'");
}

#[test]
fn test_solution_summary() {
    let steps = vec![
        SolutionStep::new("Step 1", vec![Move::R]),
        SolutionStep::new("Step 2", vec![Move::U, Move::F]),
    ];

    let solution = Solution::with_method(steps, 150, "Test Method");
    let summary = solution.summary();

    assert!(summary.contains("Test Method"), "Summary should include method name");
    assert!(summary.contains("2 steps"), "Summary should include step count");
    assert!(summary.contains("3 moves"), "Summary should include move count");
    assert!(summary.contains("150ms"), "Summary should include time");
}

#[test]
fn test_2x2_solver_has_steps() {
    let mut cube = Cube::new(2);
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);

    let solution = solve_2x2(&cube).expect("Should solve");

    assert!(!solution.steps.is_empty(), "2x2 solution should have steps");

    // Verify each step has a description
    for step in &solution.steps {
        assert!(!step.description.is_empty(), "Each step should have a description");
    }
}

#[test]
fn test_2x2_to_generic_solution() {
    let mut cube = Cube::new(2);
    cube.apply_move(Move::R);

    let solution_2x2 = solve_2x2(&cube).expect("Should solve");
    let solution = solution_2x2.to_solution();

    assert_eq!(solution.move_count(), solution_2x2.move_count());
    assert_eq!(solution.time_ms, solution_2x2.time_ms);
    assert!(solution.method.is_some());
    assert_eq!(solution.method.unwrap(), "2x2 Depth-Limited Search");
}

#[test]
fn test_3x3_solver_has_steps() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);

    let solution = solve_3x3(&cube).expect("Should solve");

    assert!(!solution.steps.is_empty(), "3x3 solution should have steps");

    // Verify each step has a description
    for step in &solution.steps {
        assert!(!step.description.is_empty(), "Each step should have a description");
    }
}

#[test]
fn test_3x3_to_generic_solution() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::R);

    let solution_3x3 = solve_3x3(&cube).expect("Should solve");
    let solution = solution_3x3.to_solution();

    assert_eq!(solution.move_count(), solution_3x3.move_count());
    assert_eq!(solution.time_ms, solution_3x3.time_ms);
    assert!(solution.method.is_some());
    assert_eq!(solution.method.unwrap(), "Beginner's Layer-by-Layer Method");
}

#[test]
fn test_solution_step_move_count() {
    let step1 = SolutionStep::new("Empty step", vec![]);
    let step2 = SolutionStep::new("Single move", vec![Move::R]);
    let step3 = SolutionStep::new("Multiple moves", vec![Move::R, Move::U, Move::F]);

    assert_eq!(step1.move_count(), 0);
    assert_eq!(step2.move_count(), 1);
    assert_eq!(step3.move_count(), 3);
}

#[test]
fn test_solution_step_to_notation() {
    let step = SolutionStep::new(
        "Sexy move",
        vec![Move::R, Move::U, Move::RPrime, Move::UPrime]
    );

    assert_eq!(step.to_notation(), "R U R' U'");
}

#[test]
fn test_empty_solution_steps() {
    let cube = Cube::new(2);
    let solution = solve_2x2(&cube).expect("Should solve already-solved cube");

    // Solved cube should have a step indicating it's already solved
    assert_eq!(solution.step_count(), 1);
    assert!(solution.steps[0].description.contains("already solved") ||
            solution.steps[0].description.contains("Cube is already solved"));
}

#[test]
fn test_solution_with_multiple_phases() {
    // Create a multi-phase solution to test step breakdown
    let steps = vec![
        SolutionStep::with_explanation(
            "Phase 1: White cross",
            vec![Move::F, Move::R, Move::U],
            "Position the white edge pieces to form a cross on the bottom"
        ),
        SolutionStep::with_explanation(
            "Phase 2: White corners",
            vec![Move::R, Move::U, Move::RPrime, Move::UPrime],
            "Insert the white corner pieces to complete the first layer"
        ),
        SolutionStep::with_explanation(
            "Phase 3: Second layer",
            vec![Move::U, Move::R, Move::UPrime, Move::RPrime],
            "Solve the middle layer edge pieces"
        ),
    ];

    let solution = Solution::with_method(steps, 200, "Layer-by-Layer");

    // Verify acceptance criteria
    assert_eq!(solution.step_count(), 3, "Solution should be broken into logical steps");

    for step in &solution.steps {
        assert!(!step.description.is_empty(), "Each step must have a description");
        assert!(step.description.contains("Phase"), "Steps should correspond to solving phases");
        assert!(step.explanation.is_some(), "Steps should have explanations for educational purposes");
    }

    assert!(solution.method.is_some(), "Solution should indicate the solving method used");
}
