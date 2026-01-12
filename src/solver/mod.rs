//! Solving algorithms for various cube sizes
//!
//! This module provides solvers for:
//! - 2x2 cubes (Ortega method)
//! - 3x3 cubes (Beginner's method via depth-limited search)
//! - 4x4+ cubes (Reduction method)

pub mod two_by_two;
pub mod beginner_3x3;

pub use two_by_two::{solve_2x2, Solution2x2};
pub use beginner_3x3::{solve_3x3_beginner as solve_3x3, Solution3x3Beginner as Solution3x3};
