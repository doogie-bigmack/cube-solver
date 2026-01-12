/// Color calibration mode for improving detection accuracy
///
/// This module allows users to calibrate color detection by showing
/// each of the 6 Rubik's cube colors to the camera and learning their
/// specific HSV ranges under current lighting conditions.

use crate::camera::color_detect::{ColorDetectionConfig, HSV, RGB, rgb_to_hsv};
use crate::cube::Color;
use serde::{Deserialize, Serialize};

/// Color sample collected during calibration
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorSample {
    /// The RGB value of the sample
    pub rgb: RGB,
    /// The HSV value of the sample
    pub hsv: HSV,
}

impl ColorSample {
    /// Create a new color sample from RGB
    pub fn new(rgb: RGB) -> Self {
        let hsv = rgb_to_hsv(rgb);
        ColorSample { rgb, hsv }
    }
}

/// Calibration data for a single color
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorCalibration {
    /// Color being calibrated
    pub color: Color,
    /// Number of samples collected
    pub sample_count: usize,
    /// Minimum hue observed
    pub hue_min: f32,
    /// Maximum hue observed
    pub hue_max: f32,
    /// Minimum saturation observed
    pub saturation_min: f32,
    /// Maximum saturation observed
    pub saturation_max: f32,
    /// Minimum value observed
    pub value_min: f32,
    /// Maximum value observed
    pub value_max: f32,
}

impl ColorCalibration {
    /// Create a new empty calibration for a color
    pub fn new(color: Color) -> Self {
        ColorCalibration {
            color,
            sample_count: 0,
            hue_min: 360.0,
            hue_max: 0.0,
            saturation_min: 1.0,
            saturation_max: 0.0,
            value_min: 1.0,
            value_max: 0.0,
        }
    }

    /// Add a sample to this calibration
    pub fn add_sample(&mut self, sample: ColorSample) {
        self.sample_count += 1;
        let hsv = sample.hsv;

        // Update ranges
        self.hue_min = self.hue_min.min(hsv.h);
        self.hue_max = self.hue_max.max(hsv.h);
        self.saturation_min = self.saturation_min.min(hsv.s);
        self.saturation_max = self.saturation_max.max(hsv.s);
        self.value_min = self.value_min.min(hsv.v);
        self.value_max = self.value_max.max(hsv.v);
    }

    /// Check if this calibration has enough samples
    pub fn is_complete(&self) -> bool {
        self.sample_count >= 10
    }

    /// Get the midpoint hue for this color
    pub fn midpoint_hue(&self) -> f32 {
        (self.hue_min + self.hue_max) / 2.0
    }
}

/// Calibration manager for all 6 colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationManager {
    /// Calibration data for each color
    pub calibrations: Vec<ColorCalibration>,
    /// Current color being calibrated (index)
    pub current_color_index: usize,
}

impl CalibrationManager {
    /// Create a new calibration manager
    pub fn new() -> Self {
        let colors = vec![
            Color::White,
            Color::Yellow,
            Color::Red,
            Color::Orange,
            Color::Blue,
            Color::Green,
        ];

        let calibrations = colors.into_iter()
            .map(ColorCalibration::new)
            .collect();

        CalibrationManager {
            calibrations,
            current_color_index: 0,
        }
    }

    /// Get the current color being calibrated
    pub fn current_color(&self) -> Color {
        self.calibrations[self.current_color_index].color
    }

    /// Get the current calibration
    pub fn current_calibration(&self) -> &ColorCalibration {
        &self.calibrations[self.current_color_index]
    }

    /// Add a sample for the current color
    pub fn add_sample(&mut self, sample: ColorSample) {
        self.calibrations[self.current_color_index].add_sample(sample);
    }

    /// Move to the next color
    pub fn next_color(&mut self) -> bool {
        if self.current_color_index < self.calibrations.len() - 1 {
            self.current_color_index += 1;
            true
        } else {
            false
        }
    }

    /// Check if all colors are calibrated
    pub fn is_complete(&self) -> bool {
        self.calibrations.iter().all(|c| c.is_complete())
    }

    /// Get progress (0-6)
    pub fn completed_colors(&self) -> usize {
        self.calibrations.iter()
            .filter(|c| c.is_complete())
            .count()
    }

    /// Generate a ColorDetectionConfig from calibration data
    pub fn to_detection_config(&self) -> ColorDetectionConfig {
        // Add 10% margin to ranges for robustness
        const MARGIN_FACTOR: f32 = 0.1;

        let mut config = ColorDetectionConfig::default();

        for calibration in &self.calibrations {
            let hue_margin = (calibration.hue_max - calibration.hue_min) * MARGIN_FACTOR;
            let sat_margin = (calibration.saturation_max - calibration.saturation_min) * MARGIN_FACTOR;
            let val_margin = (calibration.value_max - calibration.value_min) * MARGIN_FACTOR;

            match calibration.color {
                Color::White => {
                    config.white_min_value = (calibration.value_min - val_margin).max(0.0);
                    config.white_max_saturation = (calibration.saturation_max + sat_margin).min(1.0);
                }
                Color::Yellow => {
                    config.yellow_hue_min = (calibration.hue_min - hue_margin).max(0.0);
                    config.yellow_hue_max = (calibration.hue_max + hue_margin).min(360.0);
                    config.yellow_min_value = (calibration.value_min - val_margin).max(0.0);
                }
                Color::Red => {
                    // Red wraps around 0°, special handling
                    config.red_hue_min = (calibration.hue_min - hue_margin).max(0.0);
                    config.red_hue_max = (calibration.hue_max + hue_margin).min(360.0);
                }
                Color::Orange => {
                    config.orange_hue_min = (calibration.hue_min - hue_margin).max(0.0);
                    config.orange_hue_max = (calibration.hue_max + hue_margin).min(360.0);
                }
                Color::Blue => {
                    config.blue_hue_min = (calibration.hue_min - hue_margin).max(0.0);
                    config.blue_hue_max = (calibration.hue_max + hue_margin).min(360.0);
                }
                Color::Green => {
                    config.green_hue_min = (calibration.hue_min - hue_margin).max(0.0);
                    config.green_hue_max = (calibration.hue_max + hue_margin).min(360.0);
                }
            }
        }

        // Use learned minimum saturation for chromatic colors
        let chromatic_calibrations = self.calibrations.iter()
            .filter(|c| c.color != Color::White)
            .collect::<Vec<_>>();

        if !chromatic_calibrations.is_empty() {
            let min_sat = chromatic_calibrations.iter()
                .map(|c| c.saturation_min)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.3);
            config.chromatic_min_saturation = (min_sat * 0.8).max(0.1);
        }

        config
    }

    /// Reset calibration to start over
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Save calibration to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Load calibration from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl Default for CalibrationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_sample_creation() {
        let rgb = RGB::new(255, 0, 0);
        let sample = ColorSample::new(rgb);
        assert_eq!(sample.rgb, rgb);
        assert!((sample.hsv.h - 0.0).abs() < 1.0);
    }

    #[test]
    fn test_color_calibration_new() {
        let cal = ColorCalibration::new(Color::Red);
        assert_eq!(cal.color, Color::Red);
        assert_eq!(cal.sample_count, 0);
        assert!(!cal.is_complete());
    }

    #[test]
    fn test_color_calibration_add_sample() {
        let mut cal = ColorCalibration::new(Color::Red);
        let sample = ColorSample::new(RGB::new(255, 0, 0));
        cal.add_sample(sample);
        assert_eq!(cal.sample_count, 1);
    }

    #[test]
    fn test_color_calibration_is_complete() {
        let mut cal = ColorCalibration::new(Color::Red);
        for _ in 0..10 {
            cal.add_sample(ColorSample::new(RGB::new(255, 0, 0)));
        }
        assert!(cal.is_complete());
    }

    #[test]
    fn test_calibration_manager_new() {
        let manager = CalibrationManager::new();
        assert_eq!(manager.calibrations.len(), 6);
        assert_eq!(manager.current_color_index, 0);
    }

    #[test]
    fn test_calibration_manager_current_color() {
        let manager = CalibrationManager::new();
        assert_eq!(manager.current_color(), Color::White);
    }

    #[test]
    fn test_calibration_manager_next_color() {
        let mut manager = CalibrationManager::new();
        assert!(manager.next_color());
        assert_eq!(manager.current_color(), Color::Yellow);
        for _ in 0..4 {
            manager.next_color();
        }
        assert!(!manager.next_color()); // Already at last color
    }

    #[test]
    fn test_calibration_manager_add_sample() {
        let mut manager = CalibrationManager::new();
        let sample = ColorSample::new(RGB::new(240, 240, 240));
        manager.add_sample(sample);
        assert_eq!(manager.current_calibration().sample_count, 1);
    }

    #[test]
    fn test_calibration_manager_is_complete() {
        let mut manager = CalibrationManager::new();
        assert!(!manager.is_complete());

        // Add 10 samples for each color
        for color_idx in 0..6 {
            manager.current_color_index = color_idx;
            for _ in 0..10 {
                manager.add_sample(ColorSample::new(RGB::new(128, 128, 128)));
            }
        }
        assert!(manager.is_complete());
    }

    #[test]
    fn test_calibration_manager_completed_colors() {
        let mut manager = CalibrationManager::new();
        assert_eq!(manager.completed_colors(), 0);

        // Complete first color
        for _ in 0..10 {
            manager.add_sample(ColorSample::new(RGB::new(240, 240, 240)));
        }
        assert_eq!(manager.completed_colors(), 1);
    }

    #[test]
    fn test_to_detection_config() {
        let mut manager = CalibrationManager::new();

        // Add samples for white
        for _ in 0..10 {
            manager.add_sample(ColorSample::new(RGB::new(240, 240, 240)));
        }

        let config = manager.to_detection_config();
        assert!(config.white_min_value > 0.0);
        assert!(config.white_max_saturation < 1.0);
    }

    #[test]
    fn test_serialization() {
        let manager = CalibrationManager::new();
        let json = manager.to_json().unwrap();
        let loaded = CalibrationManager::from_json(&json).unwrap();
        assert_eq!(loaded.calibrations.len(), 6);
        assert_eq!(loaded.current_color_index, 0);
    }

    #[test]
    fn test_reset() {
        let mut manager = CalibrationManager::new();
        manager.add_sample(ColorSample::new(RGB::new(240, 240, 240)));
        manager.next_color();
        manager.reset();
        assert_eq!(manager.current_color_index, 0);
        assert_eq!(manager.current_calibration().sample_count, 0);
    }

    #[test]
    fn test_midpoint_hue() {
        let mut cal = ColorCalibration::new(Color::Red);
        cal.add_sample(ColorSample::new(RGB::new(255, 0, 0))); // H=0
        cal.add_sample(ColorSample::new(RGB::new(200, 0, 0))); // H≈0
        let midpoint = cal.midpoint_hue();
        assert!(midpoint >= 0.0 && midpoint <= 10.0);
    }
}
