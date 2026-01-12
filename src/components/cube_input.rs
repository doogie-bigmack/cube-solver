//! 2D Unfolded Cube View Component
//!
//! This module provides a 2D unfolded cube view in a cross pattern layout,
//! allowing users to manually input cube colors by clicking on stickers.
//!
//! Requirements: R3.1 - 2D unfolded cube view
//! - Display cube as unfolded cross pattern
//! - Show all 6 faces
//! - Clear face labels (U, D, L, R, F, B)
//! - Scale appropriately for cube size
//!
//! Requirements: R3.2 - Click/tap to select sticker
//! - Click any sticker to select it
//! - Visual selection indicator
//! - Touch support for mobile

use dioxus::prelude::*;
use crate::cube::{Cube, Color, FaceName};

/// Represents a selected sticker position
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StickerPosition {
    pub face: FaceName,
    pub row: usize,
    pub col: usize,
}

/// Props for the CubeInput component
#[derive(Clone, PartialEq, Props)]
pub struct CubeInputProps {
    /// The cube to display
    pub cube: Cube,
    /// Optional callback when a sticker is clicked (face, row, col)
    #[props(optional)]
    pub on_sticker_click: Option<EventHandler<(FaceName, usize, usize)>>,
    /// Currently selected sticker (if any)
    #[props(optional)]
    pub selected_sticker: Option<StickerPosition>,
}

/// Layout configuration for the unfolded cube
#[derive(Clone, Debug, PartialEq)]
pub struct UnfoldedLayout {
    /// Size of each sticker in pixels
    pub sticker_size: f32,
    /// Gap between stickers in pixels
    pub gap: f32,
    /// Gap between faces in pixels
    pub face_gap: f32,
    /// Font size for face labels
    pub label_font_size: f32,
}

impl UnfoldedLayout {
    /// Create layout for a given cube size
    pub fn for_cube_size(size: usize) -> Self {
        // Scale sticker size based on cube size
        let sticker_size = match size {
            2 => 40.0,
            3 => 30.0,
            4 => 24.0,
            5 => 20.0,
            n if n <= 7 => 16.0,
            n if n <= 10 => 12.0,
            _ => 10.0,
        };

        Self {
            sticker_size,
            gap: sticker_size * 0.05,
            face_gap: sticker_size * 0.3,
            label_font_size: sticker_size * 0.8,
        }
    }

    /// Calculate total width of a face including gaps
    pub fn face_width(&self, cube_size: usize) -> f32 {
        (cube_size as f32) * self.sticker_size + ((cube_size - 1) as f32) * self.gap
    }

    /// Calculate total height of a face including gaps
    pub fn face_height(&self, cube_size: usize) -> f32 {
        self.face_width(cube_size)
    }
}

/// Position of a face in the unfolded layout (in face units)
#[derive(Clone, Copy, Debug, PartialEq)]
struct FacePosition {
    row: usize,
    col: usize,
}

impl FacePosition {
    /// Get position for each face in the cross layout
    /// Layout:
    ///     [U]
    /// [L] [F] [R] [B]
    ///     [D]
    fn for_face(face: FaceName) -> Self {
        match face {
            FaceName::U => FacePosition { row: 0, col: 1 },
            FaceName::L => FacePosition { row: 1, col: 0 },
            FaceName::F => FacePosition { row: 1, col: 1 },
            FaceName::R => FacePosition { row: 1, col: 2 },
            FaceName::B => FacePosition { row: 1, col: 3 },
            FaceName::D => FacePosition { row: 2, col: 1 },
        }
    }
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

/// 2D Unfolded Cube View Component
#[component]
pub fn CubeInput(props: CubeInputProps) -> Element {
    let cube = &props.cube;
    let layout = UnfoldedLayout::for_cube_size(cube.size());

    // Calculate total dimensions
    let face_width = layout.face_width(cube.size());
    let face_height = layout.face_height(cube.size());
    let total_width = 4.0 * face_width + 3.0 * layout.face_gap;
    let total_height = 3.0 * face_height + 2.0 * layout.face_gap;

    // Container style
    let container_style = format!(
        "position: relative; width: {}px; height: {}px; margin: 0 auto;",
        total_width, total_height
    );

    rsx! {
        div {
            class: "cube-input-container",
            style: "{container_style}",

            // Render all 6 faces in cross pattern
            for face_name in [
                FaceName::U,
                FaceName::L,
                FaceName::F,
                FaceName::R,
                FaceName::B,
                FaceName::D,
            ] {
                {render_face(&cube, face_name, &layout, props.on_sticker_click.clone(), props.selected_sticker)}
            }
        }
    }
}

/// Render a single face of the cube
fn render_face(
    cube: &Cube,
    face_name: FaceName,
    layout: &UnfoldedLayout,
    on_sticker_click: Option<EventHandler<(FaceName, usize, usize)>>,
    selected_sticker: Option<StickerPosition>,
) -> Element {
    let pos = FacePosition::for_face(face_name);
    let face_width = layout.face_width(cube.size());
    let face_height = layout.face_height(cube.size());

    // Calculate position in pixels
    let left = pos.col as f32 * (face_width + layout.face_gap);
    let top = pos.row as f32 * (face_height + layout.face_gap);

    let face_style = format!(
        "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px;",
        left, top, face_width, face_height
    );

    let face = cube.get_face(face_name);
    let face_label = format!("{:?}", face_name).chars().next().unwrap();

    rsx! {
        div {
            class: "face-container",
            style: "{face_style}",

            // Face label
            div {
                class: "face-label",
                style: format!(
                    "position: absolute; top: -{}px; left: 50%; transform: translateX(-50%); \
                     font-size: {}px; font-weight: bold; color: #4a5568;",
                    layout.label_font_size + 4.0,
                    layout.label_font_size
                ),
                "{face_label}"
            }

            // Stickers grid
            for row in 0..cube.size() {
                for col in 0..cube.size() {
                    {render_sticker(face, row, col, face_name, layout, on_sticker_click.clone(), selected_sticker)}
                }
            }
        }
    }
}

/// Render a single sticker
fn render_sticker(
    face: &crate::cube::state::Face,
    row: usize,
    col: usize,
    face_name: FaceName,
    layout: &UnfoldedLayout,
    on_sticker_click: Option<EventHandler<(FaceName, usize, usize)>>,
    selected_sticker: Option<StickerPosition>,
) -> Element {
    let color = face.get(row, col);
    let color_css = color_to_css(color);

    let left = col as f32 * (layout.sticker_size + layout.gap);
    let top = row as f32 * (layout.sticker_size + layout.gap);

    // Check if this sticker is selected
    let is_selected = selected_sticker.map_or(false, |sel| {
        sel.face == face_name && sel.row == row && sel.col == col
    });

    // Add visual indicator for selected sticker
    let sticker_style = if is_selected {
        format!(
            "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px; \
             background: {}; border: 4px solid #3182ce; border-radius: 3px; \
             cursor: pointer; transition: transform 0.1s, box-shadow 0.1s; \
             transform: scale(1.1); box-shadow: 0 0 10px rgba(49, 130, 206, 0.6); \
             z-index: 10; touch-action: manipulation;",
            left, top, layout.sticker_size, layout.sticker_size, color_css
        )
    } else {
        format!(
            "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px; \
             background: {}; border: 1px solid #2d3748; border-radius: 3px; \
             cursor: pointer; transition: transform 0.1s, box-shadow 0.1s; touch-action: manipulation;",
            left, top, layout.sticker_size, layout.sticker_size, color_css
        )
    };

    rsx! {
        div {
            class: if is_selected { "sticker selected" } else { "sticker" },
            style: "{sticker_style}",
            onmouseenter: move |_| {},
            onclick: move |_| {
                if let Some(ref handler) = on_sticker_click {
                    handler.call((face_name, row, col));
                }
            },
            // Touch events - onclick handles both mouse and touch
            ontouchstart: move |_| {},
            title: format!("{:?} face ({}, {})", face_name, row, col),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unfolded_layout_2x2() {
        let layout = UnfoldedLayout::for_cube_size(2);
        assert_eq!(layout.sticker_size, 40.0);
        assert_eq!(layout.face_width(2), 40.0 * 2.0 + 40.0 * 0.05);
    }

    #[test]
    fn test_unfolded_layout_3x3() {
        let layout = UnfoldedLayout::for_cube_size(3);
        assert_eq!(layout.sticker_size, 30.0);
        let expected_width = 30.0 * 3.0 + 2.0 * (30.0 * 0.05);
        assert_eq!(layout.face_width(3), expected_width);
    }

    #[test]
    fn test_unfolded_layout_scaling() {
        let layout_2x2 = UnfoldedLayout::for_cube_size(2);
        let layout_5x5 = UnfoldedLayout::for_cube_size(5);
        let layout_10x10 = UnfoldedLayout::for_cube_size(10);

        // Larger cubes should have smaller stickers
        assert!(layout_2x2.sticker_size > layout_5x5.sticker_size);
        assert!(layout_5x5.sticker_size > layout_10x10.sticker_size);
    }

    #[test]
    fn test_face_positions() {
        // Test cross pattern layout
        let up = FacePosition::for_face(FaceName::U);
        let left = FacePosition::for_face(FaceName::L);
        let front = FacePosition::for_face(FaceName::F);
        let right = FacePosition::for_face(FaceName::R);
        let back = FacePosition::for_face(FaceName::B);
        let down = FacePosition::for_face(FaceName::D);

        // Up should be above front
        assert_eq!(up.row, 0);
        assert_eq!(up.col, 1);

        // Left, Front, Right, Back should be in middle row
        assert_eq!(left.row, 1);
        assert_eq!(front.row, 1);
        assert_eq!(right.row, 1);
        assert_eq!(back.row, 1);

        // They should be in correct order
        assert_eq!(left.col, 0);
        assert_eq!(front.col, 1);
        assert_eq!(right.col, 2);
        assert_eq!(back.col, 3);

        // Down should be below front
        assert_eq!(down.row, 2);
        assert_eq!(down.col, 1);
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
    fn test_layout_gap_proportions() {
        let layout = UnfoldedLayout::for_cube_size(3);

        // Gap should be 5% of sticker size
        assert_eq!(layout.gap, layout.sticker_size * 0.05);

        // Face gap should be 30% of sticker size
        assert_eq!(layout.face_gap, layout.sticker_size * 0.3);

        // Label font should be 80% of sticker size
        assert_eq!(layout.label_font_size, layout.sticker_size * 0.8);
    }

    #[test]
    fn test_face_dimensions_symmetry() {
        let layout = UnfoldedLayout::for_cube_size(3);

        // Face width and height should be equal for square faces
        assert_eq!(layout.face_width(3), layout.face_height(3));
    }

    #[test]
    fn test_layout_for_various_sizes() {
        // Test that layout can be created for all supported sizes
        for size in 2..=20 {
            let layout = UnfoldedLayout::for_cube_size(size);
            assert!(layout.sticker_size > 0.0);
            assert!(layout.gap >= 0.0);
            assert!(layout.face_gap >= 0.0);
        }
    }
}
