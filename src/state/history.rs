//! History module for undo/redo functionality
//!
//! This module implements R3.7 from the PRD:
//! - Undo last color change
//! - Redo undone change
//! - Undo/redo history stack
//!
//! The history stack tracks cube states and allows navigation through them.

use crate::cube::Cube;

/// Maximum number of history states to keep in memory
const MAX_HISTORY_SIZE: usize = 100;

/// History manager for undo/redo functionality
#[derive(Debug, Clone)]
pub struct History {
    /// Stack of past cube states
    past: Vec<Cube>,
    /// Current cube state
    current: Cube,
    /// Stack of future cube states (for redo)
    future: Vec<Cube>,
    /// Maximum history size
    max_size: usize,
}

impl History {
    /// Create a new history with the given initial cube state
    pub fn new(initial_cube: Cube) -> Self {
        Self {
            past: Vec::new(),
            current: initial_cube,
            future: Vec::new(),
            max_size: MAX_HISTORY_SIZE,
        }
    }

    /// Create a new history with a custom maximum size
    pub fn with_max_size(initial_cube: Cube, max_size: usize) -> Self {
        Self {
            past: Vec::new(),
            current: initial_cube,
            future: Vec::new(),
            max_size,
        }
    }

    /// Push a new cube state onto the history
    /// This clears the future (redo) stack
    pub fn push(&mut self, new_cube: Cube) {
        // Push current state to past
        self.past.push(self.current.clone());

        // Limit history size
        if self.past.len() > self.max_size {
            self.past.remove(0);
        }

        // Update current and clear future
        self.current = new_cube;
        self.future.clear();
    }

    /// Undo the last change, returning the previous cube state
    /// Returns None if there's nothing to undo
    pub fn undo(&mut self) -> Option<Cube> {
        if let Some(previous_cube) = self.past.pop() {
            // Move current to future
            self.future.push(self.current.clone());

            // Restore previous state
            self.current = previous_cube.clone();

            Some(previous_cube)
        } else {
            None
        }
    }

    /// Redo the last undone change, returning the next cube state
    /// Returns None if there's nothing to redo
    pub fn redo(&mut self) -> Option<Cube> {
        if let Some(next_cube) = self.future.pop() {
            // Move current to past
            self.past.push(self.current.clone());

            // Restore next state
            self.current = next_cube.clone();

            Some(next_cube)
        } else {
            None
        }
    }

    /// Get the current cube state
    pub fn current(&self) -> &Cube {
        &self.current
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.past.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }

    /// Get the number of states in the past (undo available)
    pub fn past_len(&self) -> usize {
        self.past.len()
    }

    /// Get the number of states in the future (redo available)
    pub fn future_len(&self) -> usize {
        self.future.len()
    }

    /// Clear all history and reset to the current state
    pub fn clear(&mut self) {
        self.past.clear();
        self.future.clear();
    }

    /// Reset to a new cube state, clearing all history
    pub fn reset(&mut self, new_cube: Cube) {
        self.past.clear();
        self.future.clear();
        self.current = new_cube;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::{Color, FaceName};

    #[test]
    fn test_history_new() {
        let cube = Cube::new(3);
        let history = History::new(cube.clone());
        assert_eq!(history.current(), &cube);
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_history_push() {
        let cube1 = Cube::new(3);
        let mut cube2 = Cube::new(3);
        cube2.set_sticker(FaceName::F, 0, 0, Color::Red);

        let mut history = History::new(cube1.clone());
        history.push(cube2.clone());

        assert_eq!(history.current(), &cube2);
        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.past_len(), 1);
    }

    #[test]
    fn test_history_undo() {
        let cube1 = Cube::new(3);
        let mut cube2 = Cube::new(3);
        cube2.set_sticker(FaceName::F, 0, 0, Color::Red);

        let mut history = History::new(cube1.clone());
        history.push(cube2.clone());

        let undone = history.undo();
        assert!(undone.is_some());
        assert_eq!(history.current(), &cube1);
        assert!(!history.can_undo());
        assert!(history.can_redo());
    }

    #[test]
    fn test_history_redo() {
        let cube1 = Cube::new(3);
        let mut cube2 = Cube::new(3);
        cube2.set_sticker(FaceName::F, 0, 0, Color::Red);

        let mut history = History::new(cube1.clone());
        history.push(cube2.clone());
        history.undo();

        let redone = history.redo();
        assert!(redone.is_some());
        assert_eq!(history.current(), &cube2);
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_history_push_clears_future() {
        let cube1 = Cube::new(3);
        let mut cube2 = Cube::new(3);
        cube2.set_sticker(FaceName::F, 0, 0, Color::Red);
        let mut cube3 = Cube::new(3);
        cube3.set_sticker(FaceName::F, 0, 0, Color::Blue);

        let mut history = History::new(cube1.clone());
        history.push(cube2.clone());
        history.undo();

        // At this point, we have cube1 as current and cube2 in future
        assert!(history.can_redo());

        // Push cube3, which should clear the future
        history.push(cube3.clone());
        assert!(!history.can_redo());
        assert_eq!(history.current(), &cube3);
    }

    #[test]
    fn test_history_undo_when_empty() {
        let cube = Cube::new(3);
        let mut history = History::new(cube);

        let result = history.undo();
        assert!(result.is_none());
    }

    #[test]
    fn test_history_redo_when_empty() {
        let cube = Cube::new(3);
        let mut history = History::new(cube);

        let result = history.redo();
        assert!(result.is_none());
    }

    #[test]
    fn test_history_multiple_changes() {
        let cube1 = Cube::new(3);
        let mut cube2 = Cube::new(3);
        cube2.set_sticker(FaceName::F, 0, 0, Color::Red);
        let mut cube3 = Cube::new(3);
        cube3.set_sticker(FaceName::F, 0, 0, Color::Blue);
        let mut cube4 = Cube::new(3);
        cube4.set_sticker(FaceName::F, 0, 0, Color::Green);

        let mut history = History::new(cube1.clone());
        history.push(cube2.clone());
        history.push(cube3.clone());
        history.push(cube4.clone());

        assert_eq!(history.past_len(), 3);
        assert_eq!(history.current(), &cube4);

        // Undo to cube3
        history.undo();
        assert_eq!(history.current(), &cube3);

        // Undo to cube2
        history.undo();
        assert_eq!(history.current(), &cube2);

        // Undo to cube1
        history.undo();
        assert_eq!(history.current(), &cube1);

        // Can't undo anymore
        assert!(!history.can_undo());
        assert!(history.can_redo());
    }

    #[test]
    fn test_history_max_size() {
        let mut history = History::with_max_size(Cube::new(3), 3);

        for _ in 0..5 {
            let mut cube = Cube::new(3);
            cube.set_sticker(FaceName::F, 0, 0, Color::Red);
            history.push(cube);
        }

        // Should only keep 3 states in past
        assert_eq!(history.past_len(), 3);
    }

    #[test]
    fn test_history_clear() {
        let cube1 = Cube::new(3);
        let mut cube2 = Cube::new(3);
        cube2.set_sticker(FaceName::F, 0, 0, Color::Red);

        let mut history = History::new(cube1.clone());
        history.push(cube2.clone());

        // After push, can undo but not redo
        assert!(history.can_undo());
        assert!(!history.can_redo());

        history.clear();

        assert!(!history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.past_len(), 0);
        assert_eq!(history.future_len(), 0);
    }

    #[test]
    fn test_history_reset() {
        let cube1 = Cube::new(3);
        let mut cube2 = Cube::new(3);
        cube2.set_sticker(FaceName::F, 0, 0, Color::Red);
        let cube3 = Cube::new(2);

        let mut history = History::new(cube1.clone());
        history.push(cube2.clone());

        // After push, can undo but not redo
        assert!(history.can_undo());
        assert!(!history.can_redo());

        history.reset(cube3.clone());

        assert!(!history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.current(), &cube3);
    }
}
