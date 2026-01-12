/// Integration tests for scan workflow component (R4.5)
///
/// Tests the complete workflow for scanning all 6 faces of a cube

use rubiks_cube_solver::components::{FacePosition, ScannedFace, ScanWorkflowState};
use rubiks_cube_solver::cube::{Color, Cube};

#[test]
fn test_face_position_order() {
    // scan_001: Verify face scanning order
    let order = FacePosition::all_ordered();
    assert_eq!(order.len(), 6);
    assert_eq!(order[0], FacePosition::Front);
    assert_eq!(order[1], FacePosition::Right);
    assert_eq!(order[2], FacePosition::Back);
    assert_eq!(order[3], FacePosition::Left);
    assert_eq!(order[4], FacePosition::Up);
    assert_eq!(order[5], FacePosition::Down);
}

#[test]
fn test_face_position_names() {
    // scan_002: Verify face position names and abbreviations
    assert_eq!(FacePosition::Front.name(), "Front (F)");
    assert_eq!(FacePosition::Back.name(), "Back (B)");
    assert_eq!(FacePosition::Right.name(), "Right (R)");
    assert_eq!(FacePosition::Left.name(), "Left (L)");
    assert_eq!(FacePosition::Up.name(), "Up (U)");
    assert_eq!(FacePosition::Down.name(), "Down (D)");

    assert_eq!(FacePosition::Front.abbrev(), "F");
    assert_eq!(FacePosition::Back.abbrev(), "B");
    assert_eq!(FacePosition::Right.abbrev(), "R");
    assert_eq!(FacePosition::Left.abbrev(), "L");
    assert_eq!(FacePosition::Up.abbrev(), "U");
    assert_eq!(FacePosition::Down.abbrev(), "D");
}

#[test]
fn test_face_position_solved_colors() {
    // scan_003: Verify expected colors for each face in solved state
    assert_eq!(FacePosition::Front.solved_color(), Color::Green);
    assert_eq!(FacePosition::Back.solved_color(), Color::Blue);
    assert_eq!(FacePosition::Right.solved_color(), Color::Red);
    assert_eq!(FacePosition::Left.solved_color(), Color::Orange);
    assert_eq!(FacePosition::Up.solved_color(), Color::White);
    assert_eq!(FacePosition::Down.solved_color(), Color::Yellow);
}

#[test]
fn test_scanned_face_creation() {
    // scan_004: Create scanned face with colors
    let colors = vec![vec![Color::White; 3]; 3];
    let scanned = ScannedFace {
        position: FacePosition::Up,
        colors: colors.clone(),
        confirmed: true,
    };

    assert_eq!(scanned.position, FacePosition::Up);
    assert!(scanned.confirmed);
    assert_eq!(scanned.colors.len(), 3);
    assert_eq!(scanned.colors[0].len(), 3);
    assert_eq!(scanned.colors[0][0], Color::White);
}

#[test]
fn test_scanned_face_not_confirmed() {
    // scan_005: Create unconfirmed scanned face
    let colors = vec![vec![Color::Red; 2]; 2];
    let scanned = ScannedFace {
        position: FacePosition::Right,
        colors,
        confirmed: false,
    };

    assert!(!scanned.confirmed);
    assert_eq!(scanned.position, FacePosition::Right);
}

#[test]
fn test_workflow_state_not_started() {
    // scan_006: Workflow initially not started
    let state = ScanWorkflowState::NotStarted;
    assert_eq!(state, ScanWorkflowState::NotStarted);
}

#[test]
fn test_workflow_state_scanning() {
    // scan_007: Workflow in scanning state
    let state = ScanWorkflowState::Scanning(FacePosition::Front);
    assert_eq!(state, ScanWorkflowState::Scanning(FacePosition::Front));
    assert_ne!(state, ScanWorkflowState::Scanning(FacePosition::Back));
}

#[test]
fn test_workflow_state_reviewing() {
    // scan_008: Workflow in reviewing state
    let state = ScanWorkflowState::Reviewing(FacePosition::Right);
    assert_eq!(state, ScanWorkflowState::Reviewing(FacePosition::Right));
}

#[test]
fn test_workflow_state_complete() {
    // scan_009: Workflow complete state
    let state = ScanWorkflowState::Complete;
    assert_eq!(state, ScanWorkflowState::Complete);
}

#[test]
fn test_scanned_face_colors_2x2() {
    // scan_010: Scanned face for 2x2 cube
    let colors = vec![vec![Color::Yellow; 2]; 2];
    let scanned = ScannedFace {
        position: FacePosition::Down,
        colors,
        confirmed: true,
    };

    assert_eq!(scanned.colors.len(), 2);
    assert_eq!(scanned.colors[0].len(), 2);
}

#[test]
fn test_scanned_face_colors_5x5() {
    // scan_011: Scanned face for 5x5 cube
    let colors = vec![vec![Color::Blue; 5]; 5];
    let scanned = ScannedFace {
        position: FacePosition::Back,
        colors,
        confirmed: true,
    };

    assert_eq!(scanned.colors.len(), 5);
    assert_eq!(scanned.colors[0].len(), 5);
}

#[test]
fn test_all_face_positions_unique() {
    // scan_012: All face positions are unique
    let faces = FacePosition::all_ordered();
    for (i, face1) in faces.iter().enumerate() {
        for (j, face2) in faces.iter().enumerate() {
            if i != j {
                assert_ne!(face1, face2, "Faces at index {} and {} should be different", i, j);
            }
        }
    }
}

#[test]
fn test_scanned_face_clone() {
    // scan_013: Scanned faces can be cloned
    let colors = vec![vec![Color::Green; 3]; 3];
    let scanned1 = ScannedFace {
        position: FacePosition::Front,
        colors,
        confirmed: true,
    };

    let scanned2 = scanned1.clone();
    assert_eq!(scanned1, scanned2);
}

#[test]
fn test_complete_scan_workflow_simulation() {
    // scan_014: Simulate scanning all 6 faces
    let mut scanned_faces = Vec::new();

    // Scan each face in order
    for face_pos in FacePosition::all_ordered() {
        let color = face_pos.solved_color();
        let colors = vec![vec![color; 3]; 3];
        scanned_faces.push(ScannedFace {
            position: face_pos,
            colors,
            confirmed: true,
        });
    }

    // Verify we have all 6 faces
    assert_eq!(scanned_faces.len(), 6);

    // Verify each face is confirmed
    for scanned in &scanned_faces {
        assert!(scanned.confirmed);
    }

    // Verify all positions are present
    let positions: Vec<_> = scanned_faces.iter().map(|s| s.position).collect();
    assert!(positions.contains(&FacePosition::Front));
    assert!(positions.contains(&FacePosition::Back));
    assert!(positions.contains(&FacePosition::Right));
    assert!(positions.contains(&FacePosition::Left));
    assert!(positions.contains(&FacePosition::Up));
    assert!(positions.contains(&FacePosition::Down));
}

#[test]
fn test_rescan_face_replaces_previous() {
    // scan_015: Re-scanning a face should replace previous scan
    let mut scanned_faces = Vec::new();

    // Initial scan
    let colors1 = vec![vec![Color::White; 3]; 3];
    scanned_faces.push(ScannedFace {
        position: FacePosition::Up,
        colors: colors1,
        confirmed: true,
    });

    assert_eq!(scanned_faces.len(), 1);

    // Rescan same face - in real workflow, old scan would be removed
    // and new scan added
    let colors2 = vec![vec![Color::Yellow; 3]; 3];
    scanned_faces.retain(|sf| sf.position != FacePosition::Up);
    scanned_faces.push(ScannedFace {
        position: FacePosition::Up,
        colors: colors2,
        confirmed: true,
    });

    assert_eq!(scanned_faces.len(), 1);
    assert_eq!(scanned_faces[0].colors[0][0], Color::Yellow);
}

#[test]
fn test_partial_scan_workflow() {
    // scan_016: Can track partial scans (not all faces scanned yet)
    let mut scanned_faces = Vec::new();

    // Scan only 3 faces
    for face_pos in &[FacePosition::Front, FacePosition::Right, FacePosition::Up] {
        let color = face_pos.solved_color();
        let colors = vec![vec![color; 3]; 3];
        scanned_faces.push(ScannedFace {
            position: *face_pos,
            colors,
            confirmed: true,
        });
    }

    assert_eq!(scanned_faces.len(), 3);
    assert!(scanned_faces.len() < 6);
}

#[test]
fn test_cube_build_from_scans() {
    // scan_017: Build cube from scanned faces
    let size = 3;
    let mut cube = Cube::new(size);

    // Create scanned faces
    let mut scanned_faces = Vec::new();
    for face_pos in FacePosition::all_ordered() {
        let color = face_pos.solved_color();
        let colors = vec![vec![color; size]; size];
        scanned_faces.push(ScannedFace {
            position: face_pos,
            colors,
            confirmed: true,
        });
    }

    // Apply scans to cube (simulating complete_workflow logic)
    use rubiks_cube_solver::cube::FaceName;
    for scanned in scanned_faces {
        let face_name = match scanned.position {
            FacePosition::Front => FaceName::F,
            FacePosition::Back => FaceName::B,
            FacePosition::Right => FaceName::R,
            FacePosition::Left => FaceName::L,
            FacePosition::Up => FaceName::U,
            FacePosition::Down => FaceName::D,
        };

        for (row, row_colors) in scanned.colors.iter().enumerate() {
            for (col, color) in row_colors.iter().enumerate() {
                cube.set_sticker(face_name, row, col, *color);
            }
        }
    }

    // Verify cube is valid
    assert!(cube.validate().is_ok());
}
