//! State management module
//!
//! This module provides state management functionality for the application,
//! including history tracking for undo/redo operations and tutorial progress tracking.

mod history;
mod progress;

pub use history::History;
pub use progress::{Progress, LessonId, PracticeStats};
