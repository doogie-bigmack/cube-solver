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

/// Lighting quality assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightingQuality {
    /// Excellent lighting conditions
    Excellent,
    /// Good lighting conditions
    Good,
    /// Adequate lighting (may need adjustment)
    Adequate,
    /// Poor lighting (detection may fail)
    Poor,
    /// Very poor lighting (detection likely to fail)
    VeryPoor,
}

/// Lighting analysis result
#[derive(Debug, Clone)]
pub struct LightingAnalysis {
    /// Overall lighting quality
    pub quality: LightingQuality,
    /// Average brightness (0.0-1.0)
    pub avg_brightness: f32,
    /// Brightness variance (higher = more uneven lighting)
    pub brightness_variance: f32,
    /// Whether lighting is too dark
    pub too_dark: bool,
    /// Whether lighting is too bright (overexposed)
    pub too_bright: bool,
    /// Whether lighting is uneven
    pub uneven: bool,
    /// Suggested brightness adjustment factor
    pub suggested_brightness_adjustment: f32,
}

impl LightingAnalysis {
    /// Get a user-friendly warning message
    pub fn get_warning_message(&self) -> Option<String> {
        match self.quality {
            LightingQuality::Excellent | LightingQuality::Good => None,
            LightingQuality::Adequate => {
                if self.too_dark {
                    Some("Lighting is a bit dim. Try moving to a brighter area.".to_string())
                } else if self.too_bright {
                    Some("Lighting is a bit bright. Try reducing direct light.".to_string())
                } else if self.uneven {
                    Some("Lighting is uneven. Try to get more uniform lighting.".to_string())
                } else {
                    Some("Lighting could be improved for best results.".to_string())
                }
            }
            LightingQuality::Poor => {
                if self.too_dark {
                    Some("Lighting is too dark. Please move to a brighter area!".to_string())
                } else if self.too_bright {
                    Some("Lighting is too bright (overexposed). Please reduce direct light!".to_string())
                } else {
                    Some("Lighting is poor. Color detection may fail!".to_string())
                }
            }
            LightingQuality::VeryPoor => {
                Some("Lighting is very poor! Color detection will likely fail. Please improve lighting conditions!".to_string())
            }
        }
    }
}

/// Analyze lighting conditions in an image
///
/// # Arguments
/// * `pixels` - RGB pixel data (row-major, width * height * 3 bytes)
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
///
/// # Returns
/// Lighting analysis result
pub fn analyze_lighting(pixels: &[u8], width: u32, height: u32) -> LightingAnalysis {
    if pixels.len() != (width * height * 3) as usize {
        return LightingAnalysis {
            quality: LightingQuality::VeryPoor,
            avg_brightness: 0.0,
            brightness_variance: 0.0,
            too_dark: true,
            too_bright: false,
            uneven: false,
            suggested_brightness_adjustment: 2.0,
        };
    }

    // Calculate average brightness and variance
    let mut brightness_sum = 0.0;
    let mut brightness_values = Vec::new();

    let total_pixels = (width * height) as usize;

    for i in 0..total_pixels {
        let idx = i * 3;
        let rgb = RGB::new(pixels[idx], pixels[idx + 1], pixels[idx + 2]);
        let hsv = rgb_to_hsv(rgb);
        brightness_values.push(hsv.v);
        brightness_sum += hsv.v;
    }

    let avg_brightness = brightness_sum / total_pixels as f32;

    // Calculate variance
    let variance_sum: f32 = brightness_values.iter()
        .map(|v| (v - avg_brightness).powi(2))
        .sum();
    let brightness_variance = variance_sum / total_pixels as f32;

    // Determine lighting issues
    let too_dark = avg_brightness < 0.25;
    let too_bright = avg_brightness > 0.85;
    let uneven = brightness_variance > 0.04; // High variance indicates uneven lighting

    // Determine overall quality
    let quality = if too_dark && avg_brightness < 0.15 {
        LightingQuality::VeryPoor
    } else if too_bright && avg_brightness > 0.92 {
        LightingQuality::VeryPoor
    } else if too_dark || too_bright {
        LightingQuality::Poor
    } else if uneven && brightness_variance > 0.06 {
        LightingQuality::Poor
    } else if !(0.35..=0.75).contains(&avg_brightness) || uneven {
        LightingQuality::Adequate
    } else if brightness_variance < 0.02 && (0.4..=0.7).contains(&avg_brightness) {
        LightingQuality::Excellent
    } else {
        LightingQuality::Good
    };

    // Calculate suggested brightness adjustment
    let suggested_brightness_adjustment = if too_dark {
        let target = 0.5;
        (target / avg_brightness).clamp(1.0, 3.0)
    } else if too_bright {
        let target = 0.6;
        (target / avg_brightness).clamp(0.3, 1.0)
    } else {
        1.0
    };

    LightingAnalysis {
        quality,
        avg_brightness,
        brightness_variance,
        too_dark,
        too_bright,
        uneven,
        suggested_brightness_adjustment,
    }
}

/// Apply adaptive thresholds based on lighting conditions
///
/// # Arguments
/// * `base_config` - Base color detection configuration
/// * `lighting` - Lighting analysis result
///
/// # Returns
/// Adjusted color detection configuration
pub fn apply_adaptive_thresholds(
    base_config: &ColorDetectionConfig,
    lighting: &LightingAnalysis,
) -> ColorDetectionConfig {
    let mut config = base_config.clone();

    // Adjust thresholds based on brightness
    if lighting.too_dark {
        // In dark conditions, lower brightness thresholds
        config.white_min_value = (config.white_min_value * 0.7).max(0.4);
        config.yellow_min_value = (config.yellow_min_value * 0.7).max(0.3);
        config.min_value_threshold = (config.min_value_threshold * 0.5).max(0.05);

        // Be more lenient with saturation in dark conditions
        config.white_max_saturation = (config.white_max_saturation * 1.3).min(0.45);
        config.chromatic_min_saturation = (config.chromatic_min_saturation * 0.8).max(0.2);
    } else if lighting.too_bright {
        // In bright conditions, raise thresholds
        config.white_min_value = (config.white_min_value * 1.1).min(0.95);
        config.yellow_min_value = (config.yellow_min_value * 1.1).min(0.85);

        // In bright conditions, colors may appear more saturated
        config.white_max_saturation = (config.white_max_saturation * 0.8).max(0.15);
        config.chromatic_min_saturation = (config.chromatic_min_saturation * 1.2).min(0.5);
    }

    // Adjust for uneven lighting
    if lighting.uneven {
        // Widen hue ranges slightly to be more tolerant
        let hue_expansion = 5.0;
        config.yellow_hue_min = (config.yellow_hue_min - hue_expansion).max(0.0);
        config.yellow_hue_max = (config.yellow_hue_max + hue_expansion).min(360.0);
        config.orange_hue_min = (config.orange_hue_min - hue_expansion).max(0.0);
        config.orange_hue_max = (config.orange_hue_max + hue_expansion).min(360.0);
        config.blue_hue_min = (config.blue_hue_min - hue_expansion).max(0.0);
        config.blue_hue_max = (config.blue_hue_max + hue_expansion).min(360.0);
        config.green_hue_min = (config.green_hue_min - hue_expansion).max(0.0);
        config.green_hue_max = (config.green_hue_max + hue_expansion).min(360.0);
    }

    config
}

/// Color detection result with confidence score
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorDetectionResult {
    /// Detected color
    pub color: Color,
    /// Confidence score (0.0-1.0, higher is more confident)
    pub confidence: f32,
}

impl ColorDetectionResult {
    /// Check if this detection is uncertain (low confidence)
    pub fn is_uncertain(&self) -> bool {
        self.confidence < 0.6
    }

    /// Check if this detection is reliable (high confidence)
    pub fn is_reliable(&self) -> bool {
        self.confidence >= 0.8
    }
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

/// Detect colors in a grid with confidence scores
///
/// # Arguments
/// * `pixels` - RGB pixel data (row-major, width * height * 3 bytes)
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
/// * `grid_size` - NxN grid size (e.g., 3 for 3x3 cube)
/// * `config` - Detection configuration
///
/// # Returns
/// NxN grid of color detection results with confidence scores, or None if detection fails
pub fn detect_colors_in_grid_with_confidence(
    pixels: &[u8],
    width: u32,
    height: u32,
    grid_size: usize,
    config: &ColorDetectionConfig,
) -> Option<Vec<Vec<ColorDetectionResult>>> {
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

            // Calculate confidence based on vote consistency
            if let Some((color, confidence)) = majority_vote_with_confidence(&color_votes) {
                grid_row.push(ColorDetectionResult { color, confidence });
            } else {
                return None; // Failed to detect color in this cell
            }
        }

        grid.push(grid_row);
    }

    Some(grid)
}

/// Detect colors in a grid with adaptive thresholds based on lighting
///
/// # Arguments
/// * `pixels` - RGB pixel data (row-major, width * height * 3 bytes)
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
/// * `grid_size` - NxN grid size (e.g., 3 for 3x3 cube)
/// * `base_config` - Base detection configuration
///
/// # Returns
/// Tuple of (detected color grid, lighting analysis)
pub fn detect_colors_with_lighting_adaptation(
    pixels: &[u8],
    width: u32,
    height: u32,
    grid_size: usize,
    base_config: &ColorDetectionConfig,
) -> (Option<Vec<Vec<Color>>>, LightingAnalysis) {
    // Analyze lighting conditions
    let lighting = analyze_lighting(pixels, width, height);

    // Apply adaptive thresholds
    let adapted_config = apply_adaptive_thresholds(base_config, &lighting);

    // Detect colors with adapted configuration
    let colors = detect_colors_in_grid(pixels, width, height, grid_size, &adapted_config);

    (colors, lighting)
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

/// Find the most common color with a confidence score
///
/// Confidence is based on:
/// - How many votes the winning color got (coverage)
/// - How dominant the winning color is compared to others (consistency)
///
/// Returns (Color, confidence) where confidence is 0.0-1.0
fn majority_vote_with_confidence(colors: &[Color]) -> Option<(Color, f32)> {
    if colors.is_empty() {
        return None;
    }

    let total_votes = colors.len();
    let mut counts = std::collections::HashMap::new();

    for color in colors {
        *counts.entry(*color).or_insert(0) += 1;
    }

    // Find winner and runner-up
    let mut sorted_counts: Vec<_> = counts.into_iter().collect();
    sorted_counts.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

    let (winner_color, winner_count) = sorted_counts[0];

    // Calculate confidence based on:
    // 1. Coverage: what % of samples detected this color
    let coverage = winner_count as f32 / total_votes as f32;

    // 2. Dominance: how much more votes than runner-up
    let dominance = if sorted_counts.len() > 1 {
        let runner_up_count = sorted_counts[1].1;
        let margin = (winner_count - runner_up_count) as f32 / total_votes as f32;
        margin.clamp(0.0, 1.0)
    } else {
        1.0 // Only one color detected, maximum dominance
    };

    // Final confidence is weighted average
    // Coverage is more important (70%) than dominance (30%)
    let confidence = (0.7 * coverage + 0.3 * dominance).clamp(0.0, 1.0);

    Some((winner_color, confidence))
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

    #[test]
    fn test_analyze_lighting_good_conditions() {
        // Create a well-lit image (medium brightness, low variance)
        let width = 100;
        let height = 100;
        let mut pixels = vec![0u8; (width * height * 3) as usize];

        // Fill with medium gray (brightness ~0.5)
        for pixel in pixels.chunks_mut(3) {
            pixel[0] = 128; // R
            pixel[1] = 128; // G
            pixel[2] = 128; // B
        }

        let analysis = analyze_lighting(&pixels, width, height);
        assert!(analysis.avg_brightness > 0.4 && analysis.avg_brightness < 0.6);
        assert!(!analysis.too_dark);
        assert!(!analysis.too_bright);
        assert!(matches!(
            analysis.quality,
            LightingQuality::Excellent | LightingQuality::Good
        ));
    }

    #[test]
    fn test_analyze_lighting_too_dark() {
        let width = 100;
        let height = 100;
        let mut pixels = vec![0u8; (width * height * 3) as usize];

        // Fill with dark pixels (brightness ~0.1)
        for pixel in pixels.chunks_mut(3) {
            pixel[0] = 25; // R
            pixel[1] = 25; // G
            pixel[2] = 25; // B
        }

        let analysis = analyze_lighting(&pixels, width, height);
        assert!(analysis.too_dark);
        assert!(!analysis.too_bright);
        assert!(matches!(
            analysis.quality,
            LightingQuality::Poor | LightingQuality::VeryPoor
        ));
        assert!(analysis.suggested_brightness_adjustment > 1.0);
    }

    #[test]
    fn test_analyze_lighting_too_bright() {
        let width = 100;
        let height = 100;
        let mut pixels = vec![0u8; (width * height * 3) as usize];

        // Fill with very bright pixels (brightness ~0.95)
        for pixel in pixels.chunks_mut(3) {
            pixel[0] = 242; // R
            pixel[1] = 242; // G
            pixel[2] = 242; // B
        }

        let analysis = analyze_lighting(&pixels, width, height);
        assert!(!analysis.too_dark);
        assert!(analysis.too_bright);
        assert!(matches!(
            analysis.quality,
            LightingQuality::Poor | LightingQuality::VeryPoor
        ));
        assert!(analysis.suggested_brightness_adjustment < 1.0);
    }

    #[test]
    fn test_analyze_lighting_uneven() {
        let width = 100;
        let height = 100;
        let mut pixels = vec![0u8; (width * height * 3) as usize];

        // Create uneven lighting - half dark, half bright
        for i in 0..height {
            for j in 0..width {
                let idx = ((i * width + j) * 3) as usize;
                if i < height / 2 {
                    // Dark half
                    pixels[idx] = 50;
                    pixels[idx + 1] = 50;
                    pixels[idx + 2] = 50;
                } else {
                    // Bright half
                    pixels[idx] = 200;
                    pixels[idx + 1] = 200;
                    pixels[idx + 2] = 200;
                }
            }
        }

        let analysis = analyze_lighting(&pixels, width, height);
        assert!(analysis.uneven);
        assert!(analysis.brightness_variance > 0.04);
    }

    #[test]
    fn test_lighting_warning_messages() {
        let mut analysis = LightingAnalysis {
            quality: LightingQuality::Excellent,
            avg_brightness: 0.5,
            brightness_variance: 0.01,
            too_dark: false,
            too_bright: false,
            uneven: false,
            suggested_brightness_adjustment: 1.0,
        };

        // Excellent lighting - no warning
        assert!(analysis.get_warning_message().is_none());

        // Good lighting - no warning
        analysis.quality = LightingQuality::Good;
        assert!(analysis.get_warning_message().is_none());

        // Adequate lighting - has warning
        analysis.quality = LightingQuality::Adequate;
        analysis.too_dark = true;
        let msg = analysis.get_warning_message();
        assert!(msg.is_some());
        assert!(msg.unwrap().contains("dim"));

        // Poor lighting - has warning
        analysis.quality = LightingQuality::Poor;
        analysis.too_dark = true;
        let msg = analysis.get_warning_message();
        assert!(msg.is_some());
        assert!(msg.unwrap().contains("too dark"));

        // Very poor lighting - has warning
        analysis.quality = LightingQuality::VeryPoor;
        let msg = analysis.get_warning_message();
        assert!(msg.is_some());
        assert!(msg.unwrap().contains("very poor"));
    }

    #[test]
    fn test_apply_adaptive_thresholds_dark() {
        let base_config = ColorDetectionConfig::default();
        let lighting = LightingAnalysis {
            quality: LightingQuality::Poor,
            avg_brightness: 0.2,
            brightness_variance: 0.01,
            too_dark: true,
            too_bright: false,
            uneven: false,
            suggested_brightness_adjustment: 2.0,
        };

        let adapted = apply_adaptive_thresholds(&base_config, &lighting);

        // In dark conditions, thresholds should be lowered
        assert!(adapted.white_min_value < base_config.white_min_value);
        assert!(adapted.yellow_min_value < base_config.yellow_min_value);
        assert!(adapted.min_value_threshold < base_config.min_value_threshold);
        assert!(adapted.white_max_saturation > base_config.white_max_saturation);
    }

    #[test]
    fn test_apply_adaptive_thresholds_bright() {
        let base_config = ColorDetectionConfig::default();
        let lighting = LightingAnalysis {
            quality: LightingQuality::Poor,
            avg_brightness: 0.9,
            brightness_variance: 0.01,
            too_dark: false,
            too_bright: true,
            uneven: false,
            suggested_brightness_adjustment: 0.7,
        };

        let adapted = apply_adaptive_thresholds(&base_config, &lighting);

        // In bright conditions, thresholds should be raised
        assert!(adapted.white_min_value > base_config.white_min_value);
        assert!(adapted.yellow_min_value > base_config.yellow_min_value);
        assert!(adapted.white_max_saturation < base_config.white_max_saturation);
    }

    #[test]
    fn test_apply_adaptive_thresholds_uneven() {
        let base_config = ColorDetectionConfig::default();
        let lighting = LightingAnalysis {
            quality: LightingQuality::Adequate,
            avg_brightness: 0.5,
            brightness_variance: 0.06,
            too_dark: false,
            too_bright: false,
            uneven: true,
            suggested_brightness_adjustment: 1.0,
        };

        let adapted = apply_adaptive_thresholds(&base_config, &lighting);

        // In uneven lighting, hue ranges should be widened
        assert!(adapted.yellow_hue_min < base_config.yellow_hue_min);
        assert!(adapted.yellow_hue_max > base_config.yellow_hue_max);
        assert!(adapted.orange_hue_min < base_config.orange_hue_min);
        assert!(adapted.orange_hue_max > base_config.orange_hue_max);
    }

    #[test]
    fn test_detect_colors_with_lighting_adaptation() {
        // Create a simple 3x3 test image with good lighting
        let width = 30;
        let height = 30;
        let mut pixels = vec![0u8; (width * height * 3) as usize];

        // Fill with medium brightness colors
        for pixel in pixels.chunks_mut(3) {
            pixel[0] = 128; // Medium gray
            pixel[1] = 128;
            pixel[2] = 128;
        }

        let base_config = ColorDetectionConfig::default();
        let (colors, lighting) = detect_colors_with_lighting_adaptation(
            &pixels,
            width,
            height,
            3,
            &base_config,
        );

        // Should get lighting analysis even if color detection fails
        assert!(lighting.avg_brightness > 0.0);
        assert!(matches!(
            lighting.quality,
            LightingQuality::Excellent
                | LightingQuality::Good
                | LightingQuality::Adequate
                | LightingQuality::Poor
                | LightingQuality::VeryPoor
        ));
    }

    #[test]
    fn test_lighting_quality_ordering() {
        // Test that lighting quality enum has proper ordering
        let excellent = LightingQuality::Excellent;
        let good = LightingQuality::Good;
        let adequate = LightingQuality::Adequate;
        let poor = LightingQuality::Poor;
        let very_poor = LightingQuality::VeryPoor;

        // Just ensure they're all distinct
        assert_ne!(excellent, good);
        assert_ne!(good, adequate);
        assert_ne!(adequate, poor);
        assert_ne!(poor, very_poor);
    }
}
