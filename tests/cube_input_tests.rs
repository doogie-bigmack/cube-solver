//! Integration tests for 2D Unfolded Cube View (R3.1)
//!
//! Test IDs: cube_input_* (mapped to R3.1 requirement)

use rubiks_cube_solver::cube::Cube;
use rubiks_cube_solver::components::UnfoldedLayout;

#[test]
fn test_cube_input_001_layout_2x2() {
    // Create 2x2 cube
    let cube = Cube::new(2);
    let layout = UnfoldedLayout::for_cube_size(cube.size());

    assert_eq!(cube.size(), 2);
    assert_eq!(layout.sticker_size, 40.0);

    // Face width should be 2 stickers + 1 gap
    let expected_width = 40.0 * 2.0 + 40.0 * 0.05;
    assert_eq!(layout.face_width(2), expected_width);
}

#[test]
fn test_cube_input_002_layout_3x3() {
    // Create 3x3 cube
    let cube = Cube::new(3);
    let layout = UnfoldedLayout::for_cube_size(cube.size());

    assert_eq!(cube.size(), 3);
    assert_eq!(layout.sticker_size, 30.0);

    // Face width should be 3 stickers + 2 gaps
    let expected_width = 30.0 * 3.0 + 2.0 * (30.0 * 0.05);
    assert_eq!(layout.face_width(3), expected_width);
}

#[test]
fn test_cube_input_003_layout_4x4() {
    // Create 4x4 cube
    let cube = Cube::new(4);
    let layout = UnfoldedLayout::for_cube_size(cube.size());

    assert_eq!(cube.size(), 4);
    assert_eq!(layout.sticker_size, 24.0);
}

#[test]
fn test_cube_input_004_layout_5x5() {
    // Create 5x5 cube
    let cube = Cube::new(5);
    let layout = UnfoldedLayout::for_cube_size(cube.size());

    assert_eq!(cube.size(), 5);
    assert_eq!(layout.sticker_size, 20.0);
}

#[test]
fn test_cube_input_005_layout_10x10() {
    // Create 10x10 cube
    let cube = Cube::new(10);
    let layout = UnfoldedLayout::for_cube_size(cube.size());

    assert_eq!(cube.size(), 10);
    assert_eq!(layout.sticker_size, 12.0);
}

#[test]
fn test_cube_input_006_layout_20x20() {
    // Create 20x20 cube
    let cube = Cube::new(20);
    let layout = UnfoldedLayout::for_cube_size(cube.size());

    assert_eq!(cube.size(), 20);
    assert_eq!(layout.sticker_size, 10.0);
}

#[test]
fn test_cube_input_007_all_sizes_have_valid_layout() {
    // Test that all supported cube sizes (2-20) have valid layouts
    for size in 2..=20 {
        let cube = Cube::new(size);
        let layout = UnfoldedLayout::for_cube_size(cube.size());

        // Verify all layout properties are positive
        assert!(layout.sticker_size > 0.0, "Size {} has invalid sticker_size", size);
        assert!(layout.gap >= 0.0, "Size {} has invalid gap", size);
        assert!(layout.face_gap >= 0.0, "Size {} has invalid face_gap", size);
        assert!(layout.label_font_size > 0.0, "Size {} has invalid label_font_size", size);

        // Verify face dimensions are positive
        assert!(layout.face_width(size) > 0.0, "Size {} has invalid face_width", size);
        assert!(layout.face_height(size) > 0.0, "Size {} has invalid face_height", size);
    }
}

#[test]
fn test_cube_input_008_gap_proportions() {
    // Test that gaps are proportional to sticker size
    for size in [2, 3, 4, 5, 10, 20] {
        let layout = UnfoldedLayout::for_cube_size(size);

        // Gap should be 5% of sticker size
        let expected_gap = layout.sticker_size * 0.05;
        assert_eq!(layout.gap, expected_gap, "Size {} has incorrect gap proportion", size);

        // Face gap should be 30% of sticker size
        let expected_face_gap = layout.sticker_size * 0.3;
        assert_eq!(layout.face_gap, expected_face_gap, "Size {} has incorrect face_gap proportion", size);

        // Label font should be 80% of sticker size
        let expected_label = layout.sticker_size * 0.8;
        assert_eq!(layout.label_font_size, expected_label, "Size {} has incorrect label_font_size proportion", size);
    }
}

#[test]
fn test_cube_input_009_sticker_size_scaling() {
    // Test that sticker size decreases as cube size increases
    let sizes = vec![2, 3, 4, 5, 7, 10, 15, 20];
    let layouts: Vec<_> = sizes.iter().map(|&s| UnfoldedLayout::for_cube_size(s)).collect();

    for i in 1..layouts.len() {
        assert!(
            layouts[i-1].sticker_size >= layouts[i].sticker_size,
            "Sticker size should decrease or stay same as cube size increases: {}x{} ({}) vs {}x{} ({})",
            sizes[i-1], sizes[i-1], layouts[i-1].sticker_size,
            sizes[i], sizes[i], layouts[i].sticker_size
        );
    }
}

#[test]
fn test_cube_input_010_solved_cube_dimensions() {
    // Test layout dimensions for solved cubes
    let cube_3x3 = Cube::new(3);
    let layout = UnfoldedLayout::for_cube_size(3);

    // Total width should fit 4 faces horizontally + 3 gaps
    let face_width = layout.face_width(3);
    let total_width = 4.0 * face_width + 3.0 * layout.face_gap;
    assert!(total_width > 0.0);

    // Total height should fit 3 faces vertically + 2 gaps
    let face_height = layout.face_height(3);
    let total_height = 3.0 * face_height + 2.0 * layout.face_gap;
    assert!(total_height > 0.0);

    // Dimensions should be reasonable (not too large or too small)
    assert!(total_width >= 100.0 && total_width <= 2000.0,
            "Total width {} is unreasonable", total_width);
    assert!(total_height >= 100.0 && total_height <= 2000.0,
            "Total height {} is unreasonable", total_height);
}

#[test]
fn test_cube_input_011_face_width_equals_height() {
    // Test that face width and height are equal (square faces)
    for size in 2..=20 {
        let layout = UnfoldedLayout::for_cube_size(size);
        assert_eq!(
            layout.face_width(size),
            layout.face_height(size),
            "Face should be square for size {}", size
        );
    }
}

#[test]
fn test_cube_input_012_layout_consistency() {
    // Test that the same size always produces the same layout
    for size in [2, 3, 5, 10] {
        let layout1 = UnfoldedLayout::for_cube_size(size);
        let layout2 = UnfoldedLayout::for_cube_size(size);

        assert_eq!(layout1.sticker_size, layout2.sticker_size);
        assert_eq!(layout1.gap, layout2.gap);
        assert_eq!(layout1.face_gap, layout2.face_gap);
        assert_eq!(layout1.label_font_size, layout2.label_font_size);
    }
}

#[test]
fn test_cube_input_013_cross_pattern_layout_total_dimensions() {
    // Test the total dimensions for cross pattern layout
    // Layout:
    //     [U]
    // [L] [F] [R] [B]
    //     [D]

    let cube = Cube::new(3);
    let layout = UnfoldedLayout::for_cube_size(3);

    let face_w = layout.face_width(3);
    let face_h = layout.face_height(3);

    // Width: 4 faces + 3 gaps
    let total_width = 4.0 * face_w + 3.0 * layout.face_gap;

    // Height: 3 faces + 2 gaps
    let total_height = 3.0 * face_h + 2.0 * layout.face_gap;

    // Verify dimensions make sense
    assert!(total_width > total_height * 0.8, "Width should be wider than height for cross pattern");

    // The cross pattern should be wider than tall
    let aspect_ratio = total_width / total_height;
    assert!(aspect_ratio > 1.0 && aspect_ratio < 2.0,
            "Aspect ratio {} is outside expected range", aspect_ratio);
}

#[test]
fn test_cube_input_014_minimum_sizes() {
    // Test minimum cube size (2x2)
    let cube = Cube::new(2);
    let layout = UnfoldedLayout::for_cube_size(2);

    // 2x2 should have larger stickers
    assert_eq!(layout.sticker_size, 40.0);
    assert!(layout.sticker_size > 30.0);
}

#[test]
fn test_cube_input_015_maximum_sizes() {
    // Test maximum cube size (20x20)
    let cube = Cube::new(20);
    let layout = UnfoldedLayout::for_cube_size(20);

    // 20x20 should have smaller stickers
    assert_eq!(layout.sticker_size, 10.0);
    assert!(layout.sticker_size < 15.0);
}
