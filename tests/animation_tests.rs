use rubiks_cube_solver::renderer::{
    AnimationQueue, EasingFunction, RotationAnimation, RotationFace,
};
use std::thread;
use std::time::Duration;

// Test that animations complete in expected time
#[test]
fn test_animation_timing() {
    let mut anim = RotationAnimation::rotate_cw(RotationFace::R, Duration::from_millis(100));
    anim.start();

    // Initially should not be complete
    assert!(!anim.is_complete());

    // After half the duration, should still not be complete
    thread::sleep(Duration::from_millis(50));
    assert!(!anim.is_complete());

    // After full duration, should be complete
    thread::sleep(Duration::from_millis(60));
    assert!(anim.is_complete());
}

// Test that different easing functions produce different results
#[test]
fn test_easing_differences() {
    let linear = EasingFunction::Linear;
    let ease_in = EasingFunction::EaseIn;
    let ease_out = EasingFunction::EaseOut;

    let t = 0.5;
    let linear_val = linear.apply(t);
    let ease_in_val = ease_in.apply(t);
    let ease_out_val = ease_out.apply(t);

    // Linear should be exactly 0.5
    assert_eq!(linear_val, 0.5);

    // Ease in should be less than 0.5 (starts slow)
    assert!(ease_in_val < 0.5);

    // Ease out should be greater than 0.5 (starts fast)
    assert!(ease_out_val > 0.5);
}

// Test animation queue processes animations sequentially
#[test]
fn test_queue_sequential_processing() {
    let mut queue = AnimationQueue::new();

    // Add three animations
    queue.enqueue(RotationAnimation::rotate_cw(
        RotationFace::R,
        Duration::from_millis(50),
    ));
    queue.enqueue(RotationAnimation::rotate_cw(
        RotationFace::U,
        Duration::from_millis(50),
    ));
    queue.enqueue(RotationAnimation::rotate_cw(
        RotationFace::F,
        Duration::from_millis(50),
    ));

    assert_eq!(queue.queued_count(), 3);

    // First update should start R
    queue.update();
    assert_eq!(queue.current().unwrap().face, RotationFace::R);
    assert_eq!(queue.queued_count(), 2);

    // Wait for R to complete
    thread::sleep(Duration::from_millis(60));

    // Next update should start U
    queue.update();
    assert_eq!(queue.current().unwrap().face, RotationFace::U);
    assert_eq!(queue.queued_count(), 1);

    // Wait for U to complete
    thread::sleep(Duration::from_millis(60));

    // Next update should start F
    queue.update();
    assert_eq!(queue.current().unwrap().face, RotationFace::F);
    assert_eq!(queue.queued_count(), 0);
}

// Test configurable duration
#[test]
fn test_configurable_duration() {
    let short = RotationAnimation::rotate_cw(RotationFace::R, Duration::from_millis(50));
    let long = RotationAnimation::rotate_cw(RotationFace::R, Duration::from_millis(200));

    assert_eq!(short.duration, Duration::from_millis(50));
    assert_eq!(long.duration, Duration::from_millis(200));
}

// Test 90-degree rotation
#[test]
fn test_90_degree_rotation() {
    let mut anim = RotationAnimation::rotate_cw(RotationFace::R, Duration::from_millis(100));
    anim.start();
    thread::sleep(Duration::from_millis(110));

    let angle = anim.current_angle().unwrap();
    assert!((angle - 90.0).abs() < 0.1);
}

// Test 180-degree rotation
#[test]
fn test_180_degree_rotation() {
    let mut anim = RotationAnimation::rotate_180(RotationFace::F, Duration::from_millis(100));
    anim.start();
    thread::sleep(Duration::from_millis(110));

    let angle = anim.current_angle().unwrap();
    assert!((angle - 180.0).abs() < 0.1);
}

// Test counter-clockwise rotation (negative angle)
#[test]
fn test_counter_clockwise_rotation() {
    let mut anim = RotationAnimation::rotate_ccw(RotationFace::L, Duration::from_millis(100));
    anim.start();
    thread::sleep(Duration::from_millis(110));

    let angle = anim.current_angle().unwrap();
    assert!((angle + 90.0).abs() < 0.1); // Should be -90
}

// Test that animation progress is clamped to 1.0
#[test]
fn test_progress_clamped() {
    let mut anim = RotationAnimation::rotate_cw(RotationFace::R, Duration::from_millis(50));
    anim.start();
    thread::sleep(Duration::from_millis(200)); // Wait way longer than duration

    let progress = anim.progress().unwrap();
    assert_eq!(progress, 1.0); // Should be clamped
}

// Test all rotation faces are supported
#[test]
fn test_all_rotation_faces() {
    let faces = vec![
        RotationFace::R,
        RotationFace::L,
        RotationFace::U,
        RotationFace::D,
        RotationFace::F,
        RotationFace::B,
        RotationFace::M,
        RotationFace::E,
        RotationFace::S,
    ];

    for face in faces {
        let anim = RotationAnimation::rotate_cw(face, Duration::from_millis(100));
        assert_eq!(anim.face, face);
    }
}

// Test animation queue can handle many animations
#[test]
fn test_queue_many_animations() {
    let mut queue = AnimationQueue::new();

    // Add 100 animations
    for _ in 0..100 {
        queue.enqueue(RotationAnimation::rotate_cw(
            RotationFace::R,
            Duration::from_millis(1),
        ));
    }

    assert_eq!(queue.queued_count(), 100);
    assert!(queue.has_animations());
}

// Test queue clear removes all animations
#[test]
fn test_queue_clear_all() {
    let mut queue = AnimationQueue::new();

    queue.enqueue_multiple(vec![
        RotationAnimation::rotate_cw(RotationFace::R, Duration::from_millis(100)),
        RotationAnimation::rotate_cw(RotationFace::U, Duration::from_millis(100)),
        RotationAnimation::rotate_cw(RotationFace::F, Duration::from_millis(100)),
    ]);

    queue.update(); // Start first animation
    assert!(queue.has_animations());
    assert!(queue.current().is_some());

    queue.clear();
    assert!(!queue.has_animations());
    assert!(queue.current().is_none());
    assert_eq!(queue.queued_count(), 0);
}

// Test EaseInOut function
#[test]
fn test_ease_in_out_symmetry() {
    let easing = EasingFunction::EaseInOut;

    // At the start, should be close to 0
    assert!(easing.apply(0.0).abs() < 0.01);

    // At the midpoint, should be close to 0.5
    let mid = easing.apply(0.5);
    assert!((mid - 0.5).abs() < 0.1);

    // At the end, should be close to 1.0
    assert!((easing.apply(1.0) - 1.0).abs() < 0.01);
}

// Test animation doesn't start until explicitly started
#[test]
fn test_animation_manual_start() {
    let anim = RotationAnimation::rotate_cw(RotationFace::R, Duration::from_millis(100));

    assert!(!anim.is_started());
    assert!(anim.progress().is_none());
    assert!(anim.current_angle().is_none());
}

// Test custom easing and angle
#[test]
fn test_custom_animation() {
    let mut anim = RotationAnimation::new(
        RotationFace::B,
        270.0, // 270 degrees
        Duration::from_millis(150),
        EasingFunction::EaseIn,
    );

    anim.start();
    thread::sleep(Duration::from_millis(160));

    assert!(anim.is_complete());
    let angle = anim.current_angle().unwrap();
    assert!((angle - 270.0).abs() < 1.0);
}
