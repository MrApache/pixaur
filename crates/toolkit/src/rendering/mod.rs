pub mod bind_group;
pub mod bind_group_layout;
mod gpu;
mod instance;
pub mod material;
pub mod mesh;

pub use gpu::Gpu;

use crate::error::Error;
use glam::{Mat4, Vec2, Vec3};
use wgpu::*;

use crate::rendering::bind_group_layout::BindGroupLayoutBuilder;
use crate::rendering::instance::{InstanceData, InstancingPool};
use crate::rendering::material::Material;
use crate::rendering::mesh::QuadMesh;
use crate::{include_asset_content, load_asset_str, ContentManager, DrawCommand};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: Vec3,
    uv: Vec2,
}

impl Vertex {
    pub fn new(position: Vec3, uv: Vec2) -> Vertex {
        Self { position, uv }
    }

    pub fn get_layout() -> VertexBufferLayout<'static> {
        const ATTRIBUTES: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x2];

        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

pub struct Renderer {
    render_pipeline: RenderPipeline,
    mesh: QuadMesh,
    material: Material,
    buffer_pool: InstancingPool,
}

impl Renderer {
    pub fn new(gpu: &Gpu, shader: Option<&str>, surface: &Surface) -> Result<Self, Error> {
        let mut builder = BindGroupLayoutBuilder::new(&gpu.device);
        builder.add_material();
        let layout = builder.build("Default");

        let (shader, shader_label) = if let Some(shader) = shader {
            (
                load_asset_str(shader)?,
                shader,
            )
        } else {
            (
                include_asset_content!("shader.wgsl").to_string(),
                "Default",
            )
        };

        let shader = gpu.device.create_shader_module(ShaderModuleDescriptor {
            label: Some(shader_label),
            source: ShaderSource::Wgsl(shader.into()),
        });

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = gpu
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
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
            });

        Ok(Self {
            render_pipeline,
            mesh: QuadMesh::new(&gpu.device),
            material: Material::default(&gpu.device, &gpu.queue),
            buffer_pool: InstancingPool::new(gpu),
        })
    }

    pub fn render(
        &mut self,
        gpu: &Gpu,
        surface: &Surface,
        commands: &mut Vec<DrawCommand>,
        content: &ContentManager,
        window_width: f32,
        window_height: f32,
    ) -> Result<(), Error> {
        let texture = surface.get_current_texture()?;
        let image_view = texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let color_attachment = RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
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
            timestamp_writes: None,
        };

        let mut command_encoder = gpu
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let proj = Mat4::orthographic_rh_gl(0.0, window_width, 0.0, window_height, -1.0, 1.0);

        {
            self.buffer_pool.clear();

            let mut renderpass = command_encoder.begin_render_pass(&render_pass_descriptor);
            renderpass.set_pipeline(&self.render_pipeline);
            renderpass.set_bind_group(0, &self.material.bind_group, &[]);
            renderpass.set_vertex_buffer(0, self.mesh.vertex_buffer.slice(..));
            renderpass.set_index_buffer(self.mesh.index_buffer.slice(..), IndexFormat::Uint16);

            commands.iter().for_each(|command| {
                match command {
                    DrawCommand::Rect { rect, color } => self
                        .buffer_pool
                        .push(InstanceData::new(rect.min, rect.max, color, proj)),
                    DrawCommand::Texture { rect, texture } => {
                        {
                            renderpass.set_bind_group(0, &self.material.bind_group, &[]);
                            self.buffer_pool.draw_instances(gpu, &mut renderpass);
                        }

                        self.buffer_pool.push(InstanceData::new(
                            rect.min,
                            rect.max,
                            &texture.color,
                            proj,
                        ));

                        let material = content.get_texture(texture.handle);
                        renderpass.set_bind_group(0, &material.bind_group, &[]);

                        self.buffer_pool.draw_instances(gpu, &mut renderpass);
                        return;
                    }
                    DrawCommand::Text {
                        size,
                        font,
                        content,
                        color,
                    } => {}
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
