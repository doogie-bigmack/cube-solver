use rubiks_cube_solver::components::validation_feedback::{
    get_kid_friendly_message, get_validation_styles,
};
use rubiks_cube_solver::cube::validation::ValidationError;
use rubiks_cube_solver::cube::Color;

// Test: Validation styles are comprehensive
#[test]
fn test_validation_styles_complete() {
    let styles = get_validation_styles();

    // Check all required classes are present
    assert!(styles.contains(".validation-feedback"));
    assert!(styles.contains(".validation-success"));
    assert!(styles.contains(".validation-error"));
    assert!(styles.contains(".validation-warning"));
    assert!(styles.contains(".validation-icon"));
    assert!(styles.contains(".validation-content"));
    assert!(styles.contains(".validation-message"));
    assert!(styles.contains(".validation-details"));

    // Check kid-friendly styles
    assert!(styles.contains(".kid-friendly"));
}

// Test: Invalid color count - kid-friendly message
#[test]
fn test_kid_friendly_invalid_color_count_white() {
    let error = ValidationError::InvalidColorCount {
        color: Color::White,
        expected: 9,
        actual: 10,
    };

    let message = get_kid_friendly_message(&error);

    assert!(message.contains("Oops"));
    assert!(message.contains("White"));
    assert!(message.len() > 10); // Non-empty, helpful message
}

// Test: Invalid color count - kid-friendly message (Yellow)
#[test]
fn test_kid_friendly_invalid_color_count_yellow() {
    let error = ValidationError::InvalidColorCount {
        color: Color::Yellow,
        expected: 9,
        actual: 8,
    };

    let message = get_kid_friendly_message(&error);

    assert!(message.contains("Oops"));
    assert!(message.contains("Yellow"));
}

// Test: Missing colors - kid-friendly message
#[test]
fn test_kid_friendly_missing_colors() {
    let error = ValidationError::MissingColors {
        missing: vec![Color::Red, Color::Blue],
    };

    let message = get_kid_friendly_message(&error);

    assert!(message.contains("Oops") || message.contains("oops"));
    assert!(message.contains("missing") || message.contains("Missing"));
    assert!(message.contains("6 colors") || message.contains("all"));
}

// Test: Edge parity - kid-friendly message
#[test]
fn test_kid_friendly_edge_parity() {
    let error = ValidationError::EdgeParity;
    let message = get_kid_friendly_message(&error);

    assert!(message.contains("can't be solved") || message.contains("cannot be solved"));
    assert!(message.contains("reset") || message.contains("Reset"));
}

// Test: Corner parity - kid-friendly message
#[test]
fn test_kid_friendly_corner_parity() {
    let error = ValidationError::CornerParity;
    let message = get_kid_friendly_message(&error);

    assert!(message.contains("can't be solved") || message.contains("cannot be solved"));
    assert!(message.contains("reset") || message.contains("Reset"));
}

// Test: Permutation parity - kid-friendly message
#[test]
fn test_kid_friendly_permutation_parity() {
    let error = ValidationError::PermutationParity;
    let message = get_kid_friendly_message(&error);

    assert!(message.contains("can't be solved") || message.contains("cannot be solved"));
    assert!(message.contains("reset") || message.contains("Reset"));
}

// Test: All colors have kid-friendly messages
#[test]
fn test_kid_friendly_all_colors() {
    let colors = vec![
        Color::White,
        Color::Yellow,
        Color::Red,
        Color::Orange,
        Color::Blue,
        Color::Green,
    ];

    for color in colors {
        let error = ValidationError::InvalidColorCount {
            color,
            expected: 9,
            actual: 10,
        };

        let message = get_kid_friendly_message(&error);

        // Each should have a friendly message
        assert!(!message.is_empty());
        assert!(message.len() > 20); // Should be reasonably descriptive
    }
}

// Test: Validation styles have proper CSS structure
#[test]
fn test_validation_styles_css_structure() {
    let styles = get_validation_styles();

    // Should contain CSS property declarations
    assert!(styles.contains("display"));
    assert!(styles.contains("padding"));
    assert!(styles.contains("border"));
    assert!(styles.contains("background-color"));
    assert!(styles.contains("color"));

    // Should have colors for each state
    assert!(styles.contains("#d4edda")); // Success green
    assert!(styles.contains("#f8d7da")); // Error red
    assert!(styles.contains("#fff3cd")); // Warning yellow
}

// Test: Missing colors with single color
#[test]
fn test_kid_friendly_missing_single_color() {
    let error = ValidationError::MissingColors {
        missing: vec![Color::Green],
    };

    let message = get_kid_friendly_message(&error);

    assert!(message.contains("Oops") || message.contains("oops"));
    assert!(!message.is_empty());
}

// Test: Missing colors with all colors
#[test]
fn test_kid_friendly_missing_all_colors() {
    let error = ValidationError::MissingColors {
        missing: vec![
            Color::White,
            Color::Yellow,
            Color::Red,
            Color::Orange,
            Color::Blue,
            Color::Green,
        ],
    };

    let message = get_kid_friendly_message(&error);

    assert!(message.contains("missing") || message.contains("Missing"));
    assert!(!message.is_empty());
}

// Test: Validation styles for kid-friendly mode
#[test]
fn test_validation_styles_kid_friendly() {
    let styles = get_validation_styles();

    // Should have larger font sizes for kids
    assert!(styles.contains("font-size: 20px") || styles.contains("font-size:20px"));
    assert!(styles.contains("font-size: 48px") || styles.contains("font-size:48px"));

    // Should have larger padding
    assert!(styles.contains("padding: 20px") || styles.contains("padding:20px"));
}

// Test: Error messages are clear and actionable
#[test]
fn test_all_error_messages_are_actionable() {
    let errors = vec![
        ValidationError::InvalidColorCount {
            color: Color::White,
            expected: 9,
            actual: 10,
        },
        ValidationError::MissingColors {
            missing: vec![Color::Red],
        },
        ValidationError::EdgeParity,
        ValidationError::CornerParity,
        ValidationError::PermutationParity,
    ];

    for error in errors {
        let message = get_kid_friendly_message(&error);

        // Should not be empty
        assert!(!message.is_empty());

        // Should be longer than a simple "Error"
        assert!(message.len() > 15);

        // Should be kid-friendly (avoid technical jargon in most cases)
        // or provide clear action (like "reset")
        assert!(
            message.contains("Oops")
                || message.contains("Try")
                || message.contains("reset")
                || message.contains("fix")
        );
    }
}

// Test: CSS includes accessibility features
#[test]
fn test_validation_styles_accessibility() {
    let styles = get_validation_styles();

    // Should have clear visual indicators (borders)
    assert!(styles.contains("border"));

    // Should have color contrast (using distinct colors)
    assert!(styles.contains("#155724")); // Dark green text
    assert!(styles.contains("#721c24")); // Dark red text
    assert!(styles.contains("#856404")); // Dark yellow text
}
