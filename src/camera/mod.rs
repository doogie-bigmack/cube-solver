/// Camera module for webcam access and cube scanning
///
/// This module provides functionality for:
/// - Webcam access via Nokhwa (cross-platform)
/// - Live camera feed display
/// - Permission handling
/// - Frame capture for cube scanning

pub mod capture;

pub use capture::{CameraCapture, CameraConfig, CameraError, CameraFrame, list_cameras};
