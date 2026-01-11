//! 3D rendering module
//!
//! This module handles all 3D rendering using WGPU, including:
//! - WGPU context setup
//! - Cube mesh generation
//! - Camera controls
//! - Animations
//! - Piece highlighting

pub mod animations;
pub mod camera;
pub mod cube_mesh;
pub mod highlight;
pub mod wgpu_context;

pub use animations::{AnimationQueue, EasingFunction, RotationAnimation, RotationFace};
pub use camera::{Camera, OrbitController};
pub use cube_mesh::{CubeMesh, MeshConfig, Vertex};
pub use highlight::{HighlightConfig, HighlightManager, HighlightType, PieceId};
pub use wgpu_context::{WgpuContext, WgpuContextConfig};
