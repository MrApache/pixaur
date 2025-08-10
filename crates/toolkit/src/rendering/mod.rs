pub mod mesh;
pub mod bind_group;
pub mod bind_group_layout;
pub mod material;
mod instance;
mod gpu;

use std::collections::HashMap;

pub use gpu::Gpu;

use wgpu::*;
use glam::{Mat4, Vec2, Vec3};
use crate::error::Error;

use crate::rendering::bind_group_layout::BindGroupLayoutBuilder;
use crate::rendering::instance::InstanceData;
use crate::rendering::material::Material;
use crate::rendering::mesh::QuadMesh;
use crate::style::{BackgroundStyle, Texture};
use crate::widget::Rect;
use crate::{ContentManager, DrawCommand};

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

pub struct Renderer {
    render_pipeline: RenderPipeline,
    mesh: QuadMesh,
    material: Material,
    buffer_pool: BufferPool,
}

impl Renderer {
    pub fn new(gpu: &Gpu, shader: Option<&str>, surface: &Surface) -> Result<Self, Error> {
        let mut builder = BindGroupLayoutBuilder::new(&gpu.device);
        builder.add_material();
        let layout = builder.build("Default");

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
                bind_group_layouts: &[&layout],
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
                    buffers: &[Vertex::get_layout(), InstanceData::get_layout()],
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

        Ok(Self {
            render_pipeline,
            mesh: QuadMesh::new(&gpu.device),
            material: Material::default(&gpu.device, &gpu.queue),
            buffer_pool: BufferPool::new(gpu)
        })
    }

    pub fn render(
        &mut self,
        gpu: &Gpu,
        surface: &Surface,
        commands: &mut Vec<DrawCommand>,
        content: &ContentManager,
        window_width: f32,
        window_height: f32
        )-> Result<(), Error> {
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
            self.buffer_pool.clear();

            let mut renderpass = command_encoder.begin_render_pass(&render_pass_descriptor);
            renderpass.set_pipeline(&self.render_pipeline);
            renderpass.set_bind_group(0, &self.material.bind_group, &[]);
            renderpass.set_vertex_buffer(0, self.mesh.vertex_buffer.slice(..));
            renderpass.set_index_buffer(self.mesh.index_buffer.slice(..), IndexFormat::Uint16);
            self.buffer_pool.take(gpu);

            commands.iter().for_each(|command| {
                match command {
                    DrawCommand::Rect { rect, color } => self.buffer_pool.push(InstanceData::new(rect.min, rect.max, color, proj)),
                    DrawCommand::Texture { rect, texture } => {
                        {
                            renderpass.set_bind_group(0, &self.material.bind_group, &[]);
                            self.buffer_pool.draw_instances(gpu, &mut renderpass);
                        }

                        self.buffer_pool.push(InstanceData::new(rect.min, rect.max, &texture.color, proj));

                        let material = content.get_texture(texture.handle);
                        renderpass.set_bind_group(0, &material.bind_group, &[]);

                        self.buffer_pool.draw_instances(gpu, &mut renderpass);
                        return;
                    }
                    DrawCommand::Text { size, font, content, color } => {},
                }

                renderpass.set_bind_group(0, &self.material.bind_group, &[]);
                self.buffer_pool.draw_instances(gpu, &mut renderpass);
            });

        }

        gpu.queue.submit(std::iter::once(command_encoder.finish()));
        texture.present();

        Ok(())
    }
}

struct InstanceBuffer {
    instances: Vec<InstanceData>,
    instance_buffer: Buffer,
    instance_buffer_len: usize,
}

impl InstanceBuffer {
    pub fn new(gpu: &Gpu, instance_buffer_size: usize) -> Self {
        let instance_buffer = gpu.device.create_buffer(&BufferDescriptor {
            label: Some("Instance buffer"),
            size: (instance_buffer_size * std::mem::size_of::<InstanceData>()) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            instances: Vec::with_capacity(instance_buffer_size),
            instance_buffer,
            instance_buffer_len: instance_buffer_size,
        }
    }

    fn create_instance_buffer(&mut self, gpu: &Gpu, size: usize) {
        let instance_buffer = gpu.device.create_buffer(&BufferDescriptor {
            label: Some("Instance buffer"),
            size: (size * std::mem::size_of::<InstanceData>()) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        self.instance_buffer = instance_buffer;
        self.instance_buffer_len = size;
    }

    fn resize_buffer_if_needed(&mut self, gpu: &Gpu, renderpass: &mut RenderPass) {
        if self.instances.capacity() > self.instance_buffer_len {
            self.create_instance_buffer(gpu, self.instances.capacity());
            renderpass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        }
    }

    fn write_instance_buffer(&self, gpu: &Gpu) {
        gpu.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
    }

    fn draw_instances(&mut self, gpu: &Gpu, renderpass: &mut RenderPass) {
        self.resize_buffer_if_needed(gpu, renderpass);
        self.write_instance_buffer(gpu);
        renderpass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        renderpass.draw_indexed(0..6, 0, 0..self.instances.len() as u32);
    }

    fn clear(&mut self) {
        self.instances.clear();
    }
}

pub struct BufferPool {
    available: Vec<InstanceBuffer>,
    in_use: Vec<InstanceBuffer>,
    current: Option<InstanceBuffer>,
}

const INSTANCE_BUFFER_SIZE: usize = 2;
impl BufferPool {
    fn new(gpu: &Gpu) -> Self {
        let buffer = InstanceBuffer::new(gpu, INSTANCE_BUFFER_SIZE);
        Self {
            available: vec![buffer],
            in_use: vec![],
            current: None,
        }
    }

    fn take(&mut self, gpu: &Gpu) {
        if self.available.is_empty() {
            self.current = Some(InstanceBuffer::new(gpu, INSTANCE_BUFFER_SIZE));
        } else {
            self.current = self.available.pop();
        }
    }

    fn complete(&mut self) {
        let buffer = self.current.take().unwrap();
        self.in_use.push(buffer);
    }

    fn clear(&mut self) {
        self.in_use.iter_mut().for_each(|buffer| {
            buffer.clear();
        });

        self.available.append(&mut self.in_use);
    }
    
    fn push(&mut self, data: InstanceData) {
        let buffer = self.current.as_mut().unwrap();
        buffer.instances.push(data);
    }

    fn draw_instances(&mut self, gpu: &Gpu, renderpass: &mut RenderPass) {
        let buffer = self.current.as_mut().unwrap();
        buffer.draw_instances(gpu, renderpass);
        self.complete();
        self.take(gpu);
    }
}
