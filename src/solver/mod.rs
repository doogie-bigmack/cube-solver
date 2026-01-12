//! Solving algorithms for various cube sizes
//!
//! This module provides solvers for:
//! - 2x2 cubes (Depth-limited search)
//! - 3x3 cubes (Beginner's layer-by-layer method via depth-limited search)
//! - 4x4+ cubes (Reduction method - centers, edges, and parity)

pub mod solution;
pub mod two_by_two;
pub mod beginner_3x3;
pub mod reduction;
pub mod parity;

pub use solution::Solution;
pub use two_by_two::solve_2x2;
pub use beginner_3x3::solve_3x3_beginner as solve_3x3;
pub use reduction::{solve_centers, solve_edges};
pub use parity::{resolve_parity, detect_oll_parity, detect_pll_parity, ParityType, ParitySolution};
