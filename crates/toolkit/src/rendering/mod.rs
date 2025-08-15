pub mod bind_group;
pub mod bind_group_layout;
pub mod commands;
pub mod material;
pub mod mesh;

mod gpu;
mod instance;
mod text;
mod vertex;

use bevy_ecs::resource::Resource;
use bevy_ecs::schedule::IntoScheduleConfigs;
use bevy_ecs::system::{Res, ResMut};
pub use gpu::Gpu;

use crate::commands::{collect_draw_rect, collect_draw_text, sort_commands, UnsortedCommandBuffer};
use crate::error::Error;
use crate::rendering::bind_group_layout::BindGroupLayoutBuilder;
use crate::rendering::instance::{InstanceData, InstancingPool};
use crate::rendering::material::Material;
use crate::rendering::mesh::QuadMesh;
use crate::rendering::text::FontAtlasSet;
use crate::rendering::vertex::Vertex;
use crate::widget::Plugin;
use crate::{
    include_asset_content, load_asset_str, CollectDrawCommands, ContentManager, Render, Windows,
};

use glam::Mat4;
use std::collections::HashMap;
use wgpu::*;

#[derive(Resource)]
pub struct Renderer {
    render_pipeline: RenderPipeline,
    mesh: QuadMesh,
    material: Material,
    buffer_pool: InstancingPool,
    fonts: HashMap<String, FontAtlasSet>,
}

impl Renderer {
    pub fn new(gpu: &Gpu, shader: Option<&str>, surface: &Surface) -> Result<Self, Error> {
        let mut builder = BindGroupLayoutBuilder::new(&gpu.device);
        builder.add_material();
        let layout = builder.build("Default");

        let (shader, shader_label) = if let Some(shader) = shader {
            (load_asset_str(shader)?, shader)
        } else {
            (include_asset_content!("shader.wgsl").to_string(), "Default")
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

        let caps = surface.get_capabilities(&gpu.adapter);
        let format = caps
            .formats
            .iter()
            .find(|&&f| matches!(f, wgpu::TextureFormat::Rgba8Unorm))
            .unwrap_or(&caps.formats[0]);

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
                        //format: surface.get_capabilities(&gpu.adapter).formats[0],
                        format: *format,
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
            fonts: HashMap::default(),
        })
    }
}

fn render(
    mut renderer: ResMut<Renderer>,
    mut windows: ResMut<Windows>,
    mut commands: ResMut<super::commands::CommandBuffer>,
    content: Res<ContentManager>,
    gpu: Res<Gpu>,
) {
    commands.iter_mut().for_each(|command_group| {
        command_group
            .inner
            .iter_mut()
            .for_each(|(window_id, group)| {
                let window = windows.active.get_mut(window_id).unwrap();
                let texture = window.surface.get_current_texture().unwrap();
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

                let mut command_encoder =
                    gpu.device
                        .create_command_encoder(&CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });

                {
                    renderer.buffer_pool.clear();

                    let mut renderpass = command_encoder.begin_render_pass(&render_pass_descriptor);
                    renderpass.set_pipeline(&renderer.render_pipeline);
                    renderpass.set_vertex_buffer(0, renderer.mesh.vertex_buffer.slice(..));
                    renderpass.set_index_buffer(
                        renderer.mesh.index_buffer.slice(..),
                        IndexFormat::Uint16,
                    );

                    let projection = Mat4::orthographic_rh_gl(
                        0.0,
                        window.configuration.width as f32,
                        window.configuration.height as f32,
                        0.0,
                        -1.0,
                        1.0,
                    );
                    group.prepare_frame(&mut renderer, &content, &gpu, &mut renderpass, projection);
                }

                gpu.queue.submit(std::iter::once(command_encoder.finish()));
                texture.present();

                window.backend.lock().unwrap().commit();
            });
    });
}

fn cleanup(
    mut unsorted: ResMut<UnsortedCommandBuffer>,
    mut sorted: ResMut<super::commands::CommandBuffer>,
) {
    unsorted.inner.clear();
    sorted.clear();
}

pub(crate) struct RendererPlugin;
impl Plugin for RendererPlugin {
    fn init(&self, app: &mut crate::App) {
        app.insert_resource(UnsortedCommandBuffer::default());
        app.insert_resource(super::commands::CommandBuffer::default());
        app.add_systems(CollectDrawCommands, (collect_draw_text, collect_draw_rect));
        app.add_systems(
            Render,
            (
                cleanup.after(render),
                render.after(sort_commands),
                sort_commands,
            ),
        );
    }
}
