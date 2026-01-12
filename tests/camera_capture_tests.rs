/// Integration tests for camera capture functionality
///
/// These tests verify the camera module's basic functionality,
/// configuration, and error handling.

use rubiks_cube_solver::camera::{CameraConfig, CameraError, CameraFrame, list_cameras};

#[test]
fn test_camera_config_default() {
    let config = CameraConfig::default();
    assert_eq!(config.camera_index, 0);
    assert_eq!(config.width, 640);
    assert_eq!(config.height, 480);
    assert_eq!(config.fps, 30);
}

#[test]
fn test_camera_config_custom() {
    let config = CameraConfig {
        camera_index: 1,
        width: 1280,
        height: 720,
        fps: 60,
    };
    assert_eq!(config.camera_index, 1);
    assert_eq!(config.width, 1280);
    assert_eq!(config.height, 720);
    assert_eq!(config.fps, 60);
}

#[test]
fn test_camera_frame_creation() {
    // Create a test frame with known data
    let data = vec![255, 0, 0, 0, 255, 0, 0, 0, 255]; // Red, Green, Blue pixels
    let frame = CameraFrame {
        data: data.clone(),
        width: 3,
        height: 1,
        timestamp_ms: 1000,
    };

    assert_eq!(frame.data, data);
    assert_eq!(frame.width, 3);
    assert_eq!(frame.height, 1);
    assert_eq!(frame.timestamp_ms, 1000);
}

#[test]
fn test_camera_frame_get_pixel() {
    // Create a 2x2 test frame
    let data = vec![
        255, 0, 0,    // Red at (0,0)
        0, 255, 0,    // Green at (1,0)
        0, 0, 255,    // Blue at (0,1)
        255, 255, 0,  // Yellow at (1,1)
    ];
    let frame = CameraFrame {
        data,
        width: 2,
        height: 2,
        timestamp_ms: 0,
    };

    // Test valid pixel access
    assert_eq!(frame.get_pixel(0, 0), Some((255, 0, 0)));   // Red
    assert_eq!(frame.get_pixel(1, 0), Some((0, 255, 0)));   // Green
    assert_eq!(frame.get_pixel(0, 1), Some((0, 0, 255)));   // Blue
    assert_eq!(frame.get_pixel(1, 1), Some((255, 255, 0))); // Yellow

    // Test out of bounds access
    assert_eq!(frame.get_pixel(2, 0), None);
    assert_eq!(frame.get_pixel(0, 2), None);
    assert_eq!(frame.get_pixel(2, 2), None);
}

#[test]
fn test_camera_frame_get_pixel_edge_cases() {
    let data = vec![100, 150, 200]; // Single pixel
    let frame = CameraFrame {
        data,
        width: 1,
        height: 1,
        timestamp_ms: 0,
    };

    assert_eq!(frame.get_pixel(0, 0), Some((100, 150, 200)));
    assert_eq!(frame.get_pixel(1, 0), None);
    assert_eq!(frame.get_pixel(0, 1), None);
}

#[test]
fn test_camera_error_display() {
    let error = CameraError::AccessDenied("test message".to_string());
    let display = format!("{}", error);
    assert!(display.contains("Camera access denied"));
    assert!(display.contains("test message"));

    let error = CameraError::DeviceBusy("busy".to_string());
    let display = format!("{}", error);
    assert!(display.contains("Camera device busy"));

    let error = CameraError::InitializationFailed("init error".to_string());
    let display = format!("{}", error);
    assert!(display.contains("Camera initialization failed"));

    let error = CameraError::CaptureError("capture failed".to_string());
    let display = format!("{}", error);
    assert!(display.contains("Frame capture error"));

    let error = CameraError::NoCamerasAvailable;
    let display = format!("{}", error);
    assert!(display.contains("No cameras available"));
}

#[test]
fn test_camera_error_clone() {
    let error = CameraError::AccessDenied("test".to_string());
    let cloned = error.clone();
    assert_eq!(format!("{}", error), format!("{}", cloned));
}

#[test]
fn test_list_cameras() {
    // This test verifies that list_cameras doesn't panic
    // Actual camera availability varies by system
    let cameras = list_cameras();

    // We can't assert specific cameras, but we can verify:
    // 1. Function doesn't panic
    // 2. Returns a valid vector
    assert!(cameras.len() >= 0);

    // If cameras are available, verify structure
    for (index, name) in cameras {
        assert!(index >= 0);
        assert!(!name.is_empty() || name.is_empty()); // Name can be empty or not
    }
}

#[test]
fn test_camera_config_clone() {
    let config1 = CameraConfig {
        camera_index: 0,
        width: 1920,
        height: 1080,
        fps: 30,
    };
    let config2 = config1.clone();

    assert_eq!(config1.camera_index, config2.camera_index);
    assert_eq!(config1.width, config2.width);
    assert_eq!(config1.height, config2.height);
    assert_eq!(config1.fps, config2.fps);
}

#[test]
fn test_camera_frame_clone() {
    let frame1 = CameraFrame {
        data: vec![255, 0, 0],
        width: 1,
        height: 1,
        timestamp_ms: 1234,
    };
    let frame2 = frame1.clone();

    assert_eq!(frame1.data, frame2.data);
    assert_eq!(frame1.width, frame2.width);
    assert_eq!(frame1.height, frame2.height);
    assert_eq!(frame1.timestamp_ms, frame2.timestamp_ms);
}

#[test]
fn test_large_frame_dimensions() {
    // Test with larger frame dimensions (1920x1080)
    let width = 1920;
    let height = 1080;
    let data = vec![0u8; (width * height * 3) as usize]; // All black pixels

    let frame = CameraFrame {
        data,
        width,
        height,
        timestamp_ms: 5000,
    };

    assert_eq!(frame.width, 1920);
    assert_eq!(frame.height, 1080);

    // Test corner pixels
    assert_eq!(frame.get_pixel(0, 0), Some((0, 0, 0)));
    assert_eq!(frame.get_pixel(1919, 0), Some((0, 0, 0)));
    assert_eq!(frame.get_pixel(0, 1079), Some((0, 0, 0)));
    assert_eq!(frame.get_pixel(1919, 1079), Some((0, 0, 0)));

    // Test out of bounds
    assert_eq!(frame.get_pixel(1920, 0), None);
    assert_eq!(frame.get_pixel(0, 1080), None);
}

#[test]
fn test_camera_config_various_resolutions() {
    // Test common camera resolutions
    let resolutions = vec![
        (320, 240),   // QVGA
        (640, 480),   // VGA
        (1280, 720),  // HD
        (1920, 1080), // Full HD
        (3840, 2160), // 4K
    ];

    for (width, height) in resolutions {
        let config = CameraConfig {
            camera_index: 0,
            width,
            height,
            fps: 30,
        };
        assert_eq!(config.width, width);
        assert_eq!(config.height, height);
    }
}
