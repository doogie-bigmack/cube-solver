//! Move Display Component
//!
//! Displays the current move with notation and kid-friendly explanation.
//! Integrates with the highlight system to show affected pieces.

use dioxus::prelude::*;
use crate::cube::moves::{Move, WideMove};
use crate::cube::ParsedMove;

/// Props for MoveDisplay component
#[derive(Props, Clone, PartialEq)]
pub struct MoveDisplayProps {
    /// The current move to display (None if no move)
    #[props(default = None)]
    pub current_move: Option<ParsedMove>,
    /// Move number in the solution (1-indexed for display)
    #[props(default = 0)]
    pub move_number: usize,
    /// Total number of moves in the solution
    #[props(default = 0)]
    pub total_moves: usize,
}

/// Get kid-friendly explanation for a move
pub fn get_move_explanation(parsed_move: &ParsedMove) -> String {
    match parsed_move {
        ParsedMove::Basic(m) => get_basic_move_explanation(m),
        ParsedMove::Wide(w) => get_wide_move_explanation(w),
    }
}

/// Get kid-friendly explanation for basic moves
fn get_basic_move_explanation(m: &Move) -> String {
    match m {
        // R moves
        Move::R => "Turn the right face clockwise".to_string(),
        Move::RPrime => "Turn the right face counter-clockwise".to_string(),
        Move::R2 => "Turn the right face 180 degrees".to_string(),

        // L moves
        Move::L => "Turn the left face clockwise".to_string(),
        Move::LPrime => "Turn the left face counter-clockwise".to_string(),
        Move::L2 => "Turn the left face 180 degrees".to_string(),

        // U moves
        Move::U => "Turn the top face clockwise".to_string(),
        Move::UPrime => "Turn the top face counter-clockwise".to_string(),
        Move::U2 => "Turn the top face 180 degrees".to_string(),

        // D moves
        Move::D => "Turn the bottom face clockwise".to_string(),
        Move::DPrime => "Turn the bottom face counter-clockwise".to_string(),
        Move::D2 => "Turn the bottom face 180 degrees".to_string(),

        // F moves
        Move::F => "Turn the front face clockwise".to_string(),
        Move::FPrime => "Turn the front face counter-clockwise".to_string(),
        Move::F2 => "Turn the front face 180 degrees".to_string(),

        // B moves
        Move::B => "Turn the back face clockwise".to_string(),
        Move::BPrime => "Turn the back face counter-clockwise".to_string(),
        Move::B2 => "Turn the back face 180 degrees".to_string(),

        // M slice moves (middle layer)
        Move::M => "Turn the middle slice (like L) downward".to_string(),
        Move::MPrime => "Turn the middle slice (like L) upward".to_string(),
        Move::M2 => "Turn the middle slice 180 degrees".to_string(),

        // E slice moves (equator)
        Move::E => "Turn the middle horizontal slice (like D) right".to_string(),
        Move::EPrime => "Turn the middle horizontal slice (like D) left".to_string(),
        Move::E2 => "Turn the middle horizontal slice 180 degrees".to_string(),

        // S slice moves (standing)
        Move::S => "Turn the middle front-back slice (like F) clockwise".to_string(),
        Move::SPrime => "Turn the middle front-back slice (like F) counter-clockwise".to_string(),
        Move::S2 => "Turn the middle front-back slice 180 degrees".to_string(),

        // X rotation (whole cube)
        Move::X => "Rotate the whole cube like turning R".to_string(),
        Move::XPrime => "Rotate the whole cube like turning R'".to_string(),
        Move::X2 => "Rotate the whole cube 180 degrees on R axis".to_string(),

        // Y rotation (whole cube)
        Move::Y => "Rotate the whole cube like turning U".to_string(),
        Move::YPrime => "Rotate the whole cube like turning U'".to_string(),
        Move::Y2 => "Rotate the whole cube 180 degrees on U axis".to_string(),

        // Z rotation (whole cube)
        Move::Z => "Rotate the whole cube like turning F".to_string(),
        Move::ZPrime => "Rotate the whole cube like turning F'".to_string(),
        Move::Z2 => "Rotate the whole cube 180 degrees on F axis".to_string(),
    }
}

/// Get kid-friendly explanation for wide moves
fn get_wide_move_explanation(w: &WideMove) -> String {
    use crate::cube::moves::{WideFace, Direction};

    let face_name = match w.face {
        WideFace::R => "right",
        WideFace::L => "left",
        WideFace::U => "top",
        WideFace::D => "bottom",
        WideFace::F => "front",
        WideFace::B => "back",
    };

    let layers = if w.depth == 2 {
        "two layers".to_string()
    } else {
        format!("{} layers", w.depth)
    };

    let direction = match w.direction {
        Direction::Clockwise => "clockwise",
        Direction::CounterClockwise => "counter-clockwise",
        Direction::Double => "180 degrees",
    };

    format!("Turn {} of the {} side {}", layers, face_name, direction)
}

/// Get the notation string for a parsed move
pub fn get_move_notation(parsed_move: &ParsedMove) -> String {
    match parsed_move {
        ParsedMove::Basic(m) => m.to_notation().to_string(),
        ParsedMove::Wide(w) => w.to_notation().to_string(),
    }
}

/// MoveDisplay component
#[component]
pub fn MoveDisplay(props: MoveDisplayProps) -> Element {
    if let Some(ref current_move) = props.current_move {
        let notation = get_move_notation(current_move);
        let explanation = get_move_explanation(current_move);
        let progress_text = if props.total_moves > 0 {
            format!("Move {} of {}", props.move_number, props.total_moves)
        } else {
            String::new()
        };

        rsx! {
            div {
                class: "move-display",
                style: get_move_display_styles(),
                div {
                    class: "move-notation",
                    "{notation}"
                }
                div {
                    class: "move-explanation",
                    "{explanation}"
                }
                if !progress_text.is_empty() {
                    div {
                        class: "move-progress",
                        "{progress_text}"
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "move-display empty",
                style: get_move_display_styles(),
                div {
                    class: "move-notation",
                    "—"
                }
                div {
                    class: "move-explanation",
                    "No move selected"
                }
            }
        }
    }
}

/// Get CSS styles for the move display component
fn get_move_display_styles() -> &'static str {
    r#"
        padding: 20px;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        border-radius: 12px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        color: white;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        max-width: 500px;
        margin: 0 auto;
    "#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_move_explanations() {
        assert_eq!(get_basic_move_explanation(&Move::R), "Turn the right face clockwise");
        assert_eq!(get_basic_move_explanation(&Move::RPrime), "Turn the right face counter-clockwise");
        assert_eq!(get_basic_move_explanation(&Move::R2), "Turn the right face 180 degrees");

        assert_eq!(get_basic_move_explanation(&Move::L), "Turn the left face clockwise");
        assert_eq!(get_basic_move_explanation(&Move::U), "Turn the top face clockwise");
        assert_eq!(get_basic_move_explanation(&Move::D), "Turn the bottom face clockwise");
        assert_eq!(get_basic_move_explanation(&Move::F), "Turn the front face clockwise");
        assert_eq!(get_basic_move_explanation(&Move::B), "Turn the back face clockwise");
    }

    #[test]
    fn test_slice_move_explanations() {
        assert_eq!(get_basic_move_explanation(&Move::M), "Turn the middle slice (like L) downward");
        assert_eq!(get_basic_move_explanation(&Move::E), "Turn the middle horizontal slice (like D) right");
        assert_eq!(get_basic_move_explanation(&Move::S), "Turn the middle front-back slice (like F) clockwise");
    }

    #[test]
    fn test_rotation_explanations() {
        assert_eq!(get_basic_move_explanation(&Move::X), "Rotate the whole cube like turning R");
        assert_eq!(get_basic_move_explanation(&Move::Y), "Rotate the whole cube like turning U");
        assert_eq!(get_basic_move_explanation(&Move::Z), "Rotate the whole cube like turning F");
    }

    #[test]
    fn test_wide_move_explanation() {
        use crate::cube::moves::{WideFace, Direction};

        let rw = WideMove {
            face: WideFace::R,
            direction: Direction::Clockwise,
            depth: 2,
        };
        assert_eq!(get_wide_move_explanation(&rw), "Turn two layers of the right side clockwise");

        let three_uw_prime = WideMove {
            face: WideFace::U,
            direction: Direction::CounterClockwise,
            depth: 3,
        };
        assert_eq!(get_wide_move_explanation(&three_uw_prime), "Turn 3 layers of the top side counter-clockwise");
    }

    #[test]
    fn test_get_move_notation() {
        assert_eq!(get_move_notation(&ParsedMove::Basic(Move::R)), "R");
        assert_eq!(get_move_notation(&ParsedMove::Basic(Move::UPrime)), "U'");
        assert_eq!(get_move_notation(&ParsedMove::Basic(Move::F2)), "F2");
    }

    #[test]
    fn test_get_move_explanation_basic() {
        let r_move = ParsedMove::Basic(Move::R);
        assert_eq!(get_move_explanation(&r_move), "Turn the right face clockwise");
    }

    #[test]
    fn test_get_move_explanation_wide() {
        use crate::cube::moves::{WideFace, Direction};

        let rw = WideMove {
            face: WideFace::R,
            direction: Direction::Clockwise,
            depth: 2,
        };
        let wide_move = ParsedMove::Wide(rw);
        assert_eq!(get_move_explanation(&wide_move), "Turn two layers of the right side clockwise");
    }
}
