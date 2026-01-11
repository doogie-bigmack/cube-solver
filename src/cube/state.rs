//! Cube state representation for NxN cubes (2x2 to 20x20)
//!
//! This module implements R1.1 from the PRD:
//! - Cube struct supports any size from 2 to 20
//! - Each face stores NxN grid of colors
//! - Colors enum: White, Yellow, Red, Orange, Blue, Green
//! - New cube initializes to solved state

use serde::{Deserialize, Serialize};

/// The six standard Rubik's cube colors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}

impl Color {
    /// Returns the opposite color (on opposite faces of a solved cube)
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Yellow,
            Color::Yellow => Color::White,
            Color::Red => Color::Orange,
            Color::Orange => Color::Red,
            Color::Blue => Color::Green,
            Color::Green => Color::Blue,
        }
    }
}

/// Represents one face of the cube
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Face {
    /// NxN grid of colors, stored as [row][col]
    /// (0,0) is top-left when looking at the face
    stickers: Vec<Vec<Color>>,
    /// Size of the face (N for NxN)
    size: usize,
}

impl Face {
    /// Creates a new face with all stickers of the given color
    pub fn new(size: usize, color: Color) -> Self {
        assert!((2..=20).contains(&size), "Face size must be between 2 and 20");
        Self {
            stickers: vec![vec![color; size]; size],
            size,
        }
    }

    /// Gets the color at the specified position
    pub fn get(&self, row: usize, col: usize) -> Color {
        self.stickers[row][col]
    }

    /// Sets the color at the specified position
    pub fn set(&mut self, row: usize, col: usize, color: Color) {
        self.stickers[row][col] = color;
    }

    /// Returns the size of this face
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns an immutable reference to all stickers
    pub fn stickers(&self) -> &Vec<Vec<Color>> {
        &self.stickers
    }

    /// Rotates the face 90 degrees clockwise
    pub fn rotate_cw(&mut self) {
        let n = self.size;
        let mut new_stickers = vec![vec![Color::White; n]; n];
        for row in 0..n {
            for col in 0..n {
                new_stickers[col][n - 1 - row] = self.stickers[row][col];
            }
        }
        self.stickers = new_stickers;
    }

    /// Rotates the face 90 degrees counter-clockwise
    pub fn rotate_ccw(&mut self) {
        let n = self.size;
        let mut new_stickers = vec![vec![Color::White; n]; n];
        for row in 0..n {
            for col in 0..n {
                new_stickers[n - 1 - col][row] = self.stickers[row][col];
            }
        }
        self.stickers = new_stickers;
    }

    /// Rotates the face 180 degrees
    pub fn rotate_180(&mut self) {
        let n = self.size;
        let mut new_stickers = vec![vec![Color::White; n]; n];
        for row in 0..n {
            for col in 0..n {
                new_stickers[n - 1 - row][n - 1 - col] = self.stickers[row][col];
            }
        }
        self.stickers = new_stickers;
    }

    /// Gets an entire row of stickers
    pub fn get_row(&self, row: usize) -> Vec<Color> {
        self.stickers[row].clone()
    }

    /// Sets an entire row of stickers
    pub fn set_row(&mut self, row: usize, colors: Vec<Color>) {
        assert_eq!(colors.len(), self.size);
        self.stickers[row] = colors;
    }

    /// Gets an entire column of stickers (top to bottom)
    pub fn get_col(&self, col: usize) -> Vec<Color> {
        (0..self.size).map(|row| self.stickers[row][col]).collect()
    }

    /// Sets an entire column of stickers (top to bottom)
    pub fn set_col(&mut self, col: usize, colors: Vec<Color>) {
        assert_eq!(colors.len(), self.size);
        for row in 0..self.size {
            self.stickers[row][col] = colors[row];
        }
    }

    /// Checks if all stickers on this face are the same color
    pub fn is_solved(&self) -> bool {
        let first = self.stickers[0][0];
        self.stickers.iter().all(|row| row.iter().all(|&c| c == first))
    }
}

/// The six faces of a Rubik's cube
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FaceName {
    /// Up (top) - typically white
    U,
    /// Down (bottom) - typically yellow
    D,
    /// Front - typically green
    F,
    /// Back - typically blue
    B,
    /// Left - typically orange
    L,
    /// Right - typically red
    R,
}

impl FaceName {
    /// Returns all face names in standard order
    pub fn all() -> [FaceName; 6] {
        [FaceName::U, FaceName::D, FaceName::F, FaceName::B, FaceName::L, FaceName::R]
    }

    /// Returns the opposite face
    pub fn opposite(&self) -> FaceName {
        match self {
            FaceName::U => FaceName::D,
            FaceName::D => FaceName::U,
            FaceName::F => FaceName::B,
            FaceName::B => FaceName::F,
            FaceName::L => FaceName::R,
            FaceName::R => FaceName::L,
        }
    }

    /// Returns the standard color for this face on a solved cube
    pub fn standard_color(&self) -> Color {
        match self {
            FaceName::U => Color::White,
            FaceName::D => Color::Yellow,
            FaceName::F => Color::Green,
            FaceName::B => Color::Blue,
            FaceName::L => Color::Orange,
            FaceName::R => Color::Red,
        }
    }
}

/// Represents an NxN Rubik's cube
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cube {
    /// The size of the cube (N for NxN)
    size: usize,
    /// Up face (typically white)
    pub up: Face,
    /// Down face (typically yellow)
    pub down: Face,
    /// Front face (typically green)
    pub front: Face,
    /// Back face (typically blue)
    pub back: Face,
    /// Left face (typically orange)
    pub left: Face,
    /// Right face (typically red)
    pub right: Face,
}

impl Cube {
    /// Creates a new solved cube of the given size
    ///
    /// # Arguments
    /// * `size` - The size of the cube (2 to 20)
    ///
    /// # Panics
    /// Panics if size is not between 2 and 20
    pub fn new(size: usize) -> Self {
        assert!((2..=20).contains(&size), "Cube size must be between 2 and 20");
        Self {
            size,
            up: Face::new(size, Color::White),
            down: Face::new(size, Color::Yellow),
            front: Face::new(size, Color::Green),
            back: Face::new(size, Color::Blue),
            left: Face::new(size, Color::Orange),
            right: Face::new(size, Color::Red),
        }
    }

    /// Returns the size of the cube
    pub fn size(&self) -> usize {
        self.size
    }

    /// Gets a reference to the specified face
    pub fn get_face(&self, face: FaceName) -> &Face {
        match face {
            FaceName::U => &self.up,
            FaceName::D => &self.down,
            FaceName::F => &self.front,
            FaceName::B => &self.back,
            FaceName::L => &self.left,
            FaceName::R => &self.right,
        }
    }

    /// Gets a mutable reference to the specified face
    pub fn get_face_mut(&mut self, face: FaceName) -> &mut Face {
        match face {
            FaceName::U => &mut self.up,
            FaceName::D => &mut self.down,
            FaceName::F => &mut self.front,
            FaceName::B => &mut self.back,
            FaceName::L => &mut self.left,
            FaceName::R => &mut self.right,
        }
    }

    /// Checks if the cube is in the solved state
    pub fn is_solved(&self) -> bool {
        self.up.is_solved()
            && self.down.is_solved()
            && self.front.is_solved()
            && self.back.is_solved()
            && self.left.is_solved()
            && self.right.is_solved()
            && self.up.get(0, 0) == Color::White
            && self.down.get(0, 0) == Color::Yellow
            && self.front.get(0, 0) == Color::Green
            && self.back.get(0, 0) == Color::Blue
            && self.left.get(0, 0) == Color::Orange
            && self.right.get(0, 0) == Color::Red
    }

    /// Counts the number of stickers of each color
    pub fn count_colors(&self) -> std::collections::HashMap<Color, usize> {
        let mut counts = std::collections::HashMap::new();
        for face in FaceName::all() {
            let f = self.get_face(face);
            for row in 0..self.size {
                for col in 0..self.size {
                    *counts.entry(f.get(row, col)).or_insert(0) += 1;
                }
            }
        }
        counts
    }

    /// Validates that each color appears exactly N^2 times
    pub fn has_valid_color_counts(&self) -> bool {
        let expected = self.size * self.size;
        let counts = self.count_colors();

        // Must have exactly 6 colors
        if counts.len() != 6 {
            return false;
        }

        // Each color must appear exactly N^2 times
        counts.values().all(|&count| count == expected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_opposite() {
        assert_eq!(Color::White.opposite(), Color::Yellow);
        assert_eq!(Color::Yellow.opposite(), Color::White);
        assert_eq!(Color::Red.opposite(), Color::Orange);
        assert_eq!(Color::Orange.opposite(), Color::Red);
        assert_eq!(Color::Blue.opposite(), Color::Green);
        assert_eq!(Color::Green.opposite(), Color::Blue);
    }

    #[test]
    fn cube_001_create_2x2_solved() {
        let cube = Cube::new(2);
        assert_eq!(cube.size(), 2);
        assert!(cube.is_solved());
    }

    #[test]
    fn cube_002_create_3x3_solved() {
        let cube = Cube::new(3);
        assert_eq!(cube.size(), 3);
        assert!(cube.is_solved());
    }

    #[test]
    fn cube_003_create_5x5_solved() {
        let cube = Cube::new(5);
        assert_eq!(cube.size(), 5);
        assert!(cube.is_solved());
    }

    #[test]
    fn cube_004_create_10x10_solved() {
        let cube = Cube::new(10);
        assert_eq!(cube.size(), 10);
        assert!(cube.is_solved());
    }

    #[test]
    fn cube_005_create_20x20_solved() {
        let cube = Cube::new(20);
        assert_eq!(cube.size(), 20);
        assert!(cube.is_solved());
    }

    #[test]
    fn test_face_colors() {
        let cube = Cube::new(3);
        assert_eq!(cube.up.get(0, 0), Color::White);
        assert_eq!(cube.down.get(0, 0), Color::Yellow);
        assert_eq!(cube.front.get(0, 0), Color::Green);
        assert_eq!(cube.back.get(0, 0), Color::Blue);
        assert_eq!(cube.left.get(0, 0), Color::Orange);
        assert_eq!(cube.right.get(0, 0), Color::Red);
    }

    #[test]
    fn test_face_rotation_cw() {
        let mut face = Face::new(3, Color::White);
        // Set some distinct colors to verify rotation
        face.set(0, 0, Color::Red);
        face.set(0, 2, Color::Blue);
        face.set(2, 0, Color::Green);
        face.set(2, 2, Color::Orange);

        face.rotate_cw();

        // After CW rotation:
        // Top-left (Red) -> Top-right
        // Top-right (Blue) -> Bottom-right
        // Bottom-left (Green) -> Top-left
        // Bottom-right (Orange) -> Bottom-left
        assert_eq!(face.get(0, 2), Color::Red);
        assert_eq!(face.get(2, 2), Color::Blue);
        assert_eq!(face.get(0, 0), Color::Green);
        assert_eq!(face.get(2, 0), Color::Orange);
    }

    #[test]
    fn test_face_rotation_ccw() {
        let mut face = Face::new(3, Color::White);
        face.set(0, 0, Color::Red);
        face.set(0, 2, Color::Blue);
        face.set(2, 0, Color::Green);
        face.set(2, 2, Color::Orange);

        face.rotate_ccw();

        // After CCW rotation:
        // Top-left (Red) -> Bottom-left
        // Top-right (Blue) -> Top-left
        // Bottom-left (Green) -> Bottom-right
        // Bottom-right (Orange) -> Top-right
        assert_eq!(face.get(2, 0), Color::Red);
        assert_eq!(face.get(0, 0), Color::Blue);
        assert_eq!(face.get(2, 2), Color::Green);
        assert_eq!(face.get(0, 2), Color::Orange);
    }

    #[test]
    fn test_face_rotation_180() {
        let mut face = Face::new(3, Color::White);
        face.set(0, 0, Color::Red);
        face.set(0, 2, Color::Blue);
        face.set(2, 0, Color::Green);
        face.set(2, 2, Color::Orange);

        face.rotate_180();

        // After 180 rotation:
        // Top-left <-> Bottom-right
        // Top-right <-> Bottom-left
        assert_eq!(face.get(2, 2), Color::Red);
        assert_eq!(face.get(2, 0), Color::Blue);
        assert_eq!(face.get(0, 2), Color::Green);
        assert_eq!(face.get(0, 0), Color::Orange);
    }

    #[test]
    fn test_color_counts() {
        let cube = Cube::new(3);
        let counts = cube.count_colors();
        assert_eq!(counts.len(), 6);
        for (_color, count) in counts {
            assert_eq!(count, 9); // 3x3 = 9 stickers per face
        }
    }

    #[test]
    fn test_valid_color_counts() {
        let cube = Cube::new(3);
        assert!(cube.has_valid_color_counts());
    }

    #[test]
    #[should_panic(expected = "Cube size must be between 2 and 20")]
    fn test_cube_size_too_small() {
        Cube::new(1);
    }

    #[test]
    #[should_panic(expected = "Cube size must be between 2 and 20")]
    fn test_cube_size_too_large() {
        Cube::new(21);
    }

    #[test]
    fn test_get_and_set_row() {
        let mut face = Face::new(3, Color::White);
        let new_row = vec![Color::Red, Color::Blue, Color::Green];
        face.set_row(1, new_row.clone());
        assert_eq!(face.get_row(1), new_row);
    }

    #[test]
    fn test_get_and_set_col() {
        let mut face = Face::new(3, Color::White);
        let new_col = vec![Color::Red, Color::Blue, Color::Green];
        face.set_col(1, new_col.clone());
        assert_eq!(face.get_col(1), new_col);
    }

    #[test]
    fn test_face_is_solved() {
        let face = Face::new(3, Color::White);
        assert!(face.is_solved());

        let mut mixed_face = Face::new(3, Color::White);
        mixed_face.set(0, 0, Color::Red);
        assert!(!mixed_face.is_solved());
    }

    #[test]
    fn test_face_name_opposite() {
        assert_eq!(FaceName::U.opposite(), FaceName::D);
        assert_eq!(FaceName::D.opposite(), FaceName::U);
        assert_eq!(FaceName::F.opposite(), FaceName::B);
        assert_eq!(FaceName::B.opposite(), FaceName::F);
        assert_eq!(FaceName::L.opposite(), FaceName::R);
        assert_eq!(FaceName::R.opposite(), FaceName::L);
    }

    #[test]
    fn test_face_name_standard_color() {
        assert_eq!(FaceName::U.standard_color(), Color::White);
        assert_eq!(FaceName::D.standard_color(), Color::Yellow);
        assert_eq!(FaceName::F.standard_color(), Color::Green);
        assert_eq!(FaceName::B.standard_color(), Color::Blue);
        assert_eq!(FaceName::L.standard_color(), Color::Orange);
        assert_eq!(FaceName::R.standard_color(), Color::Red);
    }
}
