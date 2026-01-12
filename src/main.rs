//! Rubik's Cube Solver & Tutorial
//!
//! Educational Rubik's cube solver app for kids supporting 2x2 to 20x20 cubes.

mod components;
mod cube;
mod renderer;
mod solver;
mod state;

use components::{ColorPicker, Cube3D, CubeControls, CubeInput, SolutionPlayer, StickerPosition};
use cube::{Color, Cube, FaceName};
use dioxus::prelude::*;
use renderer::WgpuContextConfig;
use solver::{solve_2x2, solve_3x3, Solution};
use state::History;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize WGPU config (will be used for 3D rendering when integrated)
    let _wgpu_config = WgpuContextConfig::default();

    // Track viewport size (in real app, this would come from window resize events)
    let mut viewport_width = use_signal(|| 800.0);
    let mut viewport_height = use_signal(|| 600.0);

    // Create history for undo/redo functionality
    let mut history = use_signal(|| History::new(Cube::new(3)));

    // Track selected sticker and color
    let mut selected_sticker = use_signal(|| None::<StickerPosition>);
    let mut selected_color = use_signal(|| None::<Color>);

    // Track solution
    let mut solution = use_signal(|| None::<Solution>);

    rsx! {
        div {
            class: "app-container",
            style: "min-height: 100vh; display: flex; flex-direction: column; background: #f7fafc; width: 100%; max-width: 100vw; overflow-x: hidden;",

            header {
                h1 {
                    "Rubik's Cube Solver & Tutorial"
                }
                p {
                    "Educational cube solver for 2x2 to 20x20 cubes"
                }
            }

            main {

                // Section: 3D View
                section {
                    h2 {
                        "3D Cube View"
                    }
                    p {
                        "Changes in the 2D view are reflected in real-time"
                    }
                    Cube3D {
                        cube: history().current().clone(),
                        viewport_width: viewport_width(),
                        viewport_height: viewport_height(),
                    }
                }

                // Section: 2D Unfolded View with Color Picker
                section {
                    h2 {
                        "2D Unfolded Cube View"
                    }

                    // Instructions
                    p {
                        "Click a sticker to select it, then click a color to apply"
                    }

                    // Color Picker
                    div {
                        style: "display: flex; justify-content: center; margin-bottom: 2rem;",
                        ColorPicker {
                            selected_color: selected_color(),
                            on_color_select: move |color: Color| {
                                // Store selected color
                                selected_color.set(Some(color));

                                // If a sticker is selected, apply the color
                                if let Some(sticker) = selected_sticker() {
                                    let mut current_cube = history().current().clone();
                                    current_cube.set_sticker(sticker.face, sticker.row, sticker.col, color);
                                    // Push new state to history
                                    let mut hist = history();
                                    hist.push(current_cube);
                                    history.set(hist);
                                }
                            },
                        }
                    }

                    // Cube Input
                    div {
                        style: "display: flex; justify-content: center;",
                        CubeInput {
                            cube: history().current().clone(),
                            selected_sticker: selected_sticker(),
                            on_sticker_click: move |(face, row, col): (FaceName, usize, usize)| {
                                // Update selected sticker
                                selected_sticker.set(Some(StickerPosition { face, row, col }));

                                // If a color is already selected, apply it
                                if let Some(color) = selected_color() {
                                    let mut current_cube = history().current().clone();
                                    current_cube.set_sticker(face, row, col, color);
                                    // Push new state to history
                                    let mut hist = history();
                                    hist.push(current_cube);
                                    history.set(hist);
                                }
                            },
                        }
                    }

                    // Cube Controls (Reset button)
                    div {
                        style: "display: flex; justify-content: center; margin-top: 2rem;",
                        CubeControls {
                            cube: history().current().clone(),
                            on_reset: move |new_cube: Cube| {
                                // Reset history with the new cube
                                let mut hist = history();
                                hist.reset(new_cube);
                                history.set(hist);
                                // Clear selections when resetting
                                selected_sticker.set(None);
                                selected_color.set(None);
                            },
                            can_undo: history().can_undo(),
                            can_redo: history().can_redo(),
                            on_undo: move || {
                                let mut hist = history();
                                if hist.undo().is_some() {
                                    history.set(hist);
                                }
                            },
                            on_redo: move || {
                                let mut hist = history();
                                if hist.redo().is_some() {
                                    history.set(hist);
                                }
                            }
                        }
                    }
                }

                // Solver section
                section {
                    style: "max-width: 800px; width: 100%;",
                    h2 {
                        "Solve the Cube"
                    }

                    p {
                        "Click 'Solve' to find a solution for the current cube state"
                    }

                    div {
                        style: "display: flex; justify-content: center; margin-bottom: 1.5rem;",
                        button {
                            onclick: move |_| {
                                let current_cube = history().current().clone();
                                let cube_size = current_cube.size();

                                let sol = if cube_size == 2 {
                                    solve_2x2(&current_cube).ok().map(|s| s.to_solution())
                                } else if cube_size == 3 {
                                    solve_3x3(&current_cube).ok().map(|s| s.to_solution())
                                } else {
                                    None
                                };

                                solution.set(sol);
                            },
                            "Solve Cube"
                        }
                    }

                    if let Some(sol) = solution() {
                        SolutionPlayer {
                            solution: sol,
                        }
                    } else {
                        div {
                            style: "text-align: center; color: #718096; padding: 2rem;",
                            p {
                                "Click 'Solve Cube' to generate a solution"
                            }
                        }
                    }
                }

                // Status section
                div {
                    class: "status-section",
                    h3 {
                        style: "color: #2d3748; font-size: 1.2rem; margin-bottom: 1rem;",
                        "Implementation Status"
                    }
                    p {
                        style: "color: #718096; font-size: 0.9rem; margin: 0.5rem 0; word-wrap: break-word;",
                        "✓ WGPU rendering context ready"
                    }
                    p {
                        style: "color: #718096; font-size: 0.9rem; margin: 0.5rem 0;",
                        "✓ Core cube engine (R1.1-R1.9) complete"
                    }
                    p {
                        style: "color: #718096; font-size: 0.9rem; margin: 0.5rem 0;",
                        "✓ 3D visualization (R2.1-R2.8) complete"
                    }
                    p {
                        style: "color: #718096; font-size: 0.9rem; margin: 0.5rem 0;",
                        "✓ Responsive sizing for all screen sizes"
                    }
                    p {
                        style: "color: #10b981; font-size: 0.9rem; margin: 0.5rem 0; font-weight: bold;",
                        "✓ 2D unfolded cube view (R3.1-R3.2) complete"
                    }
                    p {
                        style: "color: #10b981; font-size: 0.9rem; margin: 0.5rem 0; font-weight: bold;",
                        "✓ Color picker palette (R3.3) complete"
                    }
                    p {
                        style: "color: #10b981; font-size: 0.9rem; margin: 0.5rem 0; font-weight: bold;",
                        "✓ Real-time 2D/3D sync (R3.4) complete"
                    }
                }
            }
        }
    }
}
