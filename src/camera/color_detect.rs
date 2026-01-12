/// Color detection algorithm for Rubik's cube scanning
///
/// This module implements HSV-based color detection to classify pixels
/// into one of the 6 standard Rubik's cube colors.

use crate::cube::Color;

/// HSV color representation (Hue, Saturation, Value)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HSV {
    /// Hue: 0.0-360.0 degrees
    pub h: f32,
    /// Saturation: 0.0-1.0
    pub s: f32,
    /// Value (brightness): 0.0-1.0
    pub v: f32,
}

/// RGB color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGB {
    /// Red: 0-255
    pub r: u8,
    /// Green: 0-255
    pub g: u8,
    /// Blue: 0-255
    pub b: u8,
}

impl RGB {
    /// Create a new RGB color
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }
}

/// Convert RGB to HSV color space
///
/// # Arguments
/// * `rgb` - RGB color to convert
///
/// # Returns
/// HSV representation of the color
pub fn rgb_to_hsv(rgb: RGB) -> HSV {
    let r = rgb.r as f32 / 255.0;
    let g = rgb.g as f32 / 255.0;
    let b = rgb.b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    // Calculate Value
    let v = max;

    // Calculate Saturation
    let s = if max == 0.0 { 0.0 } else { delta / max };

    // Calculate Hue
    let h = if delta == 0.0 {
        0.0 // Undefined, we'll use 0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    // Normalize hue to 0-360 range
    let h = if h < 0.0 { h + 360.0 } else { h };

    HSV { h, s, v }
}

/// Color detection thresholds for each Rubik's cube color
///
/// These ranges are based on standard Rubik's cube colors:
/// - White: High value, low saturation
/// - Yellow: Hue around 50-65°, high value
/// - Red: Hue around 0-20° or 340-360°, high saturation
/// - Orange: Hue around 20-40°, high saturation
/// - Blue: Hue around 200-240°, high saturation
/// - Green: Hue around 110-150°, high saturation
#[derive(Debug, Clone)]
pub struct ColorDetectionConfig {
    /// Minimum value (brightness) threshold for white
    pub white_min_value: f32,
    /// Maximum saturation for white
    pub white_max_saturation: f32,

    /// Yellow hue range (degrees)
    pub yellow_hue_min: f32,
    pub yellow_hue_max: f32,
    pub yellow_min_value: f32,

    /// Red hue range (wraps around 360°)
    pub red_hue_min: f32,
    pub red_hue_max: f32,

    /// Orange hue range
    pub orange_hue_min: f32,
    pub orange_hue_max: f32,

    /// Blue hue range
    pub blue_hue_min: f32,
    pub blue_hue_max: f32,

    /// Green hue range
    pub green_hue_min: f32,
    pub green_hue_max: f32,

    /// Minimum saturation for chromatic colors (not white/gray)
    pub chromatic_min_saturation: f32,

    /// Minimum value for all colors (to filter black/dark areas)
    pub min_value_threshold: f32,
}

impl Default for ColorDetectionConfig {
    fn default() -> Self {
        ColorDetectionConfig {
            // White detection
            white_min_value: 0.7,
            white_max_saturation: 0.3,

            // Yellow: 50-65 degrees
            yellow_hue_min: 45.0,
            yellow_hue_max: 70.0,
            yellow_min_value: 0.6,

            // Red: 0-15 or 345-360 degrees
            red_hue_min: 345.0,
            red_hue_max: 15.0,

            // Orange: 15-40 degrees
            orange_hue_min: 15.0,
            orange_hue_max: 45.0,

            // Blue: 200-240 degrees
            blue_hue_min: 200.0,
            blue_hue_max: 240.0,

            // Green: 100-160 degrees
            green_hue_min: 100.0,
            green_hue_max: 160.0,

            // Chromatic colors need sufficient saturation
            chromatic_min_saturation: 0.3,

            // Filter out very dark pixels
            min_value_threshold: 0.2,
        }
    }
}

/// Classify an RGB color into one of the 6 Rubik's cube colors
///
/// # Arguments
/// * `rgb` - RGB color to classify
/// * `config` - Detection configuration with color ranges
///
/// # Returns
/// Detected Rubik's cube color, or None if color cannot be classified
pub fn detect_color(rgb: RGB, config: &ColorDetectionConfig) -> Option<Color> {
    let hsv = rgb_to_hsv(rgb);

    // Filter out very dark pixels (shadows, black areas)
    if hsv.v < config.min_value_threshold {
        return None;
    }

    // Detect white (high value, low saturation)
    if hsv.v >= config.white_min_value && hsv.s <= config.white_max_saturation {
        return Some(Color::White);
    }

    // For chromatic colors, require minimum saturation
    if hsv.s < config.chromatic_min_saturation {
        return None;
    }

    // Detect yellow (hue around 50-65°)
    if hsv.h >= config.yellow_hue_min
        && hsv.h <= config.yellow_hue_max
        && hsv.v >= config.yellow_min_value {
        return Some(Color::Yellow);
    }

    // Detect red (hue wraps around 0°)
    if hsv.h >= config.red_hue_min || hsv.h <= config.red_hue_max {
        return Some(Color::Red);
    }

    // Detect orange (hue around 20-40°)
    if hsv.h >= config.orange_hue_min && hsv.h <= config.orange_hue_max {
        return Some(Color::Orange);
    }

    // Detect blue (hue around 200-240°)
    if hsv.h >= config.blue_hue_min && hsv.h <= config.blue_hue_max {
        return Some(Color::Blue);
    }

    // Detect green (hue around 110-150°)
    if hsv.h >= config.green_hue_min && hsv.h <= config.green_hue_max {
        return Some(Color::Green);
    }

    // Unable to classify
    None
}

/// Detect colors in a grid of pixels (for cube face scanning)
///
/// # Arguments
/// * `pixels` - RGB pixel data (row-major, width * height * 3 bytes)
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
/// * `grid_size` - NxN grid size (e.g., 3 for 3x3 cube)
/// * `config` - Detection configuration
///
/// # Returns
/// NxN grid of detected colors, or None if detection fails
pub fn detect_colors_in_grid(
    pixels: &[u8],
    width: u32,
    height: u32,
    grid_size: usize,
    config: &ColorDetectionConfig,
) -> Option<Vec<Vec<Color>>> {
    if pixels.len() != (width * height * 3) as usize {
        return None;
    }

    let cell_width = width / grid_size as u32;
    let cell_height = height / grid_size as u32;

    let mut grid = Vec::new();

    for row in 0..grid_size {
        let mut grid_row = Vec::new();

        for col in 0..grid_size {
            // Sample from center of each grid cell
            let center_x = (col as u32 * cell_width) + cell_width / 2;
            let center_y = (row as u32 * cell_height) + cell_height / 2;

            // Sample a small region around center for more robust detection
            let mut color_votes: Vec<Color> = Vec::new();

            for dy in -2..=2 {
                for dx in -2..=2 {
                    let x = (center_x as i32 + dx).clamp(0, width as i32 - 1) as u32;
                    let y = (center_y as i32 + dy).clamp(0, height as i32 - 1) as u32;

                    let idx = ((y * width + x) * 3) as usize;
                    let rgb = RGB::new(pixels[idx], pixels[idx + 1], pixels[idx + 2]);

                    if let Some(color) = detect_color(rgb, config) {
                        color_votes.push(color);
                    }
                }
            }

            // Use majority vote for robustness
            if let Some(color) = majority_vote(&color_votes) {
                grid_row.push(color);
            } else {
                return None; // Failed to detect color in this cell
            }
        }

        grid.push(grid_row);
    }

    Some(grid)
}

/// Find the most common color in a list (majority vote)
fn majority_vote(colors: &[Color]) -> Option<Color> {
    if colors.is_empty() {
        return None;
    }

    let mut counts = std::collections::HashMap::new();
    for color in colors {
        *counts.entry(*color).or_insert(0) += 1;
    }

    counts.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(color, _)| color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_hsv_red() {
        let rgb = RGB::new(255, 0, 0);
        let hsv = rgb_to_hsv(rgb);
        assert!((hsv.h - 0.0).abs() < 1.0);
        assert!((hsv.s - 1.0).abs() < 0.01);
        assert!((hsv.v - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_rgb_to_hsv_green() {
        let rgb = RGB::new(0, 255, 0);
        let hsv = rgb_to_hsv(rgb);
        assert!((hsv.h - 120.0).abs() < 1.0);
        assert!((hsv.s - 1.0).abs() < 0.01);
        assert!((hsv.v - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_rgb_to_hsv_blue() {
        let rgb = RGB::new(0, 0, 255);
        let hsv = rgb_to_hsv(rgb);
        assert!((hsv.h - 240.0).abs() < 1.0);
        assert!((hsv.s - 1.0).abs() < 0.01);
        assert!((hsv.v - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_rgb_to_hsv_white() {
        let rgb = RGB::new(255, 255, 255);
        let hsv = rgb_to_hsv(rgb);
        assert!((hsv.s - 0.0).abs() < 0.01);
        assert!((hsv.v - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_rgb_to_hsv_black() {
        let rgb = RGB::new(0, 0, 0);
        let hsv = rgb_to_hsv(rgb);
        assert!((hsv.s - 0.0).abs() < 0.01);
        assert!((hsv.v - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_detect_white() {
        let config = ColorDetectionConfig::default();
        let rgb = RGB::new(240, 240, 240);
        assert_eq!(detect_color(rgb, &config), Some(Color::White));
    }

    #[test]
    fn test_detect_yellow() {
        let config = ColorDetectionConfig::default();
        let rgb = RGB::new(255, 215, 0); // Gold/Yellow
        assert_eq!(detect_color(rgb, &config), Some(Color::Yellow));
    }

    #[test]
    fn test_detect_red() {
        let config = ColorDetectionConfig::default();
        let rgb = RGB::new(196, 30, 58); // Rubik's red
        assert_eq!(detect_color(rgb, &config), Some(Color::Red));
    }

    #[test]
    fn test_detect_orange() {
        let config = ColorDetectionConfig::default();
        let rgb = RGB::new(255, 88, 0); // Rubik's orange
        assert_eq!(detect_color(rgb, &config), Some(Color::Orange));
    }

    #[test]
    fn test_detect_blue() {
        let config = ColorDetectionConfig::default();
        let rgb = RGB::new(0, 81, 186); // Rubik's blue
        assert_eq!(detect_color(rgb, &config), Some(Color::Blue));
    }

    #[test]
    fn test_detect_green() {
        let config = ColorDetectionConfig::default();
        let rgb = RGB::new(0, 158, 96); // Rubik's green
        assert_eq!(detect_color(rgb, &config), Some(Color::Green));
    }

    #[test]
    fn test_detect_black_returns_none() {
        let config = ColorDetectionConfig::default();
        let rgb = RGB::new(0, 0, 0);
        assert_eq!(detect_color(rgb, &config), None);
    }

    #[test]
    fn test_detect_gray_returns_none() {
        let config = ColorDetectionConfig::default();
        let rgb = RGB::new(128, 128, 128);
        assert_eq!(detect_color(rgb, &config), None);
    }

    #[test]
    fn test_majority_vote_single_color() {
        let colors = vec![Color::Red, Color::Red, Color::Red];
        assert_eq!(majority_vote(&colors), Some(Color::Red));
    }

    #[test]
    fn test_majority_vote_mixed() {
        let colors = vec![
            Color::Red,
            Color::Blue,
            Color::Red,
            Color::Red,
            Color::Blue,
        ];
        assert_eq!(majority_vote(&colors), Some(Color::Red));
    }

    #[test]
    fn test_majority_vote_empty() {
        let colors = vec![];
        assert_eq!(majority_vote(&colors), None);
    }
}
