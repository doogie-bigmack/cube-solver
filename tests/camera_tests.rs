//! Integration tests for camera orbit controls

use rubiks_cube_solver::renderer::{Camera, OrbitController};

#[test]
fn test_camera_orbit_full_rotation() {
    let mut camera = Camera::new();
    let initial_pos = camera.position();

    // Rotate 360 degrees around azimuth
    camera.rotate(2.0 * std::f32::consts::PI, 0.0);

    // Should be back to approximately same position
    let final_pos = camera.position();
    let diff = (final_pos - initial_pos).length();
    assert!(diff < 0.1, "Full rotation should return to similar position");
}

#[test]
fn test_camera_view_matrices_consistency() {
    let camera = Camera::default();

    // Multiple calls should return same matrix
    let view1 = camera.view_matrix();
    let view2 = camera.view_matrix();

    assert_eq!(view1, view2);

    let proj1 = camera.projection_matrix();
    let proj2 = camera.projection_matrix();

    assert_eq!(proj1, proj2);
}

#[test]
fn test_orbit_controller_drag_simulation() {
    let mut controller = OrbitController::new();
    let initial_azimuth = controller.camera.azimuth;

    // Simulate dragging across screen
    controller.start_drag(0.0, 0.0);
    for i in 1..=100 {
        controller.update_drag(i as f32 * 5.0, 0.0);
    }
    controller.end_drag();

    // Target should have changed significantly
    assert!((controller.target_azimuth - initial_azimuth).abs() > 1.0);
}

#[test]
fn test_orbit_controller_smooth_convergence() {
    let mut controller = OrbitController::new();
    controller.target_azimuth = controller.camera.azimuth + 1.0;

    // Update many times
    for _ in 0..100 {
        controller.update();
    }

    // Should have converged to target
    let diff = (controller.camera.azimuth - controller.target_azimuth).abs();
    assert!(diff < 0.01, "Camera should converge to target angle");
}

#[test]
fn test_orbit_controller_vertical_drag() {
    let mut controller = OrbitController::new();
    let initial_elevation = controller.camera.elevation;

    // Drag vertically
    controller.start_drag(100.0, 100.0);
    controller.update_drag(100.0, 0.0); // 100 pixels up
    controller.end_drag();

    // Elevation should have increased
    assert!(
        controller.target_elevation > initial_elevation,
        "Upward drag should increase elevation"
    );
}

#[test]
fn test_orbit_controller_diagonal_drag() {
    let mut controller = OrbitController::new();
    let initial_azimuth = controller.camera.azimuth;
    let initial_elevation = controller.camera.elevation;

    // Drag diagonally
    controller.start_drag(100.0, 100.0);
    controller.update_drag(200.0, 50.0); // Right and up
    controller.end_drag();

    // Both angles should have changed
    assert!(
        controller.target_azimuth != initial_azimuth,
        "Azimuth should change"
    );
    assert!(
        controller.target_elevation != initial_elevation,
        "Elevation should change"
    );
}

#[test]
fn test_camera_distance_affects_position() {
    let mut camera1 = Camera::new();
    camera1.distance = 5.0;

    let mut camera2 = Camera::new();
    camera2.distance = 10.0;

    let pos1 = camera1.position();
    let pos2 = camera2.position();

    let dist1 = pos1.length();
    let dist2 = pos2.length();

    assert!(dist2 > dist1, "Larger distance should place camera farther");
    assert!((dist2 / dist1 - 2.0).abs() < 0.1, "Distance should scale linearly");
}

#[test]
fn test_camera_fov_affects_projection() {
    let mut camera1 = Camera::new();
    camera1.fov = std::f32::consts::PI / 4.0; // 45 degrees

    let mut camera2 = Camera::new();
    camera2.fov = std::f32::consts::PI / 3.0; // 60 degrees

    let proj1 = camera1.projection_matrix();
    let proj2 = camera2.projection_matrix();

    assert_ne!(proj1, proj2, "Different FOV should produce different projection");
}

#[test]
fn test_orbit_controller_multiple_drags() {
    let mut controller = OrbitController::new();

    // First drag
    controller.start_drag(0.0, 0.0);
    controller.update_drag(100.0, 0.0);
    controller.end_drag();

    let azimuth_after_first = controller.target_azimuth;

    // Second drag
    controller.start_drag(100.0, 0.0);
    controller.update_drag(200.0, 0.0);
    controller.end_drag();

    // Azimuth should have changed again
    assert!(
        controller.target_azimuth > azimuth_after_first,
        "Second drag should continue rotation"
    );
}

#[test]
fn test_orbit_controller_aspect_ratio_propagation() {
    let mut controller = OrbitController::new();
    controller.set_aspect_ratio(2.0);

    assert_eq!(
        controller.camera.aspect_ratio, 2.0,
        "Aspect ratio should propagate to camera"
    );

    // Projection should reflect aspect ratio
    let proj = controller.camera.projection_matrix();
    assert!(!proj.is_nan());
}

#[test]
fn test_camera_reset_preserves_aspect() {
    let mut camera = Camera::new();
    camera.set_aspect_ratio(16.0 / 9.0);
    camera.rotate(1.0, 1.0);

    camera.reset();

    assert_eq!(
        camera.aspect_ratio,
        16.0 / 9.0,
        "Reset should preserve aspect ratio"
    );
}

#[test]
fn test_orbit_controller_sensitivity() {
    let mut controller1 = OrbitController::new();
    controller1.rotation_sensitivity = 0.01;

    let mut controller2 = OrbitController::new();
    controller2.rotation_sensitivity = 0.001;

    // Same drag
    controller1.start_drag(0.0, 0.0);
    controller1.update_drag(100.0, 0.0);
    let delta1 = controller1.target_azimuth - controller1.camera.azimuth;

    controller2.start_drag(0.0, 0.0);
    controller2.update_drag(100.0, 0.0);
    let delta2 = controller2.target_azimuth - controller2.camera.azimuth;

    assert!(
        delta1.abs() > delta2.abs(),
        "Higher sensitivity should produce larger rotation"
    );
}
