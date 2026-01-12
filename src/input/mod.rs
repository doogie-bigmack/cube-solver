//! Input handling modules
//!
//! This module contains input handling for touch, mouse, and keyboard interactions.

pub mod touch;

pub use touch::{TouchPoint, TouchState, TouchGesture, targets};
