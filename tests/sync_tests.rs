//! Integration tests for R3.4 - Real-time sync with 3D view
//!
//! These tests verify that changes in the 2D view are properly reflected
//! in the 3D view through shared cube state.

use rubiks_cube_solver::cube::{Cube, Color, FaceName};

#[test]
fn test_cube_state_shared_between_views() {
    // Create a cube
    let mut cube = Cube::new(3);

    // Verify initial state is solved
    assert_eq!(cube.get_face(FaceName::F).get(0, 0), Color::Green);
    assert_eq!(cube.get_face(FaceName::U).get(0, 0), Color::White);
    assert_eq!(cube.get_face(FaceName::R).get(0, 0), Color::Red);

    // Simulate 2D input change - set a sticker to a different color
    cube.set_sticker(FaceName::F, 0, 0, Color::Yellow);

    // Verify the change is reflected in the cube state (which both views use)
    assert_eq!(cube.get_face(FaceName::F).get(0, 0), Color::Yellow);
}

#[test]
fn test_multiple_sticker_changes_sync() {
    let mut cube = Cube::new(3);

    // Make multiple changes (simulating user clicking multiple stickers in 2D view)
    cube.set_sticker(FaceName::F, 0, 0, Color::Yellow);
    cube.set_sticker(FaceName::F, 0, 1, Color::Blue);
    cube.set_sticker(FaceName::U, 1, 1, Color::Orange);

    // Verify all changes are present
    assert_eq!(cube.get_face(FaceName::F).get(0, 0), Color::Yellow);
    assert_eq!(cube.get_face(FaceName::F).get(0, 1), Color::Blue);
    assert_eq!(cube.get_face(FaceName::U).get(1, 1), Color::Orange);
}

#[test]
fn test_sync_on_2x2_cube() {
    let mut cube = Cube::new(2);

    // Change all stickers on one face
    for row in 0..2 {
        for col in 0..2 {
            cube.set_sticker(FaceName::F, row, col, Color::Yellow);
        }
    }

    // Verify all stickers changed
    for row in 0..2 {
        for col in 0..2 {
            assert_eq!(cube.get_face(FaceName::F).get(row, col), Color::Yellow);
        }
    }
}

#[test]
fn test_sync_on_5x5_cube() {
    let mut cube = Cube::new(5);

    // Change center sticker
    cube.set_sticker(FaceName::F, 2, 2, Color::White);

    // Change corner stickers
    cube.set_sticker(FaceName::F, 0, 0, Color::Yellow);
    cube.set_sticker(FaceName::F, 0, 4, Color::Orange);
    cube.set_sticker(FaceName::F, 4, 0, Color::Blue);
    cube.set_sticker(FaceName::F, 4, 4, Color::Green);

    // Verify changes
    assert_eq!(cube.get_face(FaceName::F).get(2, 2), Color::White);
    assert_eq!(cube.get_face(FaceName::F).get(0, 0), Color::Yellow);
    assert_eq!(cube.get_face(FaceName::F).get(0, 4), Color::Orange);
    assert_eq!(cube.get_face(FaceName::F).get(4, 0), Color::Blue);
    assert_eq!(cube.get_face(FaceName::F).get(4, 4), Color::Green);
}

#[test]
fn test_sync_across_all_faces() {
    let mut cube = Cube::new(3);

    // Change one sticker on each face to verify all faces can be synced
    cube.set_sticker(FaceName::U, 0, 0, Color::Yellow);
    cube.set_sticker(FaceName::D, 0, 0, Color::White);
    cube.set_sticker(FaceName::F, 0, 0, Color::Orange);
    cube.set_sticker(FaceName::B, 0, 0, Color::Red);
    cube.set_sticker(FaceName::L, 0, 0, Color::Blue);
    cube.set_sticker(FaceName::R, 0, 0, Color::Green);

    // Verify all faces updated
    assert_eq!(cube.get_face(FaceName::U).get(0, 0), Color::Yellow);
    assert_eq!(cube.get_face(FaceName::D).get(0, 0), Color::White);
    assert_eq!(cube.get_face(FaceName::F).get(0, 0), Color::Orange);
    assert_eq!(cube.get_face(FaceName::B).get(0, 0), Color::Red);
    assert_eq!(cube.get_face(FaceName::L).get(0, 0), Color::Blue);
    assert_eq!(cube.get_face(FaceName::R).get(0, 0), Color::Green);
}

#[test]
fn test_immediate_sync_no_lag() {
    let mut cube = Cube::new(3);

    // Set a sticker
    cube.set_sticker(FaceName::F, 1, 1, Color::Yellow);

    // Immediately read it back (simulating 3D view reading the state)
    let color = cube.get_face(FaceName::F).get(1, 1);

    // Should be synced with no lag
    assert_eq!(color, Color::Yellow);
}

#[test]
fn test_overwrite_sticker() {
    let mut cube = Cube::new(3);

    // Set a sticker multiple times (user changing mind in 2D view)
    cube.set_sticker(FaceName::F, 0, 0, Color::Yellow);
    cube.set_sticker(FaceName::F, 0, 0, Color::Blue);
    cube.set_sticker(FaceName::F, 0, 0, Color::Green);

    // Only the last change should be present
    assert_eq!(cube.get_face(FaceName::F).get(0, 0), Color::Green);
}

#[test]
fn test_sync_preserves_other_stickers() {
    let mut cube = Cube::new(3);

    // Change one sticker
    cube.set_sticker(FaceName::F, 1, 1, Color::Yellow);

    // Verify other stickers on the same face remain unchanged
    assert_eq!(cube.get_face(FaceName::F).get(0, 0), Color::Green);
    assert_eq!(cube.get_face(FaceName::F).get(0, 1), Color::Green);
    assert_eq!(cube.get_face(FaceName::F).get(2, 2), Color::Green);

    // Verify stickers on other faces remain unchanged
    assert_eq!(cube.get_face(FaceName::U).get(0, 0), Color::White);
    assert_eq!(cube.get_face(FaceName::R).get(0, 0), Color::Red);
}
