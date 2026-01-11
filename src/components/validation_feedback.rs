//! Validation feedback component
//!
//! This module implements R3.5 from the PRD:
//! - Show warning for wrong color counts
//! - Show error for unsolvable states
//! - Clear error messages

use crate::cube::validation::ValidationError;
use dioxus::prelude::*;

/// Props for the ValidationFeedback component
#[derive(Props, Clone, PartialEq)]
pub struct ValidationFeedbackProps {
    /// The validation result to display (None if valid)
    pub validation_error: Option<ValidationError>,
}

/// Validation feedback component that displays errors and warnings
///
/// # Example
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use rubiks_cube_solver::components::validation_feedback::{ValidationFeedback, ValidationFeedbackProps};
/// use rubiks_cube_solver::cube::validation::ValidationError;
/// use rubiks_cube_solver::cube::Color;
///
/// #[component]
/// fn App() -> Element {
///     let error = Some(ValidationError::InvalidColorCount {
///         color: Color::White,
///         expected: 9,
///         actual: 10,
///     });
///
///     rsx! {
///         ValidationFeedback {
///             validation_error: error
///         }
///     }
/// }
/// ```
#[component]
pub fn ValidationFeedback(props: ValidationFeedbackProps) -> Element {
    match &props.validation_error {
        None => rsx! {
            div {
                class: "validation-feedback validation-success",
                "aria-live": "polite",
                div {
                    class: "validation-icon",
                    "✓"
                }
                div {
                    class: "validation-message",
                    "Cube state is valid"
                }
            }
        },
        Some(error) => {
            let (severity, icon, message, details) = get_error_info(error);

            rsx! {
                div {
                    class: "validation-feedback validation-{severity}",
                    role: "alert",
                    "aria-live": "assertive",
                    div {
                        class: "validation-icon",
                        "{icon}"
                    }
                    div {
                        class: "validation-content",
                        div {
                            class: "validation-message",
                            "{message}"
                        }
                        if let Some(detail_text) = details {
                            div {
                                class: "validation-details",
                                "{detail_text}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Determines the severity, icon, message, and details for a validation error
fn get_error_info(error: &ValidationError) -> (&'static str, &'static str, String, Option<String>) {
    match error {
        ValidationError::InvalidColorCount { color, expected, actual } => {
            let severity = if *actual == *expected { "success" } else { "error" };
            let icon = if severity == "error" { "⚠" } else { "✓" };
            let message = format!("Wrong number of {:?} stickers", color);
            let details = Some(format!(
                "Found {} stickers, but expected {}",
                actual, expected
            ));
            (severity, icon, message, details)
        }
        ValidationError::MissingColors { missing } => {
            let color_names: Vec<String> = missing.iter().map(|c| format!("{:?}", c)).collect();
            let message = "Some colors are missing".to_string();
            let details = Some(format!("Missing colors: {}", color_names.join(", ")));
            ("error", "⚠", message, details)
        }
        ValidationError::EdgeParity => {
            let message = "Edge pieces cannot be solved".to_string();
            let details = Some(
                "This cube state has an edge parity error. The edges cannot be solved without fixing the parity.".to_string()
            );
            ("error", "⚠", message, details)
        }
        ValidationError::CornerParity => {
            let message = "Corner pieces cannot be solved".to_string();
            let details = Some(
                "This cube state has a corner parity error. The corners cannot be solved without fixing the parity.".to_string()
            );
            ("error", "⚠", message, details)
        }
        ValidationError::PermutationParity => {
            let message = "Pieces cannot be solved".to_string();
            let details = Some(
                "This cube state has a permutation parity error. The pieces cannot be solved without fixing the parity.".to_string()
            );
            ("error", "⚠", message, details)
        }
    }
}

/// Helper function to get a kid-friendly message for validation errors
pub fn get_kid_friendly_message(error: &ValidationError) -> String {
    match error {
        ValidationError::InvalidColorCount { color, .. } => {
            format!("Oops! You have the wrong number of {:?} stickers. Let's fix that!", color)
        }
        ValidationError::MissingColors { .. } => {
            "Oops! Some colors are missing. Make sure all 6 colors are on the cube!".to_string()
        }
        ValidationError::EdgeParity => {
            "This cube can't be solved! Try resetting it and entering the colors again.".to_string()
        }
        ValidationError::CornerParity => {
            "This cube can't be solved! Try resetting it and entering the colors again.".to_string()
        }
        ValidationError::PermutationParity => {
            "This cube can't be solved! Try resetting it and entering the colors again.".to_string()
        }
    }
}

/// CSS styles for the validation feedback component
pub fn get_validation_styles() -> &'static str {
    r#"
.validation-feedback {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px;
    border-radius: 8px;
    margin: 16px 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
}

.validation-success {
    background-color: #d4edda;
    border: 2px solid #28a745;
    color: #155724;
}

.validation-error {
    background-color: #f8d7da;
    border: 2px solid #dc3545;
    color: #721c24;
}

.validation-warning {
    background-color: #fff3cd;
    border: 2px solid #ffc107;
    color: #856404;
}

.validation-icon {
    font-size: 32px;
    flex-shrink: 0;
}

.validation-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.validation-message {
    font-size: 18px;
    font-weight: 600;
}

.validation-details {
    font-size: 14px;
    opacity: 0.9;
}

/* Kid-friendly styles - larger and more colorful */
.validation-feedback.kid-friendly {
    padding: 20px;
    font-size: 20px;
}

.validation-feedback.kid-friendly .validation-icon {
    font-size: 48px;
}

.validation-feedback.kid-friendly .validation-message {
    font-size: 22px;
}

.validation-feedback.kid-friendly .validation-details {
    font-size: 16px;
}
"#
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::Color;

    #[test]
    fn test_get_error_info_invalid_color_count() {
        let error = ValidationError::InvalidColorCount {
            color: Color::White,
            expected: 9,
            actual: 10,
        };
        let (severity, icon, message, details) = get_error_info(&error);

        assert_eq!(severity, "error");
        assert_eq!(icon, "⚠");
        assert!(message.contains("White"));
        assert!(details.is_some());
        let detail_text = details.unwrap();
        assert!(detail_text.contains("10"));
        assert!(detail_text.contains("9"));
    }

    #[test]
    fn test_get_error_info_missing_colors() {
        let error = ValidationError::MissingColors {
            missing: vec![Color::Red, Color::Blue],
        };
        let (severity, icon, message, _) = get_error_info(&error);

        assert_eq!(severity, "error");
        assert_eq!(icon, "⚠");
        assert!(message.contains("missing"));
    }

    #[test]
    fn test_get_error_info_edge_parity() {
        let error = ValidationError::EdgeParity;
        let (severity, icon, message, details) = get_error_info(&error);

        assert_eq!(severity, "error");
        assert_eq!(icon, "⚠");
        assert!(message.contains("Edge"));
        assert!(details.is_some());
        assert!(details.unwrap().contains("parity"));
    }

    #[test]
    fn test_get_error_info_corner_parity() {
        let error = ValidationError::CornerParity;
        let (severity, icon, message, details) = get_error_info(&error);

        assert_eq!(severity, "error");
        assert_eq!(icon, "⚠");
        assert!(message.contains("Corner"));
        assert!(details.is_some());
    }

    #[test]
    fn test_get_error_info_permutation_parity() {
        let error = ValidationError::PermutationParity;
        let (severity, icon, message, _) = get_error_info(&error);

        assert_eq!(severity, "error");
        assert_eq!(icon, "⚠");
        assert!(message.contains("Pieces"));
    }

    #[test]
    fn test_kid_friendly_message_invalid_color_count() {
        let error = ValidationError::InvalidColorCount {
            color: Color::Yellow,
            expected: 9,
            actual: 8,
        };
        let message = get_kid_friendly_message(&error);

        assert!(message.contains("Oops"));
        assert!(message.contains("Yellow"));
    }

    #[test]
    fn test_kid_friendly_message_missing_colors() {
        let error = ValidationError::MissingColors {
            missing: vec![Color::Green],
        };
        let message = get_kid_friendly_message(&error);

        assert!(message.contains("Oops"));
        assert!(message.contains("missing"));
    }

    #[test]
    fn test_kid_friendly_message_parity_errors() {
        let errors = vec![
            ValidationError::EdgeParity,
            ValidationError::CornerParity,
            ValidationError::PermutationParity,
        ];

        for error in errors {
            let message = get_kid_friendly_message(&error);
            assert!(message.contains("can't be solved"));
            assert!(message.contains("reset"));
        }
    }

    #[test]
    fn test_validation_styles_not_empty() {
        let styles = get_validation_styles();
        assert!(!styles.is_empty());
        assert!(styles.contains(".validation-feedback"));
        assert!(styles.contains(".validation-success"));
        assert!(styles.contains(".validation-error"));
    }
}
