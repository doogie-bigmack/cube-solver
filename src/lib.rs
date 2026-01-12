//! Rubik's Cube Solver & Tutorial Library
//!
//! This crate provides the core functionality for the Rubik's cube solver app,
//! including cube state representation, solving algorithms, and more.

pub mod components;
pub mod cube;
pub mod input;
#[cfg(not(target_arch = "wasm32"))]
pub mod renderer;
pub mod solver;
pub mod state;
