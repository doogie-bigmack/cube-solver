//! Integration tests for undo/redo history functionality (R3.7)
//!
//! This test suite verifies:
//! - Undo last color change
//! - Redo undone change
//! - Undo/redo history stack

use rubiks_cube_solver::cube::{Color, Cube, FaceName};
use rubiks_cube_solver::state::History;

#[test]
fn test_undo_last_color_change() {
    // R3.7: Undo last color change
    let cube = Cube::new(3);
    let mut history = History::new(cube.clone());

    // Make a color change
    let mut modified_cube = cube.clone();
    modified_cube.set_sticker(FaceName::F, 0, 0, Color::Red);
    history.push(modified_cube.clone());

    // Verify change was made
    assert_ne!(history.current(), &cube);
    assert_eq!(history.current(), &modified_cube);

    // Undo the change
    let undone = history.undo();
    assert!(undone.is_some());
    assert_eq!(history.current(), &cube);
}

#[test]
fn test_redo_undone_change() {
    // R3.7: Redo undone change
    let cube = Cube::new(3);
    let mut history = History::new(cube.clone());

    // Make a color change
    let mut modified_cube = cube.clone();
    modified_cube.set_sticker(FaceName::F, 0, 0, Color::Red);
    history.push(modified_cube.clone());

    // Undo the change
    history.undo();
    assert_eq!(history.current(), &cube);

    // Redo the change
    let redone = history.redo();
    assert!(redone.is_some());
    assert_eq!(history.current(), &modified_cube);
}

#[test]
fn test_undo_redo_history_stack() {
    // R3.7: Undo/redo history stack
    let cube1 = Cube::new(3);
    let mut history = History::new(cube1.clone());

    // Make multiple color changes
    let mut cube2 = cube1.clone();
    cube2.set_sticker(FaceName::F, 0, 0, Color::Red);
    history.push(cube2.clone());

    let mut cube3 = cube2.clone();
    cube3.set_sticker(FaceName::F, 0, 1, Color::Blue);
    history.push(cube3.clone());

    let mut cube4 = cube3.clone();
    cube4.set_sticker(FaceName::F, 1, 0, Color::Green);
    history.push(cube4.clone());

    // Current should be cube4
    assert_eq!(history.current(), &cube4);

    // Undo to cube3
    history.undo();
    assert_eq!(history.current(), &cube3);

    // Undo to cube2
    history.undo();
    assert_eq!(history.current(), &cube2);

    // Redo to cube3
    history.redo();
    assert_eq!(history.current(), &cube3);

    // Redo to cube4
    history.redo();
    assert_eq!(history.current(), &cube4);
}

#[test]
fn test_undo_redo_limits() {
    // Test that we can't undo/redo beyond limits
    let cube = Cube::new(3);
    let mut history = History::new(cube.clone());

    // Try to undo when nothing to undo
    assert!(!history.can_undo());
    let result = history.undo();
    assert!(result.is_none());

    // Try to redo when nothing to redo
    assert!(!history.can_redo());
    let result = history.redo();
    assert!(result.is_none());
}

#[test]
fn test_new_change_clears_redo_stack() {
    // Test that making a new change after undo clears the redo stack
    let cube1 = Cube::new(3);
    let mut history = History::new(cube1.clone());

    // Make first change
    let mut cube2 = cube1.clone();
    cube2.set_sticker(FaceName::F, 0, 0, Color::Red);
    history.push(cube2.clone());

    // Undo
    history.undo();
    assert!(history.can_redo());

    // Make a different change
    let mut cube3 = cube1.clone();
    cube3.set_sticker(FaceName::F, 0, 0, Color::Blue);
    history.push(cube3.clone());

    // Redo should no longer be available
    assert!(!history.can_redo());
    assert_eq!(history.current(), &cube3);
}

#[test]
fn test_multiple_stickers_undo_redo() {
    // Test undo/redo with multiple sticker changes
    let cube = Cube::new(3);
    let mut history = History::new(cube.clone());

    // Change multiple stickers in one cube state
    let mut modified = cube.clone();
    modified.set_sticker(FaceName::F, 0, 0, Color::Red);
    modified.set_sticker(FaceName::F, 0, 1, Color::Blue);
    modified.set_sticker(FaceName::F, 0, 2, Color::Green);
    history.push(modified.clone());

    // Verify all changes are present
    assert_eq!(modified.get_face(FaceName::F).get(0, 0), Color::Red);
    assert_eq!(modified.get_face(FaceName::F).get(0, 1), Color::Blue);
    assert_eq!(modified.get_face(FaceName::F).get(0, 2), Color::Green);

    // Undo should restore all original colors
    history.undo();
    assert_eq!(history.current().get_face(FaceName::F).get(0, 0), Color::Green);
    assert_eq!(history.current().get_face(FaceName::F).get(0, 1), Color::Green);
    assert_eq!(history.current().get_face(FaceName::F).get(0, 2), Color::Green);

    // Redo should restore all changes
    history.redo();
    assert_eq!(history.current().get_face(FaceName::F).get(0, 0), Color::Red);
    assert_eq!(history.current().get_face(FaceName::F).get(0, 1), Color::Blue);
    assert_eq!(history.current().get_face(FaceName::F).get(0, 2), Color::Green);
}

#[test]
fn test_undo_redo_with_different_cube_sizes() {
    // Test that undo/redo works with different cube sizes
    for size in [2, 3, 4, 5] {
        let cube = Cube::new(size);
        let mut history = History::new(cube.clone());

        // Make a change
        let mut modified = cube.clone();
        modified.set_sticker(FaceName::F, 0, 0, Color::Red);
        history.push(modified.clone());

        // Undo
        history.undo();
        assert_eq!(history.current(), &cube);

        // Redo
        history.redo();
        assert_eq!(history.current(), &modified);
    }
}

#[test]
fn test_history_preserves_cube_state_integrity() {
    // Test that undo/redo preserves complete cube state
    let cube = Cube::new(3);
    let mut history = History::new(cube.clone());

    // Make complex changes across multiple faces
    let mut modified = cube.clone();
    modified.set_sticker(FaceName::F, 0, 0, Color::Red);
    modified.set_sticker(FaceName::U, 1, 1, Color::Yellow);
    modified.set_sticker(FaceName::R, 2, 2, Color::Blue);
    history.push(modified.clone());

    // Undo
    history.undo();

    // Verify all faces are restored
    assert_eq!(history.current().get_face(FaceName::F).get(0, 0), Color::Green);
    assert_eq!(history.current().get_face(FaceName::U).get(1, 1), Color::White);
    assert_eq!(history.current().get_face(FaceName::R).get(2, 2), Color::Red);
}

#[test]
fn test_long_undo_redo_sequence() {
    // Test a long sequence of undo/redo operations
    let cube = Cube::new(3);
    let mut history = History::new(cube.clone());

    // Build up a history of 10 changes
    let colors = [
        Color::Red,
        Color::Blue,
        Color::Green,
        Color::Yellow,
        Color::Orange,
        Color::White,
        Color::Red,
        Color::Blue,
        Color::Green,
        Color::Yellow,
    ];

    for (i, color) in colors.iter().enumerate() {
        let mut modified = history.current().clone();
        modified.set_sticker(FaceName::F, 0, 0, *color);
        history.push(modified);
    }

    // Undo all changes
    for _ in 0..10 {
        assert!(history.can_undo());
        history.undo();
    }

    // Should be back to original
    assert_eq!(history.current(), &cube);
    assert!(!history.can_undo());

    // Redo all changes
    for i in 0..10 {
        assert!(history.can_redo());
        history.redo();
        assert_eq!(history.current().get_face(FaceName::F).get(0, 0), colors[i]);
    }

    assert!(!history.can_redo());
}
