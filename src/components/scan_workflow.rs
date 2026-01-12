/// Scan workflow component for scanning all 6 faces of a Rubik's cube
///
/// This component guides users through the process of scanning all 6 faces
/// of their cube using the camera. It tracks which faces have been scanned
/// and allows re-scanning individual faces if needed.

use dioxus::prelude::*;
use crate::cube::{Color, Cube};
use crate::cube::state::Face;

/// Represents a face of the cube (which one to scan)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FacePosition {
    Front,
    Back,
    Right,
    Left,
    Up,
    Down,
}

impl FacePosition {
    /// Get all face positions in recommended scanning order
    pub fn all_ordered() -> Vec<FacePosition> {
        vec![
            FacePosition::Front,
            FacePosition::Right,
            FacePosition::Back,
            FacePosition::Left,
            FacePosition::Up,
            FacePosition::Down,
        ]
    }

    /// Get the name of this face position
    pub fn name(&self) -> &'static str {
        match self {
            FacePosition::Front => "Front (F)",
            FacePosition::Back => "Back (B)",
            FacePosition::Right => "Right (R)",
            FacePosition::Left => "Left (L)",
            FacePosition::Up => "Up (U)",
            FacePosition::Down => "Down (D)",
        }
    }

    /// Get the abbreviation of this face position
    pub fn abbrev(&self) -> &'static str {
        match self {
            FacePosition::Front => "F",
            FacePosition::Back => "B",
            FacePosition::Right => "R",
            FacePosition::Left => "L",
            FacePosition::Up => "U",
            FacePosition::Down => "D",
        }
    }

    /// Get the color typically on this face in a solved cube
    pub fn solved_color(&self) -> Color {
        match self {
            FacePosition::Front => Color::Green,
            FacePosition::Back => Color::Blue,
            FacePosition::Right => Color::Red,
            FacePosition::Left => Color::Orange,
            FacePosition::Up => Color::White,
            FacePosition::Down => Color::Yellow,
        }
    }
}

/// State of a scanned face
#[derive(Debug, Clone, PartialEq)]
pub struct ScannedFace {
    /// The face position (F, B, R, L, U, D)
    pub position: FacePosition,
    /// The detected colors for this face
    pub colors: Vec<Vec<Color>>,
    /// Whether this scan is confirmed by user
    pub confirmed: bool,
}

/// State of the scan workflow
#[derive(Debug, Clone, PartialEq)]
pub enum ScanWorkflowState {
    /// Not started yet
    NotStarted,
    /// Currently scanning a specific face
    Scanning(FacePosition),
    /// Reviewing a scanned face before confirming
    Reviewing(FacePosition),
    /// All faces scanned, showing final cube
    Complete,
}

#[derive(Props, Clone, PartialEq)]
pub struct ScanWorkflowProps {
    /// Cube size (2-20)
    #[props(default = 3)]
    pub cube_size: u32,

    /// Callback when all faces are scanned
    #[props(optional)]
    pub on_complete: Option<EventHandler<Cube>>,

    /// Callback when workflow is cancelled
    #[props(optional)]
    pub on_cancel: Option<EventHandler<()>>,
}

#[component]
pub fn ScanWorkflow(props: ScanWorkflowProps) -> Element {
    let mut workflow_state = use_signal(|| ScanWorkflowState::NotStarted);
    let mut scanned_faces = use_signal(|| Vec::<ScannedFace>::new());
    let mut current_face_data = use_signal(|| Vec::<Vec<Color>>::new());

    // Start the workflow
    let start_workflow = move |_| {
        workflow_state.set(ScanWorkflowState::Scanning(FacePosition::Front));
    };

    // Simulate capturing a face (in real implementation, this would use camera)
    let mut capture_face = move |face: FacePosition| {
        // For now, simulate detection with the solved state colors
        let size = props.cube_size as usize;
        let color = face.solved_color();
        let colors = vec![vec![color; size]; size];
        current_face_data.set(colors.clone());
        workflow_state.set(ScanWorkflowState::Reviewing(face));
    };

    // Confirm the scanned face
    let confirm_face = move |_| {
        if let ScanWorkflowState::Reviewing(face) = workflow_state() {
            let colors = current_face_data();

            // Remove existing scan of this face if any
            scanned_faces.write().retain(|sf| sf.position != face);

            // Add the new scan
            scanned_faces.write().push(ScannedFace {
                position: face,
                colors,
                confirmed: true,
            });

            // Determine next face to scan
            let all_faces = FacePosition::all_ordered();
            let scanned_positions: Vec<FacePosition> = scanned_faces
                .read()
                .iter()
                .map(|sf| sf.position)
                .collect();

            if let Some(next_face) = all_faces.iter().find(|f| !scanned_positions.contains(f)) {
                workflow_state.set(ScanWorkflowState::Scanning(*next_face));
            } else {
                workflow_state.set(ScanWorkflowState::Complete);
            }
        }
    };

    // Re-scan a specific face
    let mut rescan_face = move |face: FacePosition| {
        workflow_state.set(ScanWorkflowState::Scanning(face));
    };

    // Complete the workflow and build the cube
    let complete_workflow = move |_| {
        let size = props.cube_size as usize;
        let mut cube = Cube::new(size);

        // Apply scanned faces to the cube using FaceName
        use crate::cube::FaceName;
        for scanned in scanned_faces.read().iter() {
            let face_name = match scanned.position {
                FacePosition::Front => FaceName::F,
                FacePosition::Back => FaceName::B,
                FacePosition::Right => FaceName::R,
                FacePosition::Left => FaceName::L,
                FacePosition::Up => FaceName::U,
                FacePosition::Down => FaceName::D,
            };

            // Set each sticker in the face
            for (row, row_colors) in scanned.colors.iter().enumerate() {
                for (col, color) in row_colors.iter().enumerate() {
                    cube.set_sticker(face_name, row, col, *color);
                }
            }
        }

        if let Some(handler) = &props.on_complete {
            handler.call(cube);
        }
    };

    // Cancel the workflow
    let cancel_workflow = move |_| {
        if let Some(handler) = &props.on_cancel {
            handler.call(());
        }
    };

    rsx! {
        div {
            class: "scan-workflow",
            style: "padding: 20px; max-width: 800px; margin: 0 auto;",

            // Header
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",

                h2 {
                    style: "font-size: 24px; font-weight: bold; margin: 0;",
                    "Scan Your Cube"
                }

                button {
                    onclick: cancel_workflow,
                    style: "padding: 8px 16px; background: #ef4444; color: white; \
                           border: none; border-radius: 5px; cursor: pointer; \
                           font-size: 14px; font-weight: 500;",
                    "Cancel"
                }
            }

            // Progress indicator
            div {
                style: "margin-bottom: 30px;",

                p {
                    style: "font-size: 16px; margin-bottom: 10px; color: #666;",
                    {format!("Progress: {}/6 faces scanned", scanned_faces.read().len())}
                }

                div {
                    style: "display: flex; gap: 10px; flex-wrap: wrap;",

                    for face in FacePosition::all_ordered() {
                        {
                            let is_scanned = scanned_faces.read().iter().any(|sf| sf.position == face);
                            let is_current = match workflow_state() {
                                ScanWorkflowState::Scanning(f) | ScanWorkflowState::Reviewing(f) => f == face,
                                _ => false,
                            };

                            rsx! {
                                div {
                                    key: "{face.abbrev()}",
                                    style: format!(
                                        "padding: 10px 15px; border-radius: 8px; font-size: 14px; \
                                         font-weight: 600; text-align: center; min-width: 80px; \
                                         {}",
                                        if is_current {
                                            "background: #3b82f6; color: white; border: 2px solid #2563eb;"
                                        } else if is_scanned {
                                            "background: #22c55e; color: white;"
                                        } else {
                                            "background: #e5e7eb; color: #6b7280;"
                                        }
                                    ),

                                    div { {face.abbrev()} }
                                    div {
                                        style: "font-size: 10px; font-weight: normal; margin-top: 2px;",
                                        if is_scanned { "✓" } else { "" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Main content area
            match workflow_state() {
                ScanWorkflowState::NotStarted => rsx! {
                    div {
                        style: "text-align: center; padding: 40px 20px;",

                        p {
                            style: "font-size: 18px; margin-bottom: 20px;",
                            "Follow the guide to scan all 6 faces of your cube."
                        }

                        p {
                            style: "font-size: 14px; color: #666; margin-bottom: 30px;",
                            "We'll guide you through scanning each face one at a time. \
                             Make sure you have good lighting!"
                        }

                        button {
                            onclick: start_workflow,
                            style: "padding: 15px 40px; font-size: 18px; font-weight: bold; \
                                   background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); \
                                   color: white; border: none; border-radius: 10px; cursor: pointer; \
                                   box-shadow: 0 4px 6px rgba(0,0,0,0.1);",
                            "Start Scanning"
                        }
                    }
                },

                ScanWorkflowState::Scanning(face) => rsx! {
                    div {
                        style: "padding: 20px; background: #f9fafb; border-radius: 10px;",

                        h3 {
                            style: "font-size: 20px; font-weight: bold; margin-bottom: 15px;",
                            {format!("Scanning: {}", face.name())}
                        }

                        p {
                            style: "font-size: 14px; color: #666; margin-bottom: 20px;",
                            "Position your cube so the {face.abbrev()} face is centered in the camera view."
                        }

                        // Placeholder for camera component (would integrate CameraScanner here)
                        div {
                            style: "background: #e5e7eb; height: 400px; border-radius: 10px; \
                                   display: flex; align-items: center; justify-content: center; \
                                   margin-bottom: 20px;",

                            p {
                                style: "font-size: 16px; color: #6b7280;",
                                "📷 Camera view would appear here"
                            }
                        }

                        button {
                            onclick: move |_| capture_face(face),
                            style: "padding: 12px 30px; font-size: 16px; font-weight: bold; \
                                   background: #3b82f6; color: white; border: none; \
                                   border-radius: 8px; cursor: pointer; width: 100%;",
                            "Capture Face"
                        }
                    }
                },

                ScanWorkflowState::Reviewing(face) => rsx! {
                    div {
                        style: "padding: 20px; background: #f9fafb; border-radius: 10px;",

                        h3 {
                            style: "font-size: 20px; font-weight: bold; margin-bottom: 15px;",
                            {format!("Review: {}", face.name())}
                        }

                        p {
                            style: "font-size: 14px; color: #666; margin-bottom: 20px;",
                            "Review the detected colors. If they look correct, confirm. Otherwise, re-scan."
                        }

                        // Display the detected colors as a grid
                        div {
                            style: "margin-bottom: 20px; display: flex; justify-content: center;",

                            div {
                                style: format!(
                                    "display: grid; grid-template-columns: repeat({}, 1fr); \
                                     gap: 4px; background: #000; padding: 10px; border-radius: 10px;",
                                    props.cube_size
                                ),

                                for (row_idx, row) in current_face_data().iter().enumerate() {
                                    for (col_idx, color) in row.iter().enumerate() {
                                        {
                                            let css_color = match color {
                                                Color::White => "#FFFFFF",
                                                Color::Yellow => "#FFD500",
                                                Color::Red => "#C41E3A",
                                                Color::Orange => "#FF5800",
                                                Color::Blue => "#0051BA",
                                                Color::Green => "#009E60",
                                            };

                                            rsx! {
                                                div {
                                                    key: "{row_idx}-{col_idx}",
                                                    style: format!(
                                                        "width: 40px; height: 40px; background: {}; \
                                                         border-radius: 4px; box-shadow: 0 2px 4px rgba(0,0,0,0.2);",
                                                        css_color
                                                    ),
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div {
                            style: "display: flex; gap: 10px;",

                            button {
                                onclick: move |_| rescan_face(face),
                                style: "flex: 1; padding: 12px 20px; font-size: 16px; \
                                       background: #ef4444; color: white; border: none; \
                                       border-radius: 8px; cursor: pointer; font-weight: 500;",
                                "Re-scan"
                            }

                            button {
                                onclick: confirm_face,
                                style: "flex: 1; padding: 12px 20px; font-size: 16px; \
                                       background: #22c55e; color: white; border: none; \
                                       border-radius: 8px; cursor: pointer; font-weight: 500;",
                                "Confirm"
                            }
                        }
                    }
                },

                ScanWorkflowState::Complete => rsx! {
                    div {
                        style: "text-align: center; padding: 40px 20px;",

                        div {
                            style: "font-size: 48px; margin-bottom: 20px;",
                            "✅"
                        }

                        h3 {
                            style: "font-size: 24px; font-weight: bold; margin-bottom: 15px;",
                            "All Faces Scanned!"
                        }

                        p {
                            style: "font-size: 14px; color: #666; margin-bottom: 30px;",
                            "Your cube has been scanned successfully. You can now solve it or \
                             re-scan any face if needed."
                        }

                        // Show all scanned faces for review
                        {
                            let faces_snapshot: Vec<_> = scanned_faces.read().iter().cloned().collect();
                            rsx! {
                                div {
                                    style: "display: grid; grid-template-columns: repeat(3, 1fr); \
                                           gap: 15px; margin-bottom: 30px; max-width: 600px; margin-left: auto; margin-right: auto;",

                                    for scanned in faces_snapshot {
                                        {
                                            let position = scanned.position;
                                            rsx! {
                                                div {
                                                    key: "{position.abbrev()}",
                                                    style: "padding: 10px; background: white; border-radius: 8px; \
                                                           box-shadow: 0 2px 4px rgba(0,0,0,0.1);",

                                                    p {
                                                        style: "font-size: 12px; font-weight: 600; margin-bottom: 8px; \
                                                               text-align: center;",
                                                        {position.name()}
                                                    }

                                                    div {
                                                        style: format!(
                                                            "display: grid; grid-template-columns: repeat({}, 1fr); \
                                                             gap: 2px; background: #000; padding: 5px; border-radius: 4px;",
                                                            props.cube_size
                                                        ),

                                                        for (row_idx, row) in scanned.colors.iter().enumerate() {
                                                            for (col_idx, color) in row.iter().enumerate() {
                                                                {
                                                                    let css_color = match color {
                                                                        Color::White => "#FFFFFF",
                                                                        Color::Yellow => "#FFD500",
                                                                        Color::Red => "#C41E3A",
                                                                        Color::Orange => "#FF5800",
                                                                        Color::Blue => "#0051BA",
                                                                        Color::Green => "#009E60",
                                                                    };

                                                                    rsx! {
                                                                        div {
                                                                            key: "{row_idx}-{col_idx}",
                                                                            style: format!(
                                                                                "width: 20px; height: 20px; background: {}; \
                                                                                 border-radius: 2px;",
                                                                                css_color
                                                                            ),
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }

                                                    button {
                                                        onclick: move |_| rescan_face(position),
                                                        style: "width: 100%; margin-top: 8px; padding: 6px 12px; \
                                                               font-size: 12px; background: #3b82f6; color: white; \
                                                               border: none; border-radius: 4px; cursor: pointer;",
                                                        "Re-scan"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        button {
                            onclick: complete_workflow,
                            style: "padding: 15px 40px; font-size: 18px; font-weight: bold; \
                                   background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%); \
                                   color: white; border: none; border-radius: 10px; cursor: pointer; \
                                   box-shadow: 0 4px 6px rgba(0,0,0,0.1);",
                            "Done"
                        }
                    }
                },
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_position_all_ordered() {
        let faces = FacePosition::all_ordered();
        assert_eq!(faces.len(), 6);
        assert_eq!(faces[0], FacePosition::Front);
        assert_eq!(faces[5], FacePosition::Down);
    }

    #[test]
    fn test_face_position_names() {
        assert_eq!(FacePosition::Front.name(), "Front (F)");
        assert_eq!(FacePosition::Back.name(), "Back (B)");
        assert_eq!(FacePosition::Right.name(), "Right (R)");
        assert_eq!(FacePosition::Left.name(), "Left (L)");
        assert_eq!(FacePosition::Up.name(), "Up (U)");
        assert_eq!(FacePosition::Down.name(), "Down (D)");
    }

    #[test]
    fn test_face_position_abbreviations() {
        assert_eq!(FacePosition::Front.abbrev(), "F");
        assert_eq!(FacePosition::Back.abbrev(), "B");
        assert_eq!(FacePosition::Right.abbrev(), "R");
        assert_eq!(FacePosition::Left.abbrev(), "L");
        assert_eq!(FacePosition::Up.abbrev(), "U");
        assert_eq!(FacePosition::Down.abbrev(), "D");
    }

    #[test]
    fn test_face_position_solved_colors() {
        assert_eq!(FacePosition::Front.solved_color(), Color::Green);
        assert_eq!(FacePosition::Back.solved_color(), Color::Blue);
        assert_eq!(FacePosition::Right.solved_color(), Color::Red);
        assert_eq!(FacePosition::Left.solved_color(), Color::Orange);
        assert_eq!(FacePosition::Up.solved_color(), Color::White);
        assert_eq!(FacePosition::Down.solved_color(), Color::Yellow);
    }

    #[test]
    fn test_scanned_face_creation() {
        let colors = vec![vec![Color::White; 3]; 3];
        let scanned = ScannedFace {
            position: FacePosition::Up,
            colors,
            confirmed: true,
        };
        assert_eq!(scanned.position, FacePosition::Up);
        assert!(scanned.confirmed);
    }

    #[test]
    fn test_workflow_state_equality() {
        assert_eq!(ScanWorkflowState::NotStarted, ScanWorkflowState::NotStarted);
        assert_eq!(
            ScanWorkflowState::Scanning(FacePosition::Front),
            ScanWorkflowState::Scanning(FacePosition::Front)
        );
        assert_ne!(
            ScanWorkflowState::Scanning(FacePosition::Front),
            ScanWorkflowState::Scanning(FacePosition::Back)
        );
    }

    #[test]
    fn test_scanned_face_colors_dimensions() {
        let colors = vec![vec![Color::Red; 3]; 3];
        assert_eq!(colors.len(), 3);
        assert_eq!(colors[0].len(), 3);
    }

    #[test]
    fn test_cube_size_range() {
        // Test valid cube sizes
        assert!((2..=20).contains(&3));
        assert!((2..=20).contains(&4));
        assert!((2..=20).contains(&5));
    }
}
