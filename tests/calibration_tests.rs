/// Integration tests for color calibration functionality
///
/// Tests cover:
/// - ColorSample creation
/// - ColorCalibration accumulation
/// - CalibrationManager workflow
/// - Detection config generation
/// - Serialization/deserialization

use rubiks_cube_solver::camera::{CalibrationManager, ColorCalibration, ColorSample, RGB};
use rubiks_cube_solver::cube::Color;

#[test]
fn test_calib_001_create_color_sample() {
    let rgb = RGB::new(255, 0, 0);
    let sample = ColorSample::new(rgb);
    assert_eq!(sample.rgb.r, 255);
    assert_eq!(sample.rgb.g, 0);
    assert_eq!(sample.rgb.b, 0);
    // HSV for pure red should be around H=0, S=1, V=1
    assert!((sample.hsv.h - 0.0).abs() < 5.0);
    assert!(sample.hsv.s > 0.9);
    assert!(sample.hsv.v > 0.9);
}

#[test]
fn test_calib_002_calibration_starts_empty() {
    let cal = ColorCalibration::new(Color::Red);
    assert_eq!(cal.color, Color::Red);
    assert_eq!(cal.sample_count, 0);
    assert!(!cal.is_complete());
}

#[test]
fn test_calib_003_add_samples_to_calibration() {
    let mut cal = ColorCalibration::new(Color::Red);

    // Add 5 samples
    for _ in 0..5 {
        let sample = ColorSample::new(RGB::new(200, 30, 50));
        cal.add_sample(sample);
    }

    assert_eq!(cal.sample_count, 5);
    assert!(!cal.is_complete()); // Need 10 samples
}

#[test]
fn test_calib_004_calibration_complete_after_10_samples() {
    let mut cal = ColorCalibration::new(Color::Red);

    // Add 10 samples
    for _ in 0..10 {
        let sample = ColorSample::new(RGB::new(196, 30, 58));
        cal.add_sample(sample);
    }

    assert_eq!(cal.sample_count, 10);
    assert!(cal.is_complete());
}

#[test]
fn test_calib_005_manager_initializes_with_6_colors() {
    let manager = CalibrationManager::new();
    assert_eq!(manager.calibrations.len(), 6);
    assert_eq!(manager.current_color_index, 0);
    assert_eq!(manager.current_color(), Color::White);
}

#[test]
fn test_calib_006_manager_workflow() {
    let mut manager = CalibrationManager::new();

    // Start with White
    assert_eq!(manager.current_color(), Color::White);

    // Add 10 samples for White
    for _ in 0..10 {
        let sample = ColorSample::new(RGB::new(240, 240, 240));
        manager.add_sample(sample);
    }

    // Move to next color (Yellow)
    assert!(manager.next_color());
    assert_eq!(manager.current_color(), Color::Yellow);

    // Add 10 samples for Yellow
    for _ in 0..10 {
        let sample = ColorSample::new(RGB::new(255, 215, 0));
        manager.add_sample(sample);
    }

    assert_eq!(manager.completed_colors(), 2);
}

#[test]
fn test_calib_007_manager_next_color_stops_at_end() {
    let mut manager = CalibrationManager::new();

    // Move through all colors
    for _ in 0..5 {
        assert!(manager.next_color());
    }

    // Should be at Green (last color)
    assert_eq!(manager.current_color(), Color::Green);

    // Can't go further
    assert!(!manager.next_color());
}

#[test]
fn test_calib_008_manager_is_complete_when_all_done() {
    let mut manager = CalibrationManager::new();
    assert!(!manager.is_complete());

    // Complete all 6 colors
    for color_idx in 0..6 {
        manager.current_color_index = color_idx;
        for _ in 0..10 {
            let sample = ColorSample::new(RGB::new(128, 128, 128));
            manager.add_sample(sample);
        }
    }

    assert!(manager.is_complete());
    assert_eq!(manager.completed_colors(), 6);
}

#[test]
fn test_calib_009_generate_detection_config() {
    let mut manager = CalibrationManager::new();

    // Add white samples
    for _ in 0..10 {
        let sample = ColorSample::new(RGB::new(240, 240, 240));
        manager.add_sample(sample);
    }

    // Generate config
    let config = manager.to_detection_config();

    // Check that white detection parameters are set
    assert!(config.white_min_value > 0.0);
    assert!(config.white_max_saturation < 1.0);
}

#[test]
fn test_calib_010_detection_config_with_all_colors() {
    let mut manager = CalibrationManager::new();

    // Calibrate all colors with realistic RGB values
    let color_samples = vec![
        (Color::White, RGB::new(240, 240, 240)),
        (Color::Yellow, RGB::new(255, 215, 0)),
        (Color::Red, RGB::new(196, 30, 58)),
        (Color::Orange, RGB::new(255, 88, 0)),
        (Color::Blue, RGB::new(0, 81, 186)),
        (Color::Green, RGB::new(0, 158, 96)),
    ];

    for (idx, (_color, rgb)) in color_samples.iter().enumerate() {
        manager.current_color_index = idx;
        for _ in 0..10 {
            let sample = ColorSample::new(*rgb);
            manager.add_sample(sample);
        }
    }

    let config = manager.to_detection_config();

    // Verify all color ranges are configured
    assert!(config.white_min_value > 0.0);
    assert!(config.yellow_hue_min <= config.yellow_hue_max);  // Allow equal for uniform samples
    assert!(config.red_hue_min > 0.0 || config.red_hue_max > 0.0);
    assert!(config.orange_hue_min <= config.orange_hue_max);
    assert!(config.blue_hue_min <= config.blue_hue_max);
    assert!(config.green_hue_min <= config.green_hue_max);
}

#[test]
fn test_calib_011_serialization_round_trip() {
    let mut manager = CalibrationManager::new();

    // Add some samples
    for _ in 0..5 {
        let sample = ColorSample::new(RGB::new(240, 240, 240));
        manager.add_sample(sample);
    }

    // Serialize
    let json = manager.to_json().unwrap();
    assert!(json.contains("White"));

    // Deserialize
    let loaded = CalibrationManager::from_json(&json).unwrap();
    assert_eq!(loaded.calibrations.len(), 6);
    assert_eq!(loaded.current_color_index, 0);
    assert_eq!(loaded.current_calibration().sample_count, 5);
}

#[test]
fn test_calib_012_reset_clears_all_data() {
    let mut manager = CalibrationManager::new();

    // Add samples and move to next color
    for _ in 0..10 {
        manager.add_sample(ColorSample::new(RGB::new(240, 240, 240)));
    }
    manager.next_color();

    assert_eq!(manager.current_color_index, 1);
    assert_eq!(manager.completed_colors(), 1);

    // Reset
    manager.reset();

    assert_eq!(manager.current_color_index, 0);
    assert_eq!(manager.completed_colors(), 0);
    assert_eq!(manager.current_calibration().sample_count, 0);
}

#[test]
fn test_calib_013_calibration_ranges_expand_with_samples() {
    let mut cal = ColorCalibration::new(Color::Red);

    // Add first sample
    let sample1 = ColorSample::new(RGB::new(255, 0, 0)); // Bright red
    cal.add_sample(sample1);

    let hue_range_1 = cal.hue_max - cal.hue_min;

    // Add second sample with different brightness
    let sample2 = ColorSample::new(RGB::new(180, 0, 0)); // Darker red
    cal.add_sample(sample2);

    let hue_range_2 = cal.hue_max - cal.hue_min;

    // Value range should have expanded
    assert!(cal.value_max > cal.value_min);
    // Hue range might expand slightly or stay similar
    assert!(hue_range_2 >= hue_range_1 - 5.0); // Allow small tolerance
}

#[test]
fn test_calib_014_midpoint_hue_calculation() {
    let mut cal = ColorCalibration::new(Color::Green);

    // Add samples with hues around 120°
    for _ in 0..10 {
        let sample = ColorSample::new(RGB::new(0, 158, 96)); // Green
        cal.add_sample(sample);
    }

    let midpoint = cal.midpoint_hue();
    // Green should be around 120-150°
    assert!(midpoint > 100.0 && midpoint < 170.0);
}

#[test]
fn test_calib_015_completed_colors_count() {
    let mut manager = CalibrationManager::new();

    assert_eq!(manager.completed_colors(), 0);

    // Complete White
    for _ in 0..10 {
        manager.add_sample(ColorSample::new(RGB::new(240, 240, 240)));
    }
    assert_eq!(manager.completed_colors(), 1);

    // Complete Yellow
    manager.next_color();
    for _ in 0..10 {
        manager.add_sample(ColorSample::new(RGB::new(255, 215, 0)));
    }
    assert_eq!(manager.completed_colors(), 2);

    // Complete Red
    manager.next_color();
    for _ in 0..10 {
        manager.add_sample(ColorSample::new(RGB::new(196, 30, 58)));
    }
    assert_eq!(manager.completed_colors(), 3);
}

#[test]
fn test_calib_016_sample_variations() {
    let mut cal = ColorCalibration::new(Color::Blue);

    // Add samples with various shades of blue
    let blues = vec![
        RGB::new(0, 81, 186),  // Standard blue
        RGB::new(0, 100, 200), // Lighter blue
        RGB::new(0, 60, 150),  // Darker blue
        RGB::new(10, 90, 190), // Slightly shifted
    ];

    for rgb in blues {
        for _ in 0..3 {
            let sample = ColorSample::new(rgb);
            cal.add_sample(sample);
        }
    }

    assert_eq!(cal.sample_count, 12);
    assert!(cal.is_complete());

    // Range should accommodate all variations
    assert!(cal.hue_max - cal.hue_min > 5.0); // Some variation captured
    assert!(cal.value_max - cal.value_min > 0.1);
}
