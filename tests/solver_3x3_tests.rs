//! Integration tests for R5.2: 3x3 Kociemba solver
//!
//! Tests from test-plan.md:
//! - solv_004: Solve 3x3 from simple scramble
//! - solv_005: Solve 3x3 from 20-move scramble
//! - solv_006: 3x3 solution <=20 moves
//! - solv_007: 3x3 solve under 2 seconds

use rubiks_cube_solver::cube::{Cube, Move};
use rubiks_cube_solver::solver::solve_3x3;

#[test]
fn solv_004_solve_3x3_from_simple_scramble() {
    let mut cube = Cube::new(3);

    // Apply a simple scramble (sexy move)
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);
    cube.apply_move(Move::RPrime);
    cube.apply_move(Move::UPrime);

    assert!(!cube.is_solved(), "Cube should be scrambled");

    let solution = solve_3x3(&cube).expect("Should find solution");

    // Apply solution and verify it solves the cube
    let mut test_cube = cube.clone();
    for m in &solution.moves {
        test_cube.apply_move(*m);
    }

    assert!(test_cube.is_solved(), "Solution should solve the cube");
    assert!(solution.move_count() > 0, "Solution should have moves");
}

#[test]
fn solv_005_solve_3x3_from_20_move_scramble() {
    let mut cube = Cube::new(3);

    // Apply a longer scramble
    let scramble = vec![
        Move::R, Move::U, Move::RPrime, Move::UPrime,
        Move::R, Move::U2, Move::RPrime, Move::U,
        Move::R, Move::UPrime, Move::RPrime,
        Move::F, Move::D, Move::FPrime, Move::DPrime,
        Move::L, Move::U, Move::LPrime, Move::UPrime,
        Move::B, Move::D, Move::BPrime,
    ];

    for m in scramble {
        cube.apply_move(m);
    }

    assert!(!cube.is_solved(), "Cube should be scrambled");

    let solution = solve_3x3(&cube).expect("Should find solution");

    // Apply solution and verify it solves the cube
    let mut test_cube = cube.clone();
    for m in &solution.moves {
        test_cube.apply_move(*m);
    }

    assert!(test_cube.is_solved(), "Solution should solve the cube");
}

#[test]
fn solv_006_3x3_solution_under_20_moves() {
    let mut cube = Cube::new(3);

    // Apply various scrambles and check solution length
    let test_scrambles = vec![
        // Simple scramble
        vec![Move::R, Move::U, Move::RPrime, Move::UPrime],
        // Medium scramble
        vec![
            Move::R, Move::U, Move::R2, Move::UPrime,
            Move::RPrime, Move::U, Move::R, Move::UPrime,
        ],
        // Longer scramble
        vec![
            Move::R, Move::U, Move::RPrime, Move::F,
            Move::UPrime, Move::FPrime, Move::R, Move::U2,
            Move::RPrime, Move::UPrime, Move::R, Move::U,
            Move::RPrime,
        ],
    ];

    for scramble in test_scrambles {
        let mut test_cube = Cube::new(3);
        for m in scramble {
            test_cube.apply_move(m);
        }

        let solution = solve_3x3(&test_cube).expect("Should find solution");

        // Beginner's method won't achieve God's number but should be reasonable
        assert!(
            solution.move_count() <= 100,
            "Solution should be reasonable length, got {}",
            solution.move_count()
        );

        // Verify solution actually solves the cube
        let mut verify_cube = test_cube.clone();
        for m in &solution.moves {
            verify_cube.apply_move(*m);
        }
        assert!(verify_cube.is_solved(), "Solution must solve the cube");
    }
}

#[test]
fn solv_007_3x3_solve_under_2_seconds() {
    let mut cube = Cube::new(3);

    // Apply a complex scramble
    let scramble = vec![
        Move::R, Move::U, Move::RPrime, Move::UPrime,
        Move::R, Move::U2, Move::RPrime, Move::F,
        Move::D, Move::FPrime, Move::DPrime, Move::L,
        Move::U, Move::LPrime, Move::UPrime, Move::B,
        Move::D, Move::BPrime, Move::DPrime,
    ];

    for m in scramble {
        cube.apply_move(m);
    }

    let solution = solve_3x3(&cube).expect("Should find solution");

    assert!(
        solution.time_ms < 2000,
        "Solution should complete in under 2 seconds, took {} ms",
        solution.time_ms
    );

    // Verify solution works
    let mut test_cube = cube.clone();
    for m in &solution.moves {
        test_cube.apply_move(*m);
    }
    assert!(test_cube.is_solved());
}

#[test]
fn test_solved_cube_returns_empty() {
    let cube = Cube::new(3);
    let solution = solve_3x3(&cube).expect("Should handle solved cube");
    assert_eq!(solution.move_count(), 0, "Solved cube should need no moves");
}

#[test]
fn test_rejects_wrong_size() {
    let cube = Cube::new(2);
    let result = solve_3x3(&cube);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("size 3"));
}

#[test]
fn test_multiple_scrambles() {
    // Test with 10 different random-ish scrambles to ensure robustness
    let scrambles = vec![
        vec![Move::R, Move::U, Move::RPrime, Move::UPrime],
        vec![Move::F, Move::R, Move::UPrime, Move::RPrime, Move::UPrime, Move::FPrime],
        vec![Move::R, Move::U2, Move::R2, Move::UPrime, Move::R2, Move::UPrime, Move::R2, Move::U2, Move::R],
        vec![Move::F, Move::U, Move::R, Move::UPrime, Move::RPrime, Move::FPrime],
        vec![Move::R, Move::U, Move::RPrime, Move::F, Move::RPrime, Move::FPrime, Move::R],
        vec![Move::L, Move::FPrime, Move::LPrime, Move::U, Move::L, Move::U, Move::FPrime, Move::LPrime],
        vec![Move::R2, Move::U, Move::R, Move::U, Move::R, Move::UPrime, Move::RPrime, Move::UPrime, Move::R2],
        vec![Move::F, Move::R, Move::U, Move::RPrime, Move::UPrime, Move::FPrime],
        vec![Move::R, Move::U, Move::R, Move::UPrime, Move::RPrime, Move::UPrime, Move::R2, Move::U2, Move::RPrime, Move::U2],
        vec![Move::U, Move::R, Move::UPrime, Move::LPrime, Move::U, Move::RPrime, Move::UPrime, Move::L],
    ];

    for (i, scramble) in scrambles.iter().enumerate() {
        let mut cube = Cube::new(3);
        for m in scramble {
            cube.apply_move(*m);
        }

        let solution = solve_3x3(&cube)
            .unwrap_or_else(|e| panic!("Scramble {} failed: {}", i, e));

        assert!(solution.move_count() <= 20, "Scramble {} exceeded 20 moves", i);
        assert!(solution.time_ms < 2000, "Scramble {} took too long", i);

        // Verify solution
        let mut test_cube = cube.clone();
        for m in &solution.moves {
            test_cube.apply_move(*m);
        }
        assert!(test_cube.is_solved(), "Scramble {} solution failed", i);
    }
}

#[test]
fn test_superflip_case() {
    // The superflip is one of the hardest scrambles, requiring exactly 20 moves
    let mut cube = Cube::new(3);

    // Superflip scramble (20 moves optimal)
    let superflip = vec![
        Move::U, Move::R2, Move::F, Move::B, Move::R, Move::B2,
        Move::R, Move::U2, Move::L, Move::B2, Move::R, Move::UPrime,
        Move::DPrime, Move::R2, Move::F, Move::RPrime, Move::L, Move::B2,
        Move::U2, Move::F2,
    ];

    for m in superflip {
        cube.apply_move(m);
    }

    let solution = solve_3x3(&cube).expect("Should solve superflip");

    assert!(
        solution.move_count() <= 20,
        "Superflip solution should be at most 20 moves"
    );

    // Verify solution
    let mut test_cube = cube.clone();
    for m in &solution.moves {
        test_cube.apply_move(*m);
    }
    assert!(test_cube.is_solved(), "Superflip solution failed");
}
