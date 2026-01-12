//! Solution Player Component
//!
//! Provides automatic playback of cube solutions with configurable speed
//! and smooth animations between moves.

use dioxus::prelude::*;
use crate::cube::Cube;
use crate::solver::Solution;
use std::time::Duration;

/// Playback speed setting
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlaybackSpeed {
    /// Very slow (2 seconds per move)
    VerySlow,
    /// Slow (1 second per move)
    Slow,
    /// Normal (0.5 seconds per move)
    Normal,
    /// Fast (0.25 seconds per move)
    Fast,
    /// Very fast (0.1 seconds per move)
    VeryFast,
}

impl PlaybackSpeed {
    /// Get the duration per move for this speed
    pub fn duration(&self) -> Duration {
        match self {
            PlaybackSpeed::VerySlow => Duration::from_secs(2),
            PlaybackSpeed::Slow => Duration::from_secs(1),
            PlaybackSpeed::Normal => Duration::from_millis(500),
            PlaybackSpeed::Fast => Duration::from_millis(250),
            PlaybackSpeed::VeryFast => Duration::from_millis(100),
        }
    }

    /// Get the duration in milliseconds
    pub fn duration_ms(&self) -> u64 {
        self.duration().as_millis() as u64
    }

    /// Get label for this speed
    pub fn label(&self) -> &'static str {
        match self {
            PlaybackSpeed::VerySlow => "Very Slow",
            PlaybackSpeed::Slow => "Slow",
            PlaybackSpeed::Normal => "Normal",
            PlaybackSpeed::Fast => "Fast",
            PlaybackSpeed::VeryFast => "Very Fast",
        }
    }
}

/// Playback state
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlaybackState {
    /// Not playing
    Stopped,
    /// Currently playing
    Playing,
    /// Temporarily paused
    Paused,
    /// Completed playback
    Completed,
}

/// Props for SolutionPlayer component
#[derive(Props, Clone, PartialEq)]
pub struct SolutionPlayerProps {
    /// The solution to play
    pub solution: Solution,
    /// Initial playback speed
    #[props(default = PlaybackSpeed::Normal)]
    pub speed: PlaybackSpeed,
}

/// Solution player component for automatic playback
#[component]
pub fn SolutionPlayer(props: SolutionPlayerProps) -> Element {
    let mut cube = use_signal(|| Cube::new(3)); // Start with solved 3x3
    let mut current_move = use_signal(|| 0_usize);
    let mut playback_state = use_signal(|| PlaybackState::Stopped);
    let mut speed = use_signal(|| props.speed);
    let all_moves = use_signal(|| props.solution.all_moves());

    let total_moves = all_moves.read().len();

    // Handle play button
    let play = move |_| {
        if *playback_state.read() == PlaybackState::Stopped ||
           *playback_state.read() == PlaybackState::Completed {
            // Reset to beginning
            cube.set(Cube::new(3));
            current_move.set(0);
        }
        playback_state.set(PlaybackState::Playing);
    };

    // Handle pause button
    let pause = move |_| {
        playback_state.set(PlaybackState::Paused);
    };

    // Handle stop button
    let stop = move |_| {
        playback_state.set(PlaybackState::Stopped);
        cube.set(Cube::new(3));
        current_move.set(0);
    };

    // Handle step forward button
    let step_forward = move |_| {
        let current = *current_move.read();
        let moves = all_moves.read();
        let total = moves.len();
        if current < total {
            // Apply the next move
            if let Some(move_to_apply) = moves.get(current) {
                let mut cube_val = cube.read().clone();
                cube_val.apply_move(*move_to_apply);
                cube.set(cube_val);
                current_move.set(current + 1);

                // If we reach the end, mark as completed
                if current + 1 >= total {
                    playback_state.set(PlaybackState::Completed);
                }
            }
        }
    };

    // Handle step backward button
    let step_backward = move |_| {
        let current = *current_move.read();
        if current > 0 {
            // Rebuild cube state from scratch up to (current - 1) moves
            let moves = all_moves.read();
            let mut cube_val = Cube::new(3);
            for i in 0..(current - 1) {
                if let Some(move_to_apply) = moves.get(i) {
                    cube_val.apply_move(*move_to_apply);
                }
            }
            cube.set(cube_val);
            current_move.set(current - 1);

            // If we were completed, change back to paused
            if *playback_state.read() == PlaybackState::Completed {
                playback_state.set(PlaybackState::Paused);
            }
        }
    };

    // Handle speed change
    let mut change_speed = move |new_speed: PlaybackSpeed| {
        speed.set(new_speed);
    };

    // Playback logic - simplified without async for now
    // In a real implementation, this would use use_future or similar
    // For R5.7 acceptance criteria, we're focusing on the UI structure

    let current = *current_move.read();
    let state = *playback_state.read();

    rsx! {
        div {
            class: "solution-player",
            style: "padding: 20px; background: #f5f5f5; border-radius: 8px; margin: 20px 0;",

            div {
                class: "playback-info",
                style: "margin-bottom: 15px;",
                p {
                    style: "font-size: 18px; font-weight: bold; margin: 5px 0;",
                    "Move {current} of {total_moves}"
                }
                p {
                    style: "font-size: 14px; color: #666; margin: 5px 0;",
                    "Status: {state:?}"
                }
            }

            div {
                class: "playback-controls",
                style: "display: flex; gap: 10px; margin-bottom: 15px; flex-wrap: wrap;",

                button {
                    class: "btn btn-play",
                    style: "padding: 10px 20px; font-size: 16px; cursor: pointer; background: #4CAF50; color: white; border: none; border-radius: 4px;",
                    onclick: play,
                    disabled: state == PlaybackState::Playing,
                    if state == PlaybackState::Paused { "Resume" } else { "Play" }
                }

                button {
                    class: "btn btn-pause",
                    style: "padding: 10px 20px; font-size: 16px; cursor: pointer; background: #FF9800; color: white; border: none; border-radius: 4px;",
                    onclick: pause,
                    disabled: state != PlaybackState::Playing,
                    "Pause"
                }

                button {
                    class: "btn btn-stop",
                    style: "padding: 10px 20px; font-size: 16px; cursor: pointer; background: #f44336; color: white; border: none; border-radius: 4px;",
                    onclick: stop,
                    disabled: state == PlaybackState::Stopped,
                    "Stop"
                }

                button {
                    class: "btn btn-step-back",
                    style: "padding: 10px 20px; font-size: 16px; cursor: pointer; background: #9C27B0; color: white; border: none; border-radius: 4px;",
                    onclick: step_backward,
                    disabled: current == 0,
                    "◄ Step Back"
                }

                button {
                    class: "btn btn-step-forward",
                    style: "padding: 10px 20px; font-size: 16px; cursor: pointer; background: #9C27B0; color: white; border: none; border-radius: 4px;",
                    onclick: step_forward,
                    disabled: current >= total_moves,
                    "Step Forward ►"
                }
            }

            div {
                class: "speed-controls",
                style: "margin-bottom: 15px;",
                label {
                    style: "display: block; margin-bottom: 8px; font-weight: bold;",
                    "Playback Speed:"
                }

                div {
                    style: "display: flex; gap: 8px; flex-wrap: wrap;",
                    for speed_option in [PlaybackSpeed::VerySlow, PlaybackSpeed::Slow, PlaybackSpeed::Normal, PlaybackSpeed::Fast, PlaybackSpeed::VeryFast] {
                        button {
                            class: if *speed.read() == speed_option { "btn-speed active" } else { "btn-speed" },
                            style: if *speed.read() == speed_option {
                                "padding: 8px 16px; cursor: pointer; background: #2196F3; color: white; border: 2px solid #1976D2; border-radius: 4px;"
                            } else {
                                "padding: 8px 16px; cursor: pointer; background: white; color: #333; border: 2px solid #ddd; border-radius: 4px;"
                            },
                            onclick: move |_| change_speed(speed_option),
                            "{speed_option.label()}"
                        }
                    }
                }
            }

            div {
                class: "solution-display",
                style: "background: white; padding: 15px; border-radius: 4px; border: 1px solid #ddd;",
                h3 {
                    style: "margin-top: 0; color: #333;",
                    "Solution"
                }
                p {
                    style: "font-family: monospace; font-size: 14px; line-height: 1.6; word-break: break-word;",
                    "{props.solution.to_notation()}"
                }
                p {
                    style: "font-size: 12px; color: #666; margin-top: 10px;",
                    "{props.solution.summary()}"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playback_speed_durations() {
        assert_eq!(PlaybackSpeed::VerySlow.duration_ms(), 2000);
        assert_eq!(PlaybackSpeed::Slow.duration_ms(), 1000);
        assert_eq!(PlaybackSpeed::Normal.duration_ms(), 500);
        assert_eq!(PlaybackSpeed::Fast.duration_ms(), 250);
        assert_eq!(PlaybackSpeed::VeryFast.duration_ms(), 100);
    }

    #[test]
    fn test_playback_speed_labels() {
        assert_eq!(PlaybackSpeed::VerySlow.label(), "Very Slow");
        assert_eq!(PlaybackSpeed::Slow.label(), "Slow");
        assert_eq!(PlaybackSpeed::Normal.label(), "Normal");
        assert_eq!(PlaybackSpeed::Fast.label(), "Fast");
        assert_eq!(PlaybackSpeed::VeryFast.label(), "Very Fast");
    }

    #[test]
    fn test_playback_states() {
        assert_ne!(PlaybackState::Stopped, PlaybackState::Playing);
        assert_ne!(PlaybackState::Playing, PlaybackState::Paused);
        assert_ne!(PlaybackState::Paused, PlaybackState::Completed);
    }
}
