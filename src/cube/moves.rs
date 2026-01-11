//! Face rotation operations for Rubik's cube
//!
//! This module implements R1.2 from the PRD:
//! - All 6 face rotations work correctly (R, L, U, D, F, B)
//! - Clockwise, counter-clockwise, and double moves
//! - Adjacent face edges update correctly
//! - Works for any cube size (2x2 to 20x20)

use super::state::{Color, Cube};
use serde::{Deserialize, Serialize};

/// Represents a single move on the cube
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Move {
    /// Right face clockwise
    R,
    /// Right face counter-clockwise
    RPrime,
    /// Right face 180 degrees
    R2,
    /// Left face clockwise
    L,
    /// Left face counter-clockwise
    LPrime,
    /// Left face 180 degrees
    L2,
    /// Up face clockwise
    U,
    /// Up face counter-clockwise
    UPrime,
    /// Up face 180 degrees
    U2,
    /// Down face clockwise
    D,
    /// Down face counter-clockwise
    DPrime,
    /// Down face 180 degrees
    D2,
    /// Front face clockwise
    F,
    /// Front face counter-clockwise
    FPrime,
    /// Front face 180 degrees
    F2,
    /// Back face clockwise
    B,
    /// Back face counter-clockwise
    BPrime,
    /// Back face 180 degrees
    B2,
}

impl Move {
    /// Returns the inverse of this move
    pub fn inverse(&self) -> Move {
        match self {
            Move::R => Move::RPrime,
            Move::RPrime => Move::R,
            Move::R2 => Move::R2,
            Move::L => Move::LPrime,
            Move::LPrime => Move::L,
            Move::L2 => Move::L2,
            Move::U => Move::UPrime,
            Move::UPrime => Move::U,
            Move::U2 => Move::U2,
            Move::D => Move::DPrime,
            Move::DPrime => Move::D,
            Move::D2 => Move::D2,
            Move::F => Move::FPrime,
            Move::FPrime => Move::F,
            Move::F2 => Move::F2,
            Move::B => Move::BPrime,
            Move::BPrime => Move::B,
            Move::B2 => Move::B2,
        }
    }

    /// Returns the standard notation string for this move
    pub fn to_notation(&self) -> &'static str {
        match self {
            Move::R => "R",
            Move::RPrime => "R'",
            Move::R2 => "R2",
            Move::L => "L",
            Move::LPrime => "L'",
            Move::L2 => "L2",
            Move::U => "U",
            Move::UPrime => "U'",
            Move::U2 => "U2",
            Move::D => "D",
            Move::DPrime => "D'",
            Move::D2 => "D2",
            Move::F => "F",
            Move::FPrime => "F'",
            Move::F2 => "F2",
            Move::B => "B",
            Move::BPrime => "B'",
            Move::B2 => "B2",
        }
    }
}

impl Cube {
    /// Applies a move to the cube
    pub fn apply_move(&mut self, mv: Move) {
        match mv {
            Move::R => self.move_r(),
            Move::RPrime => self.move_r_prime(),
            Move::R2 => {
                self.move_r();
                self.move_r();
            }
            Move::L => self.move_l(),
            Move::LPrime => self.move_l_prime(),
            Move::L2 => {
                self.move_l();
                self.move_l();
            }
            Move::U => self.move_u(),
            Move::UPrime => self.move_u_prime(),
            Move::U2 => {
                self.move_u();
                self.move_u();
            }
            Move::D => self.move_d(),
            Move::DPrime => self.move_d_prime(),
            Move::D2 => {
                self.move_d();
                self.move_d();
            }
            Move::F => self.move_f(),
            Move::FPrime => self.move_f_prime(),
            Move::F2 => {
                self.move_f();
                self.move_f();
            }
            Move::B => self.move_b(),
            Move::BPrime => self.move_b_prime(),
            Move::B2 => {
                self.move_b();
                self.move_b();
            }
        }
    }

    /// Applies a sequence of moves to the cube
    pub fn apply_moves(&mut self, moves: &[Move]) {
        for mv in moves {
            self.apply_move(*mv);
        }
    }

    /// R move: Right face clockwise
    /// Affects: Right face (rotate CW), and columns from Up, Front, Down, Back
    fn move_r(&mut self) {
        let n = self.size();
        let last_col = n - 1;

        // Rotate the R face clockwise
        self.right.rotate_cw();

        // Cycle the columns: Up -> Back -> Down -> Front -> Up
        // Note: Back face is viewed from outside, so columns are reversed
        let up_col = self.up.get_col(last_col);
        let front_col = self.front.get_col(last_col);
        let down_col = self.down.get_col(last_col);
        let back_col: Vec<Color> = self.back.get_col(0).into_iter().rev().collect();

        // Up right col -> Front right col
        self.front.set_col(last_col, up_col);
        // Front right col -> Down right col
        self.down.set_col(last_col, front_col);
        // Down right col -> Back left col (reversed)
        self.back.set_col(0, down_col.into_iter().rev().collect());
        // Back left col (reversed) -> Up right col
        self.up.set_col(last_col, back_col);
    }

    /// R' move: Right face counter-clockwise
    fn move_r_prime(&mut self) {
        let n = self.size();
        let last_col = n - 1;

        // Rotate the R face counter-clockwise
        self.right.rotate_ccw();

        // Cycle the columns: Up -> Front -> Down -> Back -> Up (reverse of R)
        let up_col = self.up.get_col(last_col);
        let front_col = self.front.get_col(last_col);
        let down_col = self.down.get_col(last_col);
        let back_col: Vec<Color> = self.back.get_col(0).into_iter().rev().collect();

        // Up right col -> Back left col (reversed)
        self.back.set_col(0, up_col.into_iter().rev().collect());
        // Back left col (reversed) -> Down right col
        self.down.set_col(last_col, back_col);
        // Down right col -> Front right col
        self.front.set_col(last_col, down_col);
        // Front right col -> Up right col
        self.up.set_col(last_col, front_col);
    }

    /// L move: Left face clockwise
    /// Affects: Left face (rotate CW), and columns from Up, Front, Down, Back
    fn move_l(&mut self) {
        let n = self.size();
        let last_col = n - 1;

        // Rotate the L face clockwise
        self.left.rotate_cw();

        // Cycle the columns: Up -> Front -> Down -> Back -> Up (opposite of R)
        let up_col = self.up.get_col(0);
        let front_col = self.front.get_col(0);
        let down_col = self.down.get_col(0);
        let back_col: Vec<Color> = self.back.get_col(last_col).into_iter().rev().collect();

        // Up left col -> Back right col (reversed)
        self.back.set_col(last_col, up_col.into_iter().rev().collect());
        // Back right col (reversed) -> Down left col
        self.down.set_col(0, back_col);
        // Down left col -> Front left col
        self.front.set_col(0, down_col);
        // Front left col -> Up left col
        self.up.set_col(0, front_col);
    }

    /// L' move: Left face counter-clockwise
    fn move_l_prime(&mut self) {
        let n = self.size();
        let last_col = n - 1;

        // Rotate the L face counter-clockwise
        self.left.rotate_ccw();

        // Cycle the columns: Up -> Back -> Down -> Front -> Up
        let up_col = self.up.get_col(0);
        let front_col = self.front.get_col(0);
        let down_col = self.down.get_col(0);
        let back_col: Vec<Color> = self.back.get_col(last_col).into_iter().rev().collect();

        // Up left col -> Front left col
        self.front.set_col(0, up_col);
        // Front left col -> Down left col
        self.down.set_col(0, front_col);
        // Down left col -> Back right col (reversed)
        self.back.set_col(last_col, down_col.into_iter().rev().collect());
        // Back right col (reversed) -> Up left col
        self.up.set_col(0, back_col);
    }

    /// U move: Up face clockwise
    /// Affects: Up face (rotate CW), and top rows of Front, Right, Back, Left
    fn move_u(&mut self) {
        // Rotate the U face clockwise
        self.up.rotate_cw();

        // Cycle the rows: Front -> Left -> Back -> Right -> Front
        let front_row = self.front.get_row(0);
        let left_row = self.left.get_row(0);
        let back_row = self.back.get_row(0);
        let right_row = self.right.get_row(0);

        // Front top row -> Right top row
        self.right.set_row(0, front_row);
        // Right top row -> Back top row
        self.back.set_row(0, right_row);
        // Back top row -> Left top row
        self.left.set_row(0, back_row);
        // Left top row -> Front top row
        self.front.set_row(0, left_row);
    }

    /// U' move: Up face counter-clockwise
    fn move_u_prime(&mut self) {
        // Rotate the U face counter-clockwise
        self.up.rotate_ccw();

        // Cycle the rows: Front -> Right -> Back -> Left -> Front
        let front_row = self.front.get_row(0);
        let left_row = self.left.get_row(0);
        let back_row = self.back.get_row(0);
        let right_row = self.right.get_row(0);

        // Front top row -> Left top row
        self.left.set_row(0, front_row);
        // Left top row -> Back top row
        self.back.set_row(0, left_row);
        // Back top row -> Right top row
        self.right.set_row(0, back_row);
        // Right top row -> Front top row
        self.front.set_row(0, right_row);
    }

    /// D move: Down face clockwise
    /// Affects: Down face (rotate CW), and bottom rows of Front, Left, Back, Right
    fn move_d(&mut self) {
        let n = self.size();
        let last_row = n - 1;

        // Rotate the D face clockwise
        self.down.rotate_cw();

        // Cycle the rows: Front -> Right -> Back -> Left -> Front (opposite of U)
        let front_row = self.front.get_row(last_row);
        let left_row = self.left.get_row(last_row);
        let back_row = self.back.get_row(last_row);
        let right_row = self.right.get_row(last_row);

        // Front bottom row -> Left bottom row
        self.left.set_row(last_row, front_row);
        // Left bottom row -> Back bottom row
        self.back.set_row(last_row, left_row);
        // Back bottom row -> Right bottom row
        self.right.set_row(last_row, back_row);
        // Right bottom row -> Front bottom row
        self.front.set_row(last_row, right_row);
    }

    /// D' move: Down face counter-clockwise
    fn move_d_prime(&mut self) {
        let n = self.size();
        let last_row = n - 1;

        // Rotate the D face counter-clockwise
        self.down.rotate_ccw();

        // Cycle the rows: Front -> Left -> Back -> Right -> Front
        let front_row = self.front.get_row(last_row);
        let left_row = self.left.get_row(last_row);
        let back_row = self.back.get_row(last_row);
        let right_row = self.right.get_row(last_row);

        // Front bottom row -> Right bottom row
        self.right.set_row(last_row, front_row);
        // Right bottom row -> Back bottom row
        self.back.set_row(last_row, right_row);
        // Back bottom row -> Left bottom row
        self.left.set_row(last_row, back_row);
        // Left bottom row -> Front bottom row
        self.front.set_row(last_row, left_row);
    }

    /// F move: Front face clockwise
    /// Affects: Front face (rotate CW), and adjacent edges of Up, Right, Down, Left
    fn move_f(&mut self) {
        let n = self.size();
        let last_row = n - 1;
        let last_col = n - 1;

        // Rotate the F face clockwise
        self.front.rotate_cw();

        // Cycle: Up bottom row -> Right left col -> Down top row (reversed) -> Left right col (reversed) -> Up bottom row
        let up_row = self.up.get_row(last_row);
        let right_col = self.right.get_col(0);
        let down_row = self.down.get_row(0);
        let left_col = self.left.get_col(last_col);

        // Up bottom row -> Right left col
        self.right.set_col(0, up_row);
        // Right left col -> Down top row (reversed)
        self.down.set_row(0, right_col.into_iter().rev().collect());
        // Down top row -> Left right col
        self.left.set_col(last_col, down_row);
        // Left right col -> Up bottom row (reversed)
        self.up.set_row(last_row, left_col.into_iter().rev().collect());
    }

    /// F' move: Front face counter-clockwise
    fn move_f_prime(&mut self) {
        let n = self.size();
        let last_row = n - 1;
        let last_col = n - 1;

        // Rotate the F face counter-clockwise
        self.front.rotate_ccw();

        // Cycle: Up bottom row -> Left right col -> Down top row -> Right left col -> Up bottom row
        let up_row = self.up.get_row(last_row);
        let right_col = self.right.get_col(0);
        let down_row = self.down.get_row(0);
        let left_col = self.left.get_col(last_col);

        // Up bottom row -> Left right col (reversed)
        self.left.set_col(last_col, up_row.into_iter().rev().collect());
        // Left right col -> Down top row
        self.down.set_row(0, left_col);
        // Down top row -> Right left col (reversed)
        self.right.set_col(0, down_row.into_iter().rev().collect());
        // Right left col -> Up bottom row
        self.up.set_row(last_row, right_col);
    }

    /// B move: Back face clockwise
    /// Affects: Back face (rotate CW), and adjacent edges of Up, Left, Down, Right
    fn move_b(&mut self) {
        let n = self.size();
        let last_col = n - 1;
        let last_row = n - 1;

        // Rotate the B face clockwise
        self.back.rotate_cw();

        // Cycle: Up top row -> Left left col -> Down bottom row -> Right right col -> Up top row
        let up_row = self.up.get_row(0);
        let left_col = self.left.get_col(0);
        let down_row = self.down.get_row(last_row);
        let right_col = self.right.get_col(last_col);

        // Up top row -> Left left col (reversed)
        self.left.set_col(0, up_row.into_iter().rev().collect());
        // Left left col -> Down bottom row
        self.down.set_row(last_row, left_col);
        // Down bottom row -> Right right col (reversed)
        self.right.set_col(last_col, down_row.into_iter().rev().collect());
        // Right right col -> Up top row
        self.up.set_row(0, right_col);
    }

    /// B' move: Back face counter-clockwise
    fn move_b_prime(&mut self) {
        let n = self.size();
        let last_col = n - 1;
        let last_row = n - 1;

        // Rotate the B face counter-clockwise
        self.back.rotate_ccw();

        // Cycle: Up top row -> Right right col -> Down bottom row -> Left left col -> Up top row
        let up_row = self.up.get_row(0);
        let left_col = self.left.get_col(0);
        let down_row = self.down.get_row(last_row);
        let right_col = self.right.get_col(last_col);

        // Up top row -> Right right col
        self.right.set_col(last_col, up_row);
        // Right right col -> Down bottom row (reversed)
        self.down.set_row(last_row, right_col.into_iter().rev().collect());
        // Down bottom row -> Left left col
        self.left.set_col(0, down_row);
        // Left left col -> Up top row (reversed)
        self.up.set_row(0, left_col.into_iter().rev().collect());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::state::Color;

    #[test]
    fn cube_006_r_move() {
        let mut cube = Cube::new(3);
        cube.apply_move(Move::R);

        // Right face should be rotated
        assert!(cube.right.is_solved());

        // Check that adjacent faces were affected
        // After R: Up right column goes to Front, Front right column goes to Down,
        // Down right column goes to Back, Back left column goes to Up
        assert_eq!(cube.front.get_col(2), vec![Color::White; 3]);
        assert_eq!(cube.down.get_col(2), vec![Color::Green; 3]);
        let back_col: Vec<Color> = cube.back.get_col(0).into_iter().rev().collect();
        assert_eq!(back_col, vec![Color::Yellow; 3]);
        assert_eq!(cube.up.get_col(2), vec![Color::Blue; 3]);
    }

    #[test]
    fn cube_007_l_move() {
        let mut cube = Cube::new(3);
        cube.apply_move(Move::L);

        // Left face should be rotated
        assert!(cube.left.is_solved());

        // Check that adjacent faces were affected
        // After L: Up left column goes to Back, Back right column goes to Down,
        // Down left column goes to Front, Front left column goes to Up
        let back_col: Vec<Color> = cube.back.get_col(2).into_iter().rev().collect();
        assert_eq!(back_col, vec![Color::White; 3]);
        assert_eq!(cube.down.get_col(0), vec![Color::Blue; 3]);
        assert_eq!(cube.front.get_col(0), vec![Color::Yellow; 3]);
        assert_eq!(cube.up.get_col(0), vec![Color::Green; 3]);
    }

    #[test]
    fn cube_008_u_move() {
        let mut cube = Cube::new(3);
        cube.apply_move(Move::U);

        // Up face should be rotated
        assert!(cube.up.is_solved());

        // Check that adjacent faces were affected
        // After U: Front top row -> Right, Right top row -> Back, Back top row -> Left, Left top row -> Front
        assert_eq!(cube.right.get_row(0), vec![Color::Green; 3]);
        assert_eq!(cube.back.get_row(0), vec![Color::Red; 3]);
        assert_eq!(cube.left.get_row(0), vec![Color::Blue; 3]);
        assert_eq!(cube.front.get_row(0), vec![Color::Orange; 3]);
    }

    #[test]
    fn cube_009_d_move() {
        let mut cube = Cube::new(3);
        cube.apply_move(Move::D);

        // Down face should be rotated
        assert!(cube.down.is_solved());

        // Check that adjacent faces were affected
        // After D: Front bottom row -> Left, Left bottom row -> Back, Back bottom row -> Right, Right bottom row -> Front
        assert_eq!(cube.left.get_row(2), vec![Color::Green; 3]);
        assert_eq!(cube.back.get_row(2), vec![Color::Orange; 3]);
        assert_eq!(cube.right.get_row(2), vec![Color::Blue; 3]);
        assert_eq!(cube.front.get_row(2), vec![Color::Red; 3]);
    }

    #[test]
    fn cube_010_f_move() {
        let mut cube = Cube::new(3);
        cube.apply_move(Move::F);

        // Front face should be rotated
        assert!(cube.front.is_solved());

        // Check that adjacent faces were affected
        // After F: Up bottom row -> Right left col, Right left col -> Down top row (rev),
        // Down top row -> Left right col, Left right col -> Up bottom row (rev)
        assert_eq!(cube.right.get_col(0), vec![Color::White; 3]);
        assert_eq!(cube.down.get_row(0), vec![Color::Red, Color::Red, Color::Red]);
        assert_eq!(cube.left.get_col(2), vec![Color::Yellow; 3]);
        assert_eq!(cube.up.get_row(2), vec![Color::Orange, Color::Orange, Color::Orange]);
    }

    #[test]
    fn cube_011_b_move() {
        let mut cube = Cube::new(3);
        cube.apply_move(Move::B);

        // Back face should be rotated
        assert!(cube.back.is_solved());

        // Check that adjacent faces were affected
        // After B: Up top row -> Left left col (rev), Left left col -> Down bottom row,
        // Down bottom row -> Right right col (rev), Right right col -> Up top row
        let left_col: Vec<Color> = cube.left.get_col(0).into_iter().rev().collect();
        assert_eq!(left_col, vec![Color::White; 3]);
        assert_eq!(cube.down.get_row(2), vec![Color::Orange, Color::Orange, Color::Orange]);
        let right_col: Vec<Color> = cube.right.get_col(2).into_iter().rev().collect();
        assert_eq!(right_col, vec![Color::Yellow; 3]);
        assert_eq!(cube.up.get_row(0), vec![Color::Red; 3]);
    }

    #[test]
    fn cube_012_r_prime_move() {
        let mut cube = Cube::new(3);

        // R followed by R' should return to solved
        cube.apply_move(Move::R);
        cube.apply_move(Move::RPrime);

        assert!(cube.is_solved());
    }

    #[test]
    fn cube_013_r2_move() {
        let mut cube = Cube::new(3);

        // R2 is two R moves
        cube.apply_move(Move::R2);

        // R2 followed by R2 should return to solved
        cube.apply_move(Move::R2);

        assert!(cube.is_solved());
    }

    #[test]
    fn test_all_inverse_moves() {
        // Test that every move followed by its inverse returns to solved
        let moves = [
            (Move::R, Move::RPrime),
            (Move::L, Move::LPrime),
            (Move::U, Move::UPrime),
            (Move::D, Move::DPrime),
            (Move::F, Move::FPrime),
            (Move::B, Move::BPrime),
        ];

        for (mv, inv) in moves.iter() {
            let mut cube = Cube::new(3);
            cube.apply_move(*mv);
            cube.apply_move(*inv);
            assert!(cube.is_solved(), "Failed for {:?} and {:?}", mv, inv);
        }
    }

    #[test]
    fn test_double_moves() {
        // Test that double moves work (X2 followed by X2 = solved)
        let double_moves = [Move::R2, Move::L2, Move::U2, Move::D2, Move::F2, Move::B2];

        for mv in double_moves.iter() {
            let mut cube = Cube::new(3);
            cube.apply_move(*mv);
            cube.apply_move(*mv);
            assert!(cube.is_solved(), "Failed for {:?}", mv);
        }
    }

    #[test]
    fn cube_023_sexy_move_returns_to_solved() {
        // The "sexy move" (R U R' U') repeated 6 times returns to solved
        let mut cube = Cube::new(3);

        for _ in 0..6 {
            cube.apply_move(Move::R);
            cube.apply_move(Move::U);
            cube.apply_move(Move::RPrime);
            cube.apply_move(Move::UPrime);
        }

        assert!(cube.is_solved());
    }

    #[test]
    fn cube_024_sune_6x_returns_to_solved() {
        // Sune algorithm: R U R' U R U2 R'
        // Repeated 6 times returns to solved (order 6)
        let mut cube = Cube::new(3);

        for _ in 0..6 {
            cube.apply_move(Move::R);
            cube.apply_move(Move::U);
            cube.apply_move(Move::RPrime);
            cube.apply_move(Move::U);
            cube.apply_move(Move::R);
            cube.apply_move(Move::U2);
            cube.apply_move(Move::RPrime);
        }

        assert!(cube.is_solved());
    }

    #[test]
    fn cube_025_adjacent_edges_on_r_move() {
        let mut cube = Cube::new(3);

        // Set a distinctive pattern on the right column of front face
        cube.front.set(0, 2, Color::Red);
        cube.front.set(1, 2, Color::Blue);
        cube.front.set(2, 2, Color::Orange);

        cube.apply_move(Move::R);

        // These colors should now be on the right column of down face
        assert_eq!(cube.down.get(0, 2), Color::Red);
        assert_eq!(cube.down.get(1, 2), Color::Blue);
        assert_eq!(cube.down.get(2, 2), Color::Orange);
    }

    #[test]
    fn cube_026_adjacent_edges_on_u_move() {
        let mut cube = Cube::new(3);

        // Set a distinctive pattern on the top row of front face
        cube.front.set(0, 0, Color::Red);
        cube.front.set(0, 1, Color::Blue);
        cube.front.set(0, 2, Color::Orange);

        cube.apply_move(Move::U);

        // These colors should now be on the top row of right face
        assert_eq!(cube.right.get(0, 0), Color::Red);
        assert_eq!(cube.right.get(0, 1), Color::Blue);
        assert_eq!(cube.right.get(0, 2), Color::Orange);
    }

    #[test]
    fn test_moves_on_4x4() {
        // Test that moves work on larger cubes
        let mut cube = Cube::new(4);

        // Apply all basic moves
        cube.apply_move(Move::R);
        cube.apply_move(Move::L);
        cube.apply_move(Move::U);
        cube.apply_move(Move::D);
        cube.apply_move(Move::F);
        cube.apply_move(Move::B);

        // Apply all inverse moves to return to solved
        cube.apply_move(Move::BPrime);
        cube.apply_move(Move::FPrime);
        cube.apply_move(Move::DPrime);
        cube.apply_move(Move::UPrime);
        cube.apply_move(Move::LPrime);
        cube.apply_move(Move::RPrime);

        assert!(cube.is_solved());
    }

    #[test]
    fn test_moves_on_5x5() {
        // Test sexy move on 5x5
        let mut cube = Cube::new(5);

        for _ in 0..6 {
            cube.apply_move(Move::R);
            cube.apply_move(Move::U);
            cube.apply_move(Move::RPrime);
            cube.apply_move(Move::UPrime);
        }

        assert!(cube.is_solved());
    }

    #[test]
    fn test_moves_on_2x2() {
        // Test moves on 2x2 (smallest valid size)
        let mut cube = Cube::new(2);

        // R U R' U' repeated 6 times should return to solved
        for _ in 0..6 {
            cube.apply_move(Move::R);
            cube.apply_move(Move::U);
            cube.apply_move(Move::RPrime);
            cube.apply_move(Move::UPrime);
        }

        assert!(cube.is_solved());
    }

    #[test]
    fn test_move_inverse() {
        assert_eq!(Move::R.inverse(), Move::RPrime);
        assert_eq!(Move::RPrime.inverse(), Move::R);
        assert_eq!(Move::R2.inverse(), Move::R2);
        assert_eq!(Move::L.inverse(), Move::LPrime);
        assert_eq!(Move::U.inverse(), Move::UPrime);
        assert_eq!(Move::D.inverse(), Move::DPrime);
        assert_eq!(Move::F.inverse(), Move::FPrime);
        assert_eq!(Move::B.inverse(), Move::BPrime);
    }

    #[test]
    fn test_move_notation() {
        assert_eq!(Move::R.to_notation(), "R");
        assert_eq!(Move::RPrime.to_notation(), "R'");
        assert_eq!(Move::R2.to_notation(), "R2");
        assert_eq!(Move::L.to_notation(), "L");
        assert_eq!(Move::LPrime.to_notation(), "L'");
        assert_eq!(Move::U2.to_notation(), "U2");
    }

    #[test]
    fn test_apply_moves_sequence() {
        let mut cube = Cube::new(3);

        // Apply a sequence of moves
        let moves = vec![
            Move::R,
            Move::U,
            Move::RPrime,
            Move::UPrime,
            Move::R,
            Move::U,
            Move::RPrime,
            Move::UPrime,
            Move::R,
            Move::U,
            Move::RPrime,
            Move::UPrime,
            Move::R,
            Move::U,
            Move::RPrime,
            Move::UPrime,
            Move::R,
            Move::U,
            Move::RPrime,
            Move::UPrime,
            Move::R,
            Move::U,
            Move::RPrime,
            Move::UPrime,
        ];

        cube.apply_moves(&moves);

        // 6x sexy move returns to solved
        assert!(cube.is_solved());
    }
}
