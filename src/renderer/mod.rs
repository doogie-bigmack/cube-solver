//! 3D rendering module
//!
//! This module handles all 3D rendering using WGPU, including:
//! - WGPU context setup
//! - Cube mesh generation
//! - Camera controls
//! - Animations
//! - Piece highlighting
//!
//! Note: This module is only available with the `desktop_3d` feature enabled.

#[cfg(feature = "desktop_3d")]
pub mod animations;
#[cfg(feature = "desktop_3d")]
pub mod camera;
#[cfg(feature = "desktop_3d")]
pub mod cube_mesh;
#[cfg(feature = "desktop_3d")]
pub mod highlight;
#[cfg(feature = "desktop_3d")]
pub mod wgpu_context;

#[cfg(feature = "desktop_3d")]
pub use animations::{AnimationQueue, EasingFunction, RotationAnimation, RotationFace};
#[cfg(feature = "desktop_3d")]
pub use camera::{Camera, OrbitController};
#[cfg(feature = "desktop_3d")]
pub use cube_mesh::{CubeMesh, MeshConfig, Vertex};
#[cfg(feature = "desktop_3d")]
pub use highlight::{HighlightConfig, HighlightManager, HighlightType, PieceId};
#[cfg(feature = "desktop_3d")]
pub use wgpu_context::{WgpuContext, WgpuContextConfig};

// Stub types for when desktop_3d is not enabled
#[cfg(not(feature = "desktop_3d"))]
#[derive(Debug, Clone)]
pub struct WgpuContextConfig;

#[cfg(not(feature = "desktop_3d"))]
impl Default for WgpuContextConfig {
    fn default() -> Self {
        Self
    }
}
