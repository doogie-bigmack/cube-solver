/// Camera capture functionality for scanning Rubik's cubes via webcam
///
/// This module provides cross-platform camera access:
/// - Native platforms: Uses nokhwa library
/// - Web (WASM): Uses browser MediaDevices API (handled in UI component)

/// Error types for camera operations
#[derive(Debug, Clone)]
pub enum CameraError {
    /// Failed to access camera (permission denied or camera not found)
    AccessDenied(String),
    /// Camera is already in use by another application
    DeviceBusy(String),
    /// Failed to initialize camera
    InitializationFailed(String),
    /// Failed to capture frame
    CaptureError(String),
    /// No cameras available on the system
    NoCamerasAvailable,
}

impl std::fmt::Display for CameraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CameraError::AccessDenied(msg) => write!(f, "Camera access denied: {}", msg),
            CameraError::DeviceBusy(msg) => write!(f, "Camera device busy: {}", msg),
            CameraError::InitializationFailed(msg) => {
                write!(f, "Camera initialization failed: {}", msg)
            }
            CameraError::CaptureError(msg) => write!(f, "Frame capture error: {}", msg),
            CameraError::NoCamerasAvailable => write!(f, "No cameras available on this device"),
        }
    }
}

impl std::error::Error for CameraError {}

/// Represents a frame captured from the camera
#[derive(Debug, Clone)]
pub struct CameraFrame {
    /// Raw RGB pixel data (width * height * 3 bytes)
    pub data: Vec<u8>,
    /// Frame width in pixels
    pub width: u32,
    /// Frame height in pixels
    pub height: u32,
    /// Timestamp when frame was captured (milliseconds since epoch)
    pub timestamp_ms: u64,
}

impl CameraFrame {
    /// Get pixel RGB values at specific coordinates
    /// Returns None if coordinates are out of bounds
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<(u8, u8, u8)> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let idx = ((y * self.width + x) * 3) as usize;
        if idx + 2 < self.data.len() {
            Some((self.data[idx], self.data[idx + 1], self.data[idx + 2]))
        } else {
            None
        }
    }
}

/// Configuration for camera capture
#[derive(Debug, Clone)]
pub struct CameraConfig {
    /// Preferred camera index (0 = default camera)
    pub camera_index: u32,
    /// Preferred resolution width
    pub width: u32,
    /// Preferred resolution height
    pub height: u32,
    /// Preferred frames per second
    pub fps: u32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            camera_index: 0,
            width: 640,
            height: 480,
            fps: 30,
        }
    }
}

// ============================================================================
// NATIVE IMPLEMENTATION (using nokhwa)
// ============================================================================

#[cfg(not(target_arch = "wasm32"))]
mod native_impl {
    use super::*;
    use nokhwa::pixel_format::RgbFormat;
    use nokhwa::utils::{
        ApiBackend, CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType,
        Resolution,
    };
    use nokhwa::Camera as NokhwaCamera;
    use std::sync::{Arc, Mutex};

    /// High-level camera interface for capturing frames (Native)
    pub struct CameraCapture {
        camera: Arc<Mutex<NokhwaCamera>>,
        config: CameraConfig,
    }

    impl CameraCapture {
        /// Initialize camera with the given configuration
        pub fn new(config: CameraConfig) -> Result<Self, CameraError> {
            let requested_format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::Closest(
                CameraFormat::new(
                    Resolution::new(config.width, config.height),
                    FrameFormat::MJPEG,
                    config.fps,
                ),
            ));

            let backend = get_backend();

            let camera = NokhwaCamera::new(
                CameraIndex::Index(config.camera_index),
                requested_format,
            )
            .map_err(|e| {
                CameraError::InitializationFailed(format!("Failed to open camera: {}", e))
            })?;

            Ok(Self {
                camera: Arc::new(Mutex::new(camera)),
                config,
            })
        }

        /// Request camera permission
        pub fn request_permission(&mut self) -> Result<(), CameraError> {
            let mut camera = self
                .camera
                .lock()
                .map_err(|e| CameraError::InitializationFailed(format!("Lock error: {}", e)))?;

            camera
                .open_stream()
                .map_err(|e| CameraError::AccessDenied(format!("Permission denied: {}", e)))?;

            Ok(())
        }

        /// Capture a single frame from the camera
        pub fn capture_frame(&self) -> Result<CameraFrame, CameraError> {
            let mut camera = self
                .camera
                .lock()
                .map_err(|e| CameraError::CaptureError(format!("Lock error: {}", e)))?;

            let frame = camera
                .frame()
                .map_err(|e| CameraError::CaptureError(format!("Failed to capture frame: {}", e)))?;

            let decoded = frame
                .decode_image::<RgbFormat>()
                .map_err(|e| CameraError::CaptureError(format!("Failed to decode frame: {}", e)))?;

            let timestamp_ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;

            Ok(CameraFrame {
                data: decoded.into_raw(),
                width: self.config.width,
                height: self.config.height,
                timestamp_ms,
            })
        }

        /// Stop the camera stream
        pub fn stop(&mut self) -> Result<(), CameraError> {
            let mut camera = self
                .camera
                .lock()
                .map_err(|e| CameraError::InitializationFailed(format!("Lock error: {}", e)))?;

            camera
                .stop_stream()
                .map_err(|e| CameraError::CaptureError(format!("Failed to stop stream: {}", e)))?;

            Ok(())
        }

        /// Get camera configuration
        pub fn config(&self) -> &CameraConfig {
            &self.config
        }

        /// Check if camera is currently streaming
        pub fn is_streaming(&self) -> bool {
            if let Ok(camera) = self.camera.lock() {
                camera.is_stream_open()
            } else {
                false
            }
        }
    }

    fn get_backend() -> ApiBackend {
        #[cfg(target_os = "linux")]
        return ApiBackend::Video4Linux;

        #[cfg(target_os = "macos")]
        return ApiBackend::AVFoundation;

        #[cfg(target_os = "windows")]
        return ApiBackend::MediaFoundation;

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        return ApiBackend::Auto;
    }

    /// Query available cameras on the system
    pub fn list_cameras() -> Vec<(u32, String)> {
        let backend = get_backend();

        if let Ok(devices) = nokhwa::query(backend) {
            devices
                .into_iter()
                .enumerate()
                .map(|(idx, info)| (idx as u32, info.human_name().to_string()))
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native_impl::*;

// ============================================================================
// WASM IMPLEMENTATION (stub - uses browser API in UI component)
// ============================================================================

#[cfg(target_arch = "wasm32")]
mod wasm_impl {
    use super::*;

    /// Placeholder for WASM (web uses browser MediaDevices API in component)
    pub struct CameraCapture {
        config: CameraConfig,
    }

    impl CameraCapture {
        /// Initialize camera with the given configuration (stub for WASM)
        pub fn new(config: CameraConfig) -> Result<Self, CameraError> {
            Ok(Self { config })
        }

        /// Request camera permission (handled by browser)
        pub fn request_permission(&mut self) -> Result<(), CameraError> {
            Ok(())
        }

        /// Capture a single frame (not supported in WASM stub)
        pub fn capture_frame(&self) -> Result<CameraFrame, CameraError> {
            Err(CameraError::CaptureError(
                "Frame capture not available in WASM - use browser MediaDevices API".to_string(),
            ))
        }

        /// Stop the camera stream (handled by browser)
        pub fn stop(&mut self) -> Result<(), CameraError> {
            Ok(())
        }

        /// Get camera configuration
        pub fn config(&self) -> &CameraConfig {
            &self.config
        }

        /// Check if camera is currently streaming
        pub fn is_streaming(&self) -> bool {
            false
        }
    }

    /// Query available cameras (not supported in WASM stub)
    pub fn list_cameras() -> Vec<(u32, String)> {
        Vec::new()
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_impl::*;

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_config_default() {
        let config = CameraConfig::default();
        assert_eq!(config.camera_index, 0);
        assert_eq!(config.width, 640);
        assert_eq!(config.height, 480);
        assert_eq!(config.fps, 30);
    }

    #[test]
    fn test_camera_frame_get_pixel() {
        // Create a test frame (2x2 pixels, RGB)
        let data = vec![
            255, 0, 0, // Red pixel at (0,0)
            0, 255, 0, // Green pixel at (1,0)
            0, 0, 255, // Blue pixel at (0,1)
            255, 255, 0, // Yellow pixel at (1,1)
        ];
        let frame = CameraFrame {
            data,
            width: 2,
            height: 2,
            timestamp_ms: 0,
        };

        assert_eq!(frame.get_pixel(0, 0), Some((255, 0, 0))); // Red
        assert_eq!(frame.get_pixel(1, 0), Some((0, 255, 0))); // Green
        assert_eq!(frame.get_pixel(0, 1), Some((0, 0, 255))); // Blue
        assert_eq!(frame.get_pixel(1, 1), Some((255, 255, 0))); // Yellow
        assert_eq!(frame.get_pixel(2, 0), None); // Out of bounds
        assert_eq!(frame.get_pixel(0, 2), None); // Out of bounds
    }

    #[test]
    fn test_camera_error_display() {
        let error = CameraError::AccessDenied("test".to_string());
        assert!(error.to_string().contains("Camera access denied"));

        let error = CameraError::DeviceBusy("test".to_string());
        assert!(error.to_string().contains("Camera device busy"));

        let error = CameraError::NoCamerasAvailable;
        assert!(error.to_string().contains("No cameras available"));
    }

    #[test]
    fn test_list_cameras() {
        // This test just ensures the function doesn't panic
        let cameras = list_cameras();
        assert!(cameras.len() >= 0);
    }
}
