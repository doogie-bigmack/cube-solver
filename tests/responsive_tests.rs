//! Integration tests for responsive sizing (R2.8)
//!
//! These tests verify that the responsive sizing system works correctly
//! across all target screen sizes.

use rubiks_cube_solver::components::{ResponsiveConfig, ResponsiveDimensions};

// Test coverage for resp_001: 320px width (small phone)
#[test]
fn resp_001_small_phone_320px() {
    let config = ResponsiveConfig::default();
    let dims = ResponsiveDimensions::from_viewport(320.0, 568.0, &config);

    // Should use minimum width
    assert_eq!(dims.width, 320.0);
    // Height should match width for 1:1 aspect ratio
    assert_eq!(dims.height, 320.0);
    // Cube should have 10% padding on each side
    let expected_cube_size = 320.0 * 0.8;
    assert_eq!(dims.cube_width, expected_cube_size);
    assert_eq!(dims.cube_height, expected_cube_size);
}

// Test coverage for resp_002: 375px width (iPhone SE)
#[test]
fn resp_002_iphone_se_375px() {
    let config = ResponsiveConfig::default();
    let dims = ResponsiveDimensions::from_viewport(375.0, 667.0, &config);

    assert_eq!(dims.width, 375.0);
    assert_eq!(dims.height, 375.0);
    // Verify no horizontal overflow
    assert!(dims.width <= 375.0);
}

// Test coverage for resp_003: 390px width (iPhone 14)
#[test]
fn resp_003_iphone_14_390px() {
    let config = ResponsiveConfig::default();
    let dims = ResponsiveDimensions::from_viewport(390.0, 844.0, &config);

    assert_eq!(dims.width, 390.0);
    assert_eq!(dims.height, 390.0);
    // Verify readable at this size (cube is at least 300px after padding)
    assert!(dims.cube_width >= 300.0);
}

// Test coverage for resp_004: 768px width (iPad portrait)
#[test]
fn resp_004_ipad_portrait_768px() {
    let config = ResponsiveConfig::default();
    let dims = ResponsiveDimensions::from_viewport(768.0, 1024.0, &config);

    assert_eq!(dims.width, 768.0);
    assert_eq!(dims.height, 768.0);
    // Verify aspect ratio maintained
    let aspect_ratio = dims.cube_width / dims.cube_height;
    assert!((aspect_ratio - 1.0).abs() < 0.01);
}

// Test coverage for resp_005: 1024px width (iPad landscape / small desktop)
#[test]
fn resp_005_ipad_landscape_1024px() {
    let config = ResponsiveConfig::default();
    let dims = ResponsiveDimensions::from_viewport(1024.0, 768.0, &config);

    // Height is constraining (768 < 1024), so both should be 768
    assert_eq!(dims.width, 768.0);
    assert_eq!(dims.height, 768.0);
}

// Test coverage for resp_006: 1440px width (standard desktop)
#[test]
fn resp_006_desktop_standard_1440px() {
    let config = ResponsiveConfig::default();
    let dims = ResponsiveDimensions::from_viewport(1440.0, 900.0, &config);

    // Height is constraining (900 < 1440), so both should be 900
    assert_eq!(dims.width, 900.0);
    assert_eq!(dims.height, 900.0);
}

// Test coverage for resp_007: 1920px width (large desktop)
#[test]
fn resp_007_desktop_large_1920px() {
    let config = ResponsiveConfig::default();
    let dims = ResponsiveDimensions::from_viewport(1920.0, 1080.0, &config);

    // Height is constraining (1080 < 1920), so both should be 1080
    assert_eq!(dims.width, 1080.0);
    assert_eq!(dims.height, 1080.0);
}

// Test coverage for resp_008: 2560px width (4K monitor)
#[test]
fn resp_008_4k_monitor_2560px() {
    let config = ResponsiveConfig::default();
    let dims = ResponsiveDimensions::from_viewport(2560.0, 1440.0, &config);

    // Height is constraining (1440 < 1920 max_width), so both should be 1440
    assert_eq!(dims.width, 1440.0);
    assert_eq!(dims.height, 1440.0);
}

// Test coverage for resp_009: Landscape phone
#[test]
fn resp_009_landscape_phone() {
    let config = ResponsiveConfig::default();
    // iPhone in landscape: 844x390
    let dims = ResponsiveDimensions::from_viewport(844.0, 390.0, &config);

    // Height becomes limiting factor
    assert_eq!(dims.height, 390.0);
    assert_eq!(dims.width, 390.0); // Maintains aspect ratio
}

// Test coverage for resp_010: Portrait tablet
#[test]
fn resp_010_portrait_tablet() {
    let config = ResponsiveConfig::default();
    // iPad Mini portrait: 768x1024
    let dims = ResponsiveDimensions::from_viewport(768.0, 1024.0, &config);

    assert_eq!(dims.width, 768.0);
    assert_eq!(dims.height, 768.0);
}

// Additional comprehensive tests

#[test]
fn test_no_horizontal_scrolling_all_sizes() {
    let config = ResponsiveConfig::default();

    let test_cases = vec![
        (320.0, 568.0),   // Small phone
        (375.0, 667.0),   // iPhone SE
        (390.0, 844.0),   // iPhone 14
        (414.0, 896.0),   // iPhone 14 Pro Max
        (768.0, 1024.0),  // iPad
        (1024.0, 768.0),  // iPad landscape
        (1440.0, 900.0),  // Desktop
        (1920.0, 1080.0), // Full HD
    ];

    for (viewport_width, _viewport_height) in test_cases {
        let dims = ResponsiveDimensions::from_viewport(viewport_width, _viewport_height, &config);
        // Container width should never exceed viewport width
        assert!(
            dims.width <= viewport_width.min(config.max_width),
            "Container width {} exceeds viewport {} for size {}x{}",
            dims.width,
            viewport_width,
            viewport_width,
            _viewport_height
        );
    }
}

#[test]
fn test_readable_text_all_sizes() {
    let config = ResponsiveConfig::default();

    let test_cases = vec![
        (320.0, 568.0),   // Small phone
        (375.0, 667.0),   // iPhone SE
        (1920.0, 1080.0), // Full HD
    ];

    for (width, height) in test_cases {
        let dims = ResponsiveDimensions::from_viewport(width, height, &config);
        // Cube should be large enough to be usable (at least 250px)
        assert!(
            dims.cube_width >= 250.0,
            "Cube too small ({}) for viewport {}x{}",
            dims.cube_width,
            width,
            height
        );
    }
}

#[test]
fn test_aspect_ratio_maintained_all_configs() {
    let configs = vec![
        ResponsiveConfig::default(),
        ResponsiveConfig::mobile(),
        ResponsiveConfig::tablet(),
        ResponsiveConfig::desktop(),
    ];

    for config in configs {
        let dims = ResponsiveDimensions::from_viewport(800.0, 600.0, &config);
        let aspect_ratio = dims.cube_width / dims.cube_height;
        assert!(
            (aspect_ratio - config.aspect_ratio).abs() < 0.01,
            "Aspect ratio not maintained: expected {}, got {}",
            config.aspect_ratio,
            aspect_ratio
        );
    }
}

#[test]
fn test_mobile_config_optimizations() {
    let mobile = ResponsiveConfig::mobile();

    // Mobile should have smaller max width
    assert_eq!(mobile.max_width, 768.0);
    // Mobile should have less padding for more space
    assert_eq!(mobile.padding_percent, 0.05);

    let dims = ResponsiveDimensions::from_viewport(375.0, 667.0, &mobile);
    // Should use most of available space (95%)
    let expected_size = 375.0 * 0.9; // 5% padding on each side
    assert_eq!(dims.cube_width, expected_size);
}

#[test]
fn test_tablet_config_optimizations() {
    let tablet = ResponsiveConfig::tablet();

    assert_eq!(tablet.min_width, 768.0);
    assert_eq!(tablet.max_width, 1024.0);
    assert_eq!(tablet.padding_percent, 0.08);
}

#[test]
fn test_desktop_config_optimizations() {
    let desktop = ResponsiveConfig::desktop();

    assert_eq!(desktop.min_width, 1024.0);
    assert_eq!(desktop.max_width, 1920.0);
    assert_eq!(desktop.padding_percent, 0.1);
}

#[test]
fn test_extreme_aspect_ratios() {
    let config = ResponsiveConfig::default();

    // Very wide screen
    let dims_wide = ResponsiveDimensions::from_viewport(2000.0, 400.0, &config);
    assert_eq!(dims_wide.height, 400.0); // Height constrained
    assert_eq!(dims_wide.width, 400.0); // Width matches height for 1:1 aspect

    // Very tall screen
    let dims_tall = ResponsiveDimensions::from_viewport(400.0, 2000.0, &config);
    assert_eq!(dims_tall.width, 400.0); // Width constrained
    assert_eq!(dims_tall.height, 400.0); // Height matches width for 1:1 aspect
}

#[test]
fn test_style_strings_valid() {
    let dims = ResponsiveDimensions::from_viewport(800.0, 600.0, &ResponsiveConfig::default());

    let container_style = dims.container_style();
    assert!(container_style.contains("px"));
    assert!(container_style.contains("position"));

    let cube_style = dims.cube_style();
    assert!(cube_style.contains("px"));
    assert!(cube_style.contains("transform"));
}

#[test]
fn test_centering_calculation() {
    let dims = ResponsiveDimensions::from_viewport(1000.0, 1000.0, &ResponsiveConfig::default());

    // Cube should be centered using transform translate
    let style = dims.cube_style();
    assert!(style.contains("left: 50%"));
    assert!(style.contains("top: 50%"));
    assert!(style.contains("translate(-50%, -50%)"));
}

#[test]
fn test_various_screen_sizes() {
    let config = ResponsiveConfig::default();

    // Test common device resolutions
    let devices = vec![
        ("iPhone SE", 320.0, 568.0),
        ("iPhone 12", 390.0, 844.0),
        ("iPhone 14 Pro Max", 430.0, 932.0),
        ("iPad Mini", 768.0, 1024.0),
        ("iPad Pro 11", 834.0, 1194.0),
        ("iPad Pro 12.9", 1024.0, 1366.0),
        ("MacBook Air", 1440.0, 900.0),
        ("MacBook Pro 14", 1512.0, 982.0),
        ("MacBook Pro 16", 1728.0, 1117.0),
        ("Desktop FHD", 1920.0, 1080.0),
        ("Desktop QHD", 2560.0, 1440.0),
    ];

    for (device, width, height) in devices {
        let dims = ResponsiveDimensions::from_viewport(width, height, &config);

        // All dimensions should be positive
        assert!(dims.width > 0.0, "{}: width not positive", device);
        assert!(dims.height > 0.0, "{}: height not positive", device);
        assert!(dims.cube_width > 0.0, "{}: cube_width not positive", device);
        assert!(dims.cube_height > 0.0, "{}: cube_height not positive", device);

        // Cube dimensions should be smaller than container
        assert!(dims.cube_width < dims.width, "{}: cube wider than container", device);
        assert!(dims.cube_height < dims.height, "{}: cube taller than container", device);

        // Container should not exceed max bounds
        assert!(dims.width <= config.max_width, "{}: exceeds max width", device);
    }
}
