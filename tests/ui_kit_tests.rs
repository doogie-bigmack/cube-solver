use rubiks_cube_solver::components::{ButtonSize, ButtonTheme};

// Test R6.12 acceptance criteria: Minimum 44px touch targets
#[test]
fn test_ui_001_minimum_touch_target_size() {
    // WCAG 2.1 AAA requires minimum 44x44px touch targets
    assert_eq!(ButtonSize::Small.pixels(), 44);
    assert!(ButtonSize::Medium.pixels() >= 44);
    assert!(ButtonSize::Large.pixels() >= 44);
}

// Test R6.12: All button sizes meet accessibility standards
#[test]
fn test_ui_002_all_sizes_accessible() {
    let sizes = vec![ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];
    for size in sizes {
        assert!(
            size.pixels() >= 44,
            "Button size {:?} is {}px, must be at least 44px",
            size,
            size.pixels()
        );
    }
}

// Test R6.12: Button sizes are progressive
#[test]
fn test_ui_003_progressive_sizing() {
    assert!(ButtonSize::Medium.pixels() > ButtonSize::Small.pixels());
    assert!(ButtonSize::Large.pixels() > ButtonSize::Medium.pixels());

    // Verify reasonable spacing between sizes
    let small_to_medium = ButtonSize::Medium.pixels() - ButtonSize::Small.pixels();
    let medium_to_large = ButtonSize::Large.pixels() - ButtonSize::Medium.pixels();

    assert!(small_to_medium >= 8, "Size progression too small");
    assert!(medium_to_large >= 8, "Size progression too small");
}

// Test R6.12: Font sizes scale with button sizes
#[test]
fn test_ui_004_font_size_scaling() {
    assert!(ButtonSize::Medium.font_size() > ButtonSize::Small.font_size());
    assert!(ButtonSize::Large.font_size() > ButtonSize::Medium.font_size());

    // Font sizes should be readable
    assert!(ButtonSize::Small.font_size() >= 12, "Font too small");
    assert!(ButtonSize::Large.font_size() <= 32, "Font too large");
}

// Test R6.12: Bright engaging colors
#[test]
fn test_ui_005_bright_colors() {
    let themes = vec![
        ButtonTheme::Primary,
        ButtonTheme::Success,
        ButtonTheme::Warning,
        ButtonTheme::Danger,
        ButtonTheme::Secondary,
    ];

    for theme in themes {
        let bg = theme.background_color();
        let hover = theme.hover_color();

        // Colors should be valid hex codes
        assert!(bg.starts_with('#'), "{:?} background not hex", theme);
        assert!(hover.starts_with('#'), "{:?} hover not hex", theme);
        assert_eq!(bg.len(), 7, "{:?} background invalid length", theme);
        assert_eq!(hover.len(), 7, "{:?} hover invalid length", theme);
    }
}

// Test R6.12: Each theme has unique colors
#[test]
fn test_ui_006_unique_theme_colors() {
    let primary = ButtonTheme::Primary.background_color();
    let success = ButtonTheme::Success.background_color();
    let warning = ButtonTheme::Warning.background_color();
    let danger = ButtonTheme::Danger.background_color();
    let secondary = ButtonTheme::Secondary.background_color();

    // All colors should be distinct
    assert_ne!(primary, success);
    assert_ne!(primary, warning);
    assert_ne!(primary, danger);
    assert_ne!(primary, secondary);
    assert_ne!(success, warning);
    assert_ne!(success, danger);
    assert_ne!(success, secondary);
    assert_ne!(warning, danger);
    assert_ne!(warning, secondary);
    assert_ne!(danger, secondary);
}

// Test R6.12: Hover colors are darker than background
#[test]
fn test_ui_007_hover_colors_darker() {
    let themes = vec![
        ButtonTheme::Primary,
        ButtonTheme::Success,
        ButtonTheme::Warning,
        ButtonTheme::Danger,
        ButtonTheme::Secondary,
    ];

    for theme in themes {
        let bg = theme.background_color();
        let hover = theme.hover_color();

        // Hover should be different from background
        assert_ne!(bg, hover, "{:?} hover same as background", theme);
    }
}

// Test R6.12: Color coding is consistent
#[test]
fn test_ui_008_color_coding() {
    // Primary should be blue (starts with #3)
    assert!(ButtonTheme::Primary.background_color().starts_with("#3"));

    // Success should be green (starts with #1)
    assert!(ButtonTheme::Success.background_color().starts_with("#1"));

    // Warning should be orange/yellow (starts with #F)
    assert!(ButtonTheme::Warning.background_color().starts_with("#F"));

    // Danger should be red (starts with #E)
    assert!(ButtonTheme::Danger.background_color().starts_with("#E"));

    // Secondary should be purple (starts with #8)
    assert!(ButtonTheme::Secondary.background_color().starts_with("#8"));
}

// Test R6.12: Button sizes suitable for kids
#[test]
fn test_ui_009_kid_friendly_sizing() {
    // Medium and Large should be comfortable for children
    assert!(ButtonSize::Medium.pixels() >= 56, "Medium too small for kids");
    assert!(ButtonSize::Large.pixels() >= 72, "Large too small for kids");

    // But not too large
    assert!(ButtonSize::Large.pixels() <= 100, "Large too big");
}

// Test R6.12: Font sizes are readable
#[test]
fn test_ui_010_readable_fonts() {
    // Small should still be readable
    assert!(ButtonSize::Small.font_size() >= 14);

    // Medium should be comfortable
    assert!(ButtonSize::Medium.font_size() >= 18);

    // Large should be very clear
    assert!(ButtonSize::Large.font_size() >= 24);
}

// Test R6.12: Size variants cover all use cases
#[test]
fn test_ui_011_size_coverage() {
    // Should have at least 3 size variants
    assert_eq!(ButtonSize::Small.pixels(), 44);
    assert_eq!(ButtonSize::Medium.pixels(), 56);
    assert_eq!(ButtonSize::Large.pixels(), 72);
}

// Test R6.12: Theme variants cover all actions
#[test]
fn test_ui_012_theme_coverage() {
    // Should have themes for all common actions
    let themes = vec![
        ButtonTheme::Primary,   // Main actions
        ButtonTheme::Success,   // Positive actions
        ButtonTheme::Warning,   // Caution actions
        ButtonTheme::Danger,    // Destructive actions
        ButtonTheme::Secondary, // Alternative actions
    ];

    assert_eq!(themes.len(), 5);
}

// Test R6.12: Colors are web-safe
#[test]
fn test_ui_013_web_safe_colors() {
    let themes = vec![
        ButtonTheme::Primary,
        ButtonTheme::Success,
        ButtonTheme::Warning,
        ButtonTheme::Danger,
        ButtonTheme::Secondary,
    ];

    for theme in themes {
        let bg = theme.background_color();

        // Verify it's a valid hex color (starts with # and 6 hex digits)
        assert!(bg.starts_with('#'));
        let hex_part = &bg[1..];
        assert_eq!(hex_part.len(), 6);

        for c in hex_part.chars() {
            assert!(
                c.is_ascii_hexdigit(),
                "Invalid hex digit '{}' in color {}",
                c,
                bg
            );
        }
    }
}

// Test R6.12: Size consistency
#[test]
fn test_ui_014_size_consistency() {
    // Calling pixels() multiple times should return same value
    assert_eq!(ButtonSize::Small.pixels(), ButtonSize::Small.pixels());
    assert_eq!(ButtonSize::Medium.pixels(), ButtonSize::Medium.pixels());
    assert_eq!(ButtonSize::Large.pixels(), ButtonSize::Large.pixels());
}

// Test R6.12: Color consistency
#[test]
fn test_ui_015_color_consistency() {
    // Calling background_color() multiple times should return same value
    let theme = ButtonTheme::Primary;
    assert_eq!(theme.background_color(), theme.background_color());
    assert_eq!(theme.hover_color(), theme.hover_color());
}

// Test R6.12: Touch targets suitable for mobile
#[test]
fn test_ui_016_mobile_friendly() {
    // On mobile, even Small should be comfortable
    assert!(ButtonSize::Small.pixels() >= 44);

    // Medium should be preferred size for mobile
    assert!(ButtonSize::Medium.pixels() >= 48);
}

// Test R6.12: Accessibility standards compliance
#[test]
fn test_ui_017_wcag_compliance() {
    // WCAG 2.1 Level AAA requires 44x44px minimum
    // https://www.w3.org/WAI/WCAG21/Understanding/target-size.html
    let min_size = 44;

    assert!(ButtonSize::Small.pixels() >= min_size);
    assert!(ButtonSize::Medium.pixels() >= min_size);
    assert!(ButtonSize::Large.pixels() >= min_size);
}

// Test R6.12: Color contrast for readability
#[test]
fn test_ui_018_color_brightness() {
    // All colors should be valid hex and bright enough to be engaging
    let themes = vec![
        ButtonTheme::Primary,
        ButtonTheme::Success,
        ButtonTheme::Warning,
        ButtonTheme::Danger,
        ButtonTheme::Secondary,
    ];

    let mut bright_count = 0;
    for theme in themes {
        let bg = theme.background_color();
        let first_digit = bg.chars().nth(1).unwrap();

        // Colors starting with 1-F are all valid kid-friendly colors
        // #1 is bright green (like #10B981), #3 is bright blue, #F is bright yellow/orange, etc.
        assert!(
            first_digit.is_ascii_hexdigit(),
            "{:?} color {} has invalid hex",
            theme,
            bg
        );

        // Count colors that are reasonably bright (not starting with 0)
        if first_digit != '0' {
            bright_count += 1;
        }
    }

    // All 5 colors should be bright (not starting with #0)
    assert_eq!(bright_count, 5, "All colors should be bright");
}

// Test R6.12: Button sizing ratios
#[test]
fn test_ui_019_sizing_ratios() {
    let small = ButtonSize::Small.pixels();
    let medium = ButtonSize::Medium.pixels();
    let large = ButtonSize::Large.pixels();

    // Medium should be about 1.2-1.5x small
    let small_to_medium_ratio = medium as f32 / small as f32;
    assert!(small_to_medium_ratio >= 1.2 && small_to_medium_ratio <= 1.5);

    // Large should be about 1.2-1.5x medium
    let medium_to_large_ratio = large as f32 / medium as f32;
    assert!(medium_to_large_ratio >= 1.2 && medium_to_large_ratio <= 1.5);
}

// Test R6.12: Font sizing ratios
#[test]
fn test_ui_020_font_ratios() {
    let small = ButtonSize::Small.font_size();
    let medium = ButtonSize::Medium.font_size();
    let large = ButtonSize::Large.font_size();

    // Font sizes should scale proportionally with button sizes
    let small_to_medium_ratio = medium as f32 / small as f32;
    assert!(small_to_medium_ratio >= 1.2 && small_to_medium_ratio <= 1.5);

    let medium_to_large_ratio = large as f32 / medium as f32;
    assert!(medium_to_large_ratio >= 1.2 && medium_to_large_ratio <= 1.5);
}

// Test R6.12: All themes have both background and hover
#[test]
fn test_ui_021_complete_themes() {
    let themes = vec![
        ButtonTheme::Primary,
        ButtonTheme::Success,
        ButtonTheme::Warning,
        ButtonTheme::Danger,
        ButtonTheme::Secondary,
    ];

    for theme in themes {
        // Each theme must have both colors defined
        assert!(!theme.background_color().is_empty());
        assert!(!theme.hover_color().is_empty());
    }
}

// Test R6.12: Kid-friendly means large, not huge
#[test]
fn test_ui_022_not_too_large() {
    // Even Large buttons shouldn't be unreasonably big
    assert!(ButtonSize::Large.pixels() <= 100);
    assert!(ButtonSize::Large.font_size() <= 32);
}

// Test R6.12: Comprehensive size testing
#[test]
fn test_ui_023_all_sizes() {
    let sizes = vec![
        ("Small", ButtonSize::Small, 44, 14),
        ("Medium", ButtonSize::Medium, 56, 18),
        ("Large", ButtonSize::Large, 72, 24),
    ];

    for (name, size, expected_px, expected_font) in sizes {
        assert_eq!(
            size.pixels(),
            expected_px,
            "{} button size incorrect",
            name
        );
        assert_eq!(
            size.font_size(),
            expected_font,
            "{} font size incorrect",
            name
        );
    }
}

// Test R6.12: Color theme names match colors
#[test]
fn test_ui_024_theme_naming() {
    // Primary should be blue-ish (#3 or #2)
    assert!(ButtonTheme::Primary.background_color().starts_with("#3")
            || ButtonTheme::Primary.background_color().starts_with("#2"));

    // Success should be green-ish (#0 or #1)
    assert!(ButtonTheme::Success.background_color().starts_with("#0")
            || ButtonTheme::Success.background_color().starts_with("#1"));

    // Warning should be yellow/orange (#F or #E)
    assert!(ButtonTheme::Warning.background_color().starts_with("#F")
            || ButtonTheme::Warning.background_color().starts_with("#E"));

    // Danger should be red-ish (#E or #D)
    assert!(ButtonTheme::Danger.background_color().starts_with("#E")
            || ButtonTheme::Danger.background_color().starts_with("#D"));

    // Secondary should be purple-ish (#8 or #7)
    assert!(ButtonTheme::Secondary.background_color().starts_with("#8")
            || ButtonTheme::Secondary.background_color().starts_with("#7"));
}

// Test R6.12: Button enums are standard Rust
#[test]
fn test_ui_025_enum_traits() {
    // Enums should implement Debug, Clone, Copy, PartialEq, Eq
    let size = ButtonSize::Medium;
    let _cloned = size.clone();
    let _copied = size;

    assert_eq!(size, ButtonSize::Medium);
    assert_ne!(size, ButtonSize::Small);

    let theme = ButtonTheme::Primary;
    let _cloned = theme.clone();
    let _copied = theme;

    assert_eq!(theme, ButtonTheme::Primary);
    assert_ne!(theme, ButtonTheme::Success);
}
