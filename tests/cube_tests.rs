//! Cube State Tests for R1.1
//!
//! Tests for NxN cube state representation as specified in test-plan.md

use rubiks_cube_solver::cube::{Color, Cube, Face};

/// cube_001: Create 2x2 cube in solved state
#[test]
fn cube_001_create_2x2_solved() {
    let cube = Cube::new(2);
    assert_eq!(cube.size(), 2);
    assert!(cube.is_solved());
    assert!(cube.has_valid_color_counts());
}

/// cube_002: Create 3x3 cube in solved state
#[test]
fn cube_002_create_3x3_solved() {
    let cube = Cube::new(3);
    assert_eq!(cube.size(), 3);
    assert!(cube.is_solved());
    assert!(cube.has_valid_color_counts());
}

/// cube_003: Create 5x5 cube in solved state
#[test]
fn cube_003_create_5x5_solved() {
    let cube = Cube::new(5);
    assert_eq!(cube.size(), 5);
    assert!(cube.is_solved());
    assert!(cube.has_valid_color_counts());
}

/// cube_004: Create 10x10 cube in solved state
#[test]
fn cube_004_create_10x10_solved() {
    let cube = Cube::new(10);
    assert_eq!(cube.size(), 10);
    assert!(cube.is_solved());
    assert!(cube.has_valid_color_counts());
}

/// cube_005: Create 20x20 cube in solved state
#[test]
fn cube_005_create_20x20_solved() {
    let cube = Cube::new(20);
    assert_eq!(cube.size(), 20);
    assert!(cube.is_solved());
    assert!(cube.has_valid_color_counts());
}

/// Test that all intermediate sizes work (comprehensive coverage)
#[test]
fn test_all_cube_sizes() {
    for size in 2..=20 {
        let cube = Cube::new(size);
        assert_eq!(cube.size(), size);
        assert!(cube.is_solved(), "Cube of size {} should be solved", size);
        assert!(
            cube.has_valid_color_counts(),
            "Cube of size {} should have valid color counts",
            size
        );
    }
}

/// Test that face has correct NxN grid
#[test]
fn test_face_grid_size() {
    for size in 2..=20 {
        let face = Face::new(size, Color::White);
        assert_eq!(face.size(), size);
        assert_eq!(face.stickers().len(), size);
        for row in face.stickers() {
            assert_eq!(row.len(), size);
        }
    }
}

/// Test Colors enum has all 6 standard colors
#[test]
fn test_colors_enum() {
    let colors = [
        Color::White,
        Color::Yellow,
        Color::Red,
        Color::Orange,
        Color::Blue,
        Color::Green,
    ];

    // All colors should be distinct
    for i in 0..colors.len() {
        for j in (i + 1)..colors.len() {
            assert_ne!(colors[i], colors[j]);
        }
    }
}

/// Test that each face in a new cube has the correct color
#[test]
fn test_standard_face_colors() {
    let cube = Cube::new(3);

    // Up face should be all white
    for row in 0..3 {
        for col in 0..3 {
            assert_eq!(cube.up.get(row, col), Color::White);
        }
    }

    // Down face should be all yellow
    for row in 0..3 {
        for col in 0..3 {
            assert_eq!(cube.down.get(row, col), Color::Yellow);
        }
    }

    // Front face should be all green
    for row in 0..3 {
        for col in 0..3 {
            assert_eq!(cube.front.get(row, col), Color::Green);
        }
    }

    // Back face should be all blue
    for row in 0..3 {
        for col in 0..3 {
            assert_eq!(cube.back.get(row, col), Color::Blue);
        }
    }

    // Left face should be all orange
    for row in 0..3 {
        for col in 0..3 {
            assert_eq!(cube.left.get(row, col), Color::Orange);
        }
    }

    // Right face should be all red
    for row in 0..3 {
        for col in 0..3 {
            assert_eq!(cube.right.get(row, col), Color::Red);
        }
    }
}

/// Test color counts for 3x3 cube (should be 9 of each)
#[test]
fn test_3x3_color_counts() {
    let cube = Cube::new(3);
    let counts = cube.count_colors();

    assert_eq!(counts.get(&Color::White), Some(&9));
    assert_eq!(counts.get(&Color::Yellow), Some(&9));
    assert_eq!(counts.get(&Color::Red), Some(&9));
    assert_eq!(counts.get(&Color::Orange), Some(&9));
    assert_eq!(counts.get(&Color::Blue), Some(&9));
    assert_eq!(counts.get(&Color::Green), Some(&9));
}

/// Test color counts for various sizes
#[test]
fn test_color_counts_various_sizes() {
    for size in [2, 3, 4, 5, 7, 10, 15, 20] {
        let cube = Cube::new(size);
        let expected = size * size;
        let counts = cube.count_colors();

        for color in [
            Color::White,
            Color::Yellow,
            Color::Red,
            Color::Orange,
            Color::Blue,
            Color::Green,
        ] {
            assert_eq!(
                counts.get(&color),
                Some(&expected),
                "Size {} should have {} {} stickers",
                size,
                expected,
                format!("{:?}", color)
            );
        }
    }
}
