/// Scan correction component for handling failed or uncertain scans
///
/// This component provides:
/// - Detection of uncertain colors (low confidence)
/// - Manual correction UI for fixing incorrect colors
/// - Retry scan option to re-scan the face

use dioxus::prelude::*;
use crate::cube::Color;
use crate::camera::color_detect::ColorDetectionResult;

/// Position of a sticker in the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StickerPosition {
    pub row: usize,
    pub col: usize,
}

/// Scan correction state
#[derive(Debug, Clone, PartialEq)]
pub enum CorrectionState {
    /// Reviewing the scan, no corrections needed yet
    Reviewing,
    /// User is correcting colors
    Correcting,
    /// Corrections complete, ready to confirm
    Complete,
}

#[derive(Props, Clone, PartialEq)]
pub struct ScanCorrectionProps {
    /// Detected colors with confidence scores
    pub detected_colors: Vec<Vec<ColorDetectionResult>>,

    /// Callback when user confirms the corrected scan
    #[props(optional)]
    pub on_confirm: Option<EventHandler<Vec<Vec<Color>>>>,

    /// Callback when user wants to retry the scan
    #[props(optional)]
    pub on_retry: Option<EventHandler<()>>,

    /// Callback when user cancels
    #[props(optional)]
    pub on_cancel: Option<EventHandler<()>>,
}

#[component]
pub fn ScanCorrection(props: ScanCorrectionProps) -> Element {
    let mut correction_state = use_signal(|| CorrectionState::Reviewing);
    let mut selected_sticker = use_signal(|| Option::<StickerPosition>::None);
    let mut corrected_colors = use_signal(|| {
        // Initialize with detected colors
        props.detected_colors
            .iter()
            .map(|row| row.iter().map(|result| result.color).collect())
            .collect::<Vec<Vec<Color>>>()
    });

    // Count uncertain colors
    let uncertain_count = props.detected_colors
        .iter()
        .flatten()
        .filter(|result| result.is_uncertain())
        .count();

    let has_uncertain = uncertain_count > 0;

    // Handle sticker click
    let mut select_sticker = move |row: usize, col: usize| {
        selected_sticker.set(Some(StickerPosition { row, col }));
        correction_state.set(CorrectionState::Correcting);
    };

    // Handle color selection for correction
    let mut apply_color = move |color: Color| {
        if let Some(pos) = selected_sticker() {
            corrected_colors.write()[pos.row][pos.col] = color;
            selected_sticker.set(None);
        }
    };

    // Confirm the corrected scan
    let confirm_scan = move |_| {
        if let Some(handler) = &props.on_confirm {
            handler.call(corrected_colors());
        }
    };

    // Retry the scan
    let retry_scan = move |_| {
        if let Some(handler) = &props.on_retry {
            handler.call(());
        }
    };

    // Cancel
    let cancel = move |_| {
        if let Some(handler) = &props.on_cancel {
            handler.call(());
        }
    };

    let grid_size = props.detected_colors.len();

    rsx! {
        div {
            class: "scan-correction",
            style: "padding: 20px; max-width: 800px; margin: 0 auto; background: #f9fafb; border-radius: 12px;",

            // Header
            div {
                style: "margin-bottom: 20px;",

                h2 {
                    style: "font-size: 24px; font-weight: bold; margin-bottom: 10px;",
                    "Review Scan"
                }

                // Warning if uncertain colors detected
                if has_uncertain {
                    div {
                        style: "padding: 12px 16px; background: #fef3c7; border: 2px solid #f59e0b; \
                               border-radius: 8px; margin-bottom: 15px; display: flex; align-items: center; gap: 10px;",

                        span {
                            style: "font-size: 24px;",
                            "⚠️"
                        }

                        div {
                            p {
                                style: "font-size: 14px; font-weight: 600; color: #92400e; margin: 0 0 4px 0;",
                                {format!("⚠ {} uncertain colors detected", uncertain_count)}
                            }
                            p {
                                style: "font-size: 12px; color: #78350f; margin: 0;",
                                "Colors with low confidence are highlighted in yellow. Click them to correct."
                            }
                        }
                    }
                }

                p {
                    style: "font-size: 14px; color: #666;",
                    if has_uncertain {
                        "Some colors may not have been detected correctly. Review and correct any mistakes."
                    } else {
                        "All colors detected with good confidence! Review and confirm if correct."
                    }
                }
            }

            // Color grid with confidence indicators
            div {
                style: "margin-bottom: 20px; display: flex; justify-content: center;",

                div {
                    style: format!(
                        "display: grid; grid-template-columns: repeat({}, 1fr); \
                         gap: 6px; background: #000; padding: 15px; border-radius: 10px;",
                        grid_size
                    ),

                    for (row_idx, row) in props.detected_colors.iter().enumerate() {
                        for (col_idx, result) in row.iter().enumerate() {
                            {
                                let is_selected = selected_sticker()
                                    .map(|pos| pos.row == row_idx && pos.col == col_idx)
                                    .unwrap_or(false);
                                let is_uncertain = result.is_uncertain();

                                // Use corrected color if available
                                let display_color = corrected_colors()[row_idx][col_idx];
                                let display_css = color_to_css(display_color);

                                rsx! {
                                    div {
                                        key: "{row_idx}-{col_idx}",
                                        onclick: move |_| select_sticker(row_idx, col_idx),
                                        style: format!(
                                            "width: 50px; height: 50px; background: {}; \
                                             border-radius: 6px; cursor: pointer; \
                                             transition: all 0.2s; position: relative; \
                                             box-shadow: {}; \
                                             border: {};",
                                            display_css,
                                            if is_selected {
                                                "0 0 0 4px #3b82f6"
                                            } else {
                                                "0 2px 4px rgba(0,0,0,0.3)"
                                            },
                                            if is_uncertain {
                                                "3px solid #f59e0b"
                                            } else {
                                                "none"
                                            }
                                        ),

                                        // Confidence indicator
                                        div {
                                            style: format!(
                                                "position: absolute; top: 2px; right: 2px; \
                                                 font-size: 10px; font-weight: bold; \
                                                 background: rgba(0,0,0,0.6); color: {}; \
                                                 padding: 2px 4px; border-radius: 3px;",
                                                if result.is_reliable() { "#22c55e" }
                                                else if is_uncertain { "#f59e0b" }
                                                else { "#fbbf24" }
                                            ),
                                            {format!("{:.0}%", result.confidence * 100.0)}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Color picker (shown when correcting)
            if selected_sticker().is_some() {
                div {
                    style: "margin-bottom: 20px; padding: 15px; background: white; border-radius: 8px; \
                           box-shadow: 0 2px 8px rgba(0,0,0,0.1);",

                    p {
                        style: "font-size: 14px; font-weight: 600; margin-bottom: 10px;",
                        "Select the correct color:"
                    }

                    div {
                        style: "display: flex; gap: 10px; flex-wrap: wrap;",

                        for color in [Color::White, Color::Yellow, Color::Red, Color::Orange, Color::Blue, Color::Green] {
                            {
                                let css_color = color_to_css(color);
                                rsx! {
                                    button {
                                        key: "{color:?}",
                                        onclick: move |_| apply_color(color),
                                        style: format!(
                                            "width: 60px; height: 60px; background: {}; \
                                             border: 3px solid #333; border-radius: 8px; \
                                             cursor: pointer; transition: transform 0.2s; \
                                             box-shadow: 0 2px 4px rgba(0,0,0,0.2);",
                                            css_color
                                        ),
                                        title: "{color:?}",
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Action buttons
            div {
                style: "display: flex; gap: 10px;",

                button {
                    onclick: retry_scan,
                    style: "flex: 1; padding: 14px 20px; font-size: 16px; font-weight: 600; \
                           background: #ef4444; color: white; border: none; \
                           border-radius: 8px; cursor: pointer; transition: background 0.2s;",
                    "🔄 Retry Scan"
                }

                button {
                    onclick: cancel,
                    style: "flex: 1; padding: 14px 20px; font-size: 16px; font-weight: 600; \
                           background: #6b7280; color: white; border: none; \
                           border-radius: 8px; cursor: pointer; transition: background 0.2s;",
                    "Cancel"
                }

                button {
                    onclick: confirm_scan,
                    style: "flex: 2; padding: 14px 20px; font-size: 16px; font-weight: 600; \
                           background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%); \
                           color: white; border: none; border-radius: 8px; cursor: pointer; \
                           transition: transform 0.2s; box-shadow: 0 4px 6px rgba(0,0,0,0.1);",
                    "✓ Confirm"
                }
            }
        }
    }
}

/// Convert Color enum to CSS color string
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sticker_position_equality() {
        let pos1 = StickerPosition { row: 0, col: 0 };
        let pos2 = StickerPosition { row: 0, col: 0 };
        let pos3 = StickerPosition { row: 0, col: 1 };

        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
    }

    #[test]
    fn test_correction_state_equality() {
        assert_eq!(CorrectionState::Reviewing, CorrectionState::Reviewing);
        assert_eq!(CorrectionState::Correcting, CorrectionState::Correcting);
        assert_ne!(CorrectionState::Reviewing, CorrectionState::Correcting);
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
    fn test_detection_result_is_uncertain() {
        let uncertain = ColorDetectionResult {
            color: Color::Red,
            confidence: 0.5,
        };
        assert!(uncertain.is_uncertain());

        let certain = ColorDetectionResult {
            color: Color::Red,
            confidence: 0.8,
        };
        assert!(!certain.is_uncertain());
    }

    #[test]
    fn test_detection_result_is_reliable() {
        let reliable = ColorDetectionResult {
            color: Color::Red,
            confidence: 0.9,
        };
        assert!(reliable.is_reliable());

        let unreliable = ColorDetectionResult {
            color: Color::Red,
            confidence: 0.7,
        };
        assert!(!unreliable.is_reliable());
    }

    #[test]
    fn test_sticker_position_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();

        let pos1 = StickerPosition { row: 0, col: 0 };
        let pos2 = StickerPosition { row: 0, col: 0 };

        set.insert(pos1);
        set.insert(pos2);

        assert_eq!(set.len(), 1); // Should only have one entry
    }
}
