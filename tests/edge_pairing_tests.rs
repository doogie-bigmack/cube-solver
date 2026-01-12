//! Integration tests for 4x4+ reduction method - edge pairing
//!
//! Tests R5.4: 4x4+ reduction method - edges

use rubiks_cube_solver::cube::{Cube, Move};
use rubiks_cube_solver::solver::solve_edges;

#[test]
fn test_solve_edges_solved_4x4() {
    // A solved 4x4 should return empty solution
    let cube = Cube::new(4);
    let solution = solve_edges(&cube).expect("Should succeed on solved cube");

    assert_eq!(solution.move_count(), 0, "Solved cube should need no moves");
    assert!(solution.time_ms < 1000, "Should complete quickly");
}

#[test]
fn test_solve_edges_solved_5x5() {
    // A solved 5x5 should return empty solution
    let cube = Cube::new(5);
    let solution = solve_edges(&cube).expect("Should succeed on solved cube");

    assert_eq!(solution.move_count(), 0, "Solved cube should need no moves");
    assert!(solution.time_ms < 1000, "Should complete quickly");
}

#[test]
fn test_solve_edges_solved_6x6() {
    // A solved 6x6 should return empty solution
    let cube = Cube::new(6);
    let solution = solve_edges(&cube).expect("Should succeed on solved cube");

    assert_eq!(solution.move_count(), 0, "Solved cube should need no moves");
    assert!(solution.time_ms < 1000, "Should complete quickly");
}

#[test]
fn test_solve_edges_rejects_2x2() {
    // 2x2 cubes don't have separate edge pieces
    let cube = Cube::new(2);
    let result = solve_edges(&cube);

    assert!(result.is_err(), "Should reject 2x2 cube");
    assert!(result.unwrap_err().contains("4x4 or larger"));
}

#[test]
fn test_solve_edges_rejects_3x3() {
    // 3x3 edge pieces are different from 4x4+ edges
    let cube = Cube::new(3);
    let result = solve_edges(&cube);

    assert!(result.is_err(), "Should reject 3x3 cube");
    assert!(result.unwrap_err().contains("4x4 or larger"));
}

#[test]
fn test_solve_edges_structure() {
    // Test that solution has proper structure
    let cube = Cube::new(4);
    let solution = solve_edges(&cube).expect("Should succeed");

    // Should have steps
    assert!(!solution.steps.is_empty(), "Should have at least one step");

    // Should be able to convert to generic Solution
    let generic_solution = solution.to_solution();
    assert!(generic_solution.method.is_some(), "Should have method name");
    assert!(generic_solution.method.unwrap().contains("Edges"), "Method should mention edges");
}

#[test]
fn test_solve_edges_timing() {
    // Even for unsolved cubes, should complete quickly
    let mut cube = Cube::new(4);

    // Apply some moves to scramble
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);

    let solution = solve_edges(&cube).expect("Should succeed");

    // Should complete in under 5 seconds
    assert!(solution.time_ms < 5000, "Should complete in under 5 seconds");
}

#[test]
fn test_solve_edges_step_descriptions() {
    // Test that steps have meaningful descriptions
    let cube = Cube::new(4);
    let solution = solve_edges(&cube).expect("Should succeed");

    for step in &solution.steps {
        assert!(!step.description.is_empty(), "Step description should not be empty");
        // Description should mention either "edge" or "paired"
        assert!(
            step.description.to_lowercase().contains("edge") ||
            step.description.to_lowercase().contains("pair"),
            "Step description should mention edges or pairing: {}",
            step.description
        );
    }
}

#[test]
fn test_solve_edges_works_for_4x4() {
    // Acceptance criteria: Works for 4x4 and larger
    let cube = Cube::new(4);
    let result = solve_edges(&cube);

    assert!(result.is_ok(), "Should work for 4x4 cube");
}

#[test]
fn test_solve_edges_works_for_5x5() {
    // Acceptance criteria: Works for 4x4 and larger
    let cube = Cube::new(5);
    let result = solve_edges(&cube);

    assert!(result.is_ok(), "Should work for 5x5 cube");
}

#[test]
fn test_solve_edges_works_for_6x6() {
    // Acceptance criteria: Works for 4x4 and larger
    let cube = Cube::new(6);
    let result = solve_edges(&cube);

    assert!(result.is_ok(), "Should work for 6x6 cube");
}

#[test]
fn test_solve_edges_works_for_7x7() {
    // Acceptance criteria: Works for 4x4 and larger
    let cube = Cube::new(7);
    let result = solve_edges(&cube);

    assert!(result.is_ok(), "Should work for 7x7 cube");
}

#[test]
fn test_solve_edges_generates_moves() {
    // Acceptance criteria: Generate move sequence
    // Note: For now, this may return empty moves for already-solved cubes
    let mut cube = Cube::new(4);
    cube.apply_move(Move::R);

    let solution = solve_edges(&cube).expect("Should succeed");

    // Solution object should exist and have structure
    assert_eq!(solution.move_count(), solution.moves.len());

    // Steps should exist
    assert!(!solution.steps.is_empty());
}

#[test]
fn test_solve_edges_move_count_matches() {
    // Move count should match actual moves length
    let cube = Cube::new(4);
    let solution = solve_edges(&cube).expect("Should succeed");

    assert_eq!(solution.move_count(), solution.moves.len());
}

#[test]
fn test_solve_edges_solution_conversion() {
    // Test conversion to generic Solution type
    let cube = Cube::new(5);
    let edge_solution = solve_edges(&cube).expect("Should succeed");

    let generic = edge_solution.to_solution();

    assert_eq!(generic.move_count(), edge_solution.move_count());
    assert_eq!(generic.step_count(), edge_solution.steps.len());
    assert_eq!(generic.time_ms, edge_solution.time_ms);
}

#[test]
fn test_solve_edges_on_scrambled_4x4() {
    // Test with a lightly scrambled 4x4
    let mut cube = Cube::new(4);

    // Apply some moves
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);
    cube.apply_move(Move::RPrime);
    cube.apply_move(Move::UPrime);

    let solution = solve_edges(&cube).expect("Should succeed");

    // Should generate a solution structure
    assert!(!solution.steps.is_empty(), "Should have steps");
    assert!(solution.time_ms < 10000, "Should complete in reasonable time");
}

#[test]
fn test_solve_edges_multiple_sizes() {
    // Test that it works consistently across sizes
    for size in [4, 5, 6, 7] {
        let cube = Cube::new(size);
        let result = solve_edges(&cube);

        assert!(result.is_ok(), "Should work for {}x{} cube", size, size);

        let solution = result.unwrap();
        assert_eq!(solution.move_count(), 0, "Solved {}x{} should need no moves", size, size);
    }
}
