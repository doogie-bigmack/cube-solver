//! 3D Cube Component with Responsive Sizing
//!
//! This module provides a responsive 3D cube display component that adapts
//! to different screen sizes while maintaining proper aspect ratio.

use dioxus::prelude::*;

/// Configuration for responsive sizing behavior
#[derive(Clone, Debug, PartialEq)]
pub struct ResponsiveConfig {
    /// Minimum width in pixels (for small phones)
    pub min_width: f32,
    /// Maximum width in pixels (for large desktops)
    pub max_width: f32,
    /// Target aspect ratio (width / height)
    pub aspect_ratio: f32,
    /// Padding as percentage of container size
    pub padding_percent: f32,
}

impl Default for ResponsiveConfig {
    fn default() -> Self {
        Self {
            min_width: 320.0,
            max_width: 1920.0,
            aspect_ratio: 1.0, // Square for cube
            padding_percent: 0.1, // 10% padding
        }
    }
}

impl ResponsiveConfig {
    /// Create a config for mobile screens (320px - 768px)
    pub fn mobile() -> Self {
        Self {
            min_width: 320.0,
            max_width: 768.0,
            aspect_ratio: 1.0,
            padding_percent: 0.05,
        }
    }

    /// Create a config for tablet screens (768px - 1024px)
    pub fn tablet() -> Self {
        Self {
            min_width: 768.0,
            max_width: 1024.0,
            aspect_ratio: 1.0,
            padding_percent: 0.08,
        }
    }

    /// Create a config for desktop screens (1024px+)
    pub fn desktop() -> Self {
        Self {
            min_width: 1024.0,
            max_width: 1920.0,
            aspect_ratio: 1.0,
            padding_percent: 0.1,
        }
    }
}

/// Calculate responsive dimensions based on viewport size
#[derive(Clone, Debug, PartialEq)]
pub struct ResponsiveDimensions {
    /// Container width in pixels
    pub width: f32,
    /// Container height in pixels
    pub height: f32,
    /// Actual cube display width in pixels (after padding)
    pub cube_width: f32,
    /// Actual cube display height in pixels (after padding)
    pub cube_height: f32,
}

impl ResponsiveDimensions {
    /// Calculate dimensions from viewport size and config
    pub fn from_viewport(viewport_width: f32, viewport_height: f32, config: &ResponsiveConfig) -> Self {
        // Clamp viewport width to min/max bounds
        let clamped_width = viewport_width.max(config.min_width).min(config.max_width);

        // Calculate container dimensions maintaining aspect ratio
        let (width, height) = if viewport_height < clamped_width / config.aspect_ratio {
            // Height-constrained
            let height = viewport_height;
            let width = (height * config.aspect_ratio).min(clamped_width);
            (width, height)
        } else {
            // Width-constrained
            let width = clamped_width;
            let height = width / config.aspect_ratio;
            (width, height)
        };

        // Apply padding
        let padding = width * config.padding_percent;
        let cube_width = width - 2.0 * padding;
        let cube_height = height - 2.0 * padding;

        Self {
            width,
            height,
            cube_width,
            cube_height,
        }
    }

    /// Get CSS string for container dimensions
    pub fn container_style(&self) -> String {
        format!("width: {}px; height: {}px; position: relative;", self.width, self.height)
    }

    /// Get CSS string for cube dimensions
    pub fn cube_style(&self) -> String {
        format!(
            "width: {}px; height: {}px; position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%);",
            self.cube_width, self.cube_height
        )
    }
}

/// Props for the Cube3D component
#[derive(Props, Clone, PartialEq)]
pub struct Cube3DProps {
    /// Viewport width (from window resize events)
    #[props(default = 800.0)]
    pub viewport_width: f32,

    /// Viewport height (from window resize events)
    #[props(default = 600.0)]
    pub viewport_height: f32,

    /// Responsive configuration
    #[props(default)]
    pub config: ResponsiveConfig,
}

/// Responsive 3D Cube Display Component
///
/// This component automatically sizes the cube display based on the viewport
/// dimensions, ensuring it works well on all screen sizes from mobile to desktop.
///
/// # Acceptance Criteria
/// - Cube fills available space appropriately
/// - Maintains aspect ratio
/// - Works on mobile screens (320px+)
/// - Works on large desktop screens (1920px)
#[component]
pub fn Cube3D(props: Cube3DProps) -> Element {
    // Calculate responsive dimensions
    let dimensions = ResponsiveDimensions::from_viewport(
        props.viewport_width,
        props.viewport_height,
        &props.config,
    );

    rsx! {
        div {
            class: "cube-3d-container",
            style: "{dimensions.container_style()}",
            div {
                class: "cube-3d-viewport",
                style: "{dimensions.cube_style()}",
                // Placeholder for actual 3D rendering
                // This will be replaced with WGPU canvas integration
                div {
                    style: "width: 100%; height: 100%; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 10px; display: flex; align-items: center; justify-content: center; color: white; font-size: 1.2rem;",
                    "3D Cube Render Area"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_responsive_config_default() {
        let config = ResponsiveConfig::default();
        assert_eq!(config.min_width, 320.0);
        assert_eq!(config.max_width, 1920.0);
        assert_eq!(config.aspect_ratio, 1.0);
        assert_eq!(config.padding_percent, 0.1);
    }

    #[test]
    fn test_responsive_config_mobile() {
        let config = ResponsiveConfig::mobile();
        assert_eq!(config.min_width, 320.0);
        assert_eq!(config.max_width, 768.0);
    }

    #[test]
    fn test_responsive_config_tablet() {
        let config = ResponsiveConfig::tablet();
        assert_eq!(config.min_width, 768.0);
        assert_eq!(config.max_width, 1024.0);
    }

    #[test]
    fn test_responsive_config_desktop() {
        let config = ResponsiveConfig::desktop();
        assert_eq!(config.min_width, 1024.0);
        assert_eq!(config.max_width, 1920.0);
    }

    #[test]
    fn test_responsive_dimensions_small_phone() {
        let config = ResponsiveConfig::default();
        let dims = ResponsiveDimensions::from_viewport(320.0, 568.0, &config);

        assert_eq!(dims.width, 320.0);
        assert_eq!(dims.height, 320.0);
        assert_eq!(dims.cube_width, 320.0 * 0.8); // 10% padding on each side
        assert_eq!(dims.cube_height, 320.0 * 0.8);
    }

    #[test]
    fn test_responsive_dimensions_iphone_se() {
        let config = ResponsiveConfig::default();
        let dims = ResponsiveDimensions::from_viewport(375.0, 667.0, &config);

        assert_eq!(dims.width, 375.0);
        assert_eq!(dims.height, 375.0);
    }

    #[test]
    fn test_responsive_dimensions_iphone_14() {
        let config = ResponsiveConfig::default();
        let dims = ResponsiveDimensions::from_viewport(390.0, 844.0, &config);

        assert_eq!(dims.width, 390.0);
        assert_eq!(dims.height, 390.0);
    }

    #[test]
    fn test_responsive_dimensions_ipad_portrait() {
        let config = ResponsiveConfig::default();
        let dims = ResponsiveDimensions::from_viewport(768.0, 1024.0, &config);

        assert_eq!(dims.width, 768.0);
        assert_eq!(dims.height, 768.0);
    }

    #[test]
    fn test_responsive_dimensions_ipad_landscape() {
        let config = ResponsiveConfig::default();
        let dims = ResponsiveDimensions::from_viewport(1024.0, 768.0, &config);

        // Height is constraining (768 < 1024), so both should be 768
        assert_eq!(dims.width, 768.0);
        assert_eq!(dims.height, 768.0);
    }

    #[test]
    fn test_responsive_dimensions_desktop_standard() {
        let config = ResponsiveConfig::default();
        let dims = ResponsiveDimensions::from_viewport(1440.0, 900.0, &config);

        // Height is constraining (900 < 1440), so both should be 900
        assert_eq!(dims.width, 900.0);
        assert_eq!(dims.height, 900.0);
    }

    #[test]
    fn test_responsive_dimensions_desktop_large() {
        let config = ResponsiveConfig::default();
        let dims = ResponsiveDimensions::from_viewport(1920.0, 1080.0, &config);

        // Height is constraining (1080 < 1920), so both should be 1080
        assert_eq!(dims.width, 1080.0);
        assert_eq!(dims.height, 1080.0);
    }

    #[test]
    fn test_responsive_dimensions_4k() {
        let config = ResponsiveConfig::default();
        let dims = ResponsiveDimensions::from_viewport(2560.0, 1440.0, &config);

        // Height is constraining (1440 < 1920 max_width), so both should be 1440
        assert_eq!(dims.width, 1440.0);
        assert_eq!(dims.height, 1440.0);
    }

    #[test]
    fn test_responsive_dimensions_too_small() {
        let config = ResponsiveConfig::default();
        let dims = ResponsiveDimensions::from_viewport(280.0, 500.0, &config);

        // Should clamp to min_width
        assert_eq!(dims.width, 320.0);
    }

    #[test]
    fn test_responsive_dimensions_height_constrained() {
        let config = ResponsiveConfig::default();
        let dims = ResponsiveDimensions::from_viewport(1920.0, 400.0, &config);

        // Height is limiting factor
        assert_eq!(dims.height, 400.0);
        assert_eq!(dims.width, 400.0); // Maintains aspect ratio
    }

    #[test]
    fn test_responsive_dimensions_padding_calculation() {
        let config = ResponsiveConfig::default();
        let dims = ResponsiveDimensions::from_viewport(1000.0, 1000.0, &config);

        let expected_padding = 1000.0 * 0.1;
        assert_eq!(dims.cube_width, 1000.0 - 2.0 * expected_padding);
        assert_eq!(dims.cube_height, 1000.0 - 2.0 * expected_padding);
    }

    #[test]
    fn test_responsive_dimensions_mobile_padding() {
        let config = ResponsiveConfig::mobile();
        let dims = ResponsiveDimensions::from_viewport(375.0, 667.0, &config);

        let expected_padding = 375.0 * 0.05; // 5% for mobile
        assert_eq!(dims.cube_width, 375.0 - 2.0 * expected_padding);
    }

    #[test]
    fn test_container_style_format() {
        let dims = ResponsiveDimensions {
            width: 800.0,
            height: 600.0,
            cube_width: 720.0,
            cube_height: 540.0,
        };

        let style = dims.container_style();
        assert!(style.contains("width: 800px"));
        assert!(style.contains("height: 600px"));
        assert!(style.contains("position: relative"));
    }

    #[test]
    fn test_cube_style_format() {
        let dims = ResponsiveDimensions {
            width: 800.0,
            height: 600.0,
            cube_width: 720.0,
            cube_height: 540.0,
        };

        let style = dims.cube_style();
        assert!(style.contains("width: 720px"));
        assert!(style.contains("height: 540px"));
        assert!(style.contains("position: absolute"));
        assert!(style.contains("transform: translate(-50%, -50%)"));
    }

    #[test]
    fn test_aspect_ratio_maintained() {
        let config = ResponsiveConfig::default();

        // Test various viewport sizes
        let test_cases = vec![
            (320.0, 568.0),
            (375.0, 667.0),
            (768.0, 1024.0),
            (1440.0, 900.0),
            (1920.0, 1080.0),
        ];

        for (width, height) in test_cases {
            let dims = ResponsiveDimensions::from_viewport(width, height, &config);
            // Cube dimensions should maintain 1:1 aspect ratio
            let aspect = dims.cube_width / dims.cube_height;
            assert!((aspect - 1.0).abs() < 0.01, "Aspect ratio not maintained for {}x{}", width, height);
        }
    }
}
