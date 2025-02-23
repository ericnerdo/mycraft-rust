use crate::graphics::model;
use cgmath;

const BASE_VERTICES: &[cgmath::Vector3<f32>; 8] = &[
    cgmath::Vector3::new(-0.5, -0.5, -0.5), // 0 - back bottom left
    cgmath::Vector3::new(0.5, -0.5, -0.5),  // 1 - back bottom right
    cgmath::Vector3::new(0.5, 0.5, -0.5),   // 2 - back top right
    cgmath::Vector3::new(-0.5, 0.5, -0.5),  // 3 - back top left
    cgmath::Vector3::new(-0.5, -0.5, 0.5),  // 4 - front bottom left
    cgmath::Vector3::new(0.5, -0.5, 0.5),   // 5 - front bottom right
    cgmath::Vector3::new(0.5, 0.5, 0.5),    // 6 - front top right
    cgmath::Vector3::new(-0.5, 0.5, 0.5),   // 7 - front top left
];

const FACE_VERTEX_INDICES: &[[u32; 6]; 6] = &[
    [4, 5, 6, 6, 7, 4], // Front face
    [1, 0, 3, 3, 2, 1], // Back face
    [7, 6, 2, 2, 3, 7], // Top face
    [0, 1, 5, 5, 4, 0], // Bottom face
    [5, 1, 2, 2, 6, 5], // Right face
    [0, 4, 7, 7, 3, 0], // Left face
];

const FACE_NORMALS: &[cgmath::Vector3<f32>; 6] = &[
    cgmath::Vector3::new(0.0, 0.0, 1.0),  // Front Face
    cgmath::Vector3::new(0.0, 0.0, -1.0), // Back Face
    cgmath::Vector3::new(0.0, 1.0, 0.0),  // Top Face
    cgmath::Vector3::new(0.0, -1.0, 0.0), // Bottom Face
    cgmath::Vector3::new(1.0, 0.0, 0.0),  // Right Face
    cgmath::Vector3::new(-1.0, 0.0, 0.0), // Left Face
];

const FACE_TANGENTS: &[cgmath::Vector3<f32>; 6] = &[
    cgmath::Vector3::new(1.0, 0.0, 0.0),  // Front Face
    cgmath::Vector3::new(-1.0, 0.0, 0.0), // Back Face
    cgmath::Vector3::new(0.0, 0.0, 1.0),  // Top Face
    cgmath::Vector3::new(0.0, 0.0, -1.0), // Bottom Face
    cgmath::Vector3::new(0.0, 1.0, 0.0),  // Right Face
    cgmath::Vector3::new(0.0, -1.0, 0.0), // Left Face
];

const FACE_BITANGENTS: &[cgmath::Vector3<f32>; 6] = &[
    cgmath::Vector3::new(0.0, 1.0, 0.0),  // Front Face
    cgmath::Vector3::new(0.0, -1.0, 0.0), // Back Face
    cgmath::Vector3::new(1.0, 0.0, 0.0),  // Top Face
    cgmath::Vector3::new(-1.0, 0.0, 0.0), // Bottom Face
    cgmath::Vector3::new(0.0, 0.0, 1.0),  // Right Face
    cgmath::Vector3::new(0.0, 0.0, -1.0), // Left Face
];

const FACE_UVS: &[[cgmath::Vector2<f32>; 6]; 6] = &[
    // Front Face
    [
        cgmath::Vector2::new(0.25, 2.0 / 3.0), // Vertex 4
        cgmath::Vector2::new(0.50, 2.0 / 3.0), // Vertex 5
        cgmath::Vector2::new(0.50, 1.0 / 3.0), // Vertex 6
        cgmath::Vector2::new(0.50, 1.0 / 3.0), // Vertex 6
        cgmath::Vector2::new(0.25, 1.0 / 3.0), // Vertex 7
        cgmath::Vector2::new(0.25, 2.0 / 3.0), // Vertex 4
    ],
    // Back Face
    [
        cgmath::Vector2::new(0.75, 2.0 / 3.0), // Vertex 1
        cgmath::Vector2::new(1.00, 2.0 / 3.0), // Vertex 0
        cgmath::Vector2::new(1.00, 1.0 / 3.0), // Vertex 3
        cgmath::Vector2::new(1.00, 1.0 / 3.0), // Vertex 3
        cgmath::Vector2::new(0.75, 1.0 / 3.0), // Vertex 2
        cgmath::Vector2::new(0.75, 2.0 / 3.0), // Vertex 1
    ],
    // Top Face
    [
        cgmath::Vector2::new(0.25, 0.0 / 3.0), // Vertex 7
        cgmath::Vector2::new(0.50, 0.0 / 3.0), // Vertex 6
        cgmath::Vector2::new(0.50, 1.0 / 3.0), // Vertex 2
        cgmath::Vector2::new(0.50, 1.0 / 3.0), // Vertex 2
        cgmath::Vector2::new(0.25, 1.0 / 3.0), // Vertex 3
        cgmath::Vector2::new(0.25, 0.0 / 3.0), // Vertex 7
    ],
    // Bottom Face
    [
        cgmath::Vector2::new(0.25, 2.0 / 3.0), // Vertex 0
        cgmath::Vector2::new(0.50, 2.0 / 3.0), // Vertex 1
        cgmath::Vector2::new(0.50, 1.00),      // Vertex 5
        cgmath::Vector2::new(0.50, 1.00),      // Vertex 5
        cgmath::Vector2::new(0.25, 1.00),      // Vertex 4
        cgmath::Vector2::new(0.25, 2.0 / 3.0), // Vertex 0
    ],
    // Right Face
    [
        cgmath::Vector2::new(0.75, 2.0 / 3.0), // Vertex 5
        cgmath::Vector2::new(0.50, 2.0 / 3.0), // Vertex 1
        cgmath::Vector2::new(0.50, 1.0 / 3.0), // Vertex 2
        cgmath::Vector2::new(0.50, 1.0 / 3.0), // Vertex 2
        cgmath::Vector2::new(0.75, 1.0 / 3.0), // Vertex 6
        cgmath::Vector2::new(0.75, 2.0 / 3.0), // Vertex 5
    ],
    // Left Face
    [
        cgmath::Vector2::new(0.00, 2.0 / 3.0), // Vertex 0
        cgmath::Vector2::new(0.25, 2.0 / 3.0), // Vertex 4
        cgmath::Vector2::new(0.25, 1.0 / 3.0), // Vertex 7
        cgmath::Vector2::new(0.25, 1.0 / 3.0), // Vertex 7
        cgmath::Vector2::new(0.00, 1.0 / 3.0), // Vertex 3
        cgmath::Vector2::new(0.00, 2.0 / 3.0), // Vertex 0
    ],
];

pub fn create_base_block_mesh(material: usize, faces: [bool; 6]) -> model::Mesh {
    create_block_mesh(material, faces, Vec::new())
}

pub fn create_block_mesh(
    material: usize,
    faces: [bool; 6],
    instances: Vec<model::Instance>,
) -> model::Mesh {
    let mut vertices: Vec<model::Vertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for face in 0..FACE_VERTEX_INDICES.len() {
        if faces[face] {
            let vertex_indices = FACE_VERTEX_INDICES[face];
            for i in 0..vertex_indices.len() {
                let index = vertex_indices[i] as usize;
                vertices.push(model::Vertex {
                    position: BASE_VERTICES[index], // Base vertices - centered at origin
                    tex_coords: FACE_UVS[face][i],
                    normal: FACE_NORMALS[face],
                    tangent: FACE_TANGENTS[face],
                    bitangent: FACE_BITANGENTS[face],
                });
                indices.push(indices.len() as u32);
            }
        }
    }

    model::Mesh {
        name: String::from("block"),
        vertices,
        indices,
        material,
        instances,
    }
}
