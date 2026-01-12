/// Color calibration UI component
///
/// Provides a user interface for calibrating color detection by showing
/// each cube color to the camera and collecting samples.

use dioxus::prelude::*;
use crate::camera::{CalibrationManager, ColorSample, RGB};
use crate::cube::Color;

/// Props for ColorCalibration component
#[derive(Props, Clone, PartialEq)]
pub struct ColorCalibrationProps {
    /// Calibration manager state
    pub manager: Signal<CalibrationManager>,
    /// Optional callback when calibration is complete
    pub on_complete: Option<EventHandler<()>>,
}

/// Color name helper for display
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

/// Color to CSS color string
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

/// Get instruction text for current color
fn get_instruction_text(color: Color) -> &'static str {
    match color {
        Color::White => "Hold a WHITE sticker in front of the camera. Make sure it's well-lit and centered.",
        Color::Yellow => "Hold a YELLOW sticker in front of the camera. Make sure it's well-lit and centered.",
        Color::Red => "Hold a RED sticker in front of the camera. Make sure it's well-lit and centered.",
        Color::Orange => "Hold an ORANGE sticker in front of the camera. Make sure it's well-lit and centered.",
        Color::Blue => "Hold a BLUE sticker in front of the camera. Make sure it's well-lit and centered.",
        Color::Green => "Hold a GREEN sticker in front of the camera. Make sure it's well-lit and centered.",
    }
}

/// ColorCalibration component for calibrating color detection
#[component]
pub fn ColorCalibration(props: ColorCalibrationProps) -> Element {
    let mut manager = props.manager;

    // Read current state
    let mgr = manager.read();
    let current_color = mgr.current_color();
    let sample_count = mgr.current_calibration().sample_count;
    let completed = mgr.completed_colors();
    let is_complete = mgr.is_complete();
    drop(mgr); // Release read lock

    let instruction = get_instruction_text(current_color);
    let color_css = color_to_css(current_color);
    let name = color_name(current_color);

    // Text color depends on background
    let text_color = if current_color == Color::White || current_color == Color::Yellow {
        "#000"
    } else {
        "#FFF"
    };

    // Sample counter
    let samples = sample_count;
    let samples_needed = 10;
    let progress_pct = (samples as f32 / samples_needed as f32 * 100.0) as u32;

    // Handlers
    let capture_sample = move |_| {
        // In a real implementation, this would capture from camera
        // For now, we'll simulate with default colors
        let rgb = match current_color {
            Color::White => RGB::new(240, 240, 240),
            Color::Yellow => RGB::new(255, 215, 0),
            Color::Red => RGB::new(196, 30, 58),
            Color::Orange => RGB::new(255, 88, 0),
            Color::Blue => RGB::new(0, 81, 186),
            Color::Green => RGB::new(0, 158, 96),
        };
        let sample = ColorSample::new(rgb);
        manager.write().add_sample(sample);
    };

    let next_color = move |_| {
        if !manager.write().next_color() {
            // All colors done
            if let Some(on_complete) = &props.on_complete {
                on_complete.call(());
            }
        }
    };

    let reset = move |_| {
        manager.write().reset();
    };

    rsx! {
        div {
            class: "color-calibration-container",
            style: "{get_calibration_styles()}",

            // Header
            div {
                class: "calibration-header",
                h2 { "Color Calibration" }
                p { "Calibrate your camera for better color detection" }
            }

            // Progress
            div {
                class: "calibration-progress",
                div {
                    class: "progress-text",
                    "Calibrated: {completed} / 6 colors"
                }
                div {
                    class: "progress-bar-container",
                    div {
                        class: "progress-bar",
                        style: "width: {completed * 100 / 6}%;"
                    }
                }
            }

            if !is_complete {
                // Current color instruction
                div {
                    class: "current-color-section",
                    div {
                        class: "color-display",
                        style: "background-color: {color_css}; color: {text_color};",
                        "{name}"
                    }

                    p { class: "instruction", "{instruction}" }

                    // Sample progress for current color
                    div {
                        class: "sample-progress",
                        "Samples: {samples} / {samples_needed}"
                        div {
                            class: "sample-bar-container",
                            div {
                                class: "sample-bar",
                                style: "width: {progress_pct}%; background-color: {color_css};"
                            }
                        }
                    }

                    // Capture button
                    div {
                        class: "button-group",
                        button {
                            class: "btn-capture",
                            onclick: capture_sample,
                            disabled: samples >= samples_needed,
                            "📸 Capture Sample"
                        }

                        if samples >= samples_needed {
                            button {
                                class: "btn-next",
                                onclick: next_color,
                                "Next Color →"
                            }
                        }
                    }
                }
            } else {
                // Completion screen
                div {
                    class: "completion-screen",
                    div { class: "completion-icon", "✅" }
                    h3 { "Calibration Complete!" }
                    p { "Your camera is now calibrated for optimal color detection." }
                    button {
                        class: "btn-reset",
                        onclick: reset,
                        "Calibrate Again"
                    }
                }
            }

            // Reset button (always available)
            div {
                class: "footer-actions",
                button {
                    class: "btn-reset-small",
                    onclick: reset,
                    "↺ Start Over"
                }
            }
        }
    }
}

/// Get CSS styles for calibration component
fn get_calibration_styles() -> &'static str {
    r#"
        max-width: 600px;
        margin: 0 auto;
        padding: 20px;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    "#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_name() {
        assert_eq!(color_name(Color::White), "White");
        assert_eq!(color_name(Color::Yellow), "Yellow");
        assert_eq!(color_name(Color::Red), "Red");
        assert_eq!(color_name(Color::Orange), "Orange");
        assert_eq!(color_name(Color::Blue), "Blue");
        assert_eq!(color_name(Color::Green), "Green");
    }

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
    fn test_instruction_text() {
        let instruction = get_instruction_text(Color::White);
        assert!(instruction.contains("WHITE"));
        assert!(instruction.contains("camera"));
    }
}
