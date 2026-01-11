use rubiks_cube_solver::cube::{Cube, Move};

#[test]
fn cube_020_x_rotation() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::X);

    // After X rotation, the whole cube rotates as if doing R
    // So the view changes: front goes up, up goes back, back goes down, down goes front
    // We can verify this by doing X followed by X' returns to solved
    cube.apply_move(Move::XPrime);
    assert!(cube.is_solved());
}

#[test]
fn cube_021_y_rotation() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::Y);

    // After Y rotation, the whole cube rotates as if doing U
    // So the view changes: front goes right, right goes back, back goes left, left goes front
    // We can verify this by doing Y followed by Y' returns to solved
    cube.apply_move(Move::YPrime);
    assert!(cube.is_solved());
}

#[test]
fn cube_022_z_rotation() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::Z);

    // After Z rotation, the whole cube rotates as if doing F
    // So the view changes: up goes right, right goes down, down goes left, left goes up
    // We can verify this by doing Z followed by Z' returns to solved
    cube.apply_move(Move::ZPrime);
    assert!(cube.is_solved());
}

#[test]
fn test_x_rotation_order_4() {
    // X rotation should have order 4 (X^4 = identity)
    let mut cube = Cube::new(3);
    cube.apply_move(Move::X);
    cube.apply_move(Move::X);
    cube.apply_move(Move::X);
    cube.apply_move(Move::X);
    assert!(cube.is_solved());
}

#[test]
fn test_y_rotation_order_4() {
    // Y rotation should have order 4 (Y^4 = identity)
    let mut cube = Cube::new(3);
    cube.apply_move(Move::Y);
    cube.apply_move(Move::Y);
    cube.apply_move(Move::Y);
    cube.apply_move(Move::Y);
    assert!(cube.is_solved());
}

#[test]
fn test_z_rotation_order_4() {
    // Z rotation should have order 4 (Z^4 = identity)
    let mut cube = Cube::new(3);
    cube.apply_move(Move::Z);
    cube.apply_move(Move::Z);
    cube.apply_move(Move::Z);
    cube.apply_move(Move::Z);
    assert!(cube.is_solved());
}

#[test]
fn test_x2_rotation_order_2() {
    // X2 rotation should have order 2 (X2^2 = identity)
    let mut cube = Cube::new(3);
    cube.apply_move(Move::X2);
    cube.apply_move(Move::X2);
    assert!(cube.is_solved());
}

#[test]
fn test_y2_rotation_order_2() {
    // Y2 rotation should have order 2 (Y2^2 = identity)
    let mut cube = Cube::new(3);
    cube.apply_move(Move::Y2);
    cube.apply_move(Move::Y2);
    assert!(cube.is_solved());
}

#[test]
fn test_z2_rotation_order_2() {
    // Z2 rotation should have order 2 (Z2^2 = identity)
    let mut cube = Cube::new(3);
    cube.apply_move(Move::Z2);
    cube.apply_move(Move::Z2);
    assert!(cube.is_solved());
}

#[test]
fn test_combined_rotations() {
    // Test that rotations can be combined
    let mut cube = Cube::new(3);
    cube.apply_move(Move::X);
    cube.apply_move(Move::Y);
    cube.apply_move(Move::Z);

    // Reverse the sequence
    cube.apply_move(Move::ZPrime);
    cube.apply_move(Move::YPrime);
    cube.apply_move(Move::XPrime);

    assert!(cube.is_solved());
}

#[test]
fn test_rotations_on_2x2() {
    let mut cube = Cube::new(2);
    cube.apply_move(Move::X);
    cube.apply_move(Move::XPrime);
    assert!(cube.is_solved());

    cube.apply_move(Move::Y);
    cube.apply_move(Move::YPrime);
    assert!(cube.is_solved());

    cube.apply_move(Move::Z);
    cube.apply_move(Move::ZPrime);
    assert!(cube.is_solved());
}

#[test]
fn test_rotations_on_5x5() {
    let mut cube = Cube::new(5);
    cube.apply_move(Move::X);
    cube.apply_move(Move::XPrime);
    assert!(cube.is_solved());

    cube.apply_move(Move::Y);
    cube.apply_move(Move::YPrime);
    assert!(cube.is_solved());

    cube.apply_move(Move::Z);
    cube.apply_move(Move::ZPrime);
    assert!(cube.is_solved());
}

#[test]
fn test_rotation_commutes_with_itself() {
    // X and X should commute
    let mut cube1 = Cube::new(3);
    cube1.apply_move(Move::X);
    cube1.apply_move(Move::X);

    let mut cube2 = Cube::new(3);
    cube2.apply_move(Move::X);
    cube2.apply_move(Move::X);

    assert_eq!(cube1, cube2);
}

#[test]
fn test_rotations_preserve_color_counts() {
    let mut cube = Cube::new(3);

    cube.apply_move(Move::X);
    assert!(cube.has_valid_color_counts());

    cube.apply_move(Move::Y);
    assert!(cube.has_valid_color_counts());

    cube.apply_move(Move::Z);
    assert!(cube.has_valid_color_counts());
}
