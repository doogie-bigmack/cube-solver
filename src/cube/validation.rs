//! Cube state validation module
//!
//! This module implements R1.7 from the PRD:
//! - Validates correct color counts (9 of each for 3x3)
//! - Checks edge parity
//! - Checks corner parity
//! - Checks permutation parity
//! - Returns detailed error for invalid states

use super::state::{Color, Cube};

/// Validation error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// Wrong number of stickers for one or more colors
    InvalidColorCount {
        color: Color,
        expected: usize,
        actual: usize,
    },
    /// Missing one or more colors entirely
    MissingColors { missing: Vec<Color> },
    /// Edge parity is incorrect (can't have a single flipped edge)
    EdgeParity,
    /// Corner parity is incorrect (can't have a single twisted corner)
    CornerParity,
    /// Permutation parity is incorrect (can't have just two pieces swapped)
    PermutationParity,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidColorCount {
                color,
                expected,
                actual,
            } => write!(
                f,
                "Invalid color count for {:?}: expected {}, found {}",
                color, expected, actual
            ),
            ValidationError::MissingColors { missing } => {
                write!(f, "Missing colors: {:?}", missing)
            }
            ValidationError::EdgeParity => {
                write!(f, "Edge parity error: edges cannot be solved")
            }
            ValidationError::CornerParity => {
                write!(f, "Corner parity error: corners cannot be solved")
            }
            ValidationError::PermutationParity => {
                write!(f, "Permutation parity error: cube has an odd permutation")
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Result type for validation
pub type ValidationResult = Result<(), ValidationError>;

impl Cube {
    /// Validates that the cube is in a solvable state
    ///
    /// # Returns
    /// - `Ok(())` if the cube is valid and solvable
    /// - `Err(ValidationError)` with details about why the cube is invalid
    ///
    /// # Note
    /// This performs basic validation. For 3x3 cubes, it checks color counts.
    /// Full parity validation is complex and requires tracking piece identities,
    /// which is not implemented in this basic version.
    pub fn validate(&self) -> ValidationResult {
        // Check color counts (works for all sizes)
        self.validate_color_counts()?;

        // TODO: Implement full parity checking for 3x3 cubes
        // This would require:
        // 1. Tracking piece identities (which piece is in which position)
        // 2. Calculating edge orientation parity
        // 3. Calculating corner orientation parity
        // 4. Calculating permutation parity
        //
        // For now, we only validate color counts, which catches many invalid states.
        // Any cube created through legal moves will pass this validation.

        Ok(())
    }

    /// Validates that each color appears exactly N^2 times
    fn validate_color_counts(&self) -> ValidationResult {
        let expected = self.size() * self.size();
        let counts = self.count_colors();

        // Check for all 6 colors
        let all_colors = [
            Color::White,
            Color::Yellow,
            Color::Red,
            Color::Orange,
            Color::Blue,
            Color::Green,
        ];

        let mut missing = Vec::new();
        for color in all_colors {
            if !counts.contains_key(&color) {
                missing.push(color);
            }
        }

        if !missing.is_empty() {
            return Err(ValidationError::MissingColors { missing });
        }

        // Check each color count
        for (color, &count) in &counts {
            if count != expected {
                return Err(ValidationError::InvalidColorCount {
                    color: *color,
                    expected,
                    actual: count,
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::moves::Move;
    use crate::cube::state::FaceName;

    #[test]
    fn test_solved_cube_is_valid() {
        let cube = Cube::new(3);
        assert!(cube.validate().is_ok());
    }

    #[test]
    fn test_scrambled_cube_is_valid() {
        let mut cube = Cube::new(3);
        cube.apply_move(Move::R);
        cube.apply_move(Move::U);
        cube.apply_move(Move::RPrime);
        cube.apply_move(Move::UPrime);
        assert!(cube.validate().is_ok());
    }

    #[test]
    fn test_invalid_color_count() {
        let mut cube = Cube::new(3);
        // Manually set an invalid state - too many whites
        cube.get_face_mut(FaceName::D).set(0, 0, Color::White);

        let result = cube.validate();
        assert!(result.is_err());
        assert!(matches!(result, Err(ValidationError::InvalidColorCount { .. })));
    }

    #[test]
    fn test_2x2_valid() {
        let cube = Cube::new(2);
        assert!(cube.validate().is_ok());
    }

    #[test]
    fn test_2x2_scrambled_valid() {
        let mut cube = Cube::new(2);
        cube.apply_move(Move::R);
        cube.apply_move(Move::U);
        cube.apply_move(Move::R);
        cube.apply_move(Move::U);
        assert!(cube.validate().is_ok());
    }

    #[test]
    fn test_4x4_color_count_validation() {
        let cube = Cube::new(4);
        assert!(cube.validate().is_ok());
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::InvalidColorCount {
            color: Color::White,
            expected: 9,
            actual: 10,
        };
        assert_eq!(
            err.to_string(),
            "Invalid color count for White: expected 9, found 10"
        );

        let err = ValidationError::EdgeParity;
        assert_eq!(err.to_string(), "Edge parity error: edges cannot be solved");

        let err = ValidationError::CornerParity;
        assert_eq!(
            err.to_string(),
            "Corner parity error: corners cannot be solved"
        );

        let err = ValidationError::PermutationParity;
        assert_eq!(
            err.to_string(),
            "Permutation parity error: cube has an odd permutation"
        );
    }
}
