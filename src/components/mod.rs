//! UI Components Module
//!
//! This module contains all Dioxus UI components for the Rubik's Cube
//! Solver & Tutorial application.

pub mod color_picker;
pub mod cube_3d;
pub mod cube_controls;
pub mod cube_input;
pub mod move_display;
pub mod solution_player;
pub mod validation_feedback;

pub use color_picker::{ColorPicker, ColorPickerProps};
pub use cube_3d::{Cube3D, Cube3DProps, ResponsiveConfig, ResponsiveDimensions};
pub use cube_controls::{CubeControls, CubeControlsProps};
pub use cube_input::{CubeInput, CubeInputProps, UnfoldedLayout, StickerPosition};
pub use move_display::{MoveDisplay, MoveDisplayProps, get_move_explanation, get_move_notation};
pub use solution_player::{SolutionPlayer, SolutionPlayerProps, PlaybackSpeed, PlaybackState};
pub use validation_feedback::{ValidationFeedback, ValidationFeedbackProps, get_kid_friendly_message, get_validation_styles};
