//! Piece highlighting system for the Rubik's cube
//!
//! This module implements R2.7 from the PRD:
//! - Visual highlight on hover
//! - Highlight pieces being affected by current tutorial step
//! - Configurable highlight color

use crate::cube::FaceName;
use glam::Vec3;

/// Represents a specific sticker/piece on the cube that can be highlighted
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PieceId {
    /// The face this piece belongs to
    pub face: FaceName,
    /// Row index (0-based, top to bottom)
    pub row: usize,
    /// Column index (0-based, left to right)
    pub col: usize,
}

impl PieceId {
    /// Creates a new piece identifier
    pub fn new(face: FaceName, row: usize, col: usize) -> Self {
        Self { face, row, col }
    }
}

/// Type of highlight to apply
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HighlightType {
    /// Hover highlight (e.g., from mouse cursor)
    Hover,
    /// Tutorial step highlight (shows affected pieces)
    Tutorial,
    /// Selected piece highlight
    Selected,
}

/// Configuration for highlighting
#[derive(Debug, Clone)]
pub struct HighlightConfig {
    /// Color for hover highlights (RGB)
    pub hover_color: Vec3,
    /// Color for tutorial highlights (RGB)
    pub tutorial_color: Vec3,
    /// Color for selected piece highlights (RGB)
    pub selected_color: Vec3,
    /// Intensity multiplier for highlights (0.0 to 2.0, 1.0 = no change)
    pub intensity: f32,
    /// Alpha/opacity for highlights (0.0 to 1.0)
    pub alpha: f32,
}

impl Default for HighlightConfig {
    fn default() -> Self {
        Self {
            hover_color: Vec3::new(1.0, 1.0, 0.5), // Light yellow
            tutorial_color: Vec3::new(0.3, 0.7, 1.0), // Light blue
            selected_color: Vec3::new(1.0, 0.5, 0.2), // Orange
            intensity: 1.4,
            alpha: 0.8,
        }
    }
}

/// Manages highlighting of pieces on the cube
#[derive(Debug, Clone)]
pub struct HighlightManager {
    /// Currently hovered piece (if any)
    hovered_piece: Option<PieceId>,
    /// Pieces highlighted for tutorial purposes
    tutorial_pieces: Vec<PieceId>,
    /// Currently selected piece (if any)
    selected_piece: Option<PieceId>,
    /// Configuration for highlights
    config: HighlightConfig,
}

impl HighlightManager {
    /// Creates a new highlight manager
    pub fn new() -> Self {
        Self {
            hovered_piece: None,
            tutorial_pieces: Vec::new(),
            selected_piece: None,
            config: HighlightConfig::default(),
        }
    }

    /// Creates a new highlight manager with custom configuration
    pub fn with_config(config: HighlightConfig) -> Self {
        Self {
            hovered_piece: None,
            tutorial_pieces: Vec::new(),
            selected_piece: None,
            config,
        }
    }

    /// Sets the currently hovered piece
    pub fn set_hover(&mut self, piece: Option<PieceId>) {
        self.hovered_piece = piece;
    }

    /// Gets the currently hovered piece
    pub fn get_hover(&self) -> Option<PieceId> {
        self.hovered_piece
    }

    /// Sets the currently selected piece
    pub fn set_selected(&mut self, piece: Option<PieceId>) {
        self.selected_piece = piece;
    }

    /// Gets the currently selected piece
    pub fn get_selected(&self) -> Option<PieceId> {
        self.selected_piece
    }

    /// Sets the pieces to highlight for tutorial purposes
    pub fn set_tutorial_pieces(&mut self, pieces: Vec<PieceId>) {
        self.tutorial_pieces = pieces;
    }

    /// Adds a piece to the tutorial highlight list
    pub fn add_tutorial_piece(&mut self, piece: PieceId) {
        if !self.tutorial_pieces.contains(&piece) {
            self.tutorial_pieces.push(piece);
        }
    }

    /// Removes a piece from the tutorial highlight list
    pub fn remove_tutorial_piece(&mut self, piece: &PieceId) {
        self.tutorial_pieces.retain(|p| p != piece);
    }

    /// Clears all tutorial highlights
    pub fn clear_tutorial_pieces(&mut self) {
        self.tutorial_pieces.clear();
    }

    /// Gets all tutorial pieces
    pub fn get_tutorial_pieces(&self) -> &[PieceId] {
        &self.tutorial_pieces
    }

    /// Clears all highlights
    pub fn clear_all(&mut self) {
        self.hovered_piece = None;
        self.tutorial_pieces.clear();
        self.selected_piece = None;
    }

    /// Gets the highlight type for a specific piece, if any
    ///
    /// Returns the highest priority highlight type:
    /// 1. Selected (highest priority)
    /// 2. Hover
    /// 3. Tutorial (lowest priority)
    pub fn get_piece_highlight(&self, piece: &PieceId) -> Option<HighlightType> {
        if Some(*piece) == self.selected_piece {
            return Some(HighlightType::Selected);
        }
        if Some(*piece) == self.hovered_piece {
            return Some(HighlightType::Hover);
        }
        if self.tutorial_pieces.contains(piece) {
            return Some(HighlightType::Tutorial);
        }
        None
    }

    /// Checks if a piece is highlighted
    pub fn is_highlighted(&self, piece: &PieceId) -> bool {
        self.get_piece_highlight(piece).is_some()
    }

    /// Gets the highlight color for a specific piece
    ///
    /// Returns the RGB color to apply, or None if not highlighted
    pub fn get_piece_color(&self, piece: &PieceId) -> Option<Vec3> {
        self.get_piece_highlight(piece).map(|hl_type| match hl_type {
            HighlightType::Hover => self.config.hover_color,
            HighlightType::Tutorial => self.config.tutorial_color,
            HighlightType::Selected => self.config.selected_color,
        })
    }

    /// Applies highlight to a base color
    ///
    /// Blends the base color with the highlight color based on configuration
    pub fn apply_highlight(&self, piece: &PieceId, base_color: Vec3) -> Vec3 {
        if let Some(highlight_color) = self.get_piece_color(piece) {
            // Blend base color with highlight color using alpha
            let alpha = self.config.alpha;
            let intensity = self.config.intensity;

            // Mix colors: base * (1-alpha) + highlight * alpha, then apply intensity
            let blended = base_color * (1.0 - alpha) + highlight_color * alpha;
            blended * intensity
        } else {
            base_color
        }
    }

    /// Gets the configuration
    pub fn config(&self) -> &HighlightConfig {
        &self.config
    }

    /// Gets a mutable reference to the configuration
    pub fn config_mut(&mut self) -> &mut HighlightConfig {
        &mut self.config
    }

    /// Updates the configuration
    pub fn set_config(&mut self, config: HighlightConfig) {
        self.config = config;
    }
}

impl Default for HighlightManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_id_creation() {
        let piece = PieceId::new(FaceName::F, 1, 2);
        assert_eq!(piece.face, FaceName::F);
        assert_eq!(piece.row, 1);
        assert_eq!(piece.col, 2);
    }

    #[test]
    fn test_highlight_manager_new() {
        let manager = HighlightManager::new();
        assert_eq!(manager.get_hover(), None);
        assert_eq!(manager.get_selected(), None);
        assert_eq!(manager.get_tutorial_pieces().len(), 0);
    }

    #[test]
    fn test_hover_highlight() {
        let mut manager = HighlightManager::new();
        let piece = PieceId::new(FaceName::F, 0, 0);

        manager.set_hover(Some(piece));
        assert_eq!(manager.get_hover(), Some(piece));
        assert_eq!(manager.get_piece_highlight(&piece), Some(HighlightType::Hover));

        manager.set_hover(None);
        assert_eq!(manager.get_hover(), None);
        assert_eq!(manager.get_piece_highlight(&piece), None);
    }

    #[test]
    fn test_selected_highlight() {
        let mut manager = HighlightManager::new();
        let piece = PieceId::new(FaceName::U, 1, 1);

        manager.set_selected(Some(piece));
        assert_eq!(manager.get_selected(), Some(piece));
        assert_eq!(manager.get_piece_highlight(&piece), Some(HighlightType::Selected));

        manager.set_selected(None);
        assert_eq!(manager.get_selected(), None);
    }

    #[test]
    fn test_tutorial_highlights() {
        let mut manager = HighlightManager::new();
        let piece1 = PieceId::new(FaceName::R, 0, 0);
        let piece2 = PieceId::new(FaceName::R, 0, 1);
        let piece3 = PieceId::new(FaceName::L, 2, 2);

        manager.add_tutorial_piece(piece1);
        manager.add_tutorial_piece(piece2);
        assert_eq!(manager.get_tutorial_pieces().len(), 2);
        assert_eq!(manager.get_piece_highlight(&piece1), Some(HighlightType::Tutorial));

        manager.remove_tutorial_piece(&piece1);
        assert_eq!(manager.get_tutorial_pieces().len(), 1);
        assert_eq!(manager.get_piece_highlight(&piece1), None);
        assert_eq!(manager.get_piece_highlight(&piece2), Some(HighlightType::Tutorial));

        manager.set_tutorial_pieces(vec![piece1, piece2, piece3]);
        assert_eq!(manager.get_tutorial_pieces().len(), 3);

        manager.clear_tutorial_pieces();
        assert_eq!(manager.get_tutorial_pieces().len(), 0);
    }

    #[test]
    fn test_highlight_priority() {
        let mut manager = HighlightManager::new();
        let piece = PieceId::new(FaceName::F, 1, 1);

        // Tutorial has lowest priority
        manager.add_tutorial_piece(piece);
        assert_eq!(manager.get_piece_highlight(&piece), Some(HighlightType::Tutorial));

        // Hover overrides tutorial
        manager.set_hover(Some(piece));
        assert_eq!(manager.get_piece_highlight(&piece), Some(HighlightType::Hover));

        // Selected overrides hover (and tutorial)
        manager.set_selected(Some(piece));
        assert_eq!(manager.get_piece_highlight(&piece), Some(HighlightType::Selected));

        // Remove selected, falls back to hover
        manager.set_selected(None);
        assert_eq!(manager.get_piece_highlight(&piece), Some(HighlightType::Hover));

        // Remove hover, falls back to tutorial
        manager.set_hover(None);
        assert_eq!(manager.get_piece_highlight(&piece), Some(HighlightType::Tutorial));
    }

    #[test]
    fn test_is_highlighted() {
        let mut manager = HighlightManager::new();
        let piece = PieceId::new(FaceName::B, 0, 0);

        assert!(!manager.is_highlighted(&piece));

        manager.set_hover(Some(piece));
        assert!(manager.is_highlighted(&piece));

        manager.set_hover(None);
        assert!(!manager.is_highlighted(&piece));
    }

    #[test]
    fn test_get_piece_color() {
        let mut manager = HighlightManager::new();
        let piece = PieceId::new(FaceName::D, 2, 1);

        assert_eq!(manager.get_piece_color(&piece), None);

        manager.set_hover(Some(piece));
        assert_eq!(manager.get_piece_color(&piece), Some(manager.config().hover_color));

        manager.set_selected(Some(piece));
        assert_eq!(manager.get_piece_color(&piece), Some(manager.config().selected_color));
    }

    #[test]
    fn test_apply_highlight() {
        let manager = HighlightManager::new();
        let piece = PieceId::new(FaceName::U, 0, 0);
        let base_color = Vec3::new(1.0, 0.0, 0.0); // Red

        // No highlight, returns base color
        let result = manager.apply_highlight(&piece, base_color);
        assert_eq!(result, base_color);
    }

    #[test]
    fn test_apply_highlight_with_hover() {
        let mut manager = HighlightManager::new();
        let piece = PieceId::new(FaceName::U, 0, 0);
        let base_color = Vec3::new(1.0, 0.0, 0.0); // Red

        manager.set_hover(Some(piece));
        let result = manager.apply_highlight(&piece, base_color);

        // Result should be different from base (highlighted)
        assert_ne!(result, base_color);

        // Result should have some influence from hover color
        let hover_color = manager.config().hover_color;
        let alpha = manager.config().alpha;
        let expected = (base_color * (1.0 - alpha) + hover_color * alpha) * manager.config().intensity;

        // Use epsilon comparison for floating point
        let epsilon = 0.0001;
        assert!((result.x - expected.x).abs() < epsilon);
        assert!((result.y - expected.y).abs() < epsilon);
        assert!((result.z - expected.z).abs() < epsilon);
    }

    #[test]
    fn test_clear_all() {
        let mut manager = HighlightManager::new();
        let piece1 = PieceId::new(FaceName::F, 0, 0);
        let piece2 = PieceId::new(FaceName::R, 1, 1);

        manager.set_hover(Some(piece1));
        manager.set_selected(Some(piece1));
        manager.add_tutorial_piece(piece2);

        assert!(manager.is_highlighted(&piece1));
        assert!(manager.is_highlighted(&piece2));

        manager.clear_all();

        assert!(!manager.is_highlighted(&piece1));
        assert!(!manager.is_highlighted(&piece2));
        assert_eq!(manager.get_hover(), None);
        assert_eq!(manager.get_selected(), None);
        assert_eq!(manager.get_tutorial_pieces().len(), 0);
    }

    #[test]
    fn test_custom_config() {
        let config = HighlightConfig {
            hover_color: Vec3::new(0.5, 0.5, 0.5),
            tutorial_color: Vec3::new(0.2, 0.2, 0.8),
            selected_color: Vec3::new(0.8, 0.2, 0.2),
            intensity: 2.0,
            alpha: 0.5,
        };

        let manager = HighlightManager::with_config(config.clone());
        assert_eq!(manager.config().intensity, 2.0);
        assert_eq!(manager.config().alpha, 0.5);
    }

    #[test]
    fn test_add_duplicate_tutorial_piece() {
        let mut manager = HighlightManager::new();
        let piece = PieceId::new(FaceName::F, 0, 0);

        manager.add_tutorial_piece(piece);
        manager.add_tutorial_piece(piece); // Add duplicate

        // Should only have one instance
        assert_eq!(manager.get_tutorial_pieces().len(), 1);
    }

    #[test]
    fn test_default_config() {
        let config = HighlightConfig::default();
        assert_eq!(config.hover_color, Vec3::new(1.0, 1.0, 0.5));
        assert_eq!(config.tutorial_color, Vec3::new(0.3, 0.7, 1.0));
        assert_eq!(config.selected_color, Vec3::new(1.0, 0.5, 0.2));
        assert_eq!(config.intensity, 1.4);
        assert_eq!(config.alpha, 0.8);
    }
}
