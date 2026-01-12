/// Camera scanner component for capturing cube state via webcam
///
/// This component provides a UI for scanning Rubik's cubes using the device camera.
/// It handles camera permissions, displays live feed, and captures frames for analysis.

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum CameraState {
    /// Camera not yet initialized
    NotInitialized,
    /// Requesting camera permission
    RequestingPermission,
    /// Camera permission granted and streaming
    Streaming,
    /// Camera permission denied
    PermissionDenied(String),
    /// Camera error occurred
    Error(String),
}

#[derive(Props, Clone, PartialEq)]
pub struct CameraScannerProps {
    /// Callback when a frame is captured for processing
    #[props(optional)]
    pub on_frame_captured: Option<EventHandler<Vec<u8>>>,

    /// Callback when camera state changes
    #[props(optional)]
    pub on_state_change: Option<EventHandler<CameraState>>,

    /// Width of camera feed display
    #[props(default = 640)]
    pub width: u32,

    /// Height of camera feed display
    #[props(default = 480)]
    pub height: u32,
}

#[component]
pub fn CameraScanner(props: CameraScannerProps) -> Element {
    let mut camera_state = use_signal(|| CameraState::NotInitialized);
    let mut video_element = use_signal(|| None::<String>);

    // Handle camera initialization
    let start_camera = move |_| {
        camera_state.set(CameraState::RequestingPermission);

        // On web, we use browser MediaDevices API
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use web_sys::{MediaStream, MediaStreamConstraints};

            spawn(async move {
                let window = web_sys::window().expect("no global `window` exists");
                let navigator = window.navigator();
                let media_devices = navigator
                    .media_devices()
                    .expect("mediaDevices not available");

                let mut constraints = MediaStreamConstraints::new();
                constraints.set_video(&JsValue::TRUE);

                match media_devices.get_user_media_with_constraints(&constraints) {
                    Ok(promise) => {
                        match wasm_bindgen_futures::JsFuture::from(promise).await {
                            Ok(stream) => {
                                let media_stream: MediaStream = stream.unchecked_into();
                                camera_state.set(CameraState::Streaming);

                                // Get video element and attach stream
                                if let Some(document) = window.document() {
                                    if let Some(video) = document.get_element_by_id("camera-video") {
                                        let video_el: web_sys::HtmlVideoElement = video.unchecked_into();
                                        video_el.set_src_object(Some(&media_stream));
                                        let _ = video_el.play();
                                    }
                                }
                            }
                            Err(e) => {
                                camera_state.set(CameraState::PermissionDenied(
                                    format!("Permission denied: {:?}", e)
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        camera_state.set(CameraState::Error(
                            format!("Failed to get media: {:?}", e)
                        ));
                    }
                }
            });
        }

        // On native, we use nokhwa directly (handled by backend)
        #[cfg(not(target_arch = "wasm32"))]
        {
            use crate::camera::{CameraCapture, CameraConfig};

            spawn(async move {
                match CameraCapture::new(CameraConfig::default()) {
                    Ok(mut capture) => {
                        match capture.request_permission() {
                            Ok(_) => {
                                camera_state.set(CameraState::Streaming);
                            }
                            Err(e) => {
                                camera_state.set(CameraState::PermissionDenied(
                                    format!("Permission denied: {}", e)
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        camera_state.set(CameraState::Error(
                            format!("Camera error: {}", e)
                        ));
                    }
                }
            });
        }
    };

    // Render UI based on camera state
    rsx! {
        div {
            class: "camera-scanner",
            style: "display: flex; flex-direction: column; align-items: center; gap: 20px; padding: 20px;",

            h2 {
                style: "font-size: 24px; font-weight: bold; margin: 0;",
                "Cube Scanner"
            }

            match camera_state() {
                CameraState::NotInitialized => rsx! {
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 15px;",

                        p {
                            style: "font-size: 16px; text-align: center; max-width: 400px;",
                            "Click the button below to start your camera and scan your Rubik's Cube."
                        }

                        button {
                            onclick: start_camera,
                            style: "padding: 15px 30px; font-size: 18px; font-weight: bold; \
                                   background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); \
                                   color: white; border: none; border-radius: 10px; cursor: pointer; \
                                   box-shadow: 0 4px 6px rgba(0,0,0,0.1); transition: transform 0.2s;",
                            onmouseenter: move |_| {},
                            onmouseleave: move |_| {},
                            "📷 Start Camera"
                        }
                    }
                },

                CameraState::RequestingPermission => rsx! {
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 10px;",

                        div {
                            style: "width: 50px; height: 50px; border: 4px solid #667eea; \
                                   border-top-color: transparent; border-radius: 50%; \
                                   animation: spin 1s linear infinite;",
                        }

                        p {
                            style: "font-size: 16px;",
                            "Requesting camera permission..."
                        }
                    }
                },

                CameraState::Streaming => rsx! {
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 15px;",

                        div {
                            style: format!("position: relative; width: {}px; height: {}px; \
                                          background: #000; border-radius: 10px; overflow: hidden; \
                                          box-shadow: 0 4px 12px rgba(0,0,0,0.2);",
                                          props.width, props.height),

                            // Video element for camera feed
                            video {
                                id: "camera-video",
                                autoplay: true,
                                playsinline: true,
                                style: format!("width: {}px; height: {}px; object-fit: cover;",
                                              props.width, props.height),
                            }

                            // Overlay grid guide (3x3 for standard cube scanning)
                            div {
                                style: "position: absolute; top: 50%; left: 50%; \
                                       transform: translate(-50%, -50%); \
                                       width: 60%; height: 60%; \
                                       border: 2px solid rgba(255,255,255,0.8); \
                                       border-radius: 5px; \
                                       display: grid; \
                                       grid-template-columns: repeat(3, 1fr); \
                                       grid-template-rows: repeat(3, 1fr); \
                                       gap: 2px;",

                                for i in 0..9 {
                                    div {
                                        key: "{i}",
                                        style: "border: 1px solid rgba(255,255,255,0.3);",
                                    }
                                }
                            }
                        }

                        p {
                            style: "font-size: 14px; color: #22c55e; font-weight: 600;",
                            "✓ Camera Active"
                        }

                        p {
                            style: "font-size: 14px; text-align: center; max-width: 400px;",
                            "Align your cube's face within the grid overlay. \
                             Make sure the lighting is good and the colors are clearly visible."
                        }
                    }
                },

                CameraState::PermissionDenied(msg) => rsx! {
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 15px; \
                               padding: 20px; background: #fee; border-radius: 10px; border: 2px solid #fcc;",

                        p {
                            style: "font-size: 18px; font-weight: bold; color: #c00; margin: 0;",
                            "❌ Camera Access Denied"
                        }

                        p {
                            style: "font-size: 14px; text-align: center; max-width: 400px; margin: 0;",
                            "We need camera access to scan your cube. Please check your browser \
                             settings and allow camera access for this site."
                        }

                        p {
                            style: "font-size: 12px; color: #666; margin: 0;",
                            "Error: {msg}"
                        }

                        button {
                            onclick: start_camera,
                            style: "padding: 10px 20px; font-size: 14px; \
                                   background: #667eea; color: white; border: none; \
                                   border-radius: 5px; cursor: pointer;",
                            "Try Again"
                        }
                    }
                },

                CameraState::Error(msg) => rsx! {
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 15px; \
                               padding: 20px; background: #fef3cd; border-radius: 10px; border: 2px solid #ffc;",

                        p {
                            style: "font-size: 18px; font-weight: bold; color: #856404; margin: 0;",
                            "⚠️ Camera Error"
                        }

                        p {
                            style: "font-size: 14px; text-align: center; max-width: 400px; margin: 0;",
                            "There was a problem accessing your camera. Make sure it's not being \
                             used by another application."
                        }

                        p {
                            style: "font-size: 12px; color: #666; margin: 0;",
                            "Error: {msg}"
                        }

                        button {
                            onclick: start_camera,
                            style: "padding: 10px 20px; font-size: 14px; \
                                   background: #667eea; color: white; border: none; \
                                   border-radius: 5px; cursor: pointer;",
                            "Try Again"
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
    fn test_camera_state_equality() {
        assert_eq!(CameraState::NotInitialized, CameraState::NotInitialized);
        assert_eq!(CameraState::RequestingPermission, CameraState::RequestingPermission);
        assert_eq!(CameraState::Streaming, CameraState::Streaming);

        let err1 = CameraState::Error("test".to_string());
        let err2 = CameraState::Error("test".to_string());
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_camera_state_clone() {
        let state = CameraState::Streaming;
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }
}
