pub struct Vertex {
    pub position: cgmath::Vector3<f32>,
    pub tex_coords: cgmath::Vector2<f32>,
    pub normal: cgmath::Vector3<f32>,
    pub tangent: cgmath::Vector3<f32>,
    pub bitangent: cgmath::Vector3<f32>,
}

pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub material: usize,
    pub instances: Vec<Instance>,
}
