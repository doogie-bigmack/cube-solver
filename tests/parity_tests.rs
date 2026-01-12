//! Integration tests for 4x4+ parity handling
//!
//! Tests R5.5: 4x4+ parity handling

use rubiks_cube_solver::cube::{Cube, Move};
use rubiks_cube_solver::solver::{resolve_parity, detect_oll_parity, detect_pll_parity, ParityType};

#[test]
fn test_resolve_parity_on_solved_4x4() {
    // A solved 4x4 should have no parity
    let cube = Cube::new(4);
    let solution = resolve_parity(&cube).expect("Should succeed on solved cube");

    assert_eq!(solution.parity_type, ParityType::None);
    assert_eq!(solution.move_count(), 0, "No moves needed for solved cube");
    assert!(solution.time_ms < 1000, "Should complete quickly");
    assert!(!solution.steps.is_empty(), "Should have at least one step");
}

#[test]
fn test_resolve_parity_on_solved_5x5() {
    // A solved 5x5 should have no parity
    let cube = Cube::new(5);
    let solution = resolve_parity(&cube).expect("Should succeed on 5x5");

    assert_eq!(solution.parity_type, ParityType::None);
    assert_eq!(solution.move_count(), 0);
    assert!(solution.time_ms < 1000);
}

#[test]
fn test_resolve_parity_on_solved_6x6() {
    // A solved 6x6 should have no parity
    let cube = Cube::new(6);
    let solution = resolve_parity(&cube).expect("Should succeed on 6x6");

    assert_eq!(solution.parity_type, ParityType::None);
    assert_eq!(solution.move_count(), 0);
}

#[test]
fn test_resolve_parity_rejects_2x2() {
    // 2x2 cubes don't have parity
    let cube = Cube::new(2);
    let result = resolve_parity(&cube);

    assert!(result.is_err(), "Should reject 2x2 cube");
    assert!(result.unwrap_err().contains("4x4+"));
}

#[test]
fn test_resolve_parity_rejects_3x3() {
    // 3x3 cubes don't have parity
    let cube = Cube::new(3);
    let result = resolve_parity(&cube);

    assert!(result.is_err(), "Should reject 3x3 cube");
    assert!(result.unwrap_err().contains("4x4+"));
}

#[test]
fn test_detect_oll_parity_on_solved_cube() {
    // Solved cubes should not have OLL parity
    let cube = Cube::new(4);
    assert!(!detect_oll_parity(&cube), "Solved cube should not have OLL parity");
}

#[test]
fn test_detect_pll_parity_on_solved_cube() {
    // Solved cubes should not have PLL parity
    let cube = Cube::new(4);
    assert!(!detect_pll_parity(&cube), "Solved cube should not have PLL parity");
}

#[test]
fn test_detect_oll_parity_after_scramble() {
    // After scrambling, test that detection doesn't panic
    let mut cube = Cube::new(4);
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);
    cube.apply_move(Move::RPrime);
    cube.apply_move(Move::UPrime);

    // Should not panic, returns a boolean
    let _has_oll = detect_oll_parity(&cube);
}

#[test]
fn test_detect_pll_parity_after_scramble() {
    // After scrambling, test that detection doesn't panic
    let mut cube = Cube::new(4);
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);
    cube.apply_move(Move::RPrime);
    cube.apply_move(Move::UPrime);

    // Should not panic, returns a boolean
    let _has_pll = detect_pll_parity(&cube);
}

#[test]
fn test_parity_solution_structure() {
    // Test that solution has proper structure
    let cube = Cube::new(4);
    let solution = resolve_parity(&cube).expect("Should succeed");

    // Should have steps
    assert!(!solution.steps.is_empty(), "Should have at least one step");

    // Should be able to convert to generic Solution
    let generic_solution = solution.to_solution();
    assert!(generic_solution.method.is_some(), "Should have method name");
    assert!(generic_solution.method.unwrap().contains("Parity"), "Method should mention parity");
}

#[test]
fn test_parity_solution_timing() {
    // Should complete quickly even on larger cubes
    let cube = Cube::new(5);
    let solution = resolve_parity(&cube).expect("Should succeed");

    assert!(solution.time_ms < 1000, "Should complete in under 1 second");
}

#[test]
fn test_parity_solution_step_descriptions() {
    // Test that steps have meaningful descriptions
    let cube = Cube::new(4);
    let solution = resolve_parity(&cube).expect("Should succeed");

    for step in &solution.steps {
        assert!(!step.description.is_empty(), "Step description should not be empty");
    }
}

#[test]
fn test_parity_solution_move_count() {
    // Test that move_count() method works
    let cube = Cube::new(4);
    let solution = resolve_parity(&cube).expect("Should succeed");

    assert_eq!(solution.move_count(), solution.moves.len());
}

#[test]
fn test_oll_parity_detection_on_multiple_sizes() {
    // Test OLL detection works on different cube sizes
    for size in 4..=6 {
        let cube = Cube::new(size);
        let _has_oll = detect_oll_parity(&cube);
        // Should not panic
    }
}

#[test]
fn test_pll_parity_detection_on_multiple_sizes() {
    // Test PLL detection works on different cube sizes
    for size in 4..=6 {
        let cube = Cube::new(size);
        let _has_pll = detect_pll_parity(&cube);
        // Should not panic
    }
}

#[test]
fn test_parity_types_are_distinct() {
    // Test that different parity types can be distinguished
    assert_ne!(ParityType::None, ParityType::OllParity);
    assert_ne!(ParityType::None, ParityType::PllParity);
    assert_ne!(ParityType::OllParity, ParityType::PllParity);
    assert_ne!(ParityType::OllParity, ParityType::Both);
}

#[test]
fn test_parity_solution_clone() {
    // Test that ParitySolution can be cloned
    let cube = Cube::new(4);
    let solution = resolve_parity(&cube).expect("Should succeed");
    let _cloned = solution.clone();
}

#[test]
fn test_parity_type_clone() {
    // Test that ParityType can be cloned
    let parity = ParityType::OllParity;
    let _cloned = parity.clone();
}

#[test]
fn test_resolve_parity_on_7x7() {
    // Test that parity handling works on very large cubes
    let cube = Cube::new(7);
    let solution = resolve_parity(&cube).expect("Should succeed on 7x7");

    assert!(!solution.steps.is_empty());
    assert!(solution.time_ms < 1000);
}

#[test]
fn test_parity_solution_to_generic_solution() {
    // Test conversion to generic Solution type
    let cube = Cube::new(4);
    let parity_solution = resolve_parity(&cube).expect("Should succeed");
    let generic_solution = parity_solution.to_solution();

    assert_eq!(generic_solution.step_count(), parity_solution.steps.len());
    assert!(generic_solution.method.is_some());
}

#[test]
fn test_oll_parity_detection_consistency() {
    // Test that OLL detection is consistent on same cube
    let cube = Cube::new(4);
    let result1 = detect_oll_parity(&cube);
    let result2 = detect_oll_parity(&cube);
    assert_eq!(result1, result2, "Detection should be deterministic");
}

#[test]
fn test_pll_parity_detection_consistency() {
    // Test that PLL detection is consistent on same cube
    let cube = Cube::new(4);
    let result1 = detect_pll_parity(&cube);
    let result2 = detect_pll_parity(&cube);
    assert_eq!(result1, result2, "Detection should be deterministic");
}
