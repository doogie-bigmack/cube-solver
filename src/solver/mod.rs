//! Solving algorithms for various cube sizes
//!
//! This module provides solvers for:
//! - 2x2 cubes (Ortega method)
//! - 3x3 cubes (Kociemba algorithm)
//! - 4x4+ cubes (Reduction method)

pub mod two_by_two;

pub use two_by_two::{solve_2x2, Solution2x2};
