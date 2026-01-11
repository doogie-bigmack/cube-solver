//! Cube mesh generation for any NxN Rubik's cube
//!
//! This module implements R2.2 from the PRD:
//! - Generate vertices for NxN cube
//! - Generate indices for triangles
//! - Include UV coordinates for stickers
//! - Proper normals for lighting
//! - Gap between stickers

use crate::cube::{Color, Cube, FaceName};
use glam::{Vec2, Vec3};

/// A vertex in the cube mesh with position, normal, UV coordinates, and color
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 3],
}

/// Configuration for mesh generation
#[derive(Debug, Clone)]
pub struct MeshConfig {
    /// Size of the entire cube (distance from center to edge)
    pub cube_size: f32,
    /// Gap between stickers as a fraction of sticker size (0.0 to 0.5)
    pub sticker_gap: f32,
    /// Sticker depth/thickness (how much they protrude from cube body)
    pub sticker_depth: f32,
    /// Corner radius for rounded stickers as a fraction of sticker size (0.0 to 0.5)
    pub corner_radius: f32,
    /// Number of segments per rounded corner (higher = smoother)
    pub corner_segments: u32,
}

impl Default for MeshConfig {
    fn default() -> Self {
        Self {
            cube_size: 1.0,
            sticker_gap: 0.05, // 5% gap between stickers
            sticker_depth: 0.02, // Small protrusion
            corner_radius: 0.15, // 15% of sticker size for rounded corners
            corner_segments: 4, // 4 segments per corner for smooth rounding
        }
    }
}

/// Generated mesh data for a Rubik's cube
#[derive(Debug, Clone)]
pub struct CubeMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl CubeMesh {
    /// Generates a mesh for the given cube state
    ///
    /// # Arguments
    ///
    /// * `cube` - The cube state to generate mesh for
    /// * `config` - Mesh generation configuration
    ///
    /// # Returns
    ///
    /// A `CubeMesh` containing vertices and indices
    pub fn generate(cube: &Cube, config: &MeshConfig) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let n = cube.size() as f32;
        let sticker_size = (config.cube_size * 2.0) / n;
        let gap = sticker_size * config.sticker_gap;
        let effective_sticker_size = sticker_size - gap;

        // Generate stickers for all 6 faces
        // Face order: Front, Back, Right, Left, Up, Down
        generate_face_stickers(
            cube,
            &mut vertices,
            &mut indices,
            FaceDirection::Front,
            config,
            n,
            sticker_size,
            effective_sticker_size,
        );
        generate_face_stickers(
            cube,
            &mut vertices,
            &mut indices,
            FaceDirection::Back,
            config,
            n,
            sticker_size,
            effective_sticker_size,
        );
        generate_face_stickers(
            cube,
            &mut vertices,
            &mut indices,
            FaceDirection::Right,
            config,
            n,
            sticker_size,
            effective_sticker_size,
        );
        generate_face_stickers(
            cube,
            &mut vertices,
            &mut indices,
            FaceDirection::Left,
            config,
            n,
            sticker_size,
            effective_sticker_size,
        );
        generate_face_stickers(
            cube,
            &mut vertices,
            &mut indices,
            FaceDirection::Up,
            config,
            n,
            sticker_size,
            effective_sticker_size,
        );
        generate_face_stickers(
            cube,
            &mut vertices,
            &mut indices,
            FaceDirection::Down,
            config,
            n,
            sticker_size,
            effective_sticker_size,
        );

        CubeMesh { vertices, indices }
    }

    /// Returns the number of vertices in the mesh
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of indices in the mesh
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }

    /// Returns the number of triangles in the mesh
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }
}

/// Direction/orientation of a cube face
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FaceDirection {
    Front,  // +Z
    Back,   // -Z
    Right,  // +X
    Left,   // -X
    Up,     // +Y
    Down,   // -Y
}

impl FaceDirection {
    /// Returns the normal vector for this face direction
    fn normal(&self) -> Vec3 {
        match self {
            FaceDirection::Front => Vec3::new(0.0, 0.0, 1.0),
            FaceDirection::Back => Vec3::new(0.0, 0.0, -1.0),
            FaceDirection::Right => Vec3::new(1.0, 0.0, 0.0),
            FaceDirection::Left => Vec3::new(-1.0, 0.0, 0.0),
            FaceDirection::Up => Vec3::new(0.0, 1.0, 0.0),
            FaceDirection::Down => Vec3::new(0.0, -1.0, 0.0),
        }
    }

    /// Returns the cube face name for this direction
    fn face_name(&self) -> FaceName {
        match self {
            FaceDirection::Front => FaceName::F,
            FaceDirection::Back => FaceName::B,
            FaceDirection::Right => FaceName::R,
            FaceDirection::Left => FaceName::L,
            FaceDirection::Up => FaceName::U,
            FaceDirection::Down => FaceName::D,
        }
    }
}

/// Generates stickers for a single face of the cube
fn generate_face_stickers(
    cube: &Cube,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    direction: FaceDirection,
    config: &MeshConfig,
    n: f32,
    sticker_size: f32,
    effective_size: f32,
) {
    let face_name = direction.face_name();
    let normal = direction.normal();
    let face = cube.get_face(face_name);
    let face_colors = face.stickers();

    // Calculate face offset (distance from cube center to face)
    let face_offset = config.cube_size + config.sticker_depth;

    // For each sticker on this face
    for row in 0..(n as usize) {
        for col in 0..(n as usize) {
            let color = face_colors[row][col];
            let color_rgb = color_to_rgb(color);

            // Calculate sticker center position
            let (center, u_dir, v_dir) = calculate_sticker_transform(
                direction,
                row,
                col,
                n,
                config.cube_size,
                sticker_size,
                face_offset,
            );

            // Generate rounded sticker
            generate_rounded_sticker(
                vertices,
                indices,
                center,
                u_dir,
                v_dir,
                normal,
                effective_size,
                config.corner_radius,
                config.corner_segments,
                color_rgb,
            );
        }
    }
}

/// Calculates the transformation (position and orientation) for a sticker
fn calculate_sticker_transform(
    direction: FaceDirection,
    row: usize,
    col: usize,
    n: f32,
    _cube_size: f32,
    sticker_size: f32,
    face_offset: f32,
) -> (Vec3, Vec3, Vec3) {
    // Calculate UV coordinates in face space (-1 to 1)
    let u = (col as f32 - (n - 1.0) / 2.0) * sticker_size;
    let v = ((n - 1.0) / 2.0 - row as f32) * sticker_size; // Flip V to match top-to-bottom

    match direction {
        FaceDirection::Front => {
            // Front face (+Z)
            let center = Vec3::new(u, v, face_offset);
            let u_dir = Vec3::new(1.0, 0.0, 0.0);
            let v_dir = Vec3::new(0.0, 1.0, 0.0);
            (center, u_dir, v_dir)
        }
        FaceDirection::Back => {
            // Back face (-Z)
            let center = Vec3::new(-u, v, -face_offset);
            let u_dir = Vec3::new(-1.0, 0.0, 0.0);
            let v_dir = Vec3::new(0.0, 1.0, 0.0);
            (center, u_dir, v_dir)
        }
        FaceDirection::Right => {
            // Right face (+X)
            let center = Vec3::new(face_offset, v, -u);
            let u_dir = Vec3::new(0.0, 0.0, -1.0);
            let v_dir = Vec3::new(0.0, 1.0, 0.0);
            (center, u_dir, v_dir)
        }
        FaceDirection::Left => {
            // Left face (-X)
            let center = Vec3::new(-face_offset, v, u);
            let u_dir = Vec3::new(0.0, 0.0, 1.0);
            let v_dir = Vec3::new(0.0, 1.0, 0.0);
            (center, u_dir, v_dir)
        }
        FaceDirection::Up => {
            // Up face (+Y)
            let center = Vec3::new(u, face_offset, -v);
            let u_dir = Vec3::new(1.0, 0.0, 0.0);
            let v_dir = Vec3::new(0.0, 0.0, -1.0);
            (center, u_dir, v_dir)
        }
        FaceDirection::Down => {
            // Down face (-Y)
            let center = Vec3::new(u, -face_offset, v);
            let u_dir = Vec3::new(1.0, 0.0, 0.0);
            let v_dir = Vec3::new(0.0, 0.0, 1.0);
            (center, u_dir, v_dir)
        }
    }
}

/// Generates a quad (2 triangles) for a single sticker
/// Note: This is a simple quad version. For rounded corners, see generate_rounded_sticker
fn generate_sticker_quad(
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    center: Vec3,
    u_dir: Vec3,
    v_dir: Vec3,
    normal: Vec3,
    size: f32,
    color: [f32; 3],
) {
    let half_size = size / 2.0;
    let base_index = vertices.len() as u32;

    // Four corners of the quad
    let positions = [
        center - u_dir * half_size - v_dir * half_size, // Bottom-left
        center + u_dir * half_size - v_dir * half_size, // Bottom-right
        center + u_dir * half_size + v_dir * half_size, // Top-right
        center - u_dir * half_size + v_dir * half_size, // Top-left
    ];

    // UV coordinates for the quad
    let uvs = [
        Vec2::new(0.0, 1.0), // Bottom-left
        Vec2::new(1.0, 1.0), // Bottom-right
        Vec2::new(1.0, 0.0), // Top-right
        Vec2::new(0.0, 0.0), // Top-left
    ];

    // Add vertices
    for i in 0..4 {
        vertices.push(Vertex {
            position: positions[i].to_array(),
            normal: normal.to_array(),
            uv: uvs[i].to_array(),
            color,
        });
    }

    // Add indices for two triangles (counter-clockwise winding)
    indices.extend_from_slice(&[
        base_index,
        base_index + 1,
        base_index + 2,
        base_index,
        base_index + 2,
        base_index + 3,
    ]);
}

/// Generates a sticker with rounded corners
fn generate_rounded_sticker(
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
    center: Vec3,
    u_dir: Vec3,
    v_dir: Vec3,
    normal: Vec3,
    size: f32,
    corner_radius_fraction: f32,
    corner_segments: u32,
    color: [f32; 3],
) {
    use std::f32::consts::PI;

    let half_size = size / 2.0;
    let corner_radius = size * corner_radius_fraction;
    let inner_half = half_size - corner_radius;

    let base_index = vertices.len() as u32;

    // Center vertex for triangle fan
    vertices.push(Vertex {
        position: center.to_array(),
        normal: normal.to_array(),
        uv: [0.5, 0.5],
        color,
    });

    let mut edge_vertices = Vec::new();

    // Generate vertices around the perimeter with rounded corners
    // We'll go counter-clockwise: bottom-left -> bottom-right -> top-right -> top-left

    // Bottom edge (left to right)
    edge_vertices.push(center - u_dir * inner_half - v_dir * half_size);
    edge_vertices.push(center + u_dir * inner_half - v_dir * half_size);

    // Bottom-right corner
    let corner_center = center + u_dir * inner_half - v_dir * inner_half;
    for i in 0..=corner_segments {
        let angle = PI * 1.5 + (PI * 0.5 * i as f32 / corner_segments as f32);
        let offset = u_dir * angle.cos() * corner_radius + v_dir * angle.sin() * corner_radius;
        edge_vertices.push(corner_center + offset);
    }

    // Right edge (bottom to top)
    edge_vertices.push(center + u_dir * half_size - v_dir * inner_half);
    edge_vertices.push(center + u_dir * half_size + v_dir * inner_half);

    // Top-right corner
    let corner_center = center + u_dir * inner_half + v_dir * inner_half;
    for i in 0..=corner_segments {
        let angle = PI * 0.0 + (PI * 0.5 * i as f32 / corner_segments as f32);
        let offset = u_dir * angle.cos() * corner_radius + v_dir * angle.sin() * corner_radius;
        edge_vertices.push(corner_center + offset);
    }

    // Top edge (right to left)
    edge_vertices.push(center + u_dir * inner_half + v_dir * half_size);
    edge_vertices.push(center - u_dir * inner_half + v_dir * half_size);

    // Top-left corner
    let corner_center = center - u_dir * inner_half + v_dir * inner_half;
    for i in 0..=corner_segments {
        let angle = PI * 0.5 + (PI * 0.5 * i as f32 / corner_segments as f32);
        let offset = u_dir * angle.cos() * corner_radius + v_dir * angle.sin() * corner_radius;
        edge_vertices.push(corner_center + offset);
    }

    // Left edge (top to bottom)
    edge_vertices.push(center - u_dir * half_size + v_dir * inner_half);
    edge_vertices.push(center - u_dir * half_size - v_dir * inner_half);

    // Bottom-left corner
    let corner_center = center - u_dir * inner_half - v_dir * inner_half;
    for i in 0..=corner_segments {
        let angle = PI * 1.0 + (PI * 0.5 * i as f32 / corner_segments as f32);
        let offset = u_dir * angle.cos() * corner_radius + v_dir * angle.sin() * corner_radius;
        edge_vertices.push(corner_center + offset);
    }

    // Add edge vertices to mesh
    for pos in &edge_vertices {
        let local = *pos - center;
        let u = (local.dot(u_dir) / half_size + 1.0) * 0.5;
        let v = (local.dot(v_dir) / half_size + 1.0) * 0.5;

        vertices.push(Vertex {
            position: pos.to_array(),
            normal: normal.to_array(),
            uv: [u, v],
            color,
        });
    }

    // Create triangle fan from center to all edge vertices
    let num_edge_vertices = edge_vertices.len() as u32;
    for i in 0..num_edge_vertices {
        let next = (i + 1) % num_edge_vertices;
        indices.extend_from_slice(&[
            base_index,         // Center
            base_index + 1 + i, // Current edge vertex
            base_index + 1 + next, // Next edge vertex
        ]);
    }
}

/// Converts a Color enum to RGB values (0.0 to 1.0)
fn color_to_rgb(color: Color) -> [f32; 3] {
    match color {
        Color::White => [1.0, 1.0, 1.0],
        Color::Yellow => [1.0, 1.0, 0.0],
        Color::Red => [1.0, 0.0, 0.0],
        Color::Orange => [1.0, 0.5, 0.0],
        Color::Blue => [0.0, 0.0, 1.0],
        Color::Green => [0.0, 0.8, 0.0],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::Cube;

    #[test]
    fn test_mesh_generation_2x2() {
        let cube = Cube::new(2);
        let config = MeshConfig::default();
        let mesh = CubeMesh::generate(&cube, &config);

        // 2x2 cube has 6 faces * 4 stickers = 24 stickers
        // With rounded corners, each sticker has many vertices
        assert!(mesh.vertex_count() > 0);
        assert!(mesh.index_count() > 0);
        assert!(mesh.triangle_count() > 0);

        // Sanity check: vertices should be divisible by 3 for triangles
        assert_eq!(mesh.index_count() % 3, 0);
    }

    #[test]
    fn test_mesh_generation_3x3() {
        let cube = Cube::new(3);
        let config = MeshConfig::default();
        let mesh = CubeMesh::generate(&cube, &config);

        // 3x3 cube has 6 faces * 9 stickers = 54 stickers
        // With rounded corners, each sticker has many vertices
        assert!(mesh.vertex_count() > 54); // More than 1 vertex per sticker
        assert!(mesh.index_count() > 54 * 3); // At least 1 triangle per sticker
        assert!(mesh.triangle_count() > 54);
        assert_eq!(mesh.index_count() % 3, 0); // Indices divisible by 3
    }

    #[test]
    fn test_mesh_generation_4x4() {
        let cube = Cube::new(4);
        let config = MeshConfig::default();
        let mesh = CubeMesh::generate(&cube, &config);

        // 4x4 cube has 6 faces * 16 stickers = 96 stickers
        assert!(mesh.vertex_count() > 96);
        assert!(mesh.index_count() > 96 * 3);
        assert!(mesh.triangle_count() > 96);
        assert_eq!(mesh.index_count() % 3, 0);
    }

    #[test]
    fn test_mesh_config_default() {
        let config = MeshConfig::default();
        assert_eq!(config.cube_size, 1.0);
        assert_eq!(config.sticker_gap, 0.05);
        assert_eq!(config.sticker_depth, 0.02);
    }

    #[test]
    fn test_mesh_config_custom() {
        let config = MeshConfig {
            cube_size: 2.0,
            sticker_gap: 0.1,
            sticker_depth: 0.05,
            corner_radius: 0.2,
            corner_segments: 6,
        };
        assert_eq!(config.cube_size, 2.0);
        assert_eq!(config.sticker_gap, 0.1);
        assert_eq!(config.sticker_depth, 0.05);
        assert_eq!(config.corner_radius, 0.2);
        assert_eq!(config.corner_segments, 6);
    }

    #[test]
    fn test_color_conversion() {
        assert_eq!(color_to_rgb(Color::White), [1.0, 1.0, 1.0]);
        assert_eq!(color_to_rgb(Color::Yellow), [1.0, 1.0, 0.0]);
        assert_eq!(color_to_rgb(Color::Red), [1.0, 0.0, 0.0]);
        assert_eq!(color_to_rgb(Color::Orange), [1.0, 0.5, 0.0]);
        assert_eq!(color_to_rgb(Color::Blue), [0.0, 0.0, 1.0]);
        assert_eq!(color_to_rgb(Color::Green), [0.0, 0.8, 0.0]);
    }

    #[test]
    fn test_face_direction_normal() {
        let epsilon = 0.0001;
        let front_normal = FaceDirection::Front.normal();
        assert!((front_normal.x - 0.0).abs() < epsilon);
        assert!((front_normal.y - 0.0).abs() < epsilon);
        assert!((front_normal.z - 1.0).abs() < epsilon);

        let up_normal = FaceDirection::Up.normal();
        assert!((up_normal.x - 0.0).abs() < epsilon);
        assert!((up_normal.y - 1.0).abs() < epsilon);
        assert!((up_normal.z - 0.0).abs() < epsilon);
    }

    #[test]
    fn test_face_direction_name() {
        assert_eq!(FaceDirection::Right.face_name(), FaceName::R);
        assert_eq!(FaceDirection::Left.face_name(), FaceName::L);
        assert_eq!(FaceDirection::Up.face_name(), FaceName::U);
        assert_eq!(FaceDirection::Down.face_name(), FaceName::D);
        assert_eq!(FaceDirection::Front.face_name(), FaceName::F);
        assert_eq!(FaceDirection::Back.face_name(), FaceName::B);
    }

    #[test]
    fn test_vertex_has_all_attributes() {
        let vertex = Vertex {
            position: [1.0, 2.0, 3.0],
            normal: [0.0, 1.0, 0.0],
            uv: [0.5, 0.5],
            color: [1.0, 0.0, 0.0],
        };

        assert_eq!(vertex.position, [1.0, 2.0, 3.0]);
        assert_eq!(vertex.normal, [0.0, 1.0, 0.0]);
        assert_eq!(vertex.uv, [0.5, 0.5]);
        assert_eq!(vertex.color, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_mesh_indices_are_valid() {
        let cube = Cube::new(2);
        let config = MeshConfig::default();
        let mesh = CubeMesh::generate(&cube, &config);

        // All indices should be within valid range
        let max_index = mesh.vertex_count() as u32;
        for &index in &mesh.indices {
            assert!(index < max_index, "Index {} out of range (max: {})", index, max_index);
        }
    }

    #[test]
    fn test_mesh_generation_5x5() {
        let cube = Cube::new(5);
        let config = MeshConfig::default();
        let mesh = CubeMesh::generate(&cube, &config);

        // 5x5 cube has 6 faces * 25 stickers = 150 stickers
        assert!(mesh.vertex_count() > 150);
        assert!(mesh.index_count() > 150 * 3);
        assert_eq!(mesh.index_count() % 3, 0);
    }

    #[test]
    fn test_mesh_with_custom_gap() {
        let cube = Cube::new(3);
        let config = MeshConfig {
            cube_size: 1.0,
            sticker_gap: 0.2, // Larger gap
            sticker_depth: 0.02,
            corner_radius: 0.15,
            corner_segments: 4,
        };
        let mesh = CubeMesh::generate(&cube, &config);

        // Should still generate valid mesh
        assert!(mesh.vertex_count() > 54);
        assert!(mesh.index_count() > 54 * 3);
        assert_eq!(mesh.index_count() % 3, 0);
    }
}
