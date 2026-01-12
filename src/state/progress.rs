//! Progress tracking module for tutorial system
//!
//! Tracks completed lessons and practice statistics, persisting to local storage.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Unique identifier for a lesson
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LessonId {
    /// R6.1: Beginner lesson for cube notation
    Notation,
    /// R6.2: Beginner lesson for face colors
    Colors,
    /// R6.3: 3x3 tutorial for cross
    Cross,
    /// R6.4: 3x3 tutorial for first layer corners
    FirstLayerCorners,
    /// R6.5: 3x3 tutorial for second layer
    SecondLayer,
    /// R6.6: 3x3 tutorial for OLL (orient last layer)
    OLL,
    /// R6.7: 3x3 tutorial for PLL (permute last layer)
    PLL,
    /// R6.8: 2x2 tutorial
    TwoByTwo,
    /// R6.9: 4x4 tutorial (when implemented)
    FourByFour,
}

impl LessonId {
    /// Get human-readable name for the lesson
    pub fn name(&self) -> &str {
        match self {
            LessonId::Notation => "Cube Notation",
            LessonId::Colors => "Face Colors",
            LessonId::Cross => "Cross",
            LessonId::FirstLayerCorners => "First Layer Corners",
            LessonId::SecondLayer => "Second Layer",
            LessonId::OLL => "OLL (Orient Last Layer)",
            LessonId::PLL => "PLL (Permute Last Layer)",
            LessonId::TwoByTwo => "2x2 Cube",
            LessonId::FourByFour => "4x4 Cube",
        }
    }

    /// Get all available lessons in order
    pub fn all() -> Vec<LessonId> {
        vec![
            LessonId::Notation,
            LessonId::Colors,
            LessonId::Cross,
            LessonId::FirstLayerCorners,
            LessonId::SecondLayer,
            LessonId::OLL,
            LessonId::PLL,
            LessonId::TwoByTwo,
            // FourByFour not yet implemented
        ]
    }
}

/// Statistics for practice sessions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PracticeStats {
    /// Total number of practice attempts
    pub attempts: u32,
    /// Number of successful completions
    pub successes: u32,
    /// Total time spent practicing (in seconds)
    pub total_time_seconds: u32,
    /// Best time (in seconds)
    pub best_time_seconds: Option<u32>,
    /// Average time (in seconds)
    pub average_time_seconds: Option<u32>,
}

impl PracticeStats {
    /// Create new empty practice stats
    pub fn new() -> Self {
        Self {
            attempts: 0,
            successes: 0,
            total_time_seconds: 0,
            best_time_seconds: None,
            average_time_seconds: None,
        }
    }

    /// Record a practice attempt
    pub fn record_attempt(&mut self, success: bool, time_seconds: u32) {
        self.attempts += 1;
        if success {
            self.successes += 1;
        }
        self.total_time_seconds += time_seconds;

        // Update best time (only for successful attempts)
        if success {
            if let Some(best) = self.best_time_seconds {
                if time_seconds < best {
                    self.best_time_seconds = Some(time_seconds);
                }
            } else {
                self.best_time_seconds = Some(time_seconds);
            }
        }

        // Update average time (only for successful attempts)
        if self.successes > 0 {
            self.average_time_seconds = Some(self.total_time_seconds / self.successes);
        }
    }

    /// Get success rate as a percentage (0-100)
    pub fn success_rate(&self) -> f32 {
        if self.attempts == 0 {
            0.0
        } else {
            (self.successes as f32 / self.attempts as f32) * 100.0
        }
    }
}

impl Default for PracticeStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Progress tracking data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    /// Set of completed lessons
    completed_lessons: HashSet<LessonId>,
    /// Practice statistics per lesson
    practice_stats: HashMap<LessonId, PracticeStats>,
    /// Version for forward compatibility
    version: u32,
}

impl Progress {
    /// Current version of the progress format
    const VERSION: u32 = 1;

    /// Create new empty progress tracker
    pub fn new() -> Self {
        Self {
            completed_lessons: HashSet::new(),
            practice_stats: HashMap::new(),
            version: Self::VERSION,
        }
    }

    /// Mark a lesson as completed
    pub fn complete_lesson(&mut self, lesson_id: LessonId) {
        self.completed_lessons.insert(lesson_id);
    }

    /// Check if a lesson is completed
    pub fn is_lesson_completed(&self, lesson_id: &LessonId) -> bool {
        self.completed_lessons.contains(lesson_id)
    }

    /// Get list of all completed lessons
    pub fn completed_lessons(&self) -> Vec<&LessonId> {
        self.completed_lessons.iter().collect()
    }

    /// Get number of completed lessons
    pub fn completed_count(&self) -> usize {
        self.completed_lessons.len()
    }

    /// Get total number of available lessons
    pub fn total_lessons(&self) -> usize {
        LessonId::all().len()
    }

    /// Get completion percentage (0-100)
    pub fn completion_percentage(&self) -> f32 {
        if self.total_lessons() == 0 {
            0.0
        } else {
            (self.completed_count() as f32 / self.total_lessons() as f32) * 100.0
        }
    }

    /// Record a practice attempt for a lesson
    pub fn record_practice(&mut self, lesson_id: LessonId, success: bool, time_seconds: u32) {
        let stats = self.practice_stats.entry(lesson_id).or_insert_with(PracticeStats::new);
        stats.record_attempt(success, time_seconds);
    }

    /// Get practice statistics for a lesson
    pub fn get_practice_stats(&self, lesson_id: &LessonId) -> Option<&PracticeStats> {
        self.practice_stats.get(lesson_id)
    }

    /// Get all practice statistics
    pub fn all_practice_stats(&self) -> &HashMap<LessonId, PracticeStats> {
        &self.practice_stats
    }

    /// Reset all progress (for testing or user request)
    pub fn reset(&mut self) {
        self.completed_lessons.clear();
        self.practice_stats.clear();
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serialize to pretty JSON string
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_progress_is_empty() {
        let progress = Progress::new();
        assert_eq!(progress.completed_count(), 0);
        assert_eq!(progress.all_practice_stats().len(), 0);
    }

    #[test]
    fn test_complete_lesson() {
        let mut progress = Progress::new();
        progress.complete_lesson(LessonId::Notation);
        assert!(progress.is_lesson_completed(&LessonId::Notation));
        assert!(!progress.is_lesson_completed(&LessonId::Colors));
        assert_eq!(progress.completed_count(), 1);
    }

    #[test]
    fn test_multiple_completed_lessons() {
        let mut progress = Progress::new();
        progress.complete_lesson(LessonId::Notation);
        progress.complete_lesson(LessonId::Colors);
        progress.complete_lesson(LessonId::Cross);
        assert_eq!(progress.completed_count(), 3);
    }

    #[test]
    fn test_completion_percentage() {
        let mut progress = Progress::new();
        assert_eq!(progress.completion_percentage(), 0.0);

        // Complete half of the lessons
        let total = progress.total_lessons();
        for lesson_id in LessonId::all().iter().take(total / 2) {
            progress.complete_lesson(lesson_id.clone());
        }

        let percentage = progress.completion_percentage();
        assert!(percentage > 40.0 && percentage < 60.0); // Around 50%
    }

    #[test]
    fn test_record_practice_attempt() {
        let mut progress = Progress::new();
        progress.record_practice(LessonId::Notation, true, 30);

        let stats = progress.get_practice_stats(&LessonId::Notation).unwrap();
        assert_eq!(stats.attempts, 1);
        assert_eq!(stats.successes, 1);
        assert_eq!(stats.total_time_seconds, 30);
        assert_eq!(stats.best_time_seconds, Some(30));
    }

    #[test]
    fn test_practice_stats_success_rate() {
        let mut stats = PracticeStats::new();
        assert_eq!(stats.success_rate(), 0.0);

        stats.record_attempt(true, 30);
        assert_eq!(stats.success_rate(), 100.0);

        stats.record_attempt(false, 45);
        assert_eq!(stats.success_rate(), 50.0);

        stats.record_attempt(true, 25);
        assert!((stats.success_rate() - 66.67).abs() < 0.1);
    }

    #[test]
    fn test_practice_best_time() {
        let mut stats = PracticeStats::new();
        stats.record_attempt(true, 30);
        assert_eq!(stats.best_time_seconds, Some(30));

        stats.record_attempt(true, 25);
        assert_eq!(stats.best_time_seconds, Some(25));

        stats.record_attempt(true, 35);
        assert_eq!(stats.best_time_seconds, Some(25)); // Should still be 25
    }

    #[test]
    fn test_practice_average_time() {
        let mut stats = PracticeStats::new();
        stats.record_attempt(true, 30);
        assert_eq!(stats.average_time_seconds, Some(30));

        stats.record_attempt(true, 40);
        assert_eq!(stats.average_time_seconds, Some(35)); // (30+40)/2 = 35

        stats.record_attempt(true, 20);
        assert_eq!(stats.average_time_seconds, Some(30)); // (30+40+20)/3 = 30
    }

    #[test]
    fn test_failed_attempts_dont_affect_best_time() {
        let mut stats = PracticeStats::new();
        stats.record_attempt(true, 30);
        stats.record_attempt(false, 10); // Failed, should not update best
        assert_eq!(stats.best_time_seconds, Some(30));
    }

    #[test]
    fn test_reset_progress() {
        let mut progress = Progress::new();
        progress.complete_lesson(LessonId::Notation);
        progress.record_practice(LessonId::Colors, true, 30);

        progress.reset();

        assert_eq!(progress.completed_count(), 0);
        assert_eq!(progress.all_practice_stats().len(), 0);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut progress = Progress::new();
        progress.complete_lesson(LessonId::Notation);
        progress.complete_lesson(LessonId::Colors);
        progress.record_practice(LessonId::Cross, true, 45);

        let json = progress.to_json().unwrap();
        let loaded = Progress::from_json(&json).unwrap();

        assert_eq!(loaded.completed_count(), 2);
        assert!(loaded.is_lesson_completed(&LessonId::Notation));
        assert!(loaded.is_lesson_completed(&LessonId::Colors));

        let stats = loaded.get_practice_stats(&LessonId::Cross).unwrap();
        assert_eq!(stats.attempts, 1);
        assert_eq!(stats.successes, 1);
    }

    #[test]
    fn test_lesson_id_name() {
        assert_eq!(LessonId::Notation.name(), "Cube Notation");
        assert_eq!(LessonId::Colors.name(), "Face Colors");
        assert_eq!(LessonId::TwoByTwo.name(), "2x2 Cube");
    }

    #[test]
    fn test_lesson_id_all() {
        let all = LessonId::all();
        assert!(all.len() >= 8); // At least 8 lessons
        assert_eq!(all[0], LessonId::Notation);
        assert_eq!(all[1], LessonId::Colors);
    }
}
