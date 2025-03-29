use super::block_renderer::create_block_mesh;
use super::camera;
use super::controller::CameraController;
use crate::graphics::{model, state};
use crate::world::chunk;
use cgmath::Rotation3;
use std::collections::HashMap;
use winit::event::{DeviceEvent, ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

pub struct Scene {
    camera: camera::Camera,
    camera_controller: CameraController,
    chunk_provider: chunk::ChunkProvider,
    last_render: (i32, i32),
}

impl Scene {
    pub fn new() -> Self {
        let camera = camera::Camera::new((0.0, 5.0, 0.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
        let camera_controller = CameraController::new(4.0, 0.4);

        Self {
            camera,
            camera_controller,
            chunk_provider: chunk::ChunkProvider::default(),
            last_render: (0, 0),
        }
    }

    pub async fn run(self: &mut Self) {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        // State::new uses async code, so we're going to wait for it to finish
        let meshes = self.render_chunks((0, 0));
        let mut state = state::State::new(&window, &self.camera, &meshes).await;
        let mut last_render_time = instant::Instant::now();

        event_loop
            .run(|event, control_flow| {
                match event {
                    Event::DeviceEvent {
                        event: DeviceEvent::MouseMotion { delta },
                        ..
                    } => self.camera_controller.process_mouse(delta.0, delta.1),
                    Event::WindowEvent {
                        ref event,
                        window_id,
                    } if window_id == window.id() && !self.input(event) => {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                event:
                                    KeyEvent {
                                        state: ElementState::Pressed,
                                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => control_flow.exit(),
                            WindowEvent::Resized(physical_size) => {
                                state.resize(state::Size {
                                    width: physical_size.width,
                                    height: physical_size.height,
                                });
                            }
                            WindowEvent::RedrawRequested => {
                                // This tells winit that we want another frame after this one
                                window.request_redraw();

                                let now = instant::Instant::now();
                                let dt = now - last_render_time;
                                last_render_time = now;
                                self.camera_controller.update_camera(&mut self.camera, dt);
                                let chunk_x = (self.camera.position.x / 16.0) as i32;
                                let chunk_y = (self.camera.position.z / 16.0) as i32;
                                if self.last_render != (chunk_x, chunk_y) {
                                    self.last_render = (chunk_x, chunk_y);
                                    state.set_meshes(&self.render_chunks(self.last_render));
                                }
                                state.update(&self.camera);
                                match state.render() {
                                    Ok(_) => {}
                                    // Reconfigure the surface if it's lost or outdated
                                    Err(
                                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                                    ) => state.resize(state.size),
                                    // The system is out of memory, we should probably quit
                                    Err(
                                        wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other,
                                    ) => {
                                        log::error!("OutOfMemory");
                                        control_flow.exit();
                                    }

                                    // This happens when the frame takes too long to present
                                    Err(wgpu::SurfaceError::Timeout) => {
                                        log::warn!("Surface timeout")
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            })
            .unwrap();
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state,
                        ..
                    },
                ..
            } => self.camera_controller.process_keyboard(*key, *state),
            _ => false,
        }
    }

    fn render_chunks(self: &mut Self, position: (i32, i32)) -> Vec<model::Mesh> {
        let mut block_variations: HashMap<(i32, u8), ([bool; 6], Vec<model::Instance>)> =
            HashMap::new();

        for x_chunk in -5..=5 {
            for y_chunk in -5..=5 {
                let chunk = self
                    .chunk_provider
                    .get_chunk(position.0 + x_chunk, position.1 + y_chunk);
                for block_entry in chunk.chunk_map.clone() {
                    let ((x, y, z), block) = block_entry;
                    let render_faces = [
                        chunk.is_opaque(x, y + 1, z),
                        chunk.is_opaque(x, y - 1, z),
                        chunk.is_opaque(x, y, z + 1),
                        chunk.is_opaque(x, y, z - 1),
                        chunk.is_opaque(x + 1, y, z),
                        chunk.is_opaque(x - 1, y, z),
                    ];
                    if render_faces.contains(&true) {
                        let render_time = instant::Instant::now();
                        let block_index = bool_array_to_int(render_faces);
                        println!("Block index calculated in {:?}", render_time.elapsed());
                        let material_id = block.get_material_id();
                        let (_, instances) = block_variations
                            .entry((material_id, block_index))
                            .or_insert_with(|| (render_faces, Vec::new()));
                        // Create instance data for visible block
                        let position = cgmath::Vector3 {
                            x: (16 * chunk.x + x) as f32,
                            y: z as f32,
                            z: (16 * chunk.y + y) as f32,
                        };

                        let rotation = cgmath::Quaternion::from_axis_angle(
                            cgmath::Vector3::unit_z(),
                            cgmath::Deg(0.0),
                        );

                        instances.push(model::Instance { position, rotation });
                    }
                }
            }
        }

        let mut meshes: Vec<model::Mesh> = block_variations
            .into_iter()
            .map(|((material_id, _), (faces, instances))| {
                create_block_mesh(material_id as usize, faces, instances)
            })
            .collect();

        meshes
    }
}

fn bool_array_to_int(array: [bool; 6]) -> u8 {
    let mut value: u8 = 0;
    for i in 0..6 {
        if array[i] {
            value |= 1 << i;
        }
    }

    value
}
