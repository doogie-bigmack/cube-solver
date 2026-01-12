/// Camera module for webcam access and cube scanning
///
/// This module provides functionality for:
/// - Webcam access via Nokhwa (cross-platform)
/// - Live camera feed display
/// - Permission handling
/// - Frame capture for cube scanning
/// - HSV color detection for cube face scanning
/// - Color calibration for improved detection accuracy

pub mod calibration;
pub mod capture;
pub mod color_detect;

pub use calibration::{CalibrationManager, ColorCalibration, ColorSample};
pub use capture::{CameraCapture, CameraConfig, CameraError, CameraFrame, list_cameras};
pub use color_detect::{
    detect_color, detect_colors_in_grid, detect_colors_in_grid_with_confidence,
    detect_colors_with_lighting_adaptation, rgb_to_hsv, ColorDetectionConfig,
    ColorDetectionResult, HSV, LightingAnalysis, LightingQuality, RGB,
};
