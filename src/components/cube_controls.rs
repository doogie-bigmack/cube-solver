//! Cube Controls Component
//!
//! This module provides UI controls for cube manipulation including
//! reset to solved state with confirmation dialog.

use crate::cube::Cube;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CubeControlsProps {
    /// Current cube state
    pub cube: Cube,
    /// Callback when cube is reset
    pub on_reset: EventHandler<Cube>,
    /// Whether undo is available
    pub can_undo: bool,
    /// Whether redo is available
    pub can_redo: bool,
    /// Callback for undo
    pub on_undo: EventHandler<()>,
    /// Callback for redo
    pub on_redo: EventHandler<()>,
}

/// CubeControls component provides controls for manipulating the cube.
///
/// # Requirements
/// - R3.6: Reset to solved state
///   - Button to reset cube to solved
///   - Confirmation dialog
///   - Works for any size
///
/// # Example
/// ```rust,ignore
/// use dioxus::prelude::*;
/// use rubiks_cube_solver::components::CubeControls;
/// use rubiks_cube_solver::cube::Cube;
///
/// #[component]
/// fn App() -> Element {
///     let mut cube = use_signal(|| Cube::new(3));
///
///     rsx! {
///         CubeControls {
///             cube: cube(),
///             on_reset: move |new_cube: Cube| {
///                 cube.set(new_cube);
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn CubeControls(props: CubeControlsProps) -> Element {
    let mut show_confirm_dialog = use_signal(|| false);
    let cube_size = props.cube.size();

    // Compute styles based on can_undo/can_redo
    let undo_style = if props.can_undo {
        "min-width: 150px; min-height: 50px; padding: 1rem 2rem; background: linear-gradient(135deg, #48bb78 0%, #38a169 100%); color: white; border: none; border-radius: 12px; font-size: 1.1rem; font-weight: 600; cursor: pointer; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); transition: all 0.3s ease; opacity: 1;"
    } else {
        "min-width: 150px; min-height: 50px; padding: 1rem 2rem; background: #cbd5e0; color: #a0aec0; border: none; border-radius: 12px; font-size: 1.1rem; font-weight: 600; cursor: not-allowed; box-shadow: none; transition: all 0.3s ease; opacity: 0.6;"
    };

    let redo_style = if props.can_redo {
        "min-width: 150px; min-height: 50px; padding: 1rem 2rem; background: linear-gradient(135deg, #4299e1 0%, #3182ce 100%); color: white; border: none; border-radius: 12px; font-size: 1.1rem; font-weight: 600; cursor: pointer; box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); transition: all 0.3s ease; opacity: 1;"
    } else {
        "min-width: 150px; min-height: 50px; padding: 1rem 2rem; background: #cbd5e0; color: #a0aec0; border: none; border-radius: 12px; font-size: 1.1rem; font-weight: 600; cursor: not-allowed; box-shadow: none; transition: all 0.3s ease; opacity: 0.6;"
    };

    rsx! {
        div {
            class: "cube-controls",
            style: "display: flex; flex-direction: column; gap: 1rem; padding: 1rem;",

            // Undo/Redo buttons row
            div {
                style: "display: flex; gap: 1rem; justify-content: center;",

                // Undo button
                button {
                    r#type: "button",
                    class: "undo-button",
                    disabled: !props.can_undo,
                    style: "{undo_style}",
                    onclick: move |_| {
                        if props.can_undo {
                            props.on_undo.call(());
                        }
                    },
                    "↶ Undo"
                }

                // Redo button
                button {
                    r#type: "button",
                    class: "redo-button",
                    disabled: !props.can_redo,
                    style: "{redo_style}",
                    onclick: move |_| {
                        if props.can_redo {
                            props.on_redo.call(());
                        }
                    },
                    "↷ Redo"
                }
            }

            // Reset button
            button {
                r#type: "button",
                class: "reset-button",
                style: "
                    min-width: 200px;
                    min-height: 50px;
                    padding: 1rem 2rem;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    color: white;
                    border: none;
                    border-radius: 12px;
                    font-size: 1.1rem;
                    font-weight: 600;
                    cursor: pointer;
                    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                    transition: all 0.3s ease;
                ",
                onclick: move |_| {
                    show_confirm_dialog.set(true);
                },
                "🔄 Reset to Solved"
            }

            // Confirmation dialog
            if show_confirm_dialog() {
                div {
                    class: "confirm-dialog-overlay",
                    style: "
                        position: fixed;
                        top: 0;
                        left: 0;
                        right: 0;
                        bottom: 0;
                        background: rgba(0, 0, 0, 0.5);
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        z-index: 1000;
                        backdrop-filter: blur(4px);
                    ",
                    onclick: move |_| {
                        // Close dialog when clicking on overlay
                        // In a production app, you might want to check if the click
                        // was on the overlay vs the dialog content
                        show_confirm_dialog.set(false);
                    },

                    div {
                        class: "confirm-dialog",
                        style: "
                            background: white;
                            padding: 2rem;
                            border-radius: 16px;
                            box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
                            max-width: 400px;
                            width: 90%;
                            animation: slideIn 0.3s ease;
                        ",
                        onclick: move |evt| {
                            // Prevent clicks on dialog from closing it
                            evt.stop_propagation();
                        },

                        // Dialog icon
                        div {
                            style: "text-align: center; font-size: 3rem; margin-bottom: 1rem;",
                            "⚠️"
                        }

                        // Dialog title
                        h3 {
                            style: "
                                color: #2d3748;
                                font-size: 1.5rem;
                                font-weight: 700;
                                text-align: center;
                                margin-bottom: 1rem;
                            ",
                            "Reset Cube?"
                        }

                        // Dialog message
                        p {
                            style: "
                                color: #4a5568;
                                font-size: 1rem;
                                text-align: center;
                                margin-bottom: 2rem;
                                line-height: 1.6;
                            ",
                            "This will reset your {cube_size}x{cube_size} cube back to the solved state. All changes will be lost."
                        }

                        // Button group
                        div {
                            style: "
                                display: flex;
                                gap: 1rem;
                                justify-content: center;
                            ",

                            // Cancel button
                            button {
                                r#type: "button",
                                class: "cancel-button",
                                style: "
                                    min-width: 120px;
                                    min-height: 48px;
                                    padding: 0.75rem 1.5rem;
                                    background: #e2e8f0;
                                    color: #2d3748;
                                    border: 2px solid #cbd5e0;
                                    border-radius: 10px;
                                    font-size: 1rem;
                                    font-weight: 600;
                                    cursor: pointer;
                                    transition: all 0.2s ease;
                                ",
                                onclick: move |_| {
                                    show_confirm_dialog.set(false);
                                },
                                "Cancel"
                            }

                            // Confirm button
                            button {
                                r#type: "button",
                                class: "confirm-button",
                                style: "
                                    min-width: 120px;
                                    min-height: 48px;
                                    padding: 0.75rem 1.5rem;
                                    background: linear-gradient(135deg, #f56565 0%, #c53030 100%);
                                    color: white;
                                    border: none;
                                    border-radius: 10px;
                                    font-size: 1rem;
                                    font-weight: 600;
                                    cursor: pointer;
                                    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
                                    transition: all 0.2s ease;
                                ",
                                onclick: move |_| {
                                    // Create new solved cube of the same size
                                    let new_cube = Cube::new(cube_size);
                                    props.on_reset.call(new_cube);
                                    show_confirm_dialog.set(false);
                                },
                                "Reset"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_controls_creation() {
        // Test that we can create CubeControls props
        let cube = Cube::new(3);

        // Just verify the cube is in solved state
        assert!(cube.is_solved());
    }

    #[test]
    fn test_reset_creates_solved_cube() {
        // Test that resetting creates a solved cube
        for size in 2..=10 {
            let cube = Cube::new(size);
            assert!(cube.is_solved());
            assert_eq!(cube.size(), size);
        }
    }

    #[test]
    fn test_reset_works_for_all_sizes() {
        // Test R3.6 acceptance criteria: Works for any size
        for size in [2, 3, 4, 5, 7, 10, 15, 20] {
            let cube = Cube::new(size);
            assert!(cube.is_solved(), "Cube size {} should be solved", size);
        }
    }
}
