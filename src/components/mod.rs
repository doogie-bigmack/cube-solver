//! UI Components Module
//!
//! This module contains all Dioxus UI components for the Rubik's Cube
//! Solver & Tutorial application.

pub mod camera_scanner;
//pub mod color_calibration;  // TODO: Fix circular dependency with camera module
pub mod color_picker;
pub mod cube_3d;
pub mod cube_controls;
pub mod cube_input;
pub mod move_display;
//pub mod scan_correction;  // TODO: Fix type inference issues
pub mod scan_workflow;
pub mod solution_player;
pub mod ui_kit;
pub mod validation_feedback;

pub use camera_scanner::{CameraScanner, CameraState};
//pub use color_calibration::ColorCalibration;  // TODO: Fix circular dependency
pub use color_picker::ColorPicker;
pub use cube_3d::Cube3D;
pub use cube_controls::CubeControls;
pub use cube_input::{CubeInput, StickerPosition};
//pub use scan_correction::{ScanCorrection, CorrectionState};  // TODO: Fix type inference issues
pub use scan_workflow::{ScanWorkflow, FacePosition, ScannedFace, ScanWorkflowState};
pub use solution_player::SolutionPlayer;
pub use ui_kit::{
    ButtonSize, ButtonTheme, KidBadge, KidButton, KidCard, KidIconButton, KidProgress,
};
