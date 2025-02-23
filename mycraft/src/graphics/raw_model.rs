use super::model;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexRaw {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub bitangent: [f32; 3],
}

impl VertexRaw {
    pub fn new(vertex: &model::Vertex) -> VertexRaw {
        Self {
            position: vertex.position.into(),
            tex_coords: vertex.tex_coords.into(),
            normal: vertex.normal.into(),
            tangent: vertex.tangent.into(),
            bitangent: vertex.bitangent.into(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[allow(dead_code)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
    normal: [[f32; 3]; 3],
}

impl InstanceRaw {
    pub fn new(instance: &model::Instance) -> InstanceRaw {
        Self {
            model: (cgmath::Matrix4::from_translation(instance.position)
                * cgmath::Matrix4::from(instance.rotation))
                .into(),
            normal: cgmath::Matrix3::from(instance.rotation).into(),
        }
    }
}

pub struct MeshRaw {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
    pub instance_buffer: Option<wgpu::Buffer>,
    pub num_instances: u32,
}

impl MeshRaw {
    pub fn new(device: &wgpu::Device, mesh: &model::Mesh) -> MeshRaw {
        let raw_vertices = mesh.vertices.iter().map(|v| VertexRaw::new(v)).collect::<Vec<_>>();
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Base Block Vertex Buffer"),
            contents: bytemuck::cast_slice(&raw_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Base Block Index Buffer"),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let instance_buffer = if mesh.instances.is_empty() {
            None
        } else {
            let instance_data = mesh.instances.iter().map(InstanceRaw::new).collect::<Vec<_>>();

            Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }))
        };

        Self {
            name: mesh.name.clone(),
            vertex_buffer,
            index_buffer,
            material: mesh.material,
            num_elements: mesh.indices.len() as u32,
            instance_buffer,
            num_instances: if mesh.instances.is_empty() { 1 } else { mesh.instances.len() as u32 },
        }
    }
}
