//! 3D rendering module
//!
//! This module handles all 3D rendering using WGPU, including:
//! - WGPU context setup
//! - Cube mesh generation
//! - Camera controls
//! - Animations

pub mod cube_mesh;
pub mod wgpu_context;

pub use cube_mesh::{CubeMesh, MeshConfig, Vertex};
pub use wgpu_context::{WgpuContext, WgpuContextConfig};
