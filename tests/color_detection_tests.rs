/// Integration tests for HSV color detection algorithm
///
/// These tests verify the R4.3 acceptance criteria:
/// - Convert RGB to HSV
/// - Classify pixels into 6 colors
/// - Handle color variations
/// - 90%+ accuracy in good lighting

use rubiks_cube_solver::camera::{
    detect_color, detect_colors_in_grid, rgb_to_hsv, ColorDetectionConfig, RGB,
};
use rubiks_cube_solver::cube::Color;

// ===== RGB to HSV Conversion Tests =====

#[test]
fn test_rgb_to_hsv_primary_colors() {
    // Pure red
    let red = RGB::new(255, 0, 0);
    let hsv = rgb_to_hsv(red);
    assert!((hsv.h - 0.0).abs() < 1.0);
    assert!((hsv.s - 1.0).abs() < 0.01);
    assert!((hsv.v - 1.0).abs() < 0.01);

    // Pure green
    let green = RGB::new(0, 255, 0);
    let hsv = rgb_to_hsv(green);
    assert!((hsv.h - 120.0).abs() < 1.0);
    assert!((hsv.s - 1.0).abs() < 0.01);
    assert!((hsv.v - 1.0).abs() < 0.01);

    // Pure blue
    let blue = RGB::new(0, 0, 255);
    let hsv = rgb_to_hsv(blue);
    assert!((hsv.h - 240.0).abs() < 1.0);
    assert!((hsv.s - 1.0).abs() < 0.01);
    assert!((hsv.v - 1.0).abs() < 0.01);
}

#[test]
fn test_rgb_to_hsv_achromatic() {
    // Pure white
    let white = RGB::new(255, 255, 255);
    let hsv = rgb_to_hsv(white);
    assert!((hsv.s - 0.0).abs() < 0.01);
    assert!((hsv.v - 1.0).abs() < 0.01);

    // Pure black
    let black = RGB::new(0, 0, 0);
    let hsv = rgb_to_hsv(black);
    assert!((hsv.s - 0.0).abs() < 0.01);
    assert!((hsv.v - 0.0).abs() < 0.01);

    // Gray
    let gray = RGB::new(128, 128, 128);
    let hsv = rgb_to_hsv(gray);
    assert!((hsv.s - 0.0).abs() < 0.01);
    assert!((hsv.v - 0.502).abs() < 0.01); // 128/255 ≈ 0.502
}

#[test]
fn test_rgb_to_hsv_secondary_colors() {
    // Cyan
    let cyan = RGB::new(0, 255, 255);
    let hsv = rgb_to_hsv(cyan);
    assert!((hsv.h - 180.0).abs() < 1.0);

    // Magenta
    let magenta = RGB::new(255, 0, 255);
    let hsv = rgb_to_hsv(magenta);
    assert!((hsv.h - 300.0).abs() < 1.0);

    // Yellow
    let yellow = RGB::new(255, 255, 0);
    let hsv = rgb_to_hsv(yellow);
    assert!((hsv.h - 60.0).abs() < 1.0);
}

// ===== Standard Rubik's Cube Color Detection Tests =====

#[test]
fn test_detect_rubiks_white() {
    let config = ColorDetectionConfig::default();

    // Standard Rubik's white (#FFFFFF)
    assert_eq!(
        detect_color(RGB::new(255, 255, 255), &config),
        Some(Color::White)
    );

    // Slightly off-white
    assert_eq!(
        detect_color(RGB::new(240, 240, 240), &config),
        Some(Color::White)
    );
    assert_eq!(
        detect_color(RGB::new(230, 235, 240), &config),
        Some(Color::White)
    );
}

#[test]
fn test_detect_rubiks_yellow() {
    let config = ColorDetectionConfig::default();

    // Standard Rubik's yellow (#FFD500)
    assert_eq!(
        detect_color(RGB::new(255, 213, 0), &config),
        Some(Color::Yellow)
    );

    // Variations of yellow
    assert_eq!(
        detect_color(RGB::new(255, 215, 0), &config),
        Some(Color::Yellow)
    );
    assert_eq!(
        detect_color(RGB::new(245, 205, 10), &config),
        Some(Color::Yellow)
    );
}

#[test]
fn test_detect_rubiks_red() {
    let config = ColorDetectionConfig::default();

    // Standard Rubik's red (#C41E3A)
    assert_eq!(
        detect_color(RGB::new(196, 30, 58), &config),
        Some(Color::Red)
    );

    // Variations of red
    assert_eq!(
        detect_color(RGB::new(220, 20, 60), &config),
        Some(Color::Red)
    );
    assert_eq!(
        detect_color(RGB::new(180, 30, 50), &config),
        Some(Color::Red)
    );
}

#[test]
fn test_detect_rubiks_orange() {
    let config = ColorDetectionConfig::default();

    // Standard Rubik's orange (#FF5800)
    assert_eq!(
        detect_color(RGB::new(255, 88, 0), &config),
        Some(Color::Orange)
    );

    // Variations of orange
    assert_eq!(
        detect_color(RGB::new(255, 100, 0), &config),
        Some(Color::Orange)
    );
    assert_eq!(
        detect_color(RGB::new(240, 80, 10), &config),
        Some(Color::Orange)
    );
}

#[test]
fn test_detect_rubiks_blue() {
    let config = ColorDetectionConfig::default();

    // Standard Rubik's blue (#0051BA)
    assert_eq!(
        detect_color(RGB::new(0, 81, 186), &config),
        Some(Color::Blue)
    );

    // Variations of blue
    assert_eq!(
        detect_color(RGB::new(0, 90, 200), &config),
        Some(Color::Blue)
    );
    assert_eq!(
        detect_color(RGB::new(10, 70, 180), &config),
        Some(Color::Blue)
    );
}

#[test]
fn test_detect_rubiks_green() {
    let config = ColorDetectionConfig::default();

    // Standard Rubik's green (#009E60)
    assert_eq!(
        detect_color(RGB::new(0, 158, 96), &config),
        Some(Color::Green)
    );

    // Variations of green
    assert_eq!(
        detect_color(RGB::new(0, 150, 90), &config),
        Some(Color::Green)
    );
    assert_eq!(
        detect_color(RGB::new(10, 170, 100), &config),
        Some(Color::Green)
    );
}

// ===== Color Variation Handling Tests =====

#[test]
fn test_lighting_variations_bright() {
    let config = ColorDetectionConfig::default();

    // Bright lighting makes colors lighter
    assert_eq!(
        detect_color(RGB::new(255, 230, 230), &config),
        Some(Color::White)
    );
    assert_eq!(
        detect_color(RGB::new(255, 240, 100), &config),
        Some(Color::Yellow)
    );
}

#[test]
fn test_lighting_variations_dim() {
    let config = ColorDetectionConfig::default();

    // Dim lighting makes colors darker
    assert_eq!(
        detect_color(RGB::new(150, 20, 40), &config),
        Some(Color::Red)
    );
    assert_eq!(
        detect_color(RGB::new(0, 50, 120), &config),
        Some(Color::Blue)
    );
}

#[test]
fn test_shadow_regions_filtered() {
    let config = ColorDetectionConfig::default();

    // Very dark colors should return None (shadows/black areas)
    assert_eq!(detect_color(RGB::new(10, 10, 10), &config), None);
    assert_eq!(detect_color(RGB::new(5, 5, 5), &config), None);
    assert_eq!(detect_color(RGB::new(20, 20, 20), &config), None);
}

#[test]
fn test_ambiguous_colors_filtered() {
    let config = ColorDetectionConfig::default();

    // Gray/unsaturated colors should return None (not enough color information)
    assert_eq!(detect_color(RGB::new(128, 128, 128), &config), None);
    assert_eq!(detect_color(RGB::new(100, 105, 100), &config), None);
}

// ===== Grid Detection Tests =====

#[test]
fn test_detect_colors_in_2x2_grid() {
    let config = ColorDetectionConfig::default();

    // Create a simple 4x4 pixel image with 4 colors (2x2 grid)
    // Top-left: White, Top-right: Red
    // Bottom-left: Blue, Bottom-right: Green
    let mut pixels = vec![0u8; 4 * 4 * 3];

    // Top-left quadrant: White
    for y in 0..2 {
        for x in 0..2 {
            let idx = ((y * 4 + x) * 3) as usize;
            pixels[idx] = 255;
            pixels[idx + 1] = 255;
            pixels[idx + 2] = 255;
        }
    }

    // Top-right quadrant: Red
    for y in 0..2 {
        for x in 2..4 {
            let idx = ((y * 4 + x) * 3) as usize;
            pixels[idx] = 196;
            pixels[idx + 1] = 30;
            pixels[idx + 2] = 58;
        }
    }

    // Bottom-left quadrant: Blue
    for y in 2..4 {
        for x in 0..2 {
            let idx = ((y * 4 + x) * 3) as usize;
            pixels[idx] = 0;
            pixels[idx + 1] = 81;
            pixels[idx + 2] = 186;
        }
    }

    // Bottom-right quadrant: Green
    for y in 2..4 {
        for x in 2..4 {
            let idx = ((y * 4 + x) * 3) as usize;
            pixels[idx] = 0;
            pixels[idx + 1] = 158;
            pixels[idx + 2] = 96;
        }
    }

    let grid = detect_colors_in_grid(&pixels, 4, 4, 2, &config);
    assert!(grid.is_some());

    let grid = grid.unwrap();
    assert_eq!(grid.len(), 2);
    assert_eq!(grid[0].len(), 2);
    assert_eq!(grid[1].len(), 2);

    assert_eq!(grid[0][0], Color::White);
    assert_eq!(grid[0][1], Color::Red);
    assert_eq!(grid[1][0], Color::Blue);
    assert_eq!(grid[1][1], Color::Green);
}

#[test]
fn test_detect_colors_in_3x3_grid() {
    let config = ColorDetectionConfig::default();

    // Create a 9x9 pixel image with 9 colors (3x3 grid)
    let mut pixels = vec![0u8; 9 * 9 * 3];

    let colors = [
        (255, 255, 255), // White
        (255, 213, 0),   // Yellow
        (196, 30, 58),   // Red
        (255, 88, 0),    // Orange
        (0, 81, 186),    // Blue
        (0, 158, 96),    // Green
        (255, 255, 255), // White
        (255, 213, 0),   // Yellow
        (196, 30, 58),   // Red
    ];

    for (cell_idx, &(r, g, b)) in colors.iter().enumerate() {
        let cell_row = cell_idx / 3;
        let cell_col = cell_idx % 3;

        for y in (cell_row * 3)..(cell_row * 3 + 3) {
            for x in (cell_col * 3)..(cell_col * 3 + 3) {
                let idx = ((y * 9 + x) * 3) as usize;
                pixels[idx] = r;
                pixels[idx + 1] = g;
                pixels[idx + 2] = b;
            }
        }
    }

    let grid = detect_colors_in_grid(&pixels, 9, 9, 3, &config);
    assert!(grid.is_some());

    let grid = grid.unwrap();
    assert_eq!(grid.len(), 3);
    assert_eq!(grid[0][0], Color::White);
    assert_eq!(grid[0][1], Color::Yellow);
    assert_eq!(grid[0][2], Color::Red);
    assert_eq!(grid[1][0], Color::Orange);
    assert_eq!(grid[1][1], Color::Blue);
    assert_eq!(grid[1][2], Color::Green);
}

#[test]
fn test_detect_colors_in_grid_with_noise() {
    let config = ColorDetectionConfig::default();

    // Create a 9x9 pixel image, mostly red but with some noise pixels
    let mut pixels = vec![0u8; 9 * 9 * 3];

    // Fill with red
    for i in 0..(9 * 9) {
        let idx = i * 3;
        pixels[idx] = 196;
        pixels[idx + 1] = 30;
        pixels[idx + 2] = 58;
    }

    // Add some noise pixels (but majority is still red)
    pixels[0] = 100;
    pixels[1] = 100;
    pixels[2] = 100;

    let grid = detect_colors_in_grid(&pixels, 9, 9, 3, &config);
    assert!(grid.is_some());

    let grid = grid.unwrap();
    // Should detect red in all cells due to majority vote
    for row in &grid {
        for &color in row {
            assert_eq!(color, Color::Red);
        }
    }
}

// ===== Accuracy Tests =====

#[test]
fn test_color_detection_accuracy_all_six_colors() {
    let config = ColorDetectionConfig::default();

    // Test standard Rubik's cube colors
    let test_cases = vec![
        (RGB::new(255, 255, 255), Color::White),
        (RGB::new(255, 213, 0), Color::Yellow),
        (RGB::new(196, 30, 58), Color::Red),
        (RGB::new(255, 88, 0), Color::Orange),
        (RGB::new(0, 81, 186), Color::Blue),
        (RGB::new(0, 158, 96), Color::Green),
    ];

    let mut correct = 0;
    for (rgb, expected) in &test_cases {
        if detect_color(*rgb, &config) == Some(*expected) {
            correct += 1;
        }
    }

    let accuracy = (correct as f32 / test_cases.len() as f32) * 100.0;
    assert!(
        accuracy >= 90.0,
        "Accuracy {}% is below 90% threshold",
        accuracy
    );
    assert_eq!(correct, test_cases.len()); // Should be 100% for standard colors
}

#[test]
fn test_color_detection_accuracy_with_variations() {
    let config = ColorDetectionConfig::default();

    // Test color variations (different lighting, slight hue shifts)
    let test_cases = vec![
        // White variations
        (RGB::new(255, 255, 255), Color::White),
        (RGB::new(240, 240, 240), Color::White),
        (RGB::new(230, 235, 240), Color::White),
        // Yellow variations
        (RGB::new(255, 213, 0), Color::Yellow),
        (RGB::new(255, 215, 0), Color::Yellow),
        (RGB::new(245, 205, 10), Color::Yellow),
        // Red variations
        (RGB::new(196, 30, 58), Color::Red),
        (RGB::new(220, 20, 60), Color::Red),
        (RGB::new(180, 30, 50), Color::Red),
        // Orange variations
        (RGB::new(255, 88, 0), Color::Orange),
        (RGB::new(255, 100, 0), Color::Orange),
        (RGB::new(240, 80, 10), Color::Orange),
        // Blue variations
        (RGB::new(0, 81, 186), Color::Blue),
        (RGB::new(0, 90, 200), Color::Blue),
        (RGB::new(10, 70, 180), Color::Blue),
        // Green variations
        (RGB::new(0, 158, 96), Color::Green),
        (RGB::new(0, 150, 90), Color::Green),
        (RGB::new(10, 170, 100), Color::Green),
    ];

    let mut correct = 0;
    for (rgb, expected) in &test_cases {
        if detect_color(*rgb, &config) == Some(*expected) {
            correct += 1;
        }
    }

    let accuracy = (correct as f32 / test_cases.len() as f32) * 100.0;
    assert!(
        accuracy >= 90.0,
        "Accuracy {}% is below 90% threshold",
        accuracy
    );
}

#[test]
fn test_invalid_grid_size() {
    let config = ColorDetectionConfig::default();

    // Wrong pixel data size
    let pixels = vec![0u8; 100];
    let result = detect_colors_in_grid(&pixels, 10, 10, 3, &config);
    assert!(result.is_none());
}
