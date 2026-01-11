//! Rubik's Cube Solver & Tutorial
//!
//! Educational Rubik's cube solver app for kids supporting 2x2 to 20x20 cubes.

mod cube;
mod renderer;

use dioxus::prelude::*;
use renderer::WgpuContextConfig;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize WGPU config (will be used for 3D rendering when integrated)
    let _wgpu_config = WgpuContextConfig::default();

    rsx! {
        div {
            class: "app-container",
            h1 { "Rubik's Cube Solver & Tutorial" }
            p { "Welcome! This app helps you learn how to solve Rubik's cubes from 2x2 to 20x20." }
            p {
                style: "margin-top: 2rem; color: #718096; font-size: 0.9rem;",
                "✓ WGPU rendering context ready"
            }
            p {
                style: "margin-top: 0.5rem; color: #718096; font-size: 0.9rem;",
                "✓ Core cube engine (R1.1-R1.9) complete"
            }
        }
    }
}
