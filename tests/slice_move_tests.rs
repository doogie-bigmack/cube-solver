// Integration tests for R1.4: Slice moves (M, E, S)
//
// Test IDs from test-plan.md:
// - cube_017: M slice move on 3x3
// - cube_018: E slice move on 3x3
// - cube_019: S slice move on 3x3

use rubiks_cube_solver::cube::{Cube, Move, Color};

#[test]
fn cube_017_m_slice_move_on_3x3() {
    // Test M slice (between L and R) on 3x3
    let mut cube = Cube::new(3);

    // Apply M move
    cube.apply_move(Move::M);

    // M turns like L, cycle: Front -> Up -> Back -> Down -> Front
    // (with reversals for the Back face due to orientation)

    // The Front face's middle column (Green) should go to Up
    assert_eq!(cube.up.get_col(1), vec![Color::Green; 3]);

    // The Up face's middle column (White) should go to Back (reversed)
    let back_col: Vec<Color> = cube.back.get_col(1).into_iter().rev().collect();
    assert_eq!(back_col, vec![Color::White; 3]);

    // The Back face's middle column (Blue, needs reversal) should go to Down
    assert_eq!(cube.down.get_col(1), vec![Color::Blue; 3]);

    // The Down face's middle column (Yellow) should go to Front
    assert_eq!(cube.front.get_col(1), vec![Color::Yellow; 3]);
}

#[test]
fn cube_018_e_slice_move_on_3x3() {
    // Test E slice (between U and D) on 3x3
    let mut cube = Cube::new(3);

    // Apply E move
    cube.apply_move(Move::E);

    // After E: middle row cycles Front -> Left -> Back -> Right -> Front
    // The Front face's middle row (Green) should go to Left
    assert_eq!(cube.left.get_row(1), vec![Color::Green; 3]);

    // The Left face's middle row (Orange) should go to Back
    assert_eq!(cube.back.get_row(1), vec![Color::Orange; 3]);

    // The Back face's middle row (Blue) should go to Right
    assert_eq!(cube.right.get_row(1), vec![Color::Blue; 3]);

    // The Right face's middle row (Red) should go to Front
    assert_eq!(cube.front.get_row(1), vec![Color::Red; 3]);
}

#[test]
fn cube_019_s_slice_move_on_3x3() {
    // Test S slice (between F and B) on 3x3
    let mut cube = Cube::new(3);

    // Apply S move
    cube.apply_move(Move::S);

    // After S: Up middle row -> Right middle col -> Down middle row (reversed) -> Left middle col -> Up middle row
    // The Up face's middle row (White) should go to Right middle col
    assert_eq!(cube.right.get_col(1), vec![Color::White; 3]);

    // The Right face's middle column (Red) should go to Down middle row (reversed)
    let down_row: Vec<Color> = cube.down.get_row(1).into_iter().rev().collect();
    assert_eq!(down_row, vec![Color::Red; 3]);

    // The Down face's middle row (Yellow, now reversed) should have gone to Left middle col
    assert_eq!(cube.left.get_col(1), vec![Color::Yellow; 3]);

    // The Left face's middle column (Orange, now reversed) should have gone to Up middle row
    let up_row: Vec<Color> = cube.up.get_row(1).into_iter().rev().collect();
    assert_eq!(up_row, vec![Color::Orange; 3]);
}

#[test]
fn test_m_inverse_returns_to_solved() {
    // Test that M followed by M' returns to solved state
    let mut cube = Cube::new(3);

    cube.apply_move(Move::M);
    cube.apply_move(Move::MPrime);

    assert!(cube.is_solved());
}

#[test]
fn test_e_inverse_returns_to_solved() {
    // Test that E followed by E' returns to solved state
    let mut cube = Cube::new(3);

    cube.apply_move(Move::E);
    cube.apply_move(Move::EPrime);

    assert!(cube.is_solved());
}

#[test]
fn test_s_inverse_returns_to_solved() {
    // Test that S followed by S' returns to solved state
    let mut cube = Cube::new(3);

    cube.apply_move(Move::S);
    cube.apply_move(Move::SPrime);

    assert!(cube.is_solved());
}

#[test]
fn test_m2_double_returns_to_solved() {
    // Test that M2 applied twice returns to solved state
    let mut cube = Cube::new(3);

    cube.apply_move(Move::M2);
    cube.apply_move(Move::M2);

    assert!(cube.is_solved());
}

#[test]
fn test_e2_double_returns_to_solved() {
    // Test that E2 applied twice returns to solved state
    let mut cube = Cube::new(3);

    cube.apply_move(Move::E2);
    cube.apply_move(Move::E2);

    assert!(cube.is_solved());
}

#[test]
fn test_s2_double_returns_to_solved() {
    // Test that S2 applied twice returns to solved state
    let mut cube = Cube::new(3);

    cube.apply_move(Move::S2);
    cube.apply_move(Move::S2);

    assert!(cube.is_solved());
}

#[test]
fn test_slice_moves_on_5x5() {
    // Test that slice moves work on 5x5 cubes
    let mut cube = Cube::new(5);

    cube.apply_move(Move::M);
    cube.apply_move(Move::E);
    cube.apply_move(Move::S);

    // Should not panic and colors should be preserved
    assert!(cube.has_valid_color_counts());
}

#[test]
fn test_slice_moves_on_7x7() {
    // Test that slice moves work on 7x7 cubes
    let mut cube = Cube::new(7);

    cube.apply_move(Move::M);
    cube.apply_move(Move::MPrime);
    cube.apply_move(Move::E);
    cube.apply_move(Move::EPrime);
    cube.apply_move(Move::S);
    cube.apply_move(Move::SPrime);

    // After inverse moves, should return to solved state
    assert!(cube.is_solved());
}

#[test]
#[should_panic(expected = "M slice moves only work on odd-sized cubes")]
fn test_m_on_2x2_panics() {
    let mut cube = Cube::new(2);
    cube.apply_move(Move::M);
}

#[test]
#[should_panic(expected = "M slice moves only work on odd-sized cubes")]
fn test_m_on_4x4_panics() {
    let mut cube = Cube::new(4);
    cube.apply_move(Move::M);
}

#[test]
#[should_panic(expected = "E slice moves only work on odd-sized cubes")]
fn test_e_on_4x4_panics() {
    let mut cube = Cube::new(4);
    cube.apply_move(Move::E);
}

#[test]
#[should_panic(expected = "S slice moves only work on odd-sized cubes")]
fn test_s_on_4x4_panics() {
    let mut cube = Cube::new(4);
    cube.apply_move(Move::S);
}

#[test]
fn test_slice_move_notation() {
    // Test notation strings for slice moves
    assert_eq!(Move::M.to_notation(), "M");
    assert_eq!(Move::MPrime.to_notation(), "M'");
    assert_eq!(Move::M2.to_notation(), "M2");
    assert_eq!(Move::E.to_notation(), "E");
    assert_eq!(Move::EPrime.to_notation(), "E'");
    assert_eq!(Move::E2.to_notation(), "E2");
    assert_eq!(Move::S.to_notation(), "S");
    assert_eq!(Move::SPrime.to_notation(), "S'");
    assert_eq!(Move::S2.to_notation(), "S2");
}

#[test]
fn test_slice_move_inverses() {
    // Test inverse moves
    assert_eq!(Move::M.inverse(), Move::MPrime);
    assert_eq!(Move::MPrime.inverse(), Move::M);
    assert_eq!(Move::M2.inverse(), Move::M2);
    assert_eq!(Move::E.inverse(), Move::EPrime);
    assert_eq!(Move::EPrime.inverse(), Move::E);
    assert_eq!(Move::E2.inverse(), Move::E2);
    assert_eq!(Move::S.inverse(), Move::SPrime);
    assert_eq!(Move::SPrime.inverse(), Move::S);
    assert_eq!(Move::S2.inverse(), Move::S2);
}

#[test]
fn test_slice_moves_preserve_color_counts() {
    // Verify that slice moves don't create or destroy stickers
    let mut cube = Cube::new(3);

    cube.apply_move(Move::M);
    assert!(cube.has_valid_color_counts());

    cube.apply_move(Move::E);
    assert!(cube.has_valid_color_counts());

    cube.apply_move(Move::S);
    assert!(cube.has_valid_color_counts());
}

#[test]
fn test_combined_slice_moves() {
    // Test combining different slice moves
    let mut cube = Cube::new(3);

    // Apply M E S sequence
    cube.apply_move(Move::M);
    cube.apply_move(Move::E);
    cube.apply_move(Move::S);

    // Should preserve color counts
    assert!(cube.has_valid_color_counts());

    // Apply inverses in reverse order to return to solved
    cube.apply_move(Move::SPrime);
    cube.apply_move(Move::EPrime);
    cube.apply_move(Move::MPrime);

    assert!(cube.is_solved());
}
