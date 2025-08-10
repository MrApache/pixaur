pub mod mesh;
pub mod bind_group;
pub mod bind_group_layout;
pub mod material;

use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::*;
use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use crate::error::Error;

use crate::rendering::bind_group_layout::BindGroupLayoutBuilder;
use crate::rendering::material::Material;
use crate::rendering::mesh::QuadMesh;
use crate::window::WindowPointer;
use crate::{Argb8888, DrawCommand};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: Vec3,
    uv: Vec2
}

impl Vertex {
    pub fn new(position: Vec3, uv: Vec2) -> Vertex {
        Self {
            position,
            uv,
        }
    }

    pub fn get_layout() -> VertexBufferLayout<'static> {
        const ATTRIBUTES: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x2];

        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &ATTRIBUTES
        }
    }
}

pub struct Gpu {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

impl Gpu {
    pub fn new(dummy: WindowPointer) -> Result<Self, Error> {
        let instance = Instance::new(&InstanceDescriptor::default());
        let surface = instance.create_surface(dummy)?;
        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))?;
        let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor::default()))?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }

    pub fn new_renderer(
        &self,
        shader: Option<&str>,
        surface: &Surface
    ) -> Result<Renderer, Error> {

        let mut builder = BindGroupLayoutBuilder::new(&self.device);
        builder.add_material();
        let layout = builder.build("Default");

        Renderer::new(self, shader, &[&layout], surface)
    }

    pub fn create_surface<'window>(&self, ptr: WindowPointer, width: u32, height: u32) -> Result<(Surface<'window>, SurfaceConfiguration), Error> {
        let surface = self.instance.create_surface(ptr)?;
        let surface_caps = surface.get_capabilities(&self.adapter);
        let format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

    
        self.confugure_surface(&surface, &config);

        Ok((surface, config))
    }

    pub fn confugure_surface(&self, surface: &Surface<'_>, configuration: &SurfaceConfiguration) {
        surface.configure(&self.device, configuration);
    }
}

pub struct Renderer {
    render_pipeline: RenderPipeline,
    mesh: QuadMesh,
    material: Material,

    instances: Vec<InstanceRawData>,
    instance_buffer: Buffer,
    instance_buffer_len: usize,
}

impl Renderer {
    fn new(gpu: &Gpu, shader: Option<&str>, layouts: &[&BindGroupLayout], surface: &Surface) -> Result<Self, Error> {
        let (shader, shader_label) = if let Some(shader) = shader {
            (std::fs::read_to_string(format!("../../../../assets/{shader}.wgsl"))?, shader)
        }
        else {
            (include_str!("../../../../assets/shader.wgsl").to_string(), "Default")
        };

        let shader = gpu.device.create_shader_module(
            ShaderModuleDescriptor {
                label: Some(shader_label),
                source: ShaderSource::Wgsl(shader.into())
            }
        );

        let pipeline_layout = gpu.device.create_pipeline_layout(
            &PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts: layouts,
                push_constant_ranges: &[],
            }
        );

        let render_pipeline = gpu.device.create_render_pipeline(
            &RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[Vertex::get_layout(), InstanceRawData::get_layout()],
                },
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(ColorTargetState {
                        format: surface.get_capabilities(&gpu.adapter).formats[0],
                        blend: Some(BlendState::ALPHA_BLENDING),
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: Some(Face::Back),
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: MultisampleState::default(),
                multiview: None,
                cache: None,
            }
        );

        const INSTANCE_BUFFER_SIZE: usize = 256;

        let instance_buffer = gpu.device.create_buffer(&BufferDescriptor {
            label: Some("Instance buffer"),
            size: (INSTANCE_BUFFER_SIZE * std::mem::size_of::<InstanceRawData>()) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Ok(Self {
            render_pipeline,
            mesh: QuadMesh::new(&gpu.device),
            material: Material::default(&gpu.device, &gpu.queue),
            instances: Vec::with_capacity(INSTANCE_BUFFER_SIZE),
            instance_buffer,
            instance_buffer_len: INSTANCE_BUFFER_SIZE,
        })
    }

    fn create_instance_buffer(&mut self, gpu: &Gpu, size: usize) {
        let instance_buffer = gpu.device.create_buffer(&BufferDescriptor {
            label: Some("Instance buffer"),
            size: (size * std::mem::size_of::<InstanceRawData>()) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        self.instance_buffer = instance_buffer;
        self.instance_buffer_len = size;
    }

    pub fn render(&mut self, gpu: &Gpu, surface: &Surface, commands: &[DrawCommand], window_width: f32, window_height: f32)-> Result<(), Error> {
        let texture = surface.get_current_texture()?;
        let image_view = texture.texture.create_view(&TextureViewDescriptor::default());

        let color_attachment = RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0
                }),
                store: StoreOp::Store,
            },
            depth_slice: None,
        };

        let render_pass_descriptor = RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None
        };

        let mut command_encoder = gpu.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        let proj = Mat4::orthographic_rh_gl(0.0, window_width, 0.0, window_height, -1.0, 1.0);

        {
            self.instances.clear();

            let mut renderpass = command_encoder.begin_render_pass(&render_pass_descriptor);
            renderpass.set_pipeline(&self.render_pipeline);
            renderpass.set_bind_group(0, &self.material.bind_group, &[]);
            renderpass.set_vertex_buffer(0, self.mesh.vertex_buffer.slice(..));
            renderpass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            renderpass.set_index_buffer(self.mesh.index_buffer.slice(..), IndexFormat::Uint16);

            commands.iter().for_each(|command| {
                match command {
                    DrawCommand::Text { size, font, content, color } => {},
                    DrawCommand::Rect { rect, color } => self.instances.push(InstanceRawData::new(rect.min, rect.max, color, proj)),
                }
            });

            if self.instances.capacity() > self.instance_buffer_len {
                self.create_instance_buffer(gpu, self.instances.capacity());
                renderpass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            }

            gpu.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
            renderpass.draw_indexed(0..6, 0, 0..self.instances.len() as u32);
        }

        gpu.queue.submit(std::iter::once(command_encoder.finish()));
        texture.present();

        Ok(())
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRawData {
    model: Mat4,
    color_start: Vec4,
    color_end: Vec4,
    use_gradient: u32,
    _padding: [u32; 3]
}

impl InstanceRawData {
    pub fn new(position: Vec2, size: Vec2, color: &crate::Color, proj: Mat4) -> Self {
        let model = proj * Mat4::from_scale_rotation_translation(Vec3::new(size.x, size.y, 0.0), Quat::IDENTITY, Vec3::new(position.x, position.y, 0.0));
        let (color_start, color_end, use_gradient) : (Vec4, Vec4, u32) = match color {
            crate::Color::Simple(argb8888) => (argb8888.into(), Argb8888::TRANSPARENT.into(), 0),
            crate::Color::LinearGradient(linear_gradient) => ((&linear_gradient.from).into(), (&linear_gradient.to).into(), 1),
        };

        Self {
            model,
            color_start,
            color_end,
            use_gradient,
            _padding: [0, 0, 0],
        }
    }

    fn get_layout() -> VertexBufferLayout<'static> {
        use std::mem;
        VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRawData>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: 6,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as BufferAddress,
                    shader_location: 7,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as BufferAddress,
                    shader_location: 8,
                    format: VertexFormat::Float32x4,
                },

                VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as BufferAddress,
                    shader_location: 9,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 20]>() as BufferAddress,
                    shader_location: 10,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: 96,
                    shader_location: 11,
                    format: VertexFormat::Uint32,
                },
            ],
        }
    }
}
