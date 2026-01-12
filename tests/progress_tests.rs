//! Integration tests for progress tracking system (R6.11)
//!
//! Tests the complete progress tracking workflow including lesson completion,
//! practice statistics, and local storage persistence.

use rubiks_cube_solver::state::{Progress, LessonId, PracticeStats};

#[test]
fn test_progress_001_track_single_lesson_completion() {
    let mut progress = Progress::new();

    // Initially no lessons completed
    assert_eq!(progress.completed_count(), 0);
    assert!(!progress.is_lesson_completed(&LessonId::Notation));

    // Complete notation lesson
    progress.complete_lesson(LessonId::Notation);

    // Verify completion
    assert_eq!(progress.completed_count(), 1);
    assert!(progress.is_lesson_completed(&LessonId::Notation));
    assert!(!progress.is_lesson_completed(&LessonId::Colors));
}

#[test]
fn test_progress_002_track_multiple_lesson_completions() {
    let mut progress = Progress::new();

    // Complete multiple lessons
    progress.complete_lesson(LessonId::Notation);
    progress.complete_lesson(LessonId::Colors);
    progress.complete_lesson(LessonId::Cross);
    progress.complete_lesson(LessonId::FirstLayerCorners);

    // Verify all completed
    assert_eq!(progress.completed_count(), 4);
    assert!(progress.is_lesson_completed(&LessonId::Notation));
    assert!(progress.is_lesson_completed(&LessonId::Colors));
    assert!(progress.is_lesson_completed(&LessonId::Cross));
    assert!(progress.is_lesson_completed(&LessonId::FirstLayerCorners));
}

#[test]
fn test_progress_003_calculate_completion_percentage() {
    let mut progress = Progress::new();

    // No lessons completed - 0%
    assert_eq!(progress.completion_percentage(), 0.0);

    // Complete one lesson
    progress.complete_lesson(LessonId::Notation);
    let percentage = progress.completion_percentage();
    assert!(percentage > 0.0 && percentage <= 100.0);

    // Complete all lessons
    for lesson in LessonId::all() {
        progress.complete_lesson(lesson);
    }
    assert_eq!(progress.completion_percentage(), 100.0);
}

#[test]
fn test_progress_004_track_practice_attempts() {
    let mut progress = Progress::new();

    // Record practice attempts
    progress.record_practice(LessonId::Cross, true, 45);
    progress.record_practice(LessonId::Cross, true, 30);
    progress.record_practice(LessonId::Cross, false, 60);

    let stats = progress.get_practice_stats(&LessonId::Cross).unwrap();
    assert_eq!(stats.attempts, 3);
    assert_eq!(stats.successes, 2);
    assert_eq!(stats.best_time_seconds, Some(30));
}

#[test]
fn test_progress_005_calculate_success_rate() {
    let mut progress = Progress::new();

    // Record 10 attempts with 7 successes
    for i in 0..10 {
        let success = i < 7;
        progress.record_practice(LessonId::OLL, success, 40);
    }

    let stats = progress.get_practice_stats(&LessonId::OLL).unwrap();
    assert_eq!(stats.success_rate(), 70.0);
}

#[test]
fn test_progress_006_track_best_time() {
    let mut progress = Progress::new();

    // Record attempts with varying times
    progress.record_practice(LessonId::PLL, true, 50);
    progress.record_practice(LessonId::PLL, true, 35);
    progress.record_practice(LessonId::PLL, true, 42);
    progress.record_practice(LessonId::PLL, true, 28); // Best time
    progress.record_practice(LessonId::PLL, true, 45);

    let stats = progress.get_practice_stats(&LessonId::PLL).unwrap();
    assert_eq!(stats.best_time_seconds, Some(28));
}

#[test]
fn test_progress_007_track_average_time() {
    let mut progress = Progress::new();

    // Record attempts: 30, 40, 50 seconds (average = 40)
    progress.record_practice(LessonId::SecondLayer, true, 30);
    progress.record_practice(LessonId::SecondLayer, true, 40);
    progress.record_practice(LessonId::SecondLayer, true, 50);

    let stats = progress.get_practice_stats(&LessonId::SecondLayer).unwrap();
    assert_eq!(stats.average_time_seconds, Some(40));
}

#[test]
fn test_progress_008_persist_to_json() {
    let mut progress = Progress::new();

    // Setup progress data
    progress.complete_lesson(LessonId::Notation);
    progress.complete_lesson(LessonId::Colors);
    progress.record_practice(LessonId::Cross, true, 45);
    progress.record_practice(LessonId::Cross, true, 35);

    // Serialize to JSON
    let json = progress.to_json().expect("Should serialize to JSON");
    assert!(json.contains("Notation"));
    assert!(json.contains("Colors"));

    // Deserialize and verify
    let loaded = Progress::from_json(&json).expect("Should deserialize from JSON");
    assert_eq!(loaded.completed_count(), 2);
    assert!(loaded.is_lesson_completed(&LessonId::Notation));
    assert!(loaded.is_lesson_completed(&LessonId::Colors));

    let stats = loaded.get_practice_stats(&LessonId::Cross).unwrap();
    assert_eq!(stats.attempts, 2);
    assert_eq!(stats.best_time_seconds, Some(35));
}

#[test]
fn test_progress_009_pretty_json_format() {
    let mut progress = Progress::new();
    progress.complete_lesson(LessonId::TwoByTwo);

    let pretty_json = progress.to_json_pretty().expect("Should serialize to pretty JSON");

    // Pretty JSON should have newlines and indentation
    assert!(pretty_json.contains('\n'));
    assert!(pretty_json.len() > progress.to_json().unwrap().len());
}

#[test]
fn test_progress_010_reset_all_progress() {
    let mut progress = Progress::new();

    // Setup some progress
    progress.complete_lesson(LessonId::Notation);
    progress.complete_lesson(LessonId::Colors);
    progress.record_practice(LessonId::Cross, true, 30);

    // Verify progress exists
    assert_eq!(progress.completed_count(), 2);
    assert!(progress.get_practice_stats(&LessonId::Cross).is_some());

    // Reset
    progress.reset();

    // Verify everything cleared
    assert_eq!(progress.completed_count(), 0);
    assert!(progress.get_practice_stats(&LessonId::Cross).is_none());
}

#[test]
fn test_progress_011_multiple_lessons_practice_stats() {
    let mut progress = Progress::new();

    // Practice multiple lessons
    progress.record_practice(LessonId::Cross, true, 40);
    progress.record_practice(LessonId::OLL, true, 35);
    progress.record_practice(LessonId::PLL, true, 50);

    // Verify all stats tracked independently
    assert!(progress.get_practice_stats(&LessonId::Cross).is_some());
    assert!(progress.get_practice_stats(&LessonId::OLL).is_some());
    assert!(progress.get_practice_stats(&LessonId::PLL).is_some());

    let all_stats = progress.all_practice_stats();
    assert_eq!(all_stats.len(), 3);
}

#[test]
fn test_progress_012_lesson_names() {
    assert_eq!(LessonId::Notation.name(), "Cube Notation");
    assert_eq!(LessonId::Colors.name(), "Face Colors");
    assert_eq!(LessonId::Cross.name(), "Cross");
    assert_eq!(LessonId::FirstLayerCorners.name(), "First Layer Corners");
    assert_eq!(LessonId::SecondLayer.name(), "Second Layer");
    assert_eq!(LessonId::OLL.name(), "OLL (Orient Last Layer)");
    assert_eq!(LessonId::PLL.name(), "PLL (Permute Last Layer)");
    assert_eq!(LessonId::TwoByTwo.name(), "2x2 Cube");
}

#[test]
fn test_progress_013_all_lessons_list() {
    let all = LessonId::all();

    // Should have at least 8 lessons
    assert!(all.len() >= 8);

    // Should be in expected order
    assert_eq!(all[0], LessonId::Notation);
    assert_eq!(all[1], LessonId::Colors);
    assert_eq!(all[2], LessonId::Cross);
}

#[test]
fn test_progress_014_completed_lessons_list() {
    let mut progress = Progress::new();

    progress.complete_lesson(LessonId::Notation);
    progress.complete_lesson(LessonId::Cross);
    progress.complete_lesson(LessonId::OLL);

    let completed = progress.completed_lessons();
    assert_eq!(completed.len(), 3);

    // Check all completed lessons are in the list
    assert!(completed.contains(&&LessonId::Notation));
    assert!(completed.contains(&&LessonId::Cross));
    assert!(completed.contains(&&LessonId::OLL));
}

#[test]
fn test_progress_015_duplicate_completion_ignored() {
    let mut progress = Progress::new();

    // Complete same lesson multiple times
    progress.complete_lesson(LessonId::Notation);
    progress.complete_lesson(LessonId::Notation);
    progress.complete_lesson(LessonId::Notation);

    // Should only count once
    assert_eq!(progress.completed_count(), 1);
}

#[test]
fn test_progress_016_failed_attempts_count() {
    let mut progress = Progress::new();

    // Mix of successful and failed attempts
    progress.record_practice(LessonId::Cross, true, 30);
    progress.record_practice(LessonId::Cross, false, 60);
    progress.record_practice(LessonId::Cross, false, 45);
    progress.record_practice(LessonId::Cross, true, 25);

    let stats = progress.get_practice_stats(&LessonId::Cross).unwrap();
    assert_eq!(stats.attempts, 4);
    assert_eq!(stats.successes, 2);
    assert_eq!(stats.success_rate(), 50.0);
}

#[test]
fn test_progress_017_practice_total_time_tracking() {
    let mut progress = Progress::new();

    // Record multiple practice sessions
    progress.record_practice(LessonId::PLL, true, 30);
    progress.record_practice(LessonId::PLL, true, 45);
    progress.record_practice(LessonId::PLL, false, 60);

    let stats = progress.get_practice_stats(&LessonId::PLL).unwrap();
    assert_eq!(stats.total_time_seconds, 135); // 30 + 45 + 60
}

#[test]
fn test_progress_018_json_roundtrip_with_complex_data() {
    let mut progress = Progress::new();

    // Setup complex progress data
    for lesson in LessonId::all().iter().take(5) {
        progress.complete_lesson(lesson.clone());
    }

    progress.record_practice(LessonId::Cross, true, 40);
    progress.record_practice(LessonId::Cross, false, 50);
    progress.record_practice(LessonId::OLL, true, 35);
    progress.record_practice(LessonId::PLL, true, 45);
    progress.record_practice(LessonId::PLL, true, 30);

    // Serialize and deserialize
    let json = progress.to_json().unwrap();
    let loaded = Progress::from_json(&json).unwrap();

    // Verify all data preserved
    assert_eq!(loaded.completed_count(), 5);
    assert_eq!(loaded.all_practice_stats().len(), 3);

    let cross_stats = loaded.get_practice_stats(&LessonId::Cross).unwrap();
    assert_eq!(cross_stats.attempts, 2);
    assert_eq!(cross_stats.successes, 1);
}

#[test]
fn test_progress_019_zero_attempts_success_rate() {
    let stats = PracticeStats::new();
    assert_eq!(stats.success_rate(), 0.0);
    assert_eq!(stats.attempts, 0);
    assert_eq!(stats.successes, 0);
}

#[test]
fn test_progress_020_practice_stats_default() {
    let stats = PracticeStats::default();
    assert_eq!(stats.attempts, 0);
    assert_eq!(stats.successes, 0);
    assert_eq!(stats.total_time_seconds, 0);
    assert_eq!(stats.best_time_seconds, None);
    assert_eq!(stats.average_time_seconds, None);
}
