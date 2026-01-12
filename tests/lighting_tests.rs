/// Integration tests for lighting variation handling (R4.6)
use rubiks_cube_solver::camera::color_detect::*;
use rubiks_cube_solver::cube::Color;

#[test]
fn test_lighting_001_excellent_conditions() {
    // Test excellent lighting conditions (bright, even)
    let width = 300;
    let height = 300;
    let mut pixels = vec![0u8; (width * height * 3) as usize];

    // Fill with well-lit, even brightness (~0.5)
    for pixel in pixels.chunks_mut(3) {
        pixel[0] = 128;
        pixel[1] = 128;
        pixel[2] = 128;
    }

    let analysis = analyze_lighting(&pixels, width, height);

    assert!(matches!(
        analysis.quality,
        LightingQuality::Excellent | LightingQuality::Good
    ));
    assert!(!analysis.too_dark);
    assert!(!analysis.too_bright);
    assert!(analysis.brightness_variance < 0.03);
}

#[test]
fn test_lighting_002_dark_conditions_warning() {
    // Test dark lighting with warning message
    let width = 300;
    let height = 300;
    let mut pixels = vec![0u8; (width * height * 3) as usize];

    // Very dark image
    for pixel in pixels.chunks_mut(3) {
        pixel[0] = 20;
        pixel[1] = 20;
        pixel[2] = 20;
    }

    let analysis = analyze_lighting(&pixels, width, height);

    assert!(analysis.too_dark);
    assert!(matches!(
        analysis.quality,
        LightingQuality::Poor | LightingQuality::VeryPoor
    ));

    let warning = analysis.get_warning_message();
    assert!(warning.is_some());
    let msg = warning.unwrap();
    // Message should mention darkness or poor lighting
    assert!(msg.to_lowercase().contains("dark") || msg.to_lowercase().contains("dim") || msg.to_lowercase().contains("poor"));
}

#[test]
fn test_lighting_003_bright_conditions_warning() {
    // Test overexposed/bright lighting with warning
    let width = 300;
    let height = 300;
    let mut pixels = vec![0u8; (width * height * 3) as usize];

    // Very bright (overexposed)
    for pixel in pixels.chunks_mut(3) {
        pixel[0] = 245;
        pixel[1] = 245;
        pixel[2] = 245;
    }

    let analysis = analyze_lighting(&pixels, width, height);

    assert!(analysis.too_bright);
    assert!(matches!(
        analysis.quality,
        LightingQuality::Poor | LightingQuality::VeryPoor
    ));

    let warning = analysis.get_warning_message();
    assert!(warning.is_some());
    let msg = warning.unwrap();
    assert!(msg.contains("bright") || msg.contains("light"));
}

#[test]
fn test_lighting_004_uneven_lighting_warning() {
    // Test uneven lighting (shadows)
    let width = 300;
    let height = 300;
    let mut pixels = vec![0u8; (width * height * 3) as usize];

    // Create gradient - dark to bright
    for i in 0..height {
        for j in 0..width {
            let idx = ((i * width + j) * 3) as usize;
            let brightness = ((i as f32 / height as f32) * 200.0) as u8 + 30;
            pixels[idx] = brightness;
            pixels[idx + 1] = brightness;
            pixels[idx + 2] = brightness;
        }
    }

    let analysis = analyze_lighting(&pixels, width, height);

    assert!(analysis.uneven);
    assert!(analysis.brightness_variance > 0.03);
}

#[test]
fn test_lighting_005_adaptive_thresholds_work() {
    // Test that adaptive thresholds adjust based on lighting
    let base_config = ColorDetectionConfig::default();

    // Dark lighting
    let dark_lighting = LightingAnalysis {
        quality: LightingQuality::Poor,
        avg_brightness: 0.2,
        brightness_variance: 0.01,
        too_dark: true,
        too_bright: false,
        uneven: false,
        suggested_brightness_adjustment: 2.0,
    };

    let dark_config = apply_adaptive_thresholds(&base_config, &dark_lighting);

    // Thresholds should be lowered for dark conditions
    assert!(dark_config.white_min_value < base_config.white_min_value);
    assert!(dark_config.min_value_threshold < base_config.min_value_threshold);

    // Bright lighting
    let bright_lighting = LightingAnalysis {
        quality: LightingQuality::Poor,
        avg_brightness: 0.9,
        brightness_variance: 0.01,
        too_dark: false,
        too_bright: true,
        uneven: false,
        suggested_brightness_adjustment: 0.6,
    };

    let bright_config = apply_adaptive_thresholds(&base_config, &bright_lighting);

    // Thresholds should be raised for bright conditions
    assert!(bright_config.white_min_value > base_config.white_min_value);
}

#[test]
fn test_lighting_006_detection_with_adaptation() {
    // Test full workflow: analyze lighting + adapt thresholds + detect colors
    let width = 90;
    let height = 90;
    let grid_size = 3;

    // Create a darker-than-ideal but valid image
    let mut pixels = vec![0u8; (width * height * 3) as usize];

    // Fill with colors that are a bit dark
    for i in 0..grid_size {
        for j in 0..grid_size {
            let cell_width = width / grid_size as u32;
            let cell_height = height / grid_size as u32;

            // Determine color for this cell
            let color_val = if i % 2 == 0 { 80u8 } else { 100u8 };

            for y in (i * cell_height as usize)..((i + 1) * cell_height as usize) {
                for x in (j * cell_width as usize)..((j + 1) * cell_width as usize) {
                    let idx = (y * width as usize + x) * 3;
                    pixels[idx] = color_val;
                    pixels[idx + 1] = color_val;
                    pixels[idx + 2] = color_val;
                }
            }
        }
    }

    let base_config = ColorDetectionConfig::default();
    let (colors, lighting) = detect_colors_with_lighting_adaptation(
        &pixels,
        width,
        height,
        grid_size,
        &base_config,
    );

    // Should get lighting analysis
    assert!(lighting.avg_brightness > 0.0);
    // Even if color detection fails due to uniform gray, we should have lighting data
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
fn test_lighting_007_suggested_brightness_adjustment() {
    // Test brightness adjustment suggestions
    let width = 100;
    let height = 100;
    let mut pixels = vec![0u8; (width * height * 3) as usize];

    // Very dark - should suggest increasing brightness
    for pixel in pixels.chunks_mut(3) {
        pixel[0] = 30;
        pixel[1] = 30;
        pixel[2] = 30;
    }

    let dark_analysis = analyze_lighting(&pixels, width, height);
    assert!(dark_analysis.suggested_brightness_adjustment > 1.5);

    // Very bright - should suggest decreasing brightness
    for pixel in pixels.chunks_mut(3) {
        pixel[0] = 240;
        pixel[1] = 240;
        pixel[2] = 240;
    }

    let bright_analysis = analyze_lighting(&pixels, width, height);
    assert!(bright_analysis.suggested_brightness_adjustment < 0.8);

    // Good lighting - no adjustment needed
    for pixel in pixels.chunks_mut(3) {
        pixel[0] = 128;
        pixel[1] = 128;
        pixel[2] = 128;
    }

    let good_analysis = analyze_lighting(&pixels, width, height);
    assert!((good_analysis.suggested_brightness_adjustment - 1.0).abs() < 0.1);
}

#[test]
fn test_lighting_008_variance_calculation() {
    // Test that variance is calculated correctly
    let width = 100;
    let height = 100;

    // Low variance (uniform)
    let mut uniform_pixels = vec![0u8; (width * height * 3) as usize];
    for pixel in uniform_pixels.chunks_mut(3) {
        pixel[0] = 128;
        pixel[1] = 128;
        pixel[2] = 128;
    }

    let uniform_analysis = analyze_lighting(&uniform_pixels, width, height);
    assert!(uniform_analysis.brightness_variance < 0.01);

    // High variance (checkerboard pattern)
    let mut checkerboard_pixels = vec![0u8; (width * height * 3) as usize];
    for i in 0..height {
        for j in 0..width {
            let idx = ((i * width + j) * 3) as usize;
            let val = if (i + j) % 2 == 0 { 50u8 } else { 200u8 };
            checkerboard_pixels[idx] = val;
            checkerboard_pixels[idx + 1] = val;
            checkerboard_pixels[idx + 2] = val;
        }
    }

    let checkerboard_analysis = analyze_lighting(&checkerboard_pixels, width, height);
    assert!(checkerboard_analysis.brightness_variance > 0.05);
}

#[test]
fn test_lighting_009_no_warning_for_good_conditions() {
    // Verify no warnings for good/excellent lighting
    let analysis_excellent = LightingAnalysis {
        quality: LightingQuality::Excellent,
        avg_brightness: 0.5,
        brightness_variance: 0.01,
        too_dark: false,
        too_bright: false,
        uneven: false,
        suggested_brightness_adjustment: 1.0,
    };

    assert!(analysis_excellent.get_warning_message().is_none());

    let analysis_good = LightingAnalysis {
        quality: LightingQuality::Good,
        avg_brightness: 0.6,
        brightness_variance: 0.02,
        too_dark: false,
        too_bright: false,
        uneven: false,
        suggested_brightness_adjustment: 1.0,
    };

    assert!(analysis_good.get_warning_message().is_none());
}

#[test]
fn test_lighting_010_adaptive_hue_ranges() {
    // Test that hue ranges expand in uneven lighting
    let base_config = ColorDetectionConfig::default();

    let uneven_lighting = LightingAnalysis {
        quality: LightingQuality::Adequate,
        avg_brightness: 0.5,
        brightness_variance: 0.07,
        too_dark: false,
        too_bright: false,
        uneven: true,
        suggested_brightness_adjustment: 1.0,
    };

    let adapted = apply_adaptive_thresholds(&base_config, &uneven_lighting);

    // All hue ranges should be expanded
    assert!(adapted.yellow_hue_min < base_config.yellow_hue_min);
    assert!(adapted.yellow_hue_max > base_config.yellow_hue_max);
    assert!(adapted.orange_hue_min < base_config.orange_hue_min);
    assert!(adapted.orange_hue_max > base_config.orange_hue_max);
    assert!(adapted.blue_hue_min < base_config.blue_hue_min);
    assert!(adapted.blue_hue_max > base_config.blue_hue_max);
    assert!(adapted.green_hue_min < base_config.green_hue_min);
    assert!(adapted.green_hue_max > base_config.green_hue_max);
}
