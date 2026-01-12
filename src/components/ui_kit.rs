//! Kid-friendly UI components
//!
//! This module implements R6.12 from the PRD:
//! - Minimum 44px touch targets
//! - Clear icons for all actions
//! - Simple, uncluttered layout
//! - Bright, engaging colors

use dioxus::prelude::*;

/// Button size variants for different use cases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    /// Small button - 44px min (meets accessibility standards)
    Small,
    /// Medium button - 56px (comfortable for kids)
    Medium,
    /// Large button - 72px (easy for small fingers)
    Large,
}

impl ButtonSize {
    /// Returns the pixel size for this button size
    pub fn pixels(&self) -> usize {
        match self {
            ButtonSize::Small => 44,
            ButtonSize::Medium => 56,
            ButtonSize::Large => 72,
        }
    }

    /// Returns the font size for this button size
    pub fn font_size(&self) -> usize {
        match self {
            ButtonSize::Small => 14,
            ButtonSize::Medium => 18,
            ButtonSize::Large => 24,
        }
    }
}

/// Button color themes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonTheme {
    /// Primary action button (bright blue)
    Primary,
    /// Success action button (bright green)
    Success,
    /// Warning action button (bright orange)
    Warning,
    /// Danger action button (bright red)
    Danger,
    /// Secondary action button (purple)
    Secondary,
}

impl ButtonTheme {
    /// Returns the background color for this theme
    pub fn background_color(&self) -> &'static str {
        match self {
            ButtonTheme::Primary => "#3B82F6",    // Bright blue
            ButtonTheme::Success => "#10B981",    // Bright green
            ButtonTheme::Warning => "#F59E0B",    // Bright orange
            ButtonTheme::Danger => "#EF4444",     // Bright red
            ButtonTheme::Secondary => "#8B5CF6",  // Bright purple
        }
    }

    /// Returns the hover color for this theme
    pub fn hover_color(&self) -> &'static str {
        match self {
            ButtonTheme::Primary => "#2563EB",
            ButtonTheme::Success => "#059669",
            ButtonTheme::Warning => "#D97706",
            ButtonTheme::Danger => "#DC2626",
            ButtonTheme::Secondary => "#7C3AED",
        }
    }
}

/// Props for the KidButton component
#[derive(Props, Clone, PartialEq)]
pub struct KidButtonProps {
    /// Button click handler
    pub onclick: EventHandler<MouseEvent>,
    /// Button label text
    pub label: String,
    /// Optional icon emoji
    #[props(default = "".to_string())]
    pub icon: String,
    /// Button size
    #[props(default = ButtonSize::Medium)]
    pub size: ButtonSize,
    /// Button theme
    #[props(default = ButtonTheme::Primary)]
    pub theme: ButtonTheme,
    /// Whether button is disabled
    #[props(default = false)]
    pub disabled: bool,
}

/// Kid-friendly button component with large touch targets and bright colors
#[component]
pub fn KidButton(props: KidButtonProps) -> Element {
    let size_px = props.size.pixels();
    let font_size = props.size.font_size();
    let bg_color = props.theme.background_color();
    let hover_color = props.theme.hover_color();

    let button_style = format!(
        "min-width: {}px; min-height: {}px; font-size: {}px; background: {}; padding: 12px 24px; \
         border: none; border-radius: 12px; color: white; font-weight: bold; cursor: pointer; \
         box-shadow: 0 4px 6px rgba(0,0,0,0.1); transition: all 0.2s; display: flex; \
         align-items: center; justify-content: center; gap: 8px; {}",
        size_px,
        size_px,
        font_size,
        bg_color,
        if props.disabled { "opacity: 0.5; cursor: not-allowed;" } else { "" }
    );

    let hover_style = if !props.disabled {
        format!(
            "button:hover {{ background: {} !important; transform: translateY(-2px); \
             box-shadow: 0 6px 8px rgba(0,0,0,0.15); }}",
            hover_color
        )
    } else {
        String::new()
    };

    rsx! {
        style { {hover_style} }
        button {
            style: "{button_style}",
            onclick: move |evt| {
                if !props.disabled {
                    props.onclick.call(evt);
                }
            },
            disabled: props.disabled,
            if !props.icon.is_empty() {
                span {
                    style: "font-size: {font_size * 120 / 100}px;",
                    "{props.icon}"
                }
            }
            span { "{props.label}" }
        }
    }
}

/// Props for the KidCard component
#[derive(Props, Clone, PartialEq)]
pub struct KidCardProps {
    /// Card title
    #[props(default = "".to_string())]
    pub title: String,
    /// Card content
    pub children: Element,
}

/// Kid-friendly card component with rounded corners and shadows
#[component]
pub fn KidCard(props: KidCardProps) -> Element {
    rsx! {
        div {
            style: "background: white; border-radius: 16px; padding: 24px; box-shadow: 0 4px 12px rgba(0,0,0,0.1); \
                    margin: 16px; max-width: 800px;",
            if !props.title.is_empty() {
                h2 {
                    style: "margin: 0 0 16px 0; font-size: 28px; color: #1F2937; font-weight: bold;",
                    "{props.title}"
                }
            }
            div { {props.children} }
        }
    }
}

/// Props for the KidIconButton component
#[derive(Props, Clone, PartialEq)]
pub struct KidIconButtonProps {
    /// Button click handler
    pub onclick: EventHandler<MouseEvent>,
    /// Icon emoji
    pub icon: String,
    /// Accessible label (for screen readers)
    pub label: String,
    /// Button size
    #[props(default = ButtonSize::Medium)]
    pub size: ButtonSize,
    /// Button theme
    #[props(default = ButtonTheme::Primary)]
    pub theme: ButtonTheme,
    /// Whether button is disabled
    #[props(default = false)]
    pub disabled: bool,
}

/// Kid-friendly icon-only button (square, with large icon)
#[component]
pub fn KidIconButton(props: KidIconButtonProps) -> Element {
    let size_px = props.size.pixels();
    let font_size = props.size.font_size() * 140 / 100; // 1.4x for icons
    let bg_color = props.theme.background_color();
    let hover_color = props.theme.hover_color();

    let button_style = format!(
        "width: {}px; height: {}px; font-size: {}px; background: {}; border: none; \
         border-radius: 12px; color: white; cursor: pointer; box-shadow: 0 4px 6px rgba(0,0,0,0.1); \
         transition: all 0.2s; display: flex; align-items: center; justify-content: center; {}",
        size_px,
        size_px,
        font_size,
        bg_color,
        if props.disabled { "opacity: 0.5; cursor: not-allowed;" } else { "" }
    );

    let hover_style = if !props.disabled {
        format!(
            "button:hover {{ background: {} !important; transform: scale(1.1); \
             box-shadow: 0 6px 8px rgba(0,0,0,0.15); }}",
            hover_color
        )
    } else {
        String::new()
    };

    rsx! {
        style { {hover_style} }
        button {
            style: "{button_style}",
            onclick: move |evt| {
                if !props.disabled {
                    props.onclick.call(evt);
                }
            },
            disabled: props.disabled,
            "aria-label": "{props.label}",
            title: "{props.label}",
            "{props.icon}"
        }
    }
}

/// Props for the KidBadge component
#[derive(Props, Clone, PartialEq)]
pub struct KidBadgeProps {
    /// Badge text
    pub text: String,
    /// Badge color theme
    #[props(default = ButtonTheme::Primary)]
    pub theme: ButtonTheme,
}

/// Kid-friendly badge component for displaying status or counts
#[component]
pub fn KidBadge(props: KidBadgeProps) -> Element {
    let bg_color = props.theme.background_color();

    rsx! {
        span {
            style: "display: inline-block; padding: 6px 12px; background: {bg_color}; color: white; \
                    border-radius: 20px; font-size: 14px; font-weight: bold; margin: 4px;",
            "{props.text}"
        }
    }
}

/// Props for the KidProgress component
#[derive(Props, Clone, PartialEq)]
pub struct KidProgressProps {
    /// Current progress value (0-100)
    pub value: f32,
    /// Progress bar label
    #[props(default = "".to_string())]
    pub label: String,
}

/// Kid-friendly progress bar with bright colors
#[component]
pub fn KidProgress(props: KidProgressProps) -> Element {
    let clamped_value = props.value.max(0.0).min(100.0);

    rsx! {
        div {
            style: "margin: 16px 0;",
            if !props.label.is_empty() {
                div {
                    style: "font-size: 16px; font-weight: bold; color: #1F2937; margin-bottom: 8px;",
                    "{props.label}"
                }
            }
            div {
                style: "width: 100%; height: 32px; background: #E5E7EB; border-radius: 16px; overflow: hidden;",
                div {
                    style: "height: 100%; background: linear-gradient(90deg, #10B981, #3B82F6); \
                            width: {clamped_value}%; transition: width 0.3s ease; display: flex; \
                            align-items: center; justify-content: center; color: white; font-weight: bold; \
                            font-size: 14px;",
                    if clamped_value > 10.0 {
                        "{clamped_value:.0}%"
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_size_pixels() {
        assert_eq!(ButtonSize::Small.pixels(), 44);
        assert_eq!(ButtonSize::Medium.pixels(), 56);
        assert_eq!(ButtonSize::Large.pixels(), 72);
    }

    #[test]
    fn test_button_size_meets_accessibility() {
        // All sizes must be at least 44px (WCAG accessibility guideline)
        assert!(ButtonSize::Small.pixels() >= 44);
        assert!(ButtonSize::Medium.pixels() >= 44);
        assert!(ButtonSize::Large.pixels() >= 44);
    }

    #[test]
    fn test_button_size_font_scaling() {
        assert_eq!(ButtonSize::Small.font_size(), 14);
        assert_eq!(ButtonSize::Medium.font_size(), 18);
        assert_eq!(ButtonSize::Large.font_size(), 24);
    }

    #[test]
    fn test_button_theme_colors() {
        assert_eq!(ButtonTheme::Primary.background_color(), "#3B82F6");
        assert_eq!(ButtonTheme::Success.background_color(), "#10B981");
        assert_eq!(ButtonTheme::Warning.background_color(), "#F59E0B");
        assert_eq!(ButtonTheme::Danger.background_color(), "#EF4444");
        assert_eq!(ButtonTheme::Secondary.background_color(), "#8B5CF6");
    }

    #[test]
    fn test_button_theme_hover_colors() {
        assert_eq!(ButtonTheme::Primary.hover_color(), "#2563EB");
        assert_eq!(ButtonTheme::Success.hover_color(), "#059669");
        assert_eq!(ButtonTheme::Warning.hover_color(), "#D97706");
        assert_eq!(ButtonTheme::Danger.hover_color(), "#DC2626");
        assert_eq!(ButtonTheme::Secondary.hover_color(), "#7C3AED");
    }

    #[test]
    fn test_button_sizes_progressive() {
        // Sizes should be progressively larger
        assert!(ButtonSize::Medium.pixels() > ButtonSize::Small.pixels());
        assert!(ButtonSize::Large.pixels() > ButtonSize::Medium.pixels());
    }

    #[test]
    fn test_font_sizes_progressive() {
        // Font sizes should scale with button size
        assert!(ButtonSize::Medium.font_size() > ButtonSize::Small.font_size());
        assert!(ButtonSize::Large.font_size() > ButtonSize::Medium.font_size());
    }

    #[test]
    fn test_all_colors_are_hex() {
        // All colors should be valid hex codes
        for theme in [
            ButtonTheme::Primary,
            ButtonTheme::Success,
            ButtonTheme::Warning,
            ButtonTheme::Danger,
            ButtonTheme::Secondary,
        ] {
            assert!(theme.background_color().starts_with('#'));
            assert!(theme.hover_color().starts_with('#'));
            assert_eq!(theme.background_color().len(), 7); // #RRGGBB
            assert_eq!(theme.hover_color().len(), 7);
        }
    }

    #[test]
    fn test_colors_are_bright() {
        // Kid-friendly colors should be bright (not completely dark)
        // Check that at least some colors are bright (checking just existence of bright colors)
        let themes = [
            ButtonTheme::Primary,
            ButtonTheme::Success,
            ButtonTheme::Warning,
            ButtonTheme::Danger,
            ButtonTheme::Secondary,
        ];

        let mut bright_count = 0;
        for theme in themes {
            let bg = theme.background_color();
            let hover = theme.hover_color();

            // First digit after # indicates brightness
            let bg_brightness = bg.chars().nth(1).unwrap();
            let hover_brightness = hover.chars().nth(1).unwrap();

            // Count as bright if first digit is 3+ or A-F
            if (bg_brightness >= '3' && bg_brightness <= '9') || (bg_brightness >= 'A' && bg_brightness <= 'F') {
                bright_count += 1;
            }

            // All hover colors should at least be valid
            assert!(hover_brightness >= '0' && (hover_brightness <= '9' || (hover_brightness >= 'A' && hover_brightness <= 'F')));
        }

        // At least 3 of 5 themes should be bright
        assert!(bright_count >= 3, "Not enough bright colors: {}", bright_count);
    }
}
