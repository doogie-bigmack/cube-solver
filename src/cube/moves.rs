//! Face rotation operations for Rubik's cube
//!
//! This module implements R1.2, R1.3, and R1.4 from the PRD:
//! - All 6 face rotations work correctly (R, L, U, D, F, B)
//! - Clockwise, counter-clockwise, and double moves
//! - Adjacent face edges update correctly
//! - Works for any cube size (2x2 to 20x20)
//! - Wide moves (Rw, Lw, Uw, Dw, Fw, Bw) rotate multiple layers
//! - Slice moves (M, E, S) for odd-sized cubes only

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
    /// M slice (middle between L and R, turns like L)
    M,
    /// M slice counter-clockwise
    MPrime,
    /// M slice 180 degrees
    M2,
    /// E slice (equator between U and D, turns like D)
    E,
    /// E slice counter-clockwise
    EPrime,
    /// E slice 180 degrees
    E2,
    /// S slice (standing between F and B, turns like F)
    S,
    /// S slice counter-clockwise
    SPrime,
    /// S slice 180 degrees
    S2,
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
            Move::M => Move::MPrime,
            Move::MPrime => Move::M,
            Move::M2 => Move::M2,
            Move::E => Move::EPrime,
            Move::EPrime => Move::E,
            Move::E2 => Move::E2,
            Move::S => Move::SPrime,
            Move::SPrime => Move::S,
            Move::S2 => Move::S2,
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
            Move::M => "M",
            Move::MPrime => "M'",
            Move::M2 => "M2",
            Move::E => "E",
            Move::EPrime => "E'",
            Move::E2 => "E2",
            Move::S => "S",
            Move::SPrime => "S'",
            Move::S2 => "S2",
        }
    }
}

/// Represents a wide move that rotates multiple layers
/// Wide moves are only valid for cubes 3x3 and larger
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WideMove {
    /// The base face to rotate
    pub face: WideFace,
    /// The direction of the rotation
    pub direction: Direction,
    /// Number of layers to rotate (1 = single outer layer, 2 = Rw/Lw etc.)
    /// For depth n, rotates the outer n layers
    pub depth: usize,
}

/// The six possible faces for wide moves
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WideFace {
    R,
    L,
    U,
    D,
    F,
    B,
}

/// Direction of rotation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
    Double,
}

impl WideMove {
    /// Creates a new wide move
    ///
    /// # Arguments
    /// * `face` - The face to rotate
    /// * `direction` - The direction of rotation
    /// * `depth` - Number of layers to rotate (minimum 2 for wide moves)
    pub fn new(face: WideFace, direction: Direction, depth: usize) -> Self {
        Self { face, direction, depth }
    }

    /// Creates a standard Rw move (2 layers)
    pub fn rw() -> Self {
        Self::new(WideFace::R, Direction::Clockwise, 2)
    }

    /// Creates a standard Rw' move (2 layers)
    pub fn rw_prime() -> Self {
        Self::new(WideFace::R, Direction::CounterClockwise, 2)
    }

    /// Creates a standard Rw2 move (2 layers)
    pub fn rw2() -> Self {
        Self::new(WideFace::R, Direction::Double, 2)
    }

    /// Creates a standard Lw move (2 layers)
    pub fn lw() -> Self {
        Self::new(WideFace::L, Direction::Clockwise, 2)
    }

    /// Creates a standard Lw' move (2 layers)
    pub fn lw_prime() -> Self {
        Self::new(WideFace::L, Direction::CounterClockwise, 2)
    }

    /// Creates a standard Lw2 move (2 layers)
    pub fn lw2() -> Self {
        Self::new(WideFace::L, Direction::Double, 2)
    }

    /// Creates a standard Uw move (2 layers)
    pub fn uw() -> Self {
        Self::new(WideFace::U, Direction::Clockwise, 2)
    }

    /// Creates a standard Uw' move (2 layers)
    pub fn uw_prime() -> Self {
        Self::new(WideFace::U, Direction::CounterClockwise, 2)
    }

    /// Creates a standard Uw2 move (2 layers)
    pub fn uw2() -> Self {
        Self::new(WideFace::U, Direction::Double, 2)
    }

    /// Creates a standard Dw move (2 layers)
    pub fn dw() -> Self {
        Self::new(WideFace::D, Direction::Clockwise, 2)
    }

    /// Creates a standard Dw' move (2 layers)
    pub fn dw_prime() -> Self {
        Self::new(WideFace::D, Direction::CounterClockwise, 2)
    }

    /// Creates a standard Dw2 move (2 layers)
    pub fn dw2() -> Self {
        Self::new(WideFace::D, Direction::Double, 2)
    }

    /// Creates a standard Fw move (2 layers)
    pub fn fw() -> Self {
        Self::new(WideFace::F, Direction::Clockwise, 2)
    }

    /// Creates a standard Fw' move (2 layers)
    pub fn fw_prime() -> Self {
        Self::new(WideFace::F, Direction::CounterClockwise, 2)
    }

    /// Creates a standard Fw2 move (2 layers)
    pub fn fw2() -> Self {
        Self::new(WideFace::F, Direction::Double, 2)
    }

    /// Creates a standard Bw move (2 layers)
    pub fn bw() -> Self {
        Self::new(WideFace::B, Direction::Clockwise, 2)
    }

    /// Creates a standard Bw' move (2 layers)
    pub fn bw_prime() -> Self {
        Self::new(WideFace::B, Direction::CounterClockwise, 2)
    }

    /// Creates a standard Bw2 move (2 layers)
    pub fn bw2() -> Self {
        Self::new(WideFace::B, Direction::Double, 2)
    }

    /// Returns the inverse of this wide move
    pub fn inverse(&self) -> Self {
        Self {
            face: self.face,
            direction: match self.direction {
                Direction::Clockwise => Direction::CounterClockwise,
                Direction::CounterClockwise => Direction::Clockwise,
                Direction::Double => Direction::Double,
            },
            depth: self.depth,
        }
    }

    /// Returns the notation string for this move
    pub fn to_notation(&self) -> String {
        let face_char = match self.face {
            WideFace::R => "R",
            WideFace::L => "L",
            WideFace::U => "U",
            WideFace::D => "D",
            WideFace::F => "F",
            WideFace::B => "B",
        };

        let depth_prefix = if self.depth > 2 {
            format!("{}", self.depth)
        } else {
            String::new()
        };

        let direction_suffix = match self.direction {
            Direction::Clockwise => "",
            Direction::CounterClockwise => "'",
            Direction::Double => "2",
        };

        format!("{}{}w{}", depth_prefix, face_char, direction_suffix)
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
            Move::M => self.move_m(),
            Move::MPrime => self.move_m_prime(),
            Move::M2 => {
                self.move_m();
                self.move_m();
            }
            Move::E => self.move_e(),
            Move::EPrime => self.move_e_prime(),
            Move::E2 => {
                self.move_e();
                self.move_e();
            }
            Move::S => self.move_s(),
            Move::SPrime => self.move_s_prime(),
            Move::S2 => {
                self.move_s();
                self.move_s();
            }
        }
    }

    /// Applies a sequence of moves to the cube
    pub fn apply_moves(&mut self, moves: &[Move]) {
        for mv in moves {
            self.apply_move(*mv);
        }
    }

    /// Applies a wide move to the cube
    ///
    /// Wide moves rotate multiple layers. For example, Rw rotates the R face
    /// and the adjacent inner layer together.
    ///
    /// # Panics
    /// Panics if the cube size is less than 3 (wide moves require at least 3x3)
    /// Panics if the depth is larger than half the cube size
    pub fn apply_wide_move(&mut self, wide_move: WideMove) {
        let n = self.size();
        assert!(n >= 3, "Wide moves require at least a 3x3 cube");
        assert!(
            wide_move.depth <= n / 2 + n % 2,
            "Wide move depth cannot exceed half the cube size"
        );
        assert!(wide_move.depth >= 1, "Wide move depth must be at least 1");

        match wide_move.direction {
            Direction::Clockwise => self.apply_wide_move_cw(wide_move.face, wide_move.depth),
            Direction::CounterClockwise => {
                self.apply_wide_move_ccw(wide_move.face, wide_move.depth)
            }
            Direction::Double => {
                self.apply_wide_move_cw(wide_move.face, wide_move.depth);
                self.apply_wide_move_cw(wide_move.face, wide_move.depth);
            }
        }
    }

    /// Applies a clockwise wide move for the specified face and depth
    fn apply_wide_move_cw(&mut self, face: WideFace, depth: usize) {
        match face {
            WideFace::R => self.wide_r_cw(depth),
            WideFace::L => self.wide_l_cw(depth),
            WideFace::U => self.wide_u_cw(depth),
            WideFace::D => self.wide_d_cw(depth),
            WideFace::F => self.wide_f_cw(depth),
            WideFace::B => self.wide_b_cw(depth),
        }
    }

    /// Applies a counter-clockwise wide move for the specified face and depth
    fn apply_wide_move_ccw(&mut self, face: WideFace, depth: usize) {
        match face {
            WideFace::R => self.wide_r_ccw(depth),
            WideFace::L => self.wide_l_ccw(depth),
            WideFace::U => self.wide_u_ccw(depth),
            WideFace::D => self.wide_d_ccw(depth),
            WideFace::F => self.wide_f_ccw(depth),
            WideFace::B => self.wide_b_ccw(depth),
        }
    }

    /// Wide R move clockwise: rotates R face and `depth` layers
    fn wide_r_cw(&mut self, depth: usize) {
        let n = self.size();

        // Rotate the R face clockwise
        self.right.rotate_cw();

        // Cycle columns for each layer from the right edge inward
        for layer in 0..depth {
            let col_idx = n - 1 - layer;
            let back_col_idx = layer;

            let up_col = self.up.get_col(col_idx);
            let front_col = self.front.get_col(col_idx);
            let down_col = self.down.get_col(col_idx);
            let back_col: Vec<Color> = self.back.get_col(back_col_idx).into_iter().rev().collect();

            self.front.set_col(col_idx, up_col);
            self.down.set_col(col_idx, front_col);
            self.back
                .set_col(back_col_idx, down_col.into_iter().rev().collect());
            self.up.set_col(col_idx, back_col);
        }
    }

    /// Wide R move counter-clockwise: rotates R face and `depth` layers
    fn wide_r_ccw(&mut self, depth: usize) {
        let n = self.size();

        // Rotate the R face counter-clockwise
        self.right.rotate_ccw();

        // Cycle columns for each layer from the right edge inward
        for layer in 0..depth {
            let col_idx = n - 1 - layer;
            let back_col_idx = layer;

            let up_col = self.up.get_col(col_idx);
            let front_col = self.front.get_col(col_idx);
            let down_col = self.down.get_col(col_idx);
            let back_col: Vec<Color> = self.back.get_col(back_col_idx).into_iter().rev().collect();

            self.back
                .set_col(back_col_idx, up_col.into_iter().rev().collect());
            self.down.set_col(col_idx, back_col);
            self.front.set_col(col_idx, down_col);
            self.up.set_col(col_idx, front_col);
        }
    }

    /// Wide L move clockwise: rotates L face and `depth` layers
    fn wide_l_cw(&mut self, depth: usize) {
        let n = self.size();

        // Rotate the L face clockwise
        self.left.rotate_cw();

        // Cycle columns for each layer from the left edge inward
        for layer in 0..depth {
            let col_idx = layer;
            let back_col_idx = n - 1 - layer;

            let up_col = self.up.get_col(col_idx);
            let front_col = self.front.get_col(col_idx);
            let down_col = self.down.get_col(col_idx);
            let back_col: Vec<Color> = self.back.get_col(back_col_idx).into_iter().rev().collect();

            self.back
                .set_col(back_col_idx, up_col.into_iter().rev().collect());
            self.down.set_col(col_idx, back_col);
            self.front.set_col(col_idx, down_col);
            self.up.set_col(col_idx, front_col);
        }
    }

    /// Wide L move counter-clockwise: rotates L face and `depth` layers
    fn wide_l_ccw(&mut self, depth: usize) {
        let n = self.size();

        // Rotate the L face counter-clockwise
        self.left.rotate_ccw();

        // Cycle columns for each layer from the left edge inward
        for layer in 0..depth {
            let col_idx = layer;
            let back_col_idx = n - 1 - layer;

            let up_col = self.up.get_col(col_idx);
            let front_col = self.front.get_col(col_idx);
            let down_col = self.down.get_col(col_idx);
            let back_col: Vec<Color> = self.back.get_col(back_col_idx).into_iter().rev().collect();

            self.front.set_col(col_idx, up_col);
            self.down.set_col(col_idx, front_col);
            self.back
                .set_col(back_col_idx, down_col.into_iter().rev().collect());
            self.up.set_col(col_idx, back_col);
        }
    }

    /// Wide U move clockwise: rotates U face and `depth` layers
    fn wide_u_cw(&mut self, depth: usize) {
        // Rotate the U face clockwise
        self.up.rotate_cw();

        // Cycle rows for each layer from the top edge downward
        for layer in 0..depth {
            let row_idx = layer;

            let front_row = self.front.get_row(row_idx);
            let left_row = self.left.get_row(row_idx);
            let back_row = self.back.get_row(row_idx);
            let right_row = self.right.get_row(row_idx);

            self.right.set_row(row_idx, front_row);
            self.back.set_row(row_idx, right_row);
            self.left.set_row(row_idx, back_row);
            self.front.set_row(row_idx, left_row);
        }
    }

    /// Wide U move counter-clockwise: rotates U face and `depth` layers
    fn wide_u_ccw(&mut self, depth: usize) {
        // Rotate the U face counter-clockwise
        self.up.rotate_ccw();

        // Cycle rows for each layer from the top edge downward
        for layer in 0..depth {
            let row_idx = layer;

            let front_row = self.front.get_row(row_idx);
            let left_row = self.left.get_row(row_idx);
            let back_row = self.back.get_row(row_idx);
            let right_row = self.right.get_row(row_idx);

            self.left.set_row(row_idx, front_row);
            self.back.set_row(row_idx, left_row);
            self.right.set_row(row_idx, back_row);
            self.front.set_row(row_idx, right_row);
        }
    }

    /// Wide D move clockwise: rotates D face and `depth` layers
    fn wide_d_cw(&mut self, depth: usize) {
        let n = self.size();

        // Rotate the D face clockwise
        self.down.rotate_cw();

        // Cycle rows for each layer from the bottom edge upward
        for layer in 0..depth {
            let row_idx = n - 1 - layer;

            let front_row = self.front.get_row(row_idx);
            let left_row = self.left.get_row(row_idx);
            let back_row = self.back.get_row(row_idx);
            let right_row = self.right.get_row(row_idx);

            self.left.set_row(row_idx, front_row);
            self.back.set_row(row_idx, left_row);
            self.right.set_row(row_idx, back_row);
            self.front.set_row(row_idx, right_row);
        }
    }

    /// Wide D move counter-clockwise: rotates D face and `depth` layers
    fn wide_d_ccw(&mut self, depth: usize) {
        let n = self.size();

        // Rotate the D face counter-clockwise
        self.down.rotate_ccw();

        // Cycle rows for each layer from the bottom edge upward
        for layer in 0..depth {
            let row_idx = n - 1 - layer;

            let front_row = self.front.get_row(row_idx);
            let left_row = self.left.get_row(row_idx);
            let back_row = self.back.get_row(row_idx);
            let right_row = self.right.get_row(row_idx);

            self.right.set_row(row_idx, front_row);
            self.back.set_row(row_idx, right_row);
            self.left.set_row(row_idx, back_row);
            self.front.set_row(row_idx, left_row);
        }
    }

    /// Wide F move clockwise: rotates F face and `depth` layers
    fn wide_f_cw(&mut self, depth: usize) {
        let n = self.size();

        // Rotate the F face clockwise
        self.front.rotate_cw();

        // Cycle for each layer from the front face backward
        for layer in 0..depth {
            let up_row_idx = n - 1 - layer;
            let down_row_idx = layer;
            let right_col_idx = layer;
            let left_col_idx = n - 1 - layer;

            let up_row = self.up.get_row(up_row_idx);
            let right_col = self.right.get_col(right_col_idx);
            let down_row = self.down.get_row(down_row_idx);
            let left_col = self.left.get_col(left_col_idx);

            self.right.set_col(right_col_idx, up_row);
            self.down
                .set_row(down_row_idx, right_col.into_iter().rev().collect());
            self.left.set_col(left_col_idx, down_row);
            self.up
                .set_row(up_row_idx, left_col.into_iter().rev().collect());
        }
    }

    /// Wide F move counter-clockwise: rotates F face and `depth` layers
    fn wide_f_ccw(&mut self, depth: usize) {
        let n = self.size();

        // Rotate the F face counter-clockwise
        self.front.rotate_ccw();

        // Cycle for each layer from the front face backward
        for layer in 0..depth {
            let up_row_idx = n - 1 - layer;
            let down_row_idx = layer;
            let right_col_idx = layer;
            let left_col_idx = n - 1 - layer;

            let up_row = self.up.get_row(up_row_idx);
            let right_col = self.right.get_col(right_col_idx);
            let down_row = self.down.get_row(down_row_idx);
            let left_col = self.left.get_col(left_col_idx);

            self.left
                .set_col(left_col_idx, up_row.into_iter().rev().collect());
            self.down.set_row(down_row_idx, left_col);
            self.right
                .set_col(right_col_idx, down_row.into_iter().rev().collect());
            self.up.set_row(up_row_idx, right_col);
        }
    }

    /// Wide B move clockwise: rotates B face and `depth` layers
    fn wide_b_cw(&mut self, depth: usize) {
        let n = self.size();

        // Rotate the B face clockwise
        self.back.rotate_cw();

        // Cycle for each layer from the back face forward
        for layer in 0..depth {
            let up_row_idx = layer;
            let down_row_idx = n - 1 - layer;
            let left_col_idx = layer;
            let right_col_idx = n - 1 - layer;

            let up_row = self.up.get_row(up_row_idx);
            let left_col = self.left.get_col(left_col_idx);
            let down_row = self.down.get_row(down_row_idx);
            let right_col = self.right.get_col(right_col_idx);

            self.left
                .set_col(left_col_idx, up_row.into_iter().rev().collect());
            self.down.set_row(down_row_idx, left_col);
            self.right
                .set_col(right_col_idx, down_row.into_iter().rev().collect());
            self.up.set_row(up_row_idx, right_col);
        }
    }

    /// Wide B move counter-clockwise: rotates B face and `depth` layers
    fn wide_b_ccw(&mut self, depth: usize) {
        let n = self.size();

        // Rotate the B face counter-clockwise
        self.back.rotate_ccw();

        // Cycle for each layer from the back face forward
        for layer in 0..depth {
            let up_row_idx = layer;
            let down_row_idx = n - 1 - layer;
            let left_col_idx = layer;
            let right_col_idx = n - 1 - layer;

            let up_row = self.up.get_row(up_row_idx);
            let left_col = self.left.get_col(left_col_idx);
            let down_row = self.down.get_row(down_row_idx);
            let right_col = self.right.get_col(right_col_idx);

            self.right.set_col(right_col_idx, up_row);
            self.down
                .set_row(down_row_idx, right_col.into_iter().rev().collect());
            self.left.set_col(left_col_idx, down_row);
            self.up
                .set_row(up_row_idx, left_col.into_iter().rev().collect());
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

    /// M slice move: Middle slice between L and R (turns like L)
    /// Only works on odd-sized cubes (3x3, 5x5, 7x7, etc.)
    ///
    /// # Panics
    /// Panics if the cube is even-sized (no middle slice exists)
    fn move_m(&mut self) {
        let n = self.size();
        if n % 2 == 0 {
            panic!("M slice moves only work on odd-sized cubes");
        }

        let mid = n / 2;

        // M turns like L, following the same cycle as L
        // Cycle: Up -> Back (reversed) -> Down -> Front -> Up
        let up_col = self.up.get_col(mid);
        let front_col = self.front.get_col(mid);
        let down_col = self.down.get_col(mid);
        let back_col: Vec<Color> = self.back.get_col(mid).into_iter().rev().collect();

        // Up middle col -> Back middle col (reversed)
        self.back.set_col(mid, up_col.into_iter().rev().collect());
        // Back middle col (reversed) -> Down middle col
        self.down.set_col(mid, back_col);
        // Down middle col -> Front middle col
        self.front.set_col(mid, down_col);
        // Front middle col -> Up middle col
        self.up.set_col(mid, front_col);
    }

    /// M' slice move: Middle slice counter-clockwise
    fn move_m_prime(&mut self) {
        let n = self.size();
        if n % 2 == 0 {
            panic!("M slice moves only work on odd-sized cubes");
        }

        let mid = n / 2;

        // M' turns opposite to M, reversing the cycle
        // Cycle: Up -> Front -> Down -> Back (reversed) -> Up
        let up_col = self.up.get_col(mid);
        let front_col = self.front.get_col(mid);
        let down_col = self.down.get_col(mid);
        let back_col: Vec<Color> = self.back.get_col(mid).into_iter().rev().collect();

        // Up middle col -> Front middle col
        self.front.set_col(mid, up_col);
        // Front middle col -> Down middle col
        self.down.set_col(mid, front_col);
        // Down middle col -> Back middle col (reversed)
        self.back.set_col(mid, down_col.into_iter().rev().collect());
        // Back middle col (reversed) -> Up middle col
        self.up.set_col(mid, back_col);
    }

    /// E slice move: Equator slice between U and D (turns like D)
    /// Only works on odd-sized cubes (3x3, 5x5, 7x7, etc.)
    ///
    /// # Panics
    /// Panics if the cube is even-sized (no middle slice exists)
    fn move_e(&mut self) {
        let n = self.size();
        if n % 2 == 0 {
            panic!("E slice moves only work on odd-sized cubes");
        }

        let mid = n / 2;

        // E turns like D, so it follows the same cycle as D
        // Cycle: Front middle row -> Right middle row -> Back middle row -> Left middle row -> Front middle row
        let front_row = self.front.get_row(mid);
        let right_row = self.right.get_row(mid);
        let back_row = self.back.get_row(mid);
        let left_row = self.left.get_row(mid);

        // Front middle row -> Left middle row
        self.left.set_row(mid, front_row);
        // Left middle row -> Back middle row
        self.back.set_row(mid, left_row);
        // Back middle row -> Right middle row
        self.right.set_row(mid, back_row);
        // Right middle row -> Front middle row
        self.front.set_row(mid, right_row);
    }

    /// E' slice move: Equator slice counter-clockwise
    fn move_e_prime(&mut self) {
        let n = self.size();
        if n % 2 == 0 {
            panic!("E slice moves only work on odd-sized cubes");
        }

        let mid = n / 2;

        // E' turns opposite to D'
        // Cycle: Front middle row -> Left middle row -> Back middle row -> Right middle row -> Front middle row
        let front_row = self.front.get_row(mid);
        let right_row = self.right.get_row(mid);
        let back_row = self.back.get_row(mid);
        let left_row = self.left.get_row(mid);

        // Front middle row -> Right middle row
        self.right.set_row(mid, front_row);
        // Right middle row -> Back middle row
        self.back.set_row(mid, right_row);
        // Back middle row -> Left middle row
        self.left.set_row(mid, back_row);
        // Left middle row -> Front middle row
        self.front.set_row(mid, left_row);
    }

    /// S slice move: Standing slice between F and B (turns like F)
    /// Only works on odd-sized cubes (3x3, 5x5, 7x7, etc.)
    ///
    /// # Panics
    /// Panics if the cube is even-sized (no middle slice exists)
    fn move_s(&mut self) {
        let n = self.size();
        if n % 2 == 0 {
            panic!("S slice moves only work on odd-sized cubes");
        }

        let mid = n / 2;

        // S turns like F, so it follows the same cycle as F
        // Cycle: Up middle row -> Right middle col -> Down middle row -> Left middle col -> Up middle row
        let up_row = self.up.get_row(mid);
        let right_col = self.right.get_col(mid);
        let down_row = self.down.get_row(mid);
        let left_col = self.left.get_col(mid);

        // Up middle row -> Right middle col
        self.right.set_col(mid, up_row);
        // Right middle col -> Down middle row (reversed)
        self.down.set_row(mid, right_col.into_iter().rev().collect());
        // Down middle row -> Left middle col
        self.left.set_col(mid, down_row);
        // Left middle col -> Up middle row (reversed)
        self.up.set_row(mid, left_col.into_iter().rev().collect());
    }

    /// S' slice move: Standing slice counter-clockwise
    fn move_s_prime(&mut self) {
        let n = self.size();
        if n % 2 == 0 {
            panic!("S slice moves only work on odd-sized cubes");
        }

        let mid = n / 2;

        // S' turns opposite to F'
        // Cycle: Up middle row -> Left middle col -> Down middle row -> Right middle col -> Up middle row
        let up_row = self.up.get_row(mid);
        let right_col = self.right.get_col(mid);
        let down_row = self.down.get_row(mid);
        let left_col = self.left.get_col(mid);

        // Up middle row -> Left middle col (reversed)
        self.left.set_col(mid, up_row.into_iter().rev().collect());
        // Left middle col -> Down middle row
        self.down.set_row(mid, left_col);
        // Down middle row -> Right middle col (reversed)
        self.right.set_col(mid, down_row.into_iter().rev().collect());
        // Right middle col -> Up middle row
        self.up.set_row(mid, right_col);
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

    // ===== Wide Move Tests (R1.3) =====

    #[test]
    fn cube_014_rw_move_on_4x4() {
        // Test Rw (wide R) on 4x4
        let mut cube = Cube::new(4);
        cube.apply_wide_move(WideMove::rw());

        // R face should be rotated
        assert!(cube.right.is_solved());

        // Check that 2 columns were moved (outer and inner)
        // Rightmost column (col 3) and second-rightmost column (col 2)
        // should have been cycled
        assert_eq!(cube.front.get_col(3), vec![Color::White; 4]);
        assert_eq!(cube.front.get_col(2), vec![Color::White; 4]);
    }

    #[test]
    fn cube_015_lw_move_on_4x4() {
        // Test Lw (wide L) on 4x4
        let mut cube = Cube::new(4);
        cube.apply_wide_move(WideMove::lw());

        // L face should be rotated
        assert!(cube.left.is_solved());

        // Check that 2 columns were moved
        assert_eq!(cube.front.get_col(0), vec![Color::Yellow; 4]);
        assert_eq!(cube.front.get_col(1), vec![Color::Yellow; 4]);
    }

    #[test]
    fn cube_016_3rw_move_on_5x5() {
        // Test 3Rw (3-wide R) on 5x5
        let mut cube = Cube::new(5);
        cube.apply_wide_move(WideMove::new(WideFace::R, Direction::Clockwise, 3));

        // R face should be rotated
        assert!(cube.right.is_solved());

        // Check that 3 columns were moved
        assert_eq!(cube.front.get_col(4), vec![Color::White; 5]);
        assert_eq!(cube.front.get_col(3), vec![Color::White; 5]);
        assert_eq!(cube.front.get_col(2), vec![Color::White; 5]);
    }

    #[test]
    fn test_rw_inverse_returns_to_solved() {
        // Rw followed by Rw' should return to solved
        let mut cube = Cube::new(4);
        cube.apply_wide_move(WideMove::rw());
        cube.apply_wide_move(WideMove::rw_prime());
        assert!(cube.is_solved());
    }

    #[test]
    fn test_lw_inverse_returns_to_solved() {
        let mut cube = Cube::new(4);
        cube.apply_wide_move(WideMove::lw());
        cube.apply_wide_move(WideMove::lw_prime());
        assert!(cube.is_solved());
    }

    #[test]
    fn test_uw_inverse_returns_to_solved() {
        let mut cube = Cube::new(4);
        cube.apply_wide_move(WideMove::uw());
        cube.apply_wide_move(WideMove::uw_prime());
        assert!(cube.is_solved());
    }

    #[test]
    fn test_dw_inverse_returns_to_solved() {
        let mut cube = Cube::new(4);
        cube.apply_wide_move(WideMove::dw());
        cube.apply_wide_move(WideMove::dw_prime());
        assert!(cube.is_solved());
    }

    #[test]
    fn test_fw_inverse_returns_to_solved() {
        let mut cube = Cube::new(4);
        cube.apply_wide_move(WideMove::fw());
        cube.apply_wide_move(WideMove::fw_prime());
        assert!(cube.is_solved());
    }

    #[test]
    fn test_bw_inverse_returns_to_solved() {
        let mut cube = Cube::new(4);
        cube.apply_wide_move(WideMove::bw());
        cube.apply_wide_move(WideMove::bw_prime());
        assert!(cube.is_solved());
    }

    #[test]
    fn test_rw2_double_returns_to_solved() {
        // Rw2 followed by Rw2 should return to solved
        let mut cube = Cube::new(4);
        cube.apply_wide_move(WideMove::rw2());
        cube.apply_wide_move(WideMove::rw2());
        assert!(cube.is_solved());
    }

    #[test]
    fn test_all_wide_moves_double_return_to_solved() {
        let wide_moves = [
            WideMove::rw2(),
            WideMove::lw2(),
            WideMove::uw2(),
            WideMove::dw2(),
            WideMove::fw2(),
            WideMove::bw2(),
        ];

        for mv in wide_moves.iter() {
            let mut cube = Cube::new(4);
            cube.apply_wide_move(*mv);
            cube.apply_wide_move(*mv);
            assert!(
                cube.is_solved(),
                "Failed for wide move: {:?}",
                mv.to_notation()
            );
        }
    }

    #[test]
    fn test_wide_moves_on_5x5() {
        // Test all wide moves and their inverses on 5x5
        let mut cube = Cube::new(5);

        cube.apply_wide_move(WideMove::rw());
        cube.apply_wide_move(WideMove::uw());
        cube.apply_wide_move(WideMove::rw_prime());
        cube.apply_wide_move(WideMove::uw_prime());

        // Apply inverse sequence to return to solved
        cube.apply_wide_move(WideMove::uw());
        cube.apply_wide_move(WideMove::rw());
        cube.apply_wide_move(WideMove::uw_prime());
        cube.apply_wide_move(WideMove::rw_prime());

        assert!(cube.is_solved());
    }

    #[test]
    fn test_3_layer_wide_move_on_6x6() {
        // Test 3-layer wide move on 6x6
        let mut cube = Cube::new(6);
        let mv = WideMove::new(WideFace::R, Direction::Clockwise, 3);
        cube.apply_wide_move(mv);
        cube.apply_wide_move(mv.inverse());
        assert!(cube.is_solved());
    }

    #[test]
    fn test_wide_move_notation() {
        assert_eq!(WideMove::rw().to_notation(), "Rw");
        assert_eq!(WideMove::rw_prime().to_notation(), "Rw'");
        assert_eq!(WideMove::rw2().to_notation(), "Rw2");
        assert_eq!(WideMove::lw().to_notation(), "Lw");

        // Test depth-prefixed notation
        let wide_3 = WideMove::new(WideFace::R, Direction::Clockwise, 3);
        assert_eq!(wide_3.to_notation(), "3Rw");

        let wide_3_prime = WideMove::new(WideFace::U, Direction::CounterClockwise, 4);
        assert_eq!(wide_3_prime.to_notation(), "4Uw'");
    }

    #[test]
    fn test_wide_move_inverse() {
        let rw = WideMove::rw();
        let rw_inv = rw.inverse();
        assert_eq!(rw_inv.direction, Direction::CounterClockwise);
        assert_eq!(rw_inv.face, WideFace::R);
        assert_eq!(rw_inv.depth, 2);

        let rw2 = WideMove::rw2();
        let rw2_inv = rw2.inverse();
        assert_eq!(rw2_inv.direction, Direction::Double); // Double is its own inverse
    }

    #[test]
    fn test_wide_uw_on_4x4() {
        // Test Uw (wide U) on 4x4
        let mut cube = Cube::new(4);
        cube.apply_wide_move(WideMove::uw());

        // U face should be rotated
        assert!(cube.up.is_solved());

        // Check that 2 rows were cycled (top and second row)
        assert_eq!(cube.right.get_row(0), vec![Color::Green; 4]);
        assert_eq!(cube.right.get_row(1), vec![Color::Green; 4]);
    }

    #[test]
    fn test_wide_dw_on_4x4() {
        // Test Dw (wide D) on 4x4
        let mut cube = Cube::new(4);
        cube.apply_wide_move(WideMove::dw());

        // D face should be rotated
        assert!(cube.down.is_solved());

        // Check that 2 bottom rows were cycled
        assert_eq!(cube.left.get_row(3), vec![Color::Green; 4]);
        assert_eq!(cube.left.get_row(2), vec![Color::Green; 4]);
    }

    #[test]
    fn test_wide_fw_on_4x4() {
        // Test Fw (wide F) on 4x4
        let mut cube = Cube::new(4);
        cube.apply_wide_move(WideMove::fw());

        // F face should be rotated
        assert!(cube.front.is_solved());

        // Check that adjacent faces were affected
        assert_eq!(cube.right.get_col(0), vec![Color::White; 4]);
        assert_eq!(cube.right.get_col(1), vec![Color::White; 4]);
    }

    #[test]
    fn test_wide_bw_on_4x4() {
        // Test Bw (wide B) on 4x4
        let mut cube = Cube::new(4);
        cube.apply_wide_move(WideMove::bw());

        // B face should be rotated
        assert!(cube.back.is_solved());

        // Bw should move the back 2 layers
        // This cycles Up top row, Left left col, Down bottom row, Right right col
    }

    #[test]
    #[should_panic(expected = "Wide moves require at least a 3x3 cube")]
    fn test_wide_move_on_2x2_panics() {
        let mut cube = Cube::new(2);
        cube.apply_wide_move(WideMove::rw());
    }

    #[test]
    fn test_wide_moves_preserve_color_counts() {
        // Verify that wide moves don't create or destroy stickers
        let mut cube = Cube::new(5);

        cube.apply_wide_move(WideMove::rw());
        cube.apply_wide_move(WideMove::uw());
        cube.apply_wide_move(WideMove::fw());

        assert!(cube.has_valid_color_counts());
    }
}
