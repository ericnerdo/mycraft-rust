use super::drawable::DrawModel;
use super::light::LightUniform;
use super::perspective::{CameraUniform, Projection};
use super::render_pipeline::RenderPipelineBuilder;
use super::{bind_group, buffer, drawable, material, model, raw_model, state};
use crate::game::camera::Camera;
use cgmath;
use instant;

#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

pub struct State<'a> {
    pub window: &'a winit::window::Window,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: Size,
    render_pipeline: wgpu::RenderPipeline,
    light_render_pipeline: wgpu::RenderPipeline,
    clear_color: wgpu::Color,
    projection: Projection,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_uniform: CameraUniform,
    light_buffer: wgpu::Buffer,
    light_uniform: LightUniform,
    light_bind_group_layout: wgpu::BindGroupLayout,
    light_bind_group: wgpu::BindGroup,
    depth_texture: material::Texture,
    drawable_state: drawable::DrawableState,
}

impl<'a> State<'a> {
    pub async fn new(
        window: &'a winit::window::Window,
        camera: &Camera,
        initial_meshes: &Vec<model::Mesh>,
    ) -> Self {
        let physical_size = window.inner_size();
        let size = state::Size {
            width: physical_size.width,
            height: physical_size.height,
        };
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: Some("Device"),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let material_bind_group_layout =
            bind_group::create_material_bind_group_layout(&device, "Material Bind Group Layout");

        let projection =
            Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0);
        let camera_bind_group_layout =
            bind_group::create_camera_bind_group_layout(&device, "Camera Bind Group Layout");
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection);
        let camera_buffer = buffer::create_camera_buffer(camera_uniform, &device);
        let camera_bind_group = bind_group::create_buffer_bind_group(
            &device,
            &camera_bind_group_layout,
            &camera_buffer,
            "Camera Bind Group",
        );

        let light_bind_group_layout =
            bind_group::create_light_bind_group_layout(&device, "Light Bind Group Layout");
        let light_uniform = LightUniform {
            position: [2.0, 2.0, 2.0],
            _padding: 0,
            color: [1.0, 1.0, 1.0],
            _padding2: 0,
        };
        let light_buffer = buffer::create_light_buffer(light_uniform, &device);
        let light_bind_group = bind_group::create_buffer_bind_group(
            &device,
            &light_bind_group_layout,
            &light_buffer,
            "Light Bind Group",
        );

        let render_pipeline: wgpu::RenderPipeline;
        {
            let mut pipeline_builder = RenderPipelineBuilder::new(&device);
            pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
            pipeline_builder.set_pixel_format(config.format);
            pipeline_builder.add_vertex_buffer_layout(buffer::create_vertex_buffer_layout());
            pipeline_builder.add_vertex_buffer_layout(buffer::create_instance_buffer_layout());
            pipeline_builder.add_bind_group_layout(&material_bind_group_layout);
            pipeline_builder.add_bind_group_layout(&camera_bind_group_layout);
            pipeline_builder.add_bind_group_layout(&light_bind_group_layout);
            render_pipeline = pipeline_builder.build("Render Pipeline");
        }

        let light_render_pipeline: wgpu::RenderPipeline;
        {
            let mut pipeline_builder = RenderPipelineBuilder::new(&device);
            pipeline_builder.set_shader_module("shaders/light.wgsl", "vs_main", "fs_main");
            pipeline_builder.set_pixel_format(config.format);
            pipeline_builder.add_vertex_buffer_layout(buffer::create_vertex_buffer_layout());
            pipeline_builder.add_bind_group_layout(&camera_bind_group_layout);
            pipeline_builder.add_bind_group_layout(&light_bind_group_layout);
            light_render_pipeline = pipeline_builder.build("Light Render Pipeline");
        }

        let depth_texture =
            material::Texture::create_depth_texture(&device, &config, "depth_texture");

        let mut materials: Vec<material::Material> = Vec::new();
        for i in 0..8 {
            let texture_file = format!("textures/{}.png", i);
            let diffuse_texture =
                material::Texture::load_texture(texture_file.as_str(), &device, &queue, false)
                    .await
                    .unwrap();
            materials.push(material::Material::new(
                &device,
                texture_file.as_str(),
                diffuse_texture,
                // normal_texture,
                &material_bind_group_layout,
            ));
        }

        let render_time = instant::Instant::now();

        let raw_meshes: Vec<raw_model::MeshRaw> = initial_meshes
            .iter()
            .map(|mesh| raw_model::MeshRaw::new(&device, mesh))
            .collect();

        println!("Raw meshes rendered in: {:?}", render_time.elapsed());

        let drawable_state = drawable::DrawableState {
            meshes: raw_meshes,
            materials,
        };

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            light_render_pipeline,
            clear_color: wgpu::Color {
                r: 0.69,
                g: 0.88,
                b: 0.9,
                a: 1.0,
            },
            projection,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            light_uniform,
            light_buffer,
            light_bind_group_layout,
            light_bind_group,
            depth_texture,
            drawable_state,
        }
    }

    pub fn resize(&mut self, new_size: Size) {
        self.projection.resize(new_size.width, new_size.height);
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = self.size.width;
            self.config.height = self.size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture = material::Texture::create_depth_texture(
                &self.device,
                &self.config,
                "depth_texture",
            );
        }
    }

    pub fn set_meshes(&mut self, meshes: &Vec<model::Mesh>) {
        let render_time = instant::Instant::now();
        self.drawable_state.meshes = meshes
            .iter()
            .map(|mesh| raw_model::MeshRaw::new(&self.device, mesh))
            .collect();
        println!("Raw meshes rendered in: {:?}", render_time.elapsed());
    }

    pub fn update(&mut self, camera: &Camera) {
        self.camera_uniform
            .update_view_proj(&camera, &self.projection);
        // Update the light
        // let old_position: cgmath::Vector3<_> = self.light_uniform.position.into();
        // self.light_uniform.position = (cgmath::Quaternion::from_axis_angle(
        //     (0.0, 1.0, 0.0).into(),
        //     cgmath::Deg(60.0 * dt.as_secs_f32()),
        // ) * old_position)
        //     .into();
        // self.light_uniform.position = self.camera.position.clone().into();

        self.queue.write_buffer(
            &self.light_buffer,
            0,
            bytemuck::cast_slice(&[self.light_uniform]),
        );
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let drawable = self.surface.get_current_texture()?;
        let image_view = drawable
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut command_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &image_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        };
        {
            let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
            // render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            // render_pass.set_pipeline(&self.light_render_pipeline);
            // render_pass.draw_light_model(
            //     &self.obj_model,
            //     &self.camera_bind_group,
            //     &self.light_bind_group,
            // );

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw_model_instanced(
                &self.drawable_state,
                &self.camera_bind_group,
                &self.light_bind_group,
            );
        }

        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();

        Ok(())
    }

    pub fn set_clear_color(&mut self, new_color: wgpu::Color) {
        self.clear_color = new_color;
    }
}
