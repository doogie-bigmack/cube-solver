//! Integration tests for the highlight system (R2.7)

use rubiks_cube_solver::cube::FaceName;
use rubiks_cube_solver::renderer::{HighlightConfig, HighlightManager, HighlightType, PieceId};
use glam::Vec3;

#[test]
fn test_highlight_manager_integration() {
    let mut manager = HighlightManager::new();

    // Test creating multiple pieces
    let pieces = vec![
        PieceId::new(FaceName::F, 0, 0),
        PieceId::new(FaceName::F, 0, 1),
        PieceId::new(FaceName::F, 0, 2),
        PieceId::new(FaceName::R, 1, 1),
    ];

    // Set tutorial pieces
    manager.set_tutorial_pieces(pieces.clone());
    assert_eq!(manager.get_tutorial_pieces().len(), 4);

    // Hover over one piece
    manager.set_hover(Some(pieces[0]));

    // Check that all pieces are highlighted
    for piece in &pieces {
        assert!(manager.is_highlighted(piece));
    }

    // The hovered piece should have Hover type
    assert_eq!(
        manager.get_piece_highlight(&pieces[0]),
        Some(HighlightType::Hover)
    );

    // Other tutorial pieces should have Tutorial type
    assert_eq!(
        manager.get_piece_highlight(&pieces[1]),
        Some(HighlightType::Tutorial)
    );
}

#[test]
fn test_highlight_all_faces() {
    let mut manager = HighlightManager::new();

    // Create pieces from all 6 faces
    let all_face_pieces = vec![
        PieceId::new(FaceName::F, 1, 1),
        PieceId::new(FaceName::B, 1, 1),
        PieceId::new(FaceName::R, 1, 1),
        PieceId::new(FaceName::L, 1, 1),
        PieceId::new(FaceName::U, 1, 1),
        PieceId::new(FaceName::D, 1, 1),
    ];

    // Set all as tutorial pieces
    manager.set_tutorial_pieces(all_face_pieces.clone());

    // All should be highlighted
    for piece in &all_face_pieces {
        assert!(manager.is_highlighted(piece));
        assert_eq!(
            manager.get_piece_highlight(piece),
            Some(HighlightType::Tutorial)
        );
    }

    // Select one piece from each face
    for (i, piece) in all_face_pieces.iter().enumerate() {
        manager.set_selected(Some(*piece));
        assert_eq!(
            manager.get_piece_highlight(piece),
            Some(HighlightType::Selected)
        );

        // Other pieces should still be tutorial
        for (j, other_piece) in all_face_pieces.iter().enumerate() {
            if i != j {
                assert_eq!(
                    manager.get_piece_highlight(other_piece),
                    Some(HighlightType::Tutorial)
                );
            }
        }
    }
}

#[test]
fn test_highlight_color_blending() {
    let config = HighlightConfig {
        hover_color: Vec3::new(1.0, 1.0, 0.0), // Yellow
        tutorial_color: Vec3::new(0.0, 0.0, 1.0), // Blue
        selected_color: Vec3::new(1.0, 0.0, 0.0), // Red
        intensity: 1.0,
        alpha: 0.5,
    };

    let mut manager = HighlightManager::with_config(config);
    let piece = PieceId::new(FaceName::F, 1, 1);
    let base_color = Vec3::new(0.0, 1.0, 0.0); // Green

    // Test hover blend
    manager.set_hover(Some(piece));
    let hover_result = manager.apply_highlight(&piece, base_color);

    // Should be a blend of green and yellow (50% each since alpha = 0.5)
    let expected_hover = Vec3::new(0.5, 1.0, 0.0); // Green + Yellow
    let epsilon = 0.001;
    assert!((hover_result.x - expected_hover.x).abs() < epsilon);
    assert!((hover_result.y - expected_hover.y).abs() < epsilon);
    assert!((hover_result.z - expected_hover.z).abs() < epsilon);

    // Test selected blend
    manager.set_selected(Some(piece));
    let selected_result = manager.apply_highlight(&piece, base_color);

    // Should be a blend of green and red (50% each)
    let expected_selected = Vec3::new(0.5, 0.5, 0.0); // Green + Red
    assert!((selected_result.x - expected_selected.x).abs() < epsilon);
    assert!((selected_result.y - expected_selected.y).abs() < epsilon);
    assert!((selected_result.z - expected_selected.z).abs() < epsilon);
}

#[test]
fn test_highlight_workflow_simulation() {
    // Simulate a typical user workflow
    let mut manager = HighlightManager::new();

    // Step 1: Show tutorial pieces (e.g., for F2L tutorial)
    let tutorial_pieces = vec![
        PieceId::new(FaceName::F, 2, 0), // Front bottom-left
        PieceId::new(FaceName::F, 2, 1), // Front bottom-center
        PieceId::new(FaceName::F, 2, 2), // Front bottom-right
    ];
    manager.set_tutorial_pieces(tutorial_pieces.clone());

    // Verify tutorial pieces are highlighted
    for piece in &tutorial_pieces {
        assert_eq!(
            manager.get_piece_highlight(piece),
            Some(HighlightType::Tutorial)
        );
    }

    // Step 2: User hovers over a piece
    manager.set_hover(Some(tutorial_pieces[0]));
    assert_eq!(
        manager.get_piece_highlight(&tutorial_pieces[0]),
        Some(HighlightType::Hover)
    );

    // Step 3: User clicks to select the piece
    manager.set_selected(Some(tutorial_pieces[0]));
    assert_eq!(
        manager.get_piece_highlight(&tutorial_pieces[0]),
        Some(HighlightType::Selected)
    );

    // Step 4: User hovers over another piece while one is selected
    manager.set_hover(Some(tutorial_pieces[1]));
    assert_eq!(
        manager.get_piece_highlight(&tutorial_pieces[0]),
        Some(HighlightType::Selected)
    ); // Still selected
    assert_eq!(
        manager.get_piece_highlight(&tutorial_pieces[1]),
        Some(HighlightType::Hover)
    ); // Hovered

    // Step 5: Clear tutorial, move to next lesson
    manager.clear_tutorial_pieces();
    assert_eq!(manager.get_tutorial_pieces().len(), 0);

    // Selected piece should still be highlighted
    assert_eq!(
        manager.get_piece_highlight(&tutorial_pieces[0]),
        Some(HighlightType::Selected)
    );

    // Tutorial pieces are no longer highlighted (except selected one)
    assert_eq!(manager.get_piece_highlight(&tutorial_pieces[2]), None);
}

#[test]
fn test_highlight_persistence() {
    let mut manager = HighlightManager::new();
    let piece = PieceId::new(FaceName::U, 0, 0);

    // Set hover
    manager.set_hover(Some(piece));
    assert!(manager.is_highlighted(&piece));

    // Clone the manager
    let manager_clone = manager.clone();

    // Both should have the same state
    assert_eq!(manager.get_hover(), manager_clone.get_hover());
    assert!(manager_clone.is_highlighted(&piece));

    // Modify original
    manager.set_hover(None);
    assert!(!manager.is_highlighted(&piece));

    // Clone should still have the old state
    assert!(manager_clone.is_highlighted(&piece));
}

#[test]
fn test_config_update() {
    let mut manager = HighlightManager::new();
    let piece = PieceId::new(FaceName::F, 1, 1);
    let base_color = Vec3::new(1.0, 0.0, 0.0);

    manager.set_hover(Some(piece));

    // Apply with default config
    let result1 = manager.apply_highlight(&piece, base_color);

    // Update config
    let new_config = HighlightConfig {
        hover_color: Vec3::new(0.0, 1.0, 0.0),
        tutorial_color: Vec3::new(0.0, 0.0, 1.0),
        selected_color: Vec3::new(1.0, 1.0, 0.0),
        intensity: 2.0,
        alpha: 0.9,
    };
    manager.set_config(new_config);

    // Apply with new config
    let result2 = manager.apply_highlight(&piece, base_color);

    // Results should be different
    assert_ne!(result1, result2);
}

#[test]
fn test_multiple_piece_types() {
    let mut manager = HighlightManager::new();

    // Create pieces representing different piece types
    let center_piece = PieceId::new(FaceName::F, 1, 1); // 3x3 center
    let edge_piece = PieceId::new(FaceName::F, 0, 1); // Top edge
    let corner_piece = PieceId::new(FaceName::F, 0, 0); // Top-left corner

    // Highlight all three
    manager.add_tutorial_piece(center_piece);
    manager.add_tutorial_piece(edge_piece);
    manager.set_selected(Some(corner_piece));

    // All should be highlighted
    assert!(manager.is_highlighted(&center_piece));
    assert!(manager.is_highlighted(&edge_piece));
    assert!(manager.is_highlighted(&corner_piece));

    // But with different types
    assert_eq!(
        manager.get_piece_highlight(&center_piece),
        Some(HighlightType::Tutorial)
    );
    assert_eq!(
        manager.get_piece_highlight(&edge_piece),
        Some(HighlightType::Tutorial)
    );
    assert_eq!(
        manager.get_piece_highlight(&corner_piece),
        Some(HighlightType::Selected)
    );
}

#[test]
fn test_large_tutorial_set() {
    let mut manager = HighlightManager::new();

    // Create a large set of tutorial pieces (entire face)
    let mut pieces = Vec::new();
    for row in 0..3 {
        for col in 0..3 {
            pieces.push(PieceId::new(FaceName::F, row, col));
        }
    }

    manager.set_tutorial_pieces(pieces.clone());
    assert_eq!(manager.get_tutorial_pieces().len(), 9);

    // All should be highlighted
    for piece in &pieces {
        assert!(manager.is_highlighted(piece));
    }

    // Remove pieces one by one
    for piece in pieces.iter().take(5) {
        manager.remove_tutorial_piece(piece);
    }

    assert_eq!(manager.get_tutorial_pieces().len(), 4);
}

#[test]
fn test_highlight_intensity() {
    // Test that higher intensity produces brighter results
    let base_color = Vec3::new(1.0, 0.0, 0.0);
    let piece = PieceId::new(FaceName::F, 0, 0);

    let config_low = HighlightConfig {
        hover_color: Vec3::new(1.0, 1.0, 1.0),
        tutorial_color: Vec3::new(1.0, 1.0, 1.0),
        selected_color: Vec3::new(1.0, 1.0, 1.0),
        intensity: 0.5,
        alpha: 0.5,
    };

    let config_high = HighlightConfig {
        hover_color: Vec3::new(1.0, 1.0, 1.0),
        tutorial_color: Vec3::new(1.0, 1.0, 1.0),
        selected_color: Vec3::new(1.0, 1.0, 1.0),
        intensity: 2.0,
        alpha: 0.5,
    };

    let mut manager_low = HighlightManager::with_config(config_low);
    let mut manager_high = HighlightManager::with_config(config_high);

    manager_low.set_hover(Some(piece));
    manager_high.set_hover(Some(piece));

    let result_low = manager_low.apply_highlight(&piece, base_color);
    let result_high = manager_high.apply_highlight(&piece, base_color);

    // Calculate brightness (sum of RGB components)
    let brightness_low = result_low.x + result_low.y + result_low.z;
    let brightness_high = result_high.x + result_high.y + result_high.z;

    // Higher intensity should produce brighter result
    assert!(brightness_high > brightness_low,
        "High intensity ({}) should be brighter than low intensity ({})",
        brightness_high, brightness_low);

    // Also verify that intensity actually scales the result
    let expected_ratio = 2.0 / 0.5; // high / low intensity
    let actual_ratio = brightness_high / brightness_low;

    // Ratio should be close to 4.0 (2.0 / 0.5)
    assert!((actual_ratio - expected_ratio).abs() < 0.1,
        "Expected ratio ~{}, got {}",
        expected_ratio, actual_ratio);
}
