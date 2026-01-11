//! Camera orbit controls for 3D cube visualization
//!
//! This module provides camera controls with:
//! - Mouse drag to rotate camera around cube
//! - Touch drag support for mobile
//! - Smooth interpolation
//! - Angle constraints

use glam::{Mat4, Vec3};

/// Camera with orbit controls
#[derive(Debug, Clone)]
pub struct Camera {
    /// Current azimuth angle (rotation around Y axis) in radians
    pub azimuth: f32,
    /// Current elevation angle (rotation around X axis) in radians
    pub elevation: f32,
    /// Distance from target
    pub distance: f32,
    /// Target point the camera looks at
    pub target: Vec3,
    /// Field of view in radians
    pub fov: f32,
    /// Aspect ratio (width / height)
    pub aspect_ratio: f32,
    /// Near clipping plane
    pub near: f32,
    /// Far clipping plane
    pub far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            azimuth: 0.7, // ~40 degrees
            elevation: 0.5, // ~30 degrees
            distance: 5.0,
            target: Vec3::ZERO,
            fov: std::f32::consts::PI / 4.0, // 45 degrees
            aspect_ratio: 1.0,
            near: 0.1,
            far: 100.0,
        }
    }
}

impl Camera {
    /// Create a new camera with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a camera with custom parameters
    pub fn with_config(
        distance: f32,
        azimuth: f32,
        elevation: f32,
        aspect_ratio: f32,
    ) -> Self {
        Self {
            azimuth,
            elevation,
            distance,
            aspect_ratio,
            ..Default::default()
        }
    }

    /// Get the camera position in world space
    pub fn position(&self) -> Vec3 {
        let x = self.distance * self.elevation.cos() * self.azimuth.sin();
        let y = self.distance * self.elevation.sin();
        let z = self.distance * self.elevation.cos() * self.azimuth.cos();
        self.target + Vec3::new(x, y, z)
    }

    /// Get the view matrix (camera transform)
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position(), self.target, Vec3::Y)
    }

    /// Get the projection matrix
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }

    /// Get combined view-projection matrix
    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// Rotate camera by delta angles (in radians)
    pub fn rotate(&mut self, delta_azimuth: f32, delta_elevation: f32) {
        self.azimuth += delta_azimuth;
        self.elevation += delta_elevation;

        // Constrain elevation to prevent gimbal lock
        // Keep between -89 and +89 degrees
        const MAX_ELEVATION: f32 = std::f32::consts::PI / 2.0 - 0.01;
        self.elevation = self.elevation.clamp(-MAX_ELEVATION, MAX_ELEVATION);

        // Normalize azimuth to [0, 2π)
        self.azimuth = self.azimuth.rem_euclid(2.0 * std::f32::consts::PI);
    }

    /// Set aspect ratio (width / height)
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    /// Reset camera to default position
    pub fn reset(&mut self) {
        *self = Self {
            aspect_ratio: self.aspect_ratio,
            ..Default::default()
        };
    }
}

/// Orbit controller for handling user input
#[derive(Debug, Clone)]
pub struct OrbitController {
    /// Camera being controlled
    pub camera: Camera,
    /// Rotation sensitivity (radians per pixel)
    pub rotation_sensitivity: f32,
    /// Whether currently dragging
    pub is_dragging: bool,
    /// Last mouse/touch position (x, y) in screen coordinates
    pub last_position: Option<(f32, f32)>,
    /// Smooth interpolation factor (0.0 = no smoothing, 1.0 = instant)
    pub smoothing: f32,
    /// Target azimuth for smooth interpolation
    pub target_azimuth: f32,
    /// Target elevation for smooth interpolation
    pub target_elevation: f32,
    /// Target distance for smooth zoom
    pub target_distance: f32,
    /// Minimum zoom distance (closest to cube)
    pub min_distance: f32,
    /// Maximum zoom distance (farthest from cube)
    pub max_distance: f32,
    /// Zoom sensitivity (distance change per scroll unit)
    pub zoom_sensitivity: f32,
}

impl Default for OrbitController {
    fn default() -> Self {
        let camera = Camera::default();
        Self {
            target_azimuth: camera.azimuth,
            target_elevation: camera.elevation,
            target_distance: camera.distance,
            camera,
            rotation_sensitivity: 0.005, // 0.005 radians per pixel ≈ 0.3 degrees
            is_dragging: false,
            last_position: None,
            smoothing: 0.15,
            min_distance: 2.0,  // Closest zoom (2 units from cube)
            max_distance: 15.0, // Farthest zoom (15 units from cube)
            zoom_sensitivity: 0.5, // 0.5 units per scroll notch
        }
    }
}

impl OrbitController {
    /// Create a new orbit controller
    pub fn new() -> Self {
        Self::default()
    }

    /// Create orbit controller with custom camera
    pub fn with_camera(camera: Camera) -> Self {
        Self {
            target_azimuth: camera.azimuth,
            target_elevation: camera.elevation,
            target_distance: camera.distance,
            camera,
            ..Default::default()
        }
    }

    /// Start dragging (mouse down / touch start)
    pub fn start_drag(&mut self, x: f32, y: f32) {
        self.is_dragging = true;
        self.last_position = Some((x, y));
    }

    /// Update drag (mouse move / touch move)
    pub fn update_drag(&mut self, x: f32, y: f32) {
        if !self.is_dragging {
            return;
        }

        if let Some((last_x, last_y)) = self.last_position {
            let delta_x = x - last_x;
            let delta_y = y - last_y;

            // Convert pixel delta to angle delta
            let delta_azimuth = delta_x * self.rotation_sensitivity;
            let delta_elevation = -delta_y * self.rotation_sensitivity; // Negative for natural feel

            // Update target angles
            self.target_azimuth += delta_azimuth;
            self.target_elevation += delta_elevation;

            // Constrain target elevation
            const MAX_ELEVATION: f32 = std::f32::consts::PI / 2.0 - 0.01;
            self.target_elevation = self.target_elevation.clamp(-MAX_ELEVATION, MAX_ELEVATION);

            // Normalize target azimuth
            self.target_azimuth = self.target_azimuth.rem_euclid(2.0 * std::f32::consts::PI);
        }

        self.last_position = Some((x, y));
    }

    /// End dragging (mouse up / touch end)
    pub fn end_drag(&mut self) {
        self.is_dragging = false;
        self.last_position = None;
    }

    /// Zoom in or out by delta amount
    /// Positive delta zooms in (closer), negative zooms out (farther)
    pub fn zoom(&mut self, delta: f32) {
        self.target_distance -= delta * self.zoom_sensitivity;
        self.target_distance = self.target_distance.clamp(self.min_distance, self.max_distance);
    }

    /// Zoom to specific distance
    pub fn zoom_to(&mut self, distance: f32) {
        self.target_distance = distance.clamp(self.min_distance, self.max_distance);
    }

    /// Handle pinch zoom (for touch devices)
    /// scale > 1.0 means zoom in, scale < 1.0 means zoom out
    pub fn pinch_zoom(&mut self, scale: f32) {
        // Convert scale to distance change
        // scale of 1.1 (10% increase) should zoom in by reducing distance 10%
        // scale of 0.9 (10% decrease) should zoom out by increasing distance 10%
        self.target_distance = self.camera.distance / scale;
        self.target_distance = self.target_distance.clamp(self.min_distance, self.max_distance);
    }

    /// Update camera with smooth interpolation
    /// Call this every frame
    pub fn update(&mut self) {
        // Lerp towards target angles
        self.camera.azimuth +=
            (self.target_azimuth - self.camera.azimuth) * self.smoothing;
        self.camera.elevation +=
            (self.target_elevation - self.camera.elevation) * self.smoothing;

        // Lerp towards target distance (smooth zoom)
        self.camera.distance +=
            (self.target_distance - self.camera.distance) * self.smoothing;
    }

    /// Set aspect ratio
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.camera.set_aspect_ratio(aspect_ratio);
    }

    /// Reset camera to default position
    pub fn reset(&mut self) {
        self.camera.reset();
        self.target_azimuth = self.camera.azimuth;
        self.target_elevation = self.camera.elevation;
        self.target_distance = self.camera.distance;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_default() {
        let camera = Camera::default();
        assert_eq!(camera.distance, 5.0);
        assert_eq!(camera.target, Vec3::ZERO);
        assert!(camera.azimuth > 0.0);
        assert!(camera.elevation > 0.0);
    }

    #[test]
    fn test_camera_position() {
        let camera = Camera::default();
        let pos = camera.position();

        // Position should be at distance from target
        let dist = (pos - camera.target).length();
        assert!((dist - camera.distance).abs() < 0.001);
    }

    #[test]
    fn test_camera_rotate() {
        let mut camera = Camera::default();
        let initial_azimuth = camera.azimuth;
        let initial_elevation = camera.elevation;

        camera.rotate(0.1, 0.1);

        assert!((camera.azimuth - (initial_azimuth + 0.1)).abs() < 0.001);
        assert!((camera.elevation - (initial_elevation + 0.1)).abs() < 0.001);
    }

    #[test]
    fn test_camera_elevation_clamping() {
        let mut camera = Camera::default();

        // Try to rotate past vertical
        camera.rotate(0.0, 10.0); // Large positive elevation
        assert!(camera.elevation < std::f32::consts::PI / 2.0);

        camera.rotate(0.0, -20.0); // Large negative elevation
        assert!(camera.elevation > -std::f32::consts::PI / 2.0);
    }

    #[test]
    fn test_camera_azimuth_wrapping() {
        let mut camera = Camera::default();

        // Rotate more than 360 degrees
        camera.rotate(7.0, 0.0); // > 2π

        // Should wrap to [0, 2π)
        assert!(camera.azimuth >= 0.0);
        assert!(camera.azimuth < 2.0 * std::f32::consts::PI);
    }

    #[test]
    fn test_camera_matrices() {
        let camera = Camera::default();

        // View matrix should be invertible
        let view = camera.view_matrix();
        assert!(!view.is_nan());

        // Projection matrix should be invertible
        let proj = camera.projection_matrix();
        assert!(!proj.is_nan());

        // Combined matrix should be valid
        let vp = camera.view_projection_matrix();
        assert!(!vp.is_nan());
    }

    #[test]
    fn test_orbit_controller_default() {
        let controller = OrbitController::default();
        assert!(!controller.is_dragging);
        assert_eq!(controller.last_position, None);
    }

    #[test]
    fn test_orbit_controller_drag_lifecycle() {
        let mut controller = OrbitController::new();

        // Start drag
        controller.start_drag(100.0, 100.0);
        assert!(controller.is_dragging);
        assert_eq!(controller.last_position, Some((100.0, 100.0)));

        // Update drag
        controller.update_drag(150.0, 120.0);
        assert_eq!(controller.last_position, Some((150.0, 120.0)));

        // End drag
        controller.end_drag();
        assert!(!controller.is_dragging);
        assert_eq!(controller.last_position, None);
    }

    #[test]
    fn test_orbit_controller_rotation() {
        let mut controller = OrbitController::new();
        let initial_azimuth = controller.target_azimuth;

        controller.start_drag(100.0, 100.0);
        controller.update_drag(200.0, 100.0); // Drag 100 pixels right

        // Target azimuth should have changed
        assert!(controller.target_azimuth > initial_azimuth);
    }

    #[test]
    fn test_orbit_controller_smooth_update() {
        let mut controller = OrbitController::new();
        controller.target_azimuth = 1.0;
        controller.camera.azimuth = 0.0;

        // Update once
        controller.update();

        // Camera should have moved towards target but not reached it
        assert!(controller.camera.azimuth > 0.0);
        assert!(controller.camera.azimuth < 1.0);
    }

    #[test]
    fn test_orbit_controller_reset() {
        let mut controller = OrbitController::new();

        // Change state
        controller.camera.rotate(1.0, 1.0);
        controller.target_azimuth = 5.0;

        // Reset
        controller.reset();

        // Should be back to default
        let default_camera = Camera::default();
        assert!((controller.camera.azimuth - default_camera.azimuth).abs() < 0.001);
        assert_eq!(controller.target_azimuth, controller.camera.azimuth);
    }

    #[test]
    fn test_orbit_controller_no_drag_when_not_started() {
        let mut controller = OrbitController::new();
        let initial_azimuth = controller.target_azimuth;

        // Try to update drag without starting
        controller.update_drag(200.0, 100.0);

        // Nothing should change
        assert_eq!(controller.target_azimuth, initial_azimuth);
    }

    #[test]
    fn test_aspect_ratio_update() {
        let mut controller = OrbitController::new();
        controller.set_aspect_ratio(16.0 / 9.0);
        assert_eq!(controller.camera.aspect_ratio, 16.0 / 9.0);
    }

    #[test]
    fn test_zoom_in() {
        let mut controller = OrbitController::new();
        let initial_distance = controller.target_distance;

        // Positive delta zooms in (closer)
        controller.zoom(1.0);

        assert!(controller.target_distance < initial_distance);
    }

    #[test]
    fn test_zoom_out() {
        let mut controller = OrbitController::new();
        let initial_distance = controller.target_distance;

        // Negative delta zooms out (farther)
        controller.zoom(-1.0);

        assert!(controller.target_distance > initial_distance);
    }

    #[test]
    fn test_zoom_min_clamp() {
        let mut controller = OrbitController::new();

        // Try to zoom in past minimum
        controller.zoom(1000.0); // Very large zoom in

        assert_eq!(controller.target_distance, controller.min_distance);
    }

    #[test]
    fn test_zoom_max_clamp() {
        let mut controller = OrbitController::new();

        // Try to zoom out past maximum
        controller.zoom(-1000.0); // Very large zoom out

        assert_eq!(controller.target_distance, controller.max_distance);
    }

    #[test]
    fn test_zoom_to() {
        let mut controller = OrbitController::new();

        controller.zoom_to(7.0);
        assert_eq!(controller.target_distance, 7.0);

        // Should clamp to min
        controller.zoom_to(1.0);
        assert_eq!(controller.target_distance, controller.min_distance);

        // Should clamp to max
        controller.zoom_to(20.0);
        assert_eq!(controller.target_distance, controller.max_distance);
    }

    #[test]
    fn test_pinch_zoom_in() {
        let mut controller = OrbitController::new();
        let initial_distance = controller.target_distance;

        // scale > 1.0 means zoom in
        controller.pinch_zoom(1.2);

        assert!(controller.target_distance < initial_distance);
    }

    #[test]
    fn test_pinch_zoom_out() {
        let mut controller = OrbitController::new();
        let initial_distance = controller.target_distance;

        // scale < 1.0 means zoom out
        controller.pinch_zoom(0.8);

        assert!(controller.target_distance > initial_distance);
    }

    #[test]
    fn test_pinch_zoom_clamping() {
        let mut controller = OrbitController::new();

        // Try to pinch zoom in very far
        controller.pinch_zoom(100.0);
        assert!(controller.target_distance >= controller.min_distance);
        assert!(controller.target_distance <= controller.max_distance);

        // Try to pinch zoom out very far
        controller.pinch_zoom(0.01);
        assert!(controller.target_distance >= controller.min_distance);
        assert!(controller.target_distance <= controller.max_distance);
    }

    #[test]
    fn test_smooth_zoom_update() {
        let mut controller = OrbitController::new();
        controller.target_distance = 10.0;
        controller.camera.distance = 5.0;

        // Update once
        controller.update();

        // Camera distance should have moved towards target but not reached it
        assert!(controller.camera.distance > 5.0);
        assert!(controller.camera.distance < 10.0);
    }

    #[test]
    fn test_zoom_sensitivity() {
        let mut controller = OrbitController::new();
        let sensitivity = controller.zoom_sensitivity;
        let initial_distance = controller.target_distance;

        controller.zoom(1.0);

        // Change should be 1.0 * sensitivity
        assert!((controller.target_distance - (initial_distance - sensitivity)).abs() < 0.001);
    }

    #[test]
    fn test_zoom_reset() {
        let mut controller = OrbitController::new();

        // Change zoom
        controller.zoom(3.0);
        controller.update();

        // Reset
        controller.reset();

        // Should be back to default
        let default_camera = Camera::default();
        assert_eq!(controller.target_distance, default_camera.distance);
    }

    #[test]
    fn test_zoom_limits_configurable() {
        let mut controller = OrbitController::new();
        controller.min_distance = 1.0;
        controller.max_distance = 20.0;

        controller.zoom_to(0.5);
        assert_eq!(controller.target_distance, 1.0);

        controller.zoom_to(25.0);
        assert_eq!(controller.target_distance, 20.0);
    }
}
