//! Core cube logic module
//!
//! This module contains all the core Rubik's cube logic including:
//! - state: Cube state representation for NxN cubes
//! - moves: Face rotation operations
//! - notation: Move notation parser
//! - validation: Cube state validation
//! - scramble: Scramble generator

pub mod state;
pub mod moves;
pub mod notation;

// Re-export main types
pub use state::{Color, Cube, Face, FaceName};
pub use moves::{Move, WideMove, WideFace, Direction};
pub use notation::{ParsedMove, NotationError, parse_move, parse_algorithm};
