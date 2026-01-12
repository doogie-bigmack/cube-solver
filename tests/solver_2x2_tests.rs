//! Integration tests for R5.1: 2x2 solver
//!
//! Tests from test-plan.md:
//! - solv_001: Solve 2x2 from simple scramble
//! - solv_002: Solve 2x2 from complex scramble
//! - solv_003: 2x2 solution under 1 second

use rubiks_cube_solver::cube::{Cube, Move};
use rubiks_cube_solver::solver::solve_2x2;

#[test]
fn solv_001_solve_2x2_from_simple_scramble() {
    let mut cube = Cube::new(2);

    // Simple 3-move scramble
    let scramble = vec![Move::R, Move::U, Move::F];

    for mv in scramble {
        cube.apply_move(mv);
    }

    // Solve it
    let solution = solve_2x2(&cube).expect("Should find solution");

    // Verify the solution works
    let mut test_cube = cube.clone();
    for mv in &solution.moves {
        test_cube.apply_move(*mv);
    }

    assert!(test_cube.is_solved(), "Cube should be solved after applying solution");
    println!("Simple scramble solved in {} moves, {} ms", solution.move_count(), solution.time_ms);
}

#[test]
fn solv_002_solve_2x2_from_complex_scramble() {
    let mut cube = Cube::new(2);

    // More complex 6-move scramble
    let scramble = vec![
        Move::R, Move::U, Move::RPrime, Move::UPrime,
        Move::F, Move::FPrime,
    ];

    for mv in scramble {
        cube.apply_move(mv);
    }

    // Solve it
    let solution = solve_2x2(&cube).expect("Should find solution");

    // Verify the solution works
    let mut test_cube = cube.clone();
    for mv in &solution.moves {
        test_cube.apply_move(*mv);
    }

    assert!(test_cube.is_solved(), "Cube should be solved after applying solution");
    println!("Complex scramble solved in {} moves, {} ms", solution.move_count(), solution.time_ms);
}

#[test]
fn solv_003_2x2_solution_under_1_second() {
    let mut cube = Cube::new(2);

    // Standard scramble
    let scramble = vec![
        Move::R, Move::U, Move::RPrime, Move::UPrime,
    ];

    for mv in scramble {
        cube.apply_move(mv);
    }

    let solution = solve_2x2(&cube).expect("Should find solution");

    // Main requirement: solve in under 1 second
    assert!(solution.time_ms < 1000, "Solution took {} ms, should be under 1000ms", solution.time_ms);

    // Verify correctness
    let mut test_cube = cube.clone();
    for mv in &solution.moves {
        test_cube.apply_move(*mv);
    }
    assert!(test_cube.is_solved());

    println!("Solved in {} ms with {} moves", solution.time_ms, solution.move_count());
}

#[test]
fn test_solve_various_2x2_scrambles() {
    // Test several different scrambles to ensure consistency
    let test_cases = vec![
        vec![Move::R],
        vec![Move::R, Move::U],
        vec![Move::R2, Move::U2],
        vec![Move::R, Move::U, Move::RPrime],
        vec![Move::F, Move::R, Move::U, Move::FPrime],
    ];

    for (i, scramble) in test_cases.iter().enumerate() {
        let mut cube = Cube::new(2);

        for &mv in scramble {
            cube.apply_move(mv);
        }

        let solution = solve_2x2(&cube).expect(&format!("Should solve test case {}", i));

        // Verify solution
        let mut test_cube = cube.clone();
        for mv in &solution.moves {
            test_cube.apply_move(*mv);
        }

        assert!(test_cube.is_solved(), "Test case {} should be solved", i);
        assert!(solution.time_ms < 2000, "Test case {} took {} ms", i, solution.time_ms);
    }
}

#[test]
fn test_2x2_solver_performance() {
    // Test that solver consistently performs well
    let mut total_time = 0u128;
    let num_tests = 5;

    for _ in 0..num_tests {
        let mut cube = Cube::new(2);

        // Standard scramble
        let scramble = vec![Move::R, Move::U, Move::F, Move::RPrime];

        for mv in scramble {
            cube.apply_move(mv);
        }

        let solution = solve_2x2(&cube).expect("Should solve");

        // Verify solution works
        let mut test_cube = cube.clone();
        for mv in &solution.moves {
            test_cube.apply_move(*mv);
        }
        assert!(test_cube.is_solved());

        total_time += solution.time_ms;
    }

    let avg_time = total_time / num_tests;
    println!("Average solve time over {} tests: {} ms", num_tests, avg_time);

    // Average should be well under 1 second
    assert!(avg_time < 1000, "Average time {} ms exceeds 1000ms", avg_time);
}

#[test]
fn test_solver_rejects_invalid_size() {
    let cube_3x3 = Cube::new(3);
    let result = solve_2x2(&cube_3x3);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("size 2"));
}

#[test]
fn test_solver_already_solved() {
    let cube = Cube::new(2);
    let solution = solve_2x2(&cube).expect("Should handle solved cube");

    assert_eq!(solution.move_count(), 0, "Solved cube should require 0 moves");
    assert!(cube.is_solved());
}
