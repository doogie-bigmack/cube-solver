//! 3x3 Rubik's Cube Kociemba Solver
//!
//! This implementation uses the Kociemba two-phase algorithm via the kewb crate.
//! The algorithm solves any 3x3 cube in at most 20 moves (God's number) in under 2 seconds.
//!
//! This module provides a bridge between our Cube representation and kewb's representation.

use crate::cube::{Cube, Color, Move};
use std::time::Instant;

/// Solution for a 3x3 cube using Kociemba algorithm
#[derive(Debug, Clone)]
pub struct Solution3x3 {
    /// List of moves to solve the cube
    pub moves: Vec<Move>,
    /// Time taken to find the solution (in milliseconds)
    pub time_ms: u128,
}

impl Solution3x3 {
    /// Create a new solution
    pub fn new(moves: Vec<Move>, time_ms: u128) -> Self {
        Self { moves, time_ms }
    }

    /// Get the number of moves in the solution
    pub fn move_count(&self) -> usize {
        self.moves.len()
    }
}

/// Solves a 3x3 Rubik's Cube using the Kociemba two-phase algorithm
///
/// # Arguments
/// * `cube` - The 3x3 cube to solve (must be size 3)
///
/// # Returns
/// * `Ok(Solution3x3)` - The solution with moves and timing
/// * `Err(String)` - If the cube is not solvable or not size 3
///
/// # Example
/// ```
/// use rubiks_cube_solver::cube::Cube;
/// use rubiks_cube_solver::solver::solve_3x3;
///
/// let mut cube = Cube::new(3);
/// // Scramble the cube...
/// let solution = solve_3x3(&cube).expect("Should solve");
/// assert!(solution.move_count() <= 20); // God's number
/// assert!(solution.time_ms < 2000); // Under 2 seconds
/// ```
pub fn solve_3x3(cube: &Cube) -> Result<Solution3x3, String> {
    let start = Instant::now();

    if cube.size() != 3 {
        return Err("Cube must be size 3 for 3x3 solver".to_string());
    }

    if !cube.validate().is_ok() {
        return Err("Cube is not in a valid state".to_string());
    }

    // If already solved, return empty solution
    if cube.is_solved() {
        let elapsed = start.elapsed().as_millis();
        return Ok(Solution3x3::new(vec![], elapsed));
    }

    // Convert our cube to kewb's FaceCube format
    let face_string = cube_to_face_string(cube);

    // Debug output
    eprintln!("DEBUG: Generated face string: {}", face_string);
    eprintln!("DEBUG: Length: {}", face_string.len());


    // Use kewb to solve
    let solution_moves = solve_with_kewb(&face_string)?;

    let elapsed = start.elapsed().as_millis();
    Ok(Solution3x3::new(solution_moves, elapsed))
}

/// Convert our Cube representation to kewb's face string format
///
/// kewb uses a 54-character string where each character represents which
/// face center that sticker belongs to (not the actual color).
///
/// - Characters 0-8: Up face stickers
/// - Characters 9-17: Right face stickers
/// - Characters 18-26: Front face stickers
/// - Characters 27-35: Down face stickers
/// - Characters 36-44: Left face stickers
/// - Characters 45-53: Back face stickers
///
/// Each character is determined by which face center has that color:
/// - 'U' = sticker matches Up center color (white)
/// - 'R' = sticker matches Right center color (red)
/// - 'F' = sticker matches Front center color (green)
/// - 'D' = sticker matches Down center color (yellow)
/// - 'L' = sticker matches Left center color (orange)
/// - 'B' = sticker matches Back center color (blue)
fn cube_to_face_string(cube: &Cube) -> String {
    let mut result = String::with_capacity(54);

    // Get center colors to determine face identities
    let up_center = cube.up.get(1, 1);      // White
    let right_center = cube.right.get(1, 1);  // Red
    let front_center = cube.front.get(1, 1);  // Green
    let down_center = cube.down.get(1, 1);   // Yellow
    let left_center = cube.left.get(1, 1);   // Orange
    let back_center = cube.back.get(1, 1);   // Blue

    // Helper to convert color to kewb face letter based on center matching
    let color_to_char = |color: Color| -> char {
        if color == up_center {
            'U'
        } else if color == right_center {
            'R'
        } else if color == front_center {
            'F'
        } else if color == down_center {
            'D'
        } else if color == left_center {
            'L'
        } else if color == back_center {
            'B'
        } else {
            panic!("Unknown color: {:?}", color)
        }
    };

    // Up face - indices 0-8
    for row in 0..3 {
        for col in 0..3 {
            result.push(color_to_char(cube.up.get(row, col)));
        }
    }

    // Right face - indices 9-17
    for row in 0..3 {
        for col in 0..3 {
            result.push(color_to_char(cube.right.get(row, col)));
        }
    }

    // Front face - indices 18-26
    for row in 0..3 {
        for col in 0..3 {
            result.push(color_to_char(cube.front.get(row, col)));
        }
    }

    // Down face - indices 27-35
    for row in 0..3 {
        for col in 0..3 {
            result.push(color_to_char(cube.down.get(row, col)));
        }
    }

    // Left face - indices 36-44
    for row in 0..3 {
        for col in 0..3 {
            result.push(color_to_char(cube.left.get(row, col)));
        }
    }

    // Back face - indices 45-53
    for row in 0..3 {
        for col in 0..3 {
            result.push(color_to_char(cube.back.get(row, col)));
        }
    }

    result
}

/// Solve using the kewb library
fn solve_with_kewb(face_string: &str) -> Result<Vec<Move>, String> {
    // Parse the face string into a FaceCube
    let face_cube = kewb::FaceCube::try_from(face_string)
        .map_err(|e| format!("Invalid cube state: {:?}", e))?;

    // Convert FaceCube to State
    let state = kewb::State::try_from(&face_cube)
        .map_err(|e| format!("Invalid cube state: {:?}", e))?;

    // Solve using kewb's solve function
    // max_length: 21 (optimal solutions up to 20 moves)
    // timeout: 5.0 seconds (should be plenty for any 3x3)
    let solution = kewb::solve(state, 21, Some(5.0))
        .ok_or_else(|| "No solution found within constraints".to_string())?;

    // Convert kewb moves to our Move enum
    parse_kewb_solution(&solution.to_string())
}

/// Parse kewb's solution string into our Move enum
fn parse_kewb_solution(solution: &str) -> Result<Vec<Move>, String> {
    let mut moves = Vec::new();

    for move_str in solution.split_whitespace() {
        let m = match move_str {
            "U" => Move::U,
            "U'" => Move::UPrime,
            "U2" => Move::U2,
            "D" => Move::D,
            "D'" => Move::DPrime,
            "D2" => Move::D2,
            "R" => Move::R,
            "R'" => Move::RPrime,
            "R2" => Move::R2,
            "L" => Move::L,
            "L'" => Move::LPrime,
            "L2" => Move::L2,
            "F" => Move::F,
            "F'" => Move::FPrime,
            "F2" => Move::F2,
            "B" => Move::B,
            "B'" => Move::BPrime,
            "B2" => Move::B2,
            _ => return Err(format!("Unknown move: {}", move_str)),
        };
        moves.push(m);
    }

    Ok(moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solved_cube_returns_empty_solution() {
        let cube = Cube::new(3);
        let solution = solve_3x3(&cube).expect("Should solve");
        assert_eq!(solution.move_count(), 0);
        assert!(solution.time_ms < 100); // Very fast for solved cube
    }

    #[test]
    fn test_face_string_conversion_solved_cube() {
        let cube = Cube::new(3);
        let face_string = cube_to_face_string(&cube);

        // Debug: print the generated face string
        println!("Generated face string: {}", face_string);

        // Solved cube should have 9 of each letter
        assert_eq!(face_string.len(), 54);
        assert_eq!(face_string.matches('U').count(), 9);
        assert_eq!(face_string.matches('R').count(), 9);
        assert_eq!(face_string.matches('F').count(), 9);
        assert_eq!(face_string.matches('D').count(), 9);
        assert_eq!(face_string.matches('L').count(), 9);
        assert_eq!(face_string.matches('B').count(), 9);

        // Test that kewb can parse it
        let face_cube = kewb::FaceCube::try_from(face_string.as_str());
        assert!(face_cube.is_ok(), "kewb should accept solved cube string: {:?}", face_cube.err());
    }

    #[test]
    fn test_simple_scramble() {
        let mut cube = Cube::new(3);
        cube.apply_move(Move::R);
        cube.apply_move(Move::U);
        cube.apply_move(Move::RPrime);
        cube.apply_move(Move::UPrime);

        let solution = solve_3x3(&cube).expect("Should solve");
        assert!(solution.move_count() <= 20);
        assert!(solution.time_ms < 2000);

        // Apply solution and verify it solves the cube
        let mut test_cube = cube.clone();
        for m in &solution.moves {
            test_cube.apply_move(*m);
        }
        assert!(test_cube.is_solved());
    }

    #[test]
    fn test_rejects_non_3x3() {
        let cube = Cube::new(2);
        let result = solve_3x3(&cube);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("size 3"));
    }
}
    #[test]
    fn test_compare_with_kewb_scramble() {
        // Start with solved cube
        let mut our_cube = Cube::new(3);
        println!("
=== SOLVED STATE ===");
        let solved_str = cube_to_face_string(&our_cube);
        println!("Our solved cube: {}", solved_str);
        
        // Apply moves R U R' U'
        println!("
=== APPLYING MOVES: R U R' U' ===");
        our_cube.apply_move(Move::R);
        println!("After R:");
        println!("  {}", cube_to_face_string(&our_cube));
        
        our_cube.apply_move(Move::U);
        println!("After R U:");
        println!("  {}", cube_to_face_string(&our_cube));
        
        our_cube.apply_move(Move::RPrime);
        println!("After R U R':");
        println!("  {}", cube_to_face_string(&our_cube));
        
        our_cube.apply_move(Move::UPrime);
        println!("After R U R' U':");
        let our_final = cube_to_face_string(&our_cube);
        println!("  {}", our_final);
        
        // Now use kewb to see what it thinks
        println!("
=== KEWB'S VIEW ===");
        let kewb_moves = kewb::utils::scramble_from_string("R U R' U'").unwrap();
        println!("Kewb parsed moves: {:?}", kewb_moves);
        
        // Start with solved and apply kewb's moves
        let kewb_final_state = kewb::State::from(&kewb_moves);
        
        // We can't directly get a face string from kewb's State, so let's verify 
        // our cube can be solved
        println!("
=== VERIFICATION ===");
        println!("Our face string length: {}", our_final.len());
        println!("Our face string: {}", our_final);
        
        // Check that it only contains valid characters
        let valid_chars = ['U', 'R', 'F', 'D', 'L', 'B'];
        let mut invalid_found = false;
        for (i, c) in our_final.chars().enumerate() {
            if !valid_chars.contains(&c) {
                println!("  Invalid char at position {}: '{}' (expected one of URFLD B)", i, c);
                invalid_found = true;
            }
        }
        
        if !invalid_found {
            // Try to solve with kewb
            match kewb::FaceCube::try_from(our_final.as_str()) {
                Ok(face_cube) => {
                    match kewb::State::try_from(&face_cube) {
                        Ok(state) => {
                            match kewb::solve(state, 21, Some(5.0)) {
                                Some(solution) => {
                                    let total = solution.phase_1.len() + solution.phase_2.len();
                                    println!("  Success! Kewb can solve in {} moves", total);
                                    assert!(total > 0 && total < 20);
                                }
                                None => panic!("Kewb couldn't find a solution"),
                            }
                        }
                        Err(e) => panic!("Couldn't convert to State: {}", e),
                    }
                }
                Err(e) => panic!("Invalid face cube: {}", e),
            }
        } else {
            panic!("Face string contains invalid characters");
        }
    }

