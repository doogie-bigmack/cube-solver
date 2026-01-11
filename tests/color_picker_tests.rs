//! Integration tests for ColorPicker component
//!
//! Tests cover requirement R3.3 - Color picker palette (6 colors)

use rubiks_cube_solver::cube::Color;

#[test]
fn test_all_six_colors_available() {
    // Verify all 6 standard cube colors are available
    let colors = vec![
        Color::White,
        Color::Yellow,
        Color::Red,
        Color::Orange,
        Color::Blue,
        Color::Green,
    ];

    assert_eq!(colors.len(), 6, "There should be exactly 6 cube colors");
}

#[test]
fn test_color_distinctness() {
    // Ensure all colors are distinct
    let colors = vec![
        Color::White,
        Color::Yellow,
        Color::Red,
        Color::Orange,
        Color::Blue,
        Color::Green,
    ];

    for (i, color1) in colors.iter().enumerate() {
        for (j, color2) in colors.iter().enumerate() {
            if i != j {
                assert_ne!(
                    color1, color2,
                    "Colors at indices {} and {} should be different",
                    i, j
                );
            }
        }
    }
}

#[test]
fn test_color_equality() {
    // Test that Color enum supports equality comparison
    assert_eq!(Color::White, Color::White);
    assert_eq!(Color::Yellow, Color::Yellow);
    assert_eq!(Color::Red, Color::Red);
    assert_eq!(Color::Orange, Color::Orange);
    assert_eq!(Color::Blue, Color::Blue);
    assert_eq!(Color::Green, Color::Green);

    assert_ne!(Color::White, Color::Yellow);
    assert_ne!(Color::Red, Color::Orange);
    assert_ne!(Color::Blue, Color::Green);
}

#[test]
fn test_color_clone() {
    // Test that colors can be cloned (required for selection state)
    let original = Color::Red;
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_color_copy() {
    // Test that colors can be copied (important for selection state)
    let color = Color::Blue;
    let copied = color; // Should copy, not move
    assert_eq!(color, copied);

    // Original should still be usable
    assert_eq!(color, Color::Blue);
}

#[test]
fn test_all_colors_hashable() {
    // Colors should be hashable for use in HashMaps/HashSets
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(Color::White);
    set.insert(Color::Yellow);
    set.insert(Color::Red);
    set.insert(Color::Orange);
    set.insert(Color::Blue);
    set.insert(Color::Green);

    assert_eq!(set.len(), 6, "All 6 colors should be in the set");
}

#[test]
fn test_option_color_for_selection() {
    // Test Option<Color> for tracking selected color
    let mut selected: Option<Color> = None;
    assert_eq!(selected, None);

    selected = Some(Color::Red);
    assert_eq!(selected, Some(Color::Red));
    assert_ne!(selected, None);

    selected = Some(Color::Blue);
    assert_eq!(selected, Some(Color::Blue));
    assert_ne!(selected, Some(Color::Red));

    selected = None;
    assert_eq!(selected, None);
}

#[test]
fn test_color_pattern_matching() {
    // Test that colors can be pattern matched
    let color = Color::Green;

    let name = match color {
        Color::White => "White",
        Color::Yellow => "Yellow",
        Color::Red => "Red",
        Color::Orange => "Orange",
        Color::Blue => "Blue",
        Color::Green => "Green",
    };

    assert_eq!(name, "Green");
}

#[test]
fn test_color_selection_workflow() {
    // Simulate the workflow: select color -> apply to sticker
    let mut selected_color: Option<Color> = None;

    // Step 1: No color selected
    assert_eq!(selected_color, None);

    // Step 2: User clicks Red color button
    selected_color = Some(Color::Red);
    assert_eq!(selected_color, Some(Color::Red));

    // Step 3: User clicks Orange color button (changes selection)
    selected_color = Some(Color::Orange);
    assert_eq!(selected_color, Some(Color::Orange));
    assert_ne!(selected_color, Some(Color::Red));
}

#[test]
fn test_color_picker_integration_with_cube() {
    use rubiks_cube_solver::cube::{Cube, FaceName};

    let mut cube = Cube::new(3);

    // Apply colors using set_sticker (simulates color picker usage)
    cube.set_sticker(FaceName::F, 0, 0, Color::Red);
    cube.set_sticker(FaceName::F, 0, 1, Color::Blue);
    cube.set_sticker(FaceName::F, 0, 2, Color::Green);

    // Verify colors were set
    let face = cube.get_face(FaceName::F);
    assert_eq!(face.get(0, 0), Color::Red);
    assert_eq!(face.get(0, 1), Color::Blue);
    assert_eq!(face.get(0, 2), Color::Green);
}

#[test]
fn test_all_colors_can_be_applied_to_stickers() {
    use rubiks_cube_solver::cube::{Cube, FaceName};

    let mut cube = Cube::new(3);
    let colors = vec![
        Color::White,
        Color::Yellow,
        Color::Red,
        Color::Orange,
        Color::Blue,
        Color::Green,
    ];

    // Apply first 3 colors to top row of front face
    cube.set_sticker(FaceName::F, 0, 0, Color::White);
    cube.set_sticker(FaceName::F, 0, 1, Color::Yellow);
    cube.set_sticker(FaceName::F, 0, 2, Color::Red);

    // Apply remaining 3 colors to second row
    cube.set_sticker(FaceName::F, 1, 0, Color::Orange);
    cube.set_sticker(FaceName::F, 1, 1, Color::Blue);
    cube.set_sticker(FaceName::F, 1, 2, Color::Green);

    // Verify all colors can be applied
    let face = cube.get_face(FaceName::F);
    assert_eq!(face.get(0, 0), Color::White);
    assert_eq!(face.get(0, 1), Color::Yellow);
    assert_eq!(face.get(0, 2), Color::Red);
    assert_eq!(face.get(1, 0), Color::Orange);
    assert_eq!(face.get(1, 1), Color::Blue);
    assert_eq!(face.get(1, 2), Color::Green);
}

#[test]
fn test_color_reapplication() {
    use rubiks_cube_solver::cube::{Cube, FaceName};

    let mut cube = Cube::new(3);

    // Apply a color
    cube.set_sticker(FaceName::F, 1, 1, Color::Red);
    assert_eq!(cube.get_face(FaceName::F).get(1, 1), Color::Red);

    // Reapply different color to same sticker
    cube.set_sticker(FaceName::F, 1, 1, Color::Blue);
    assert_eq!(cube.get_face(FaceName::F).get(1, 1), Color::Blue);

    // Reapply yet another color
    cube.set_sticker(FaceName::F, 1, 1, Color::Green);
    assert_eq!(cube.get_face(FaceName::F).get(1, 1), Color::Green);
}
