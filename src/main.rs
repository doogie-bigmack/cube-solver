//! Rubik's Cube Solver & Tutorial
//!
//! Educational Rubik's cube solver app for kids supporting 2x2 to 20x20 cubes.

mod cube;

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            class: "app-container",
            h1 { "Rubik's Cube Solver & Tutorial" }
            p { "Welcome! This app helps you learn how to solve Rubik's cubes from 2x2 to 20x20." }
        }
    }
}
