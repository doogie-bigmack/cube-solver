//! Color Picker Component
//!
//! This module provides a color picker palette for selecting cube sticker colors.
//!
//! Requirements: R3.3 - Color picker palette (6 colors)
//! - Display 6 color buttons
//! - Click color to apply to selected sticker
//! - Show currently selected color

use dioxus::prelude::*;
use crate::cube::Color;

/// Props for the ColorPicker component
#[derive(Clone, PartialEq, Props)]
pub struct ColorPickerProps {
    /// Callback when a color is selected
    pub on_color_select: EventHandler<Color>,
    /// Currently selected color (if any)
    #[props(optional)]
    pub selected_color: Option<Color>,
    /// Size of color buttons in pixels (default: 50)
    #[props(optional)]
    pub button_size: Option<f32>,
}

/// Convert Color to CSS color string
fn color_to_css(color: Color) -> &'static str {
    match color {
        Color::White => "#FFFFFF",
        Color::Yellow => "#FFD500",
        Color::Red => "#C41E3A",
        Color::Orange => "#FF5800",
        Color::Blue => "#0051BA",
        Color::Green => "#009E60",
    }
}

/// Get color name for display
fn color_name(color: Color) -> &'static str {
    match color {
        Color::White => "White",
        Color::Yellow => "Yellow",
        Color::Red => "Red",
        Color::Orange => "Orange",
        Color::Blue => "Blue",
        Color::Green => "Green",
    }
}

/// Color Picker Component
/// Displays a palette of 6 cube colors that users can click to select
#[component]
pub fn ColorPicker(props: ColorPickerProps) -> Element {
    let button_size = props.button_size.unwrap_or(50.0);
    let gap = 12.0;

    // All 6 cube colors in standard order
    let colors = vec![
        Color::White,
        Color::Yellow,
        Color::Red,
        Color::Orange,
        Color::Blue,
        Color::Green,
    ];

    // Container style - arrange in 2 rows of 3
    let container_style = format!(
        "display: grid; grid-template-columns: repeat(3, {}px); gap: {}px; \
         padding: 16px; background: #f7fafc; border-radius: 8px; \
         border: 2px solid #e2e8f0;",
        button_size, gap
    );

    rsx! {
        div {
            class: "color-picker-container",
            style: "{container_style}",

            // Render all 6 color buttons
            for color in colors {
                {render_color_button(color, props.selected_color, button_size, props.on_color_select)}
            }
        }
    }
}

/// Render a single color button
fn render_color_button(
    color: Color,
    selected_color: Option<Color>,
    button_size: f32,
    on_color_select: EventHandler<Color>,
) -> Element {
    let color_css = color_to_css(color);
    let is_selected = selected_color == Some(color);

    // Add visual indicator for selected color
    let button_style = if is_selected {
        format!(
            "width: {}px; height: {}px; background: {}; border-radius: 8px; \
             cursor: pointer; transition: all 0.2s; \
             border: 4px solid #3182ce; box-shadow: 0 0 12px rgba(49, 130, 206, 0.6); \
             transform: scale(1.1); position: relative; z-index: 10; touch-action: manipulation;",
            button_size, button_size, color_css
        )
    } else {
        // Add subtle border to white button for visibility
        let border = if matches!(color, Color::White) {
            "border: 2px solid #cbd5e0;"
        } else {
            "border: 2px solid transparent;"
        };

        format!(
            "width: {}px; height: {}px; background: {}; border-radius: 8px; \
             cursor: pointer; transition: all 0.2s; {} \
             box-shadow: 0 2px 4px rgba(0,0,0,0.1); touch-action: manipulation;",
            button_size, button_size, color_css, border
        )
    };

    // Add hover effect class
    let hover_style = "
        .color-button:hover {
            transform: scale(1.05);
            box-shadow: 0 4px 8px rgba(0,0,0,0.2);
        }
        .color-button:active {
            transform: scale(0.95);
        }
    ";

    rsx! {
        style { "{hover_style}" }
        button {
            class: "color-button",
            style: "{button_style}",
            onclick: move |_| {
                on_color_select.call(color);
            },
            title: format!("{} color", color_name(color)),
            "aria-label": format!("Select {} color", color_name(color)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_to_css() {
        assert_eq!(color_to_css(Color::White), "#FFFFFF");
        assert_eq!(color_to_css(Color::Yellow), "#FFD500");
        assert_eq!(color_to_css(Color::Red), "#C41E3A");
        assert_eq!(color_to_css(Color::Orange), "#FF5800");
        assert_eq!(color_to_css(Color::Blue), "#0051BA");
        assert_eq!(color_to_css(Color::Green), "#009E60");
    }

    #[test]
    fn test_color_names() {
        assert_eq!(color_name(Color::White), "White");
        assert_eq!(color_name(Color::Yellow), "Yellow");
        assert_eq!(color_name(Color::Red), "Red");
        assert_eq!(color_name(Color::Orange), "Orange");
        assert_eq!(color_name(Color::Blue), "Blue");
        assert_eq!(color_name(Color::Green), "Green");
    }

    #[test]
    fn test_all_colors_have_css() {
        // Ensure all Color variants have a CSS representation
        let colors = vec![
            Color::White,
            Color::Yellow,
            Color::Red,
            Color::Orange,
            Color::Blue,
            Color::Green,
        ];

        for color in colors {
            let css = color_to_css(color);
            assert!(css.starts_with('#'));
            assert_eq!(css.len(), 7); // #RRGGBB format
        }
    }

    #[test]
    fn test_all_colors_have_names() {
        let colors = vec![
            Color::White,
            Color::Yellow,
            Color::Red,
            Color::Orange,
            Color::Blue,
            Color::Green,
        ];

        for color in colors {
            let name = color_name(color);
            assert!(!name.is_empty());
            assert!(name.len() > 2);
        }
    }
}
