//! Animation system for smooth cube rotations
//!
//! This module implements R2.5 from the PRD:
//! - Animate face rotating 90/180 degrees
//! - Easing function for smooth motion
//! - Configurable duration
//! - Queue multiple animations

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Type of easing function for animations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EasingFunction {
    /// Linear interpolation (no easing)
    Linear,
    /// Ease in (starts slow, ends fast)
    EaseIn,
    /// Ease out (starts fast, ends slow)
    EaseOut,
    /// Ease in-out (starts slow, fast in middle, ends slow)
    EaseInOut,
}

impl EasingFunction {
    /// Applies the easing function to a value between 0.0 and 1.0
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => t * (2.0 - t),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
        }
    }
}

/// Represents a single rotation animation
#[derive(Debug, Clone)]
pub struct RotationAnimation {
    /// Which face is rotating
    pub face: RotationFace,
    /// Angle to rotate in degrees (90, 180, or 270)
    pub angle_degrees: f32,
    /// Duration of the animation
    pub duration: Duration,
    /// Easing function to use
    pub easing: EasingFunction,
    /// When the animation started
    start_time: Option<Instant>,
}

/// Identifies which face/layer is being rotated
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationFace {
    R,
    L,
    U,
    D,
    F,
    B,
    M,
    E,
    S,
}

impl RotationAnimation {
    /// Creates a new rotation animation
    pub fn new(
        face: RotationFace,
        angle_degrees: f32,
        duration: Duration,
        easing: EasingFunction,
    ) -> Self {
        Self {
            face,
            angle_degrees,
            duration,
            easing,
            start_time: None,
        }
    }

    /// Creates a 90-degree clockwise rotation
    pub fn rotate_cw(face: RotationFace, duration: Duration) -> Self {
        Self::new(face, 90.0, duration, EasingFunction::EaseInOut)
    }

    /// Creates a 90-degree counter-clockwise rotation
    pub fn rotate_ccw(face: RotationFace, duration: Duration) -> Self {
        Self::new(face, -90.0, duration, EasingFunction::EaseInOut)
    }

    /// Creates a 180-degree rotation
    pub fn rotate_180(face: RotationFace, duration: Duration) -> Self {
        Self::new(face, 180.0, duration, EasingFunction::EaseInOut)
    }

    /// Starts the animation
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Returns the current progress of the animation (0.0 to 1.0)
    /// Returns None if animation hasn't started
    pub fn progress(&self) -> Option<f32> {
        let start = self.start_time?;
        let elapsed = start.elapsed();
        let progress = elapsed.as_secs_f32() / self.duration.as_secs_f32();
        Some(progress.min(1.0))
    }

    /// Returns the current rotation angle in degrees
    /// Returns None if animation hasn't started
    pub fn current_angle(&self) -> Option<f32> {
        let progress = self.progress()?;
        let eased_progress = self.easing.apply(progress);
        Some(self.angle_degrees * eased_progress)
    }

    /// Returns true if the animation is complete
    pub fn is_complete(&self) -> bool {
        self.progress().map_or(false, |p| p >= 1.0)
    }

    /// Returns true if the animation has started
    pub fn is_started(&self) -> bool {
        self.start_time.is_some()
    }
}

/// Animation queue manager
#[derive(Debug)]
pub struct AnimationQueue {
    /// Queue of pending animations
    queue: VecDeque<RotationAnimation>,
    /// Currently playing animation
    current: Option<RotationAnimation>,
}

impl Default for AnimationQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationQueue {
    /// Creates a new empty animation queue
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            current: None,
        }
    }

    /// Adds an animation to the queue
    pub fn enqueue(&mut self, animation: RotationAnimation) {
        self.queue.push_back(animation);
    }

    /// Adds multiple animations to the queue
    pub fn enqueue_multiple(&mut self, animations: impl IntoIterator<Item = RotationAnimation>) {
        for animation in animations {
            self.enqueue(animation);
        }
    }

    /// Updates the animation queue, starting new animations as needed
    /// Returns the currently playing animation, if any
    pub fn update(&mut self) -> Option<&RotationAnimation> {
        // If current animation is complete, move to next
        if let Some(ref current) = self.current {
            if current.is_complete() {
                self.current = None;
            }
        }

        // If no current animation, start next from queue
        if self.current.is_none() {
            if let Some(mut next) = self.queue.pop_front() {
                next.start();
                self.current = Some(next);
            }
        }

        self.current.as_ref()
    }

    /// Returns the currently playing animation without updating
    pub fn current(&self) -> Option<&RotationAnimation> {
        self.current.as_ref()
    }

    /// Returns the number of queued animations (not including current)
    pub fn queued_count(&self) -> usize {
        self.queue.len()
    }

    /// Returns true if there are any animations (current or queued)
    pub fn has_animations(&self) -> bool {
        self.current.is_some() || !self.queue.is_empty()
    }

    /// Clears all animations
    pub fn clear(&mut self) {
        self.queue.clear();
        self.current = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_easing_function_linear() {
        let easing = EasingFunction::Linear;
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(0.5), 0.5);
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_function_ease_in() {
        let easing = EasingFunction::EaseIn;
        assert_eq!(easing.apply(0.0), 0.0);
        assert!(easing.apply(0.5) < 0.5); // Starts slow
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_function_ease_out() {
        let easing = EasingFunction::EaseOut;
        assert_eq!(easing.apply(0.0), 0.0);
        assert!(easing.apply(0.5) > 0.5); // Starts fast
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn test_rotation_animation_new() {
        let anim = RotationAnimation::new(
            RotationFace::R,
            90.0,
            Duration::from_millis(300),
            EasingFunction::Linear,
        );
        assert_eq!(anim.face, RotationFace::R);
        assert_eq!(anim.angle_degrees, 90.0);
        assert_eq!(anim.duration, Duration::from_millis(300));
        assert!(!anim.is_started());
    }

    #[test]
    fn test_rotation_animation_shortcuts() {
        let cw = RotationAnimation::rotate_cw(RotationFace::U, Duration::from_millis(300));
        assert_eq!(cw.angle_degrees, 90.0);

        let ccw = RotationAnimation::rotate_ccw(RotationFace::U, Duration::from_millis(300));
        assert_eq!(ccw.angle_degrees, -90.0);

        let half = RotationAnimation::rotate_180(RotationFace::F, Duration::from_millis(300));
        assert_eq!(half.angle_degrees, 180.0);
    }

    #[test]
    fn test_rotation_animation_progress() {
        let mut anim = RotationAnimation::new(
            RotationFace::R,
            90.0,
            Duration::from_millis(100),
            EasingFunction::Linear,
        );

        // Not started yet
        assert_eq!(anim.progress(), None);
        assert_eq!(anim.current_angle(), None);

        // Start animation
        anim.start();
        assert!(anim.is_started());
        assert!(anim.progress().is_some());

        // Progress should be between 0 and 1
        let p = anim.progress().unwrap();
        assert!(p >= 0.0 && p <= 1.0);

        // Wait for completion
        thread::sleep(Duration::from_millis(120));
        assert!(anim.is_complete());
        assert_eq!(anim.progress().unwrap(), 1.0);
        assert_eq!(anim.current_angle().unwrap(), 90.0);
    }

    #[test]
    fn test_animation_queue_basic() {
        let mut queue = AnimationQueue::new();
        assert!(!queue.has_animations());
        assert_eq!(queue.queued_count(), 0);

        // Add animation
        queue.enqueue(RotationAnimation::rotate_cw(
            RotationFace::R,
            Duration::from_millis(100),
        ));
        assert!(queue.has_animations());
        assert_eq!(queue.queued_count(), 1);

        // Update should start the animation
        let current = queue.update();
        assert!(current.is_some());
        assert_eq!(queue.queued_count(), 0);

        // Current animation should be playing
        assert!(queue.current().is_some());
    }

    #[test]
    fn test_animation_queue_multiple() {
        let mut queue = AnimationQueue::new();

        // Add multiple animations
        queue.enqueue_multiple(vec![
            RotationAnimation::rotate_cw(RotationFace::R, Duration::from_millis(50)),
            RotationAnimation::rotate_cw(RotationFace::U, Duration::from_millis(50)),
            RotationAnimation::rotate_cw(RotationFace::F, Duration::from_millis(50)),
        ]);

        assert_eq!(queue.queued_count(), 3);

        // Start first animation
        queue.update();
        assert_eq!(queue.queued_count(), 2);
        assert_eq!(queue.current().unwrap().face, RotationFace::R);

        // Wait for first to complete
        thread::sleep(Duration::from_millis(60));
        queue.update();
        assert_eq!(queue.queued_count(), 1);
        assert_eq!(queue.current().unwrap().face, RotationFace::U);

        // Wait for second to complete
        thread::sleep(Duration::from_millis(60));
        queue.update();
        assert_eq!(queue.queued_count(), 0);
        assert_eq!(queue.current().unwrap().face, RotationFace::F);
    }

    #[test]
    fn test_animation_queue_clear() {
        let mut queue = AnimationQueue::new();
        queue.enqueue_multiple(vec![
            RotationAnimation::rotate_cw(RotationFace::R, Duration::from_millis(100)),
            RotationAnimation::rotate_cw(RotationFace::U, Duration::from_millis(100)),
        ]);

        queue.update(); // Start first animation
        assert!(queue.has_animations());

        queue.clear();
        assert!(!queue.has_animations());
        assert_eq!(queue.queued_count(), 0);
        assert!(queue.current().is_none());
    }
}
