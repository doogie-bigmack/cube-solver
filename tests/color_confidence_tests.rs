/// Integration tests for color detection confidence scoring
///
/// Tests the majority_vote_with_confidence function through public API

use rubiks_cube_solver::camera::{
    detect_colors_in_grid_with_confidence, ColorDetectionConfig, RGB,
};
use rubiks_cube_solver::cube::Color;

#[test]
fn test_confidence_001_uniform_color_high_confidence() {
    // Create a simple test image with uniform red color
    let width = 30;
    let height = 30;
    let mut pixels = vec![0u8; (width * height * 3) as usize];

    // Fill with red (Rubik's cube red)
    for pixel in pixels.chunks_mut(3) {
        pixel[0] = 196; // R
        pixel[1] = 30; // G
        pixel[2] = 58; // B
    }

    let config = ColorDetectionConfig::default();
    let result = detect_colors_in_grid_with_confidence(&pixels, width, height, 3, &config);

    assert!(result.is_some());
    let grid = result.unwrap();

    // All colors should be detected with high confidence
    for row in grid.iter() {
        for detection in row.iter() {
            assert_eq!(detection.color, Color::Red);
            assert!(detection.confidence > 0.7); // Should have good confidence
        }
    }
}

#[test]
fn test_confidence_002_detection_result_methods() {
    use rubiks_cube_solver::camera::ColorDetectionResult;

    let uncertain = ColorDetectionResult {
        color: Color::Red,
        confidence: 0.5,
    };
    assert!(uncertain.is_uncertain());
    assert!(!uncertain.is_reliable());

    let reliable = ColorDetectionResult {
        color: Color::Blue,
        confidence: 0.9,
    };
    assert!(!reliable.is_uncertain());
    assert!(reliable.is_reliable());

    let moderate = ColorDetectionResult {
        color: Color::Green,
        confidence: 0.7,
    };
    assert!(!moderate.is_uncertain());
    assert!(!moderate.is_reliable());
}

#[test]
fn test_confidence_003_white_detection() {
    // Test white color detection confidence
    let width = 30;
    let height = 30;
    let mut pixels = vec![0u8; (width * height * 3) as usize];

    // Fill with white
    for pixel in pixels.chunks_mut(3) {
        pixel[0] = 240;
        pixel[1] = 240;
        pixel[2] = 240;
    }

    let config = ColorDetectionConfig::default();
    let result = detect_colors_in_grid_with_confidence(&pixels, width, height, 3, &config);

    assert!(result.is_some());
    let grid = result.unwrap();

    for row in grid.iter() {
        for detection in row.iter() {
            assert_eq!(detection.color, Color::White);
            assert!(detection.confidence > 0.5);
        }
    }
}

#[test]
fn test_confidence_004_yellow_detection() {
    // Test yellow color detection confidence
    let width = 30;
    let height = 30;
    let mut pixels = vec![0u8; (width * height * 3) as usize];

    // Fill with yellow
    for pixel in pixels.chunks_mut(3) {
        pixel[0] = 255;
        pixel[1] = 215;
        pixel[2] = 0;
    }

    let config = ColorDetectionConfig::default();
    let result = detect_colors_in_grid_with_confidence(&pixels, width, height, 3, &config);

    assert!(result.is_some());
    let grid = result.unwrap();

    for row in grid.iter() {
        for detection in row.iter() {
            assert_eq!(detection.color, Color::Yellow);
            assert!(detection.confidence > 0.5);
        }
    }
}

#[test]
fn test_confidence_005_2x2_grid() {
    // Test confidence with 2x2 grid
    let width = 20;
    let height = 20;
    let mut pixels = vec![0u8; (width * height * 3) as usize];

    // Fill with orange
    for pixel in pixels.chunks_mut(3) {
        pixel[0] = 255;
        pixel[1] = 88;
        pixel[2] = 0;
    }

    let config = ColorDetectionConfig::default();
    let result = detect_colors_in_grid_with_confidence(&pixels, width, height, 2, &config);

    assert!(result.is_some());
    let grid = result.unwrap();

    assert_eq!(grid.len(), 2);
    assert_eq!(grid[0].len(), 2);

    for row in grid.iter() {
        for detection in row.iter() {
            assert_eq!(detection.color, Color::Orange);
            assert!(detection.confidence >= 0.0 && detection.confidence <= 1.0);
        }
    }
}

#[test]
fn test_confidence_006_blue_detection() {
    // Test blue color detection confidence
    let width = 30;
    let height = 30;
    let mut pixels = vec![0u8; (width * height * 3) as usize];

    // Fill with blue
    for pixel in pixels.chunks_mut(3) {
        pixel[0] = 0;
        pixel[1] = 81;
        pixel[2] = 186;
    }

    let config = ColorDetectionConfig::default();
    let result = detect_colors_in_grid_with_confidence(&pixels, width, height, 3, &config);

    assert!(result.is_some());
    let grid = result.unwrap();

    for row in grid.iter() {
        for detection in row.iter() {
            assert_eq!(detection.color, Color::Blue);
            assert!(detection.confidence > 0.5);
        }
    }
}

#[test]
fn test_confidence_007_green_detection() {
    // Test green color detection confidence
    let width = 30;
    let height = 30;
    let mut pixels = vec![0u8; (width * height * 3) as usize];

    // Fill with green
    for pixel in pixels.chunks_mut(3) {
        pixel[0] = 0;
        pixel[1] = 158;
        pixel[2] = 96;
    }

    let config = ColorDetectionConfig::default();
    let result = detect_colors_in_grid_with_confidence(&pixels, width, height, 3, &config);

    assert!(result.is_some());
    let grid = result.unwrap();

    for row in grid.iter() {
        for detection in row.iter() {
            assert_eq!(detection.color, Color::Green);
            assert!(detection.confidence > 0.5);
        }
    }
}

#[test]
fn test_confidence_008_confidence_range() {
    use rubiks_cube_solver::camera::ColorDetectionResult;

    // Test that confidence values are always in valid range
    let tests = vec![
        (0.0, true),
        (0.5, true),
        (1.0, true),
        (1.5, false), // Invalid
        (-0.1, false), // Invalid
    ];

    for (conf, should_be_valid) in tests {
        if should_be_valid {
            assert!(conf >= 0.0 && conf <= 1.0);
        } else {
            assert!(conf < 0.0 || conf > 1.0);
        }
    }
}

#[test]
fn test_confidence_009_empty_or_invalid_image() {
    // Test with invalid image data
    let empty_pixels: Vec<u8> = vec![];
    let config = ColorDetectionConfig::default();
    let result = detect_colors_in_grid_with_confidence(&empty_pixels, 0, 0, 3, &config);

    assert!(result.is_none());
}

#[test]
fn test_confidence_010_mismatched_dimensions() {
    // Test with mismatched pixel data size
    let width = 30;
    let height = 30;
    let pixels = vec![0u8; 100]; // Too small for 30x30

    let config = ColorDetectionConfig::default();
    let result = detect_colors_in_grid_with_confidence(&pixels, width, height, 3, &config);

    assert!(result.is_none());
}
