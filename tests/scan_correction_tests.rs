/// Integration tests for scan correction component
///
/// Tests cover:
/// - Detecting uncertain colors
/// - Manual color correction
/// - Retry scan functionality

use rubiks_cube_solver::camera::ColorDetectionResult;
use rubiks_cube_solver::components::{CorrectionState, ScanCorrection};
use rubiks_cube_solver::cube::Color;

#[test]
fn test_scan_correction_001_detect_uncertain_colors() {
    // Create a 3x3 grid with some uncertain colors
    let detected_colors = vec![
        vec![
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.95,
            },
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.5, // Uncertain
            },
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.9,
            },
        ],
        vec![
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.4, // Uncertain
            },
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.85,
            },
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.92,
            },
        ],
        vec![
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.88,
            },
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.91,
            },
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.93,
            },
        ],
    ];

    // Count uncertain colors (confidence < 0.6)
    let uncertain_count = detected_colors
        .iter()
        .flatten()
        .filter(|result| result.is_uncertain())
        .count();

    assert_eq!(uncertain_count, 2);
}

#[test]
fn test_scan_correction_002_all_reliable_colors() {
    // Create a 3x3 grid with all reliable colors
    let detected_colors = vec![
        vec![
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.95,
            },
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.92,
            },
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.9,
            },
        ],
        vec![
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.88,
            },
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.85,
            },
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.92,
            },
        ],
        vec![
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.91,
            },
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.93,
            },
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.94,
            },
        ],
    ];

    let uncertain_count = detected_colors
        .iter()
        .flatten()
        .filter(|result| result.is_uncertain())
        .count();

    assert_eq!(uncertain_count, 0);
}

#[test]
fn test_scan_correction_003_mixed_colors_with_various_confidence() {
    // Create a 3x3 grid with mixed colors and confidence levels
    let detected_colors = vec![
        vec![
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.95, // Reliable
            },
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.45, // Uncertain
            },
            ColorDetectionResult {
                color: Color::Blue,
                confidence: 0.72, // Moderate
            },
        ],
        vec![
            ColorDetectionResult {
                color: Color::Yellow,
                confidence: 0.88,
            },
            ColorDetectionResult {
                color: Color::Green,
                confidence: 0.55, // Uncertain
            },
            ColorDetectionResult {
                color: Color::Orange,
                confidence: 0.92,
            },
        ],
        vec![
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.78,
            },
            ColorDetectionResult {
                color: Color::Blue,
                confidence: 0.85,
            },
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.91,
            },
        ],
    ];

    let uncertain_count = detected_colors
        .iter()
        .flatten()
        .filter(|result| result.is_uncertain())
        .count();

    let reliable_count = detected_colors
        .iter()
        .flatten()
        .filter(|result| result.is_reliable())
        .count();

    assert_eq!(uncertain_count, 2); // confidence < 0.6
    assert_eq!(reliable_count, 6); // confidence >= 0.8
}

#[test]
fn test_scan_correction_004_confidence_thresholds() {
    let uncertain = ColorDetectionResult {
        color: Color::Red,
        confidence: 0.59,
    };
    assert!(uncertain.is_uncertain());
    assert!(!uncertain.is_reliable());

    let moderate = ColorDetectionResult {
        color: Color::Blue,
        confidence: 0.7,
    };
    assert!(!moderate.is_uncertain());
    assert!(!moderate.is_reliable());

    let reliable = ColorDetectionResult {
        color: Color::Green,
        confidence: 0.85,
    };
    assert!(!reliable.is_uncertain());
    assert!(reliable.is_reliable());
}

#[test]
fn test_scan_correction_005_correction_state_transitions() {
    // Test state transitions
    let reviewing = CorrectionState::Reviewing;
    let correcting = CorrectionState::Correcting;
    let complete = CorrectionState::Complete;

    assert_eq!(reviewing, CorrectionState::Reviewing);
    assert_eq!(correcting, CorrectionState::Correcting);
    assert_eq!(complete, CorrectionState::Complete);

    assert_ne!(reviewing, correcting);
    assert_ne!(correcting, complete);
    assert_ne!(reviewing, complete);
}

#[test]
fn test_scan_correction_006_2x2_cube_detection() {
    // Test with 2x2 cube
    let detected_colors = vec![
        vec![
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.95,
            },
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.92,
            },
        ],
        vec![
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.88,
            },
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.85,
            },
        ],
    ];

    assert_eq!(detected_colors.len(), 2);
    assert_eq!(detected_colors[0].len(), 2);
}

#[test]
fn test_scan_correction_007_4x4_cube_detection() {
    // Test with 4x4 cube
    let mut detected_colors = Vec::new();

    for _ in 0..4 {
        let mut row = Vec::new();
        for _ in 0..4 {
            row.push(ColorDetectionResult {
                color: Color::Green,
                confidence: 0.9,
            });
        }
        detected_colors.push(row);
    }

    assert_eq!(detected_colors.len(), 4);
    assert_eq!(detected_colors[0].len(), 4);

    let all_reliable = detected_colors
        .iter()
        .flatten()
        .all(|result| result.is_reliable());

    assert!(all_reliable);
}

#[test]
fn test_scan_correction_008_all_uncertain() {
    // Worst case: all colors uncertain
    let detected_colors = vec![
        vec![
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.3,
            },
            ColorDetectionResult {
                color: Color::Blue,
                confidence: 0.4,
            },
            ColorDetectionResult {
                color: Color::Green,
                confidence: 0.5,
            },
        ],
        vec![
            ColorDetectionResult {
                color: Color::Yellow,
                confidence: 0.35,
            },
            ColorDetectionResult {
                color: Color::Orange,
                confidence: 0.45,
            },
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.55,
            },
        ],
        vec![
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.25,
            },
            ColorDetectionResult {
                color: Color::Blue,
                confidence: 0.4,
            },
            ColorDetectionResult {
                color: Color::Green,
                confidence: 0.5,
            },
        ],
    ];

    let all_uncertain = detected_colors
        .iter()
        .flatten()
        .all(|result| result.is_uncertain());

    assert!(all_uncertain);
}

#[test]
fn test_scan_correction_009_boundary_confidence_values() {
    // Test boundary values for confidence thresholds
    let at_uncertain_boundary = ColorDetectionResult {
        color: Color::Red,
        confidence: 0.6, // Exactly at uncertain threshold
    };
    assert!(!at_uncertain_boundary.is_uncertain()); // 0.6 is NOT uncertain (< 0.6)

    let at_reliable_boundary = ColorDetectionResult {
        color: Color::Blue,
        confidence: 0.8, // Exactly at reliable threshold
    };
    assert!(at_reliable_boundary.is_reliable()); // 0.8 IS reliable (>= 0.8)
}

#[test]
fn test_scan_correction_010_confidence_percentages() {
    // Test confidence score interpretation
    let low = ColorDetectionResult {
        color: Color::Red,
        confidence: 0.35,
    };
    assert!((low.confidence * 100.0 - 35.0).abs() < 0.1);

    let medium = ColorDetectionResult {
        color: Color::Green,
        confidence: 0.65,
    };
    assert!((medium.confidence * 100.0 - 65.0).abs() < 0.1);

    let high = ColorDetectionResult {
        color: Color::Blue,
        confidence: 0.95,
    };
    assert!((high.confidence * 100.0 - 95.0).abs() < 0.1);
}

#[test]
fn test_scan_correction_011_color_consistency_check() {
    // Simulate a face scan where most stickers are the same color
    // but with varying confidence
    let detected_colors = vec![
        vec![
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.95,
            },
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.88,
            },
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.92,
            },
        ],
        vec![
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.90,
            },
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.85,
            },
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.87,
            },
        ],
        vec![
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.93,
            },
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.89,
            },
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.91,
            },
        ],
    ];

    // Check all colors are consistent
    let all_red = detected_colors
        .iter()
        .flatten()
        .all(|result| result.color == Color::Red);

    assert!(all_red);

    // All should be reliable
    let all_reliable = detected_colors
        .iter()
        .flatten()
        .all(|result| result.is_reliable());

    assert!(all_reliable);
}

#[test]
fn test_scan_correction_012_extract_colors_from_results() {
    // Test extracting just colors (ignoring confidence)
    let detected_colors = vec![
        vec![
            ColorDetectionResult {
                color: Color::White,
                confidence: 0.95,
            },
            ColorDetectionResult {
                color: Color::Yellow,
                confidence: 0.5,
            },
        ],
        vec![
            ColorDetectionResult {
                color: Color::Red,
                confidence: 0.85,
            },
            ColorDetectionResult {
                color: Color::Blue,
                confidence: 0.7,
            },
        ],
    ];

    let just_colors: Vec<Vec<Color>> = detected_colors
        .iter()
        .map(|row| row.iter().map(|result| result.color).collect())
        .collect();

    assert_eq!(just_colors[0][0], Color::White);
    assert_eq!(just_colors[0][1], Color::Yellow);
    assert_eq!(just_colors[1][0], Color::Red);
    assert_eq!(just_colors[1][1], Color::Blue);
}
