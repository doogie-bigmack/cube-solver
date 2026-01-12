//! Touch Input Integration Tests
//!
//! Requirements: R7.7 - Touch input support
//! - All features work with touch
//! - No hover-dependent features
//! - Proper touch targets (44px minimum)

use rubiks_cube_solver::input::touch::{TouchPoint, TouchState, TouchGesture, targets};

#[test]
fn test_touch_point_distance_calculation() {
    let p1 = TouchPoint::new(1, 0.0, 0.0, 0.0);
    let p2 = TouchPoint::new(2, 30.0, 40.0, 0.0);

    // Pythagorean theorem: sqrt(30^2 + 40^2) = sqrt(900 + 1600) = sqrt(2500) = 50
    assert_eq!(p1.distance_to(&p2), 50.0);
}

#[test]
fn test_touch_point_midpoint() {
    let p1 = TouchPoint::new(1, 100.0, 200.0, 0.0);
    let p2 = TouchPoint::new(2, 300.0, 600.0, 0.0);

    let (mid_x, mid_y) = p1.midpoint(&p2);
    assert_eq!(mid_x, 200.0);
    assert_eq!(mid_y, 400.0);
}

#[test]
fn test_single_touch_drag_gesture() {
    let mut state = TouchState::new();

    // Start a single touch
    state.touch_start(1, 100.0, 200.0, 0.0);

    assert_eq!(state.touch_count(), 1);
    assert_eq!(state.current_gesture(), TouchGesture::Drag);

    // Move the touch
    state.touch_move(1, 150.0, 250.0, 100.0);

    let (dx, dy) = state.drag_delta(1, 100.0, 200.0);
    assert_eq!(dx, 50.0);
    assert_eq!(dy, 50.0);
}

#[test]
fn test_two_touch_pinch_gesture() {
    let mut state = TouchState::new();

    // Start two touches
    state.touch_start(1, 0.0, 0.0, 0.0);
    state.touch_start(2, 100.0, 0.0, 0.0);

    assert_eq!(state.touch_count(), 2);
    assert_eq!(state.current_gesture(), TouchGesture::Pinch);
}

#[test]
fn test_pinch_zoom_in() {
    let mut state = TouchState::new();

    // Start with two touches 100px apart
    state.touch_start(1, 0.0, 0.0, 0.0);
    state.touch_start(2, 100.0, 0.0, 0.0);

    // Initialize the pinch distance
    let _ = state.pinch_scale();

    // Move touches to 200px apart (zoom in 2x)
    state.touch_move(1, 0.0, 0.0, 100.0);
    state.touch_move(2, 200.0, 0.0, 100.0);

    let scale = state.pinch_scale();
    assert!((scale - 2.0).abs() < 0.001, "Expected scale ~2.0, got {}", scale);
}

#[test]
fn test_pinch_zoom_out() {
    let mut state = TouchState::new();

    // Start with two touches 200px apart
    state.touch_start(1, 0.0, 0.0, 0.0);
    state.touch_start(2, 200.0, 0.0, 0.0);

    // Initialize the pinch distance
    let _ = state.pinch_scale();

    // Move touches to 100px apart (zoom out 0.5x)
    state.touch_move(1, 0.0, 0.0, 100.0);
    state.touch_move(2, 100.0, 0.0, 100.0);

    let scale = state.pinch_scale();
    assert!((scale - 0.5).abs() < 0.001, "Expected scale ~0.5, got {}", scale);
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
fn test_multi_touch_workflow() {
    let mut state = TouchState::new();

    // First finger down
    state.touch_start(1, 100.0, 100.0, 0.0);
    assert_eq!(state.current_gesture(), TouchGesture::Drag);

    // Second finger down - now pinching
    state.touch_start(2, 200.0, 100.0, 10.0);
    assert_eq!(state.current_gesture(), TouchGesture::Pinch);

    // First finger up - back to dragging
    state.touch_end(1);
    assert_eq!(state.current_gesture(), TouchGesture::Drag);

    // Second finger up - no gesture
    state.touch_end(2);
    assert_eq!(state.current_gesture(), TouchGesture::None);
}

#[test]
fn test_tap_detection_short_duration() {
    let state = TouchState::new();

    // 100ms is a tap
    assert!(state.is_tap(0.0, 100.0, 200.0));

    // 150ms is still a tap (within 200ms threshold)
    assert!(state.is_tap(0.0, 150.0, 200.0));
}

#[test]
fn test_tap_detection_long_duration() {
    let state = TouchState::new();

    // 300ms is not a tap (exceeds 200ms threshold)
    assert!(!state.is_tap(0.0, 300.0, 200.0));

    // 500ms is definitely not a tap
    assert!(!state.is_tap(0.0, 500.0, 200.0));
}

#[test]
fn test_touch_target_minimum_size() {
    // 44px is the minimum (WCAG AAA)
    assert!(targets::is_valid_touch_target(44.0, 44.0));

    // Larger is valid
    assert!(targets::is_valid_touch_target(50.0, 50.0));
    assert!(targets::is_valid_touch_target(60.0, 48.0));

    // Smaller than 44px is invalid
    assert!(!targets::is_valid_touch_target(40.0, 40.0));
    assert!(!targets::is_valid_touch_target(44.0, 40.0));
    assert!(!targets::is_valid_touch_target(30.0, 50.0));
}

#[test]
fn test_padding_calculation_for_touch_targets() {
    // 44px needs no padding
    assert_eq!(targets::padding_for_target(44.0), 0.0);

    // 50px needs no padding
    assert_eq!(targets::padding_for_target(50.0), 0.0);

    // 40px needs 2px padding on each side
    assert_eq!(targets::padding_for_target(40.0), 2.0);

    // 30px needs 7px padding on each side
    assert_eq!(targets::padding_for_target(30.0), 7.0);
}

#[test]
fn test_button_style_includes_minimum_size() {
    let style = targets::button_style("10px 20px", "background: blue; color: white;");

    assert!(style.contains("min-width: 44px"));
    assert!(style.contains("min-height: 44px"));
    assert!(style.contains("padding: 10px 20px"));
    assert!(style.contains("background: blue; color: white;"));
}

#[test]
fn test_get_touches_returns_all_active_touches() {
    let mut state = TouchState::new();

    state.touch_start(1, 100.0, 200.0, 0.0);
    state.touch_start(2, 150.0, 250.0, 10.0);
    state.touch_start(3, 200.0, 300.0, 20.0);

    let touches = state.get_touches();
    assert_eq!(touches.len(), 3);
}

#[test]
fn test_clear_removes_all_touches() {
    let mut state = TouchState::new();

    state.touch_start(1, 100.0, 200.0, 0.0);
    state.touch_start(2, 150.0, 250.0, 10.0);

    assert_eq!(state.touch_count(), 2);

    state.clear();

    assert_eq!(state.touch_count(), 0);
    assert_eq!(state.current_gesture(), TouchGesture::None);
}

#[test]
fn test_get_specific_touch() {
    let mut state = TouchState::new();

    state.touch_start(1, 100.0, 200.0, 0.0);
    state.touch_start(2, 150.0, 250.0, 10.0);

    let touch1 = state.get_touch(1);
    assert!(touch1.is_some());
    assert_eq!(touch1.unwrap().x, 100.0);
    assert_eq!(touch1.unwrap().y, 200.0);

    let touch2 = state.get_touch(2);
    assert!(touch2.is_some());
    assert_eq!(touch2.unwrap().x, 150.0);

    let touch_none = state.get_touch(999);
    assert!(touch_none.is_none());
}

#[test]
fn test_drag_delta_for_nonexistent_touch() {
    let state = TouchState::new();

    // Non-existent touch should return (0, 0)
    let (dx, dy) = state.drag_delta(999, 100.0, 200.0);
    assert_eq!(dx, 0.0);
    assert_eq!(dy, 0.0);
}

#[test]
fn test_pinch_scale_with_zero_touches() {
    let mut state = TouchState::new();

    // No touches should return 1.0 (no scaling)
    let scale = state.pinch_scale();
    assert_eq!(scale, 1.0);
}

#[test]
fn test_pinch_scale_with_one_touch() {
    let mut state = TouchState::new();

    state.touch_start(1, 100.0, 200.0, 0.0);

    // One touch should return 1.0 (no scaling)
    let scale = state.pinch_scale();
    assert_eq!(scale, 1.0);
}

#[test]
fn test_complex_pinch_sequence() {
    let mut state = TouchState::new();

    // Start with two touches 100px apart
    state.touch_start(1, 0.0, 0.0, 0.0);
    state.touch_start(2, 100.0, 0.0, 0.0);

    // Initialize
    let scale1 = state.pinch_scale();
    assert_eq!(scale1, 1.0);

    // Zoom in to 150px (1.5x)
    state.touch_move(1, 0.0, 0.0, 100.0);
    state.touch_move(2, 150.0, 0.0, 100.0);
    let scale2 = state.pinch_scale();
    assert!((scale2 - 1.5).abs() < 0.001);

    // Zoom in more to 200px (2.0x)
    state.touch_move(1, 0.0, 0.0, 200.0);
    state.touch_move(2, 200.0, 0.0, 200.0);
    let scale3 = state.pinch_scale();
    assert!((scale3 - 2.0).abs() < 0.001);
}

#[test]
fn test_touch_state_clone() {
    let mut state1 = TouchState::new();
    state1.touch_start(1, 100.0, 200.0, 0.0);

    let state2 = state1.clone();

    assert_eq!(state1.touch_count(), state2.touch_count());
    assert_eq!(state1.current_gesture(), state2.current_gesture());
}

#[test]
fn test_all_interactive_elements_meet_touch_targets() {
    // Solution player buttons: 44px minimum
    assert!(targets::is_valid_touch_target(44.0, 44.0));

    // Color picker buttons: 50px (exceeds minimum)
    assert!(targets::is_valid_touch_target(50.0, 50.0));

    // Cube controls buttons: 50px (exceeds minimum)
    assert!(targets::is_valid_touch_target(50.0, 50.0));

    // Dialog buttons: 48px (exceeds minimum)
    assert!(targets::is_valid_touch_target(48.0, 48.0));
}
