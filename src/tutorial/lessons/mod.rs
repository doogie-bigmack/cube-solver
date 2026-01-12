//! Tutorial lessons module
//!
//! Contains all individual lessons for the tutorial system

pub mod notation;
pub mod colors;

pub use notation::{LessonStep, NotationLesson};
pub use colors::{ColorLessonStep, ColorPair, ColorQuizQuestion, ColorsLesson};
