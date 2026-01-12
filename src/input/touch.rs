//! Touch Input Support Module
//!
//! This module provides comprehensive touch input handling for the Rubik's Cube application.
//!
//! Requirements: R7.7 - Touch input support
//! - All features work with touch
//! - No hover-dependent features
//! - Proper touch targets (44px minimum)

use std::collections::HashMap;

/// Represents a touch point with its coordinates and identifier
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchPoint {
    /// Unique identifier for this touch
    pub id: i32,
    /// X coordinate in pixels
    pub x: f64,
    /// Y coordinate in pixels
    pub y: f64,
    /// Timestamp when touch occurred
    pub timestamp: f64,
}

impl TouchPoint {
    /// Creates a new touch point
    pub fn new(id: i32, x: f64, y: f64, timestamp: f64) -> Self {
        Self { id, x, y, timestamp }
    }

    /// Calculate distance to another touch point
    pub fn distance_to(&self, other: &TouchPoint) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate the midpoint between this and another touch point
    pub fn midpoint(&self, other: &TouchPoint) -> (f64, f64) {
        ((self.x + other.x) / 2.0, (self.y + other.y) / 2.0)
    }
}

/// Manages touch state for multi-touch gestures
#[derive(Debug, Clone)]
pub struct TouchState {
    /// Active touches by their identifier
    active_touches: HashMap<i32, TouchPoint>,
    /// Initial distance between two touches (for pinch zoom)
    initial_pinch_distance: Option<f64>,
    /// Current gesture being performed
    current_gesture: TouchGesture,
}

/// Types of touch gestures supported
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchGesture {
    /// No active gesture
    None,
    /// Single finger drag
    Drag,
    /// Two finger pinch/zoom
    Pinch,
    /// Single tap
    Tap,
}

impl Default for TouchState {
    fn default() -> Self {
        Self::new()
    }
}

impl TouchState {
    /// Creates a new touch state
    pub fn new() -> Self {
        Self {
            active_touches: HashMap::new(),
            initial_pinch_distance: None,
            current_gesture: TouchGesture::None,
        }
    }

    /// Registers a new touch start event
    pub fn touch_start(&mut self, id: i32, x: f64, y: f64, timestamp: f64) {
        let point = TouchPoint::new(id, x, y, timestamp);
        self.active_touches.insert(id, point);

        // Update gesture based on number of active touches
        self.update_gesture();
    }

    /// Updates an existing touch point
    pub fn touch_move(&mut self, id: i32, x: f64, y: f64, timestamp: f64) {
        if let Some(touch) = self.active_touches.get_mut(&id) {
            touch.x = x;
            touch.y = y;
            touch.timestamp = timestamp;
        }
    }

    /// Removes a touch point
    pub fn touch_end(&mut self, id: i32) {
        self.active_touches.remove(&id);

        // Reset gesture if no more touches
        if self.active_touches.is_empty() {
            self.current_gesture = TouchGesture::None;
            self.initial_pinch_distance = None;
        } else {
            self.update_gesture();
        }
    }

    /// Clears all touch points
    pub fn clear(&mut self) {
        self.active_touches.clear();
        self.current_gesture = TouchGesture::None;
        self.initial_pinch_distance = None;
    }

    /// Gets the current gesture type
    pub fn current_gesture(&self) -> TouchGesture {
        self.current_gesture
    }

    /// Gets the number of active touches
    pub fn touch_count(&self) -> usize {
        self.active_touches.len()
    }

    /// Gets all active touch points
    pub fn get_touches(&self) -> Vec<TouchPoint> {
        self.active_touches.values().copied().collect()
    }

    /// Gets a specific touch point by ID
    pub fn get_touch(&self, id: i32) -> Option<TouchPoint> {
        self.active_touches.get(&id).copied()
    }

    /// Calculates the current pinch scale factor
    /// Returns 1.0 if not pinching, >1.0 for zoom in, <1.0 for zoom out
    pub fn pinch_scale(&mut self) -> f64 {
        if self.active_touches.len() != 2 {
            return 1.0;
        }

        let touches: Vec<TouchPoint> = self.active_touches.values().copied().collect();
        let current_distance = touches[0].distance_to(&touches[1]);

        // Initialize initial distance if this is the first pinch measurement
        if self.initial_pinch_distance.is_none() {
            self.initial_pinch_distance = Some(current_distance);
            return 1.0;
        }

        let initial = self.initial_pinch_distance.unwrap();
        if initial > 0.0 {
            current_distance / initial
        } else {
            1.0
        }
    }

    /// Gets the drag delta for single-touch drag
    /// Returns (dx, dy) or (0.0, 0.0) if not dragging
    pub fn drag_delta(&self, id: i32, start_x: f64, start_y: f64) -> (f64, f64) {
        if let Some(touch) = self.active_touches.get(&id) {
            (touch.x - start_x, touch.y - start_y)
        } else {
            (0.0, 0.0)
        }
    }

    /// Checks if a tap gesture occurred (touch up shortly after touch down)
    pub fn is_tap(&self, start_timestamp: f64, end_timestamp: f64, max_duration_ms: f64) -> bool {
        let duration = end_timestamp - start_timestamp;
        duration < max_duration_ms
    }

    /// Updates the current gesture based on active touches
    fn update_gesture(&mut self) {
        self.current_gesture = match self.active_touches.len() {
            0 => TouchGesture::None,
            1 => TouchGesture::Drag, // Could be drag or tap, determined later
            2 => TouchGesture::Pinch,
            _ => TouchGesture::None, // Ignore 3+ finger touches for now
        };
    }
}

/// Touch target size validation
pub mod targets {
    /// Minimum touch target size in pixels (WCAG AAA guideline)
    pub const MIN_TOUCH_TARGET_SIZE: f32 = 44.0;

    /// Checks if a size meets minimum touch target requirements
    pub fn is_valid_touch_target(width: f32, height: f32) -> bool {
        width >= MIN_TOUCH_TARGET_SIZE && height >= MIN_TOUCH_TARGET_SIZE
    }

    /// Calculates the minimum padding needed to reach 44px
    pub fn padding_for_target(current_size: f32) -> f32 {
        if current_size >= MIN_TOUCH_TARGET_SIZE {
            0.0
        } else {
            (MIN_TOUCH_TARGET_SIZE - current_size) / 2.0
        }
    }

    /// Gets touch-friendly button style with minimum size
    pub fn button_style(base_padding: &str, additional_props: &str) -> String {
        format!(
            "padding: {}; min-width: {}px; min-height: {}px; {}",
            base_padding, MIN_TOUCH_TARGET_SIZE, MIN_TOUCH_TARGET_SIZE, additional_props
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touch_point_creation() {
        let point = TouchPoint::new(1, 100.0, 200.0, 0.0);
        assert_eq!(point.id, 1);
        assert_eq!(point.x, 100.0);
        assert_eq!(point.y, 200.0);
        assert_eq!(point.timestamp, 0.0);
    }

    #[test]
    fn test_touch_point_distance() {
        let p1 = TouchPoint::new(1, 0.0, 0.0, 0.0);
        let p2 = TouchPoint::new(2, 3.0, 4.0, 0.0);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_touch_point_midpoint() {
        let p1 = TouchPoint::new(1, 0.0, 0.0, 0.0);
        let p2 = TouchPoint::new(2, 10.0, 20.0, 0.0);
        assert_eq!(p1.midpoint(&p2), (5.0, 10.0));
    }

    #[test]
    fn test_touch_state_new() {
        let state = TouchState::new();
        assert_eq!(state.touch_count(), 0);
        assert_eq!(state.current_gesture(), TouchGesture::None);
    }

    #[test]
    fn test_single_touch_drag() {
        let mut state = TouchState::new();
        state.touch_start(1, 100.0, 200.0, 0.0);

        assert_eq!(state.touch_count(), 1);
        assert_eq!(state.current_gesture(), TouchGesture::Drag);
    }

    #[test]
    fn test_two_touch_pinch() {
        let mut state = TouchState::new();
        state.touch_start(1, 0.0, 0.0, 0.0);
        state.touch_start(2, 100.0, 0.0, 0.0);

        assert_eq!(state.touch_count(), 2);
        assert_eq!(state.current_gesture(), TouchGesture::Pinch);
    }

    #[test]
    fn test_pinch_scale() {
        let mut state = TouchState::new();

        // Start with two touches 100px apart
        state.touch_start(1, 0.0, 0.0, 0.0);
        state.touch_start(2, 100.0, 0.0, 0.0);

        // First call initializes the distance
        let scale1 = state.pinch_scale();
        assert_eq!(scale1, 1.0);

        // Move touches to 200px apart (2x zoom)
        state.touch_move(1, 0.0, 0.0, 1.0);
        state.touch_move(2, 200.0, 0.0, 1.0);
        let scale2 = state.pinch_scale();
        assert!((scale2 - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_drag_delta() {
        let mut state = TouchState::new();
        state.touch_start(1, 100.0, 200.0, 0.0);
        state.touch_move(1, 150.0, 250.0, 1.0);

        let (dx, dy) = state.drag_delta(1, 100.0, 200.0);
        assert_eq!(dx, 50.0);
        assert_eq!(dy, 50.0);
    }

    #[test]
    fn test_touch_end_clears_gesture() {
        let mut state = TouchState::new();
        state.touch_start(1, 100.0, 200.0, 0.0);
        assert_eq!(state.current_gesture(), TouchGesture::Drag);

        state.touch_end(1);
        assert_eq!(state.touch_count(), 0);
        assert_eq!(state.current_gesture(), TouchGesture::None);
    }

    #[test]
    fn test_is_tap() {
        let state = TouchState::new();
        // 100ms tap
        assert!(state.is_tap(0.0, 100.0, 200.0));
        // 300ms is not a tap
        assert!(!state.is_tap(0.0, 300.0, 200.0));
    }

    #[test]
    fn test_valid_touch_target() {
        assert!(targets::is_valid_touch_target(44.0, 44.0));
        assert!(targets::is_valid_touch_target(50.0, 50.0));
        assert!(!targets::is_valid_touch_target(40.0, 40.0));
        assert!(!targets::is_valid_touch_target(44.0, 40.0));
    }

    #[test]
    fn test_padding_for_target() {
        assert_eq!(targets::padding_for_target(44.0), 0.0);
        assert_eq!(targets::padding_for_target(40.0), 2.0);
        assert_eq!(targets::padding_for_target(34.0), 5.0);
    }

    #[test]
    fn test_button_style() {
        let style = targets::button_style("10px 20px", "background: blue;");
        assert!(style.contains("min-width: 44px"));
        assert!(style.contains("min-height: 44px"));
        assert!(style.contains("padding: 10px 20px"));
        assert!(style.contains("background: blue;"));
    }

    #[test]
    fn test_clear_resets_state() {
        let mut state = TouchState::new();
        state.touch_start(1, 100.0, 200.0, 0.0);
        state.touch_start(2, 150.0, 250.0, 1.0);

        state.clear();
        assert_eq!(state.touch_count(), 0);
        assert_eq!(state.current_gesture(), TouchGesture::None);
    }

    #[test]
    fn test_get_touch() {
        let mut state = TouchState::new();
        state.touch_start(1, 100.0, 200.0, 0.0);

        let touch = state.get_touch(1);
        assert!(touch.is_some());
        assert_eq!(touch.unwrap().x, 100.0);
        assert_eq!(touch.unwrap().y, 200.0);

        assert!(state.get_touch(999).is_none());
    }

    #[test]
    fn test_get_touches() {
        let mut state = TouchState::new();
        state.touch_start(1, 100.0, 200.0, 0.0);
        state.touch_start(2, 150.0, 250.0, 1.0);

        let touches = state.get_touches();
        assert_eq!(touches.len(), 2);
    }
}
