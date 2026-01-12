//! Tutorial system module
//!
//! This module implements the tutorial system (R6.x) from the PRD

pub mod lessons;
pub mod practice;

pub use lessons::{LessonStep, NotationLesson, ColorLessonStep, ColorPair, ColorQuizQuestion, ColorsLesson};
pub use practice::{PracticeCase, PracticeSession, PracticeGenerator, PracticeType, Difficulty};
