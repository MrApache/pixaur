pub mod bind_group;
pub mod bind_group_layout;
mod gpu;
mod instance;
pub mod material;
pub mod mesh;

use std::collections::HashMap;

use fontdue::Font;
pub use gpu::Gpu;
use guillotiere::AtlasAllocator;

use crate::error::Error;
use glam::{Mat4, Vec2, Vec3, Vec4};
use wgpu::*;

use crate::rendering::bind_group_layout::BindGroupLayoutBuilder;
use crate::rendering::instance::{InstanceData, InstancingPool};
use crate::rendering::material::Material;
use crate::rendering::mesh::QuadMesh;
use crate::{ContentManager, DrawCommand, include_asset_content, load_asset_str};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3) -> Vertex {
        Self { position }
    }

    pub fn get_layout() -> VertexBufferLayout<'static> {
        const ATTRIBUTES: [VertexAttribute; 1] = vertex_attr_array![0 => Float32x3];

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
    fonts: HashMap<String, FontAtlasSet>
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
            fonts: HashMap::default(),
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
                    DrawCommand::Rect { rect, color } => {
                        let uv0 = Vec2::new(0.0, 0.0);
                        let uv1 = Vec2::new(1.0, 0.0);
                        let uv2 = Vec2::new(1.0, 1.0);
                        let uv3 = Vec2::new(0.0, 1.0);
                        self.buffer_pool.push(
                            InstanceData::new_uv_2(
                                uv0,
                                uv1,
                                uv2,
                                uv3,
                                rect.min,
                                rect.max,
                                color,
                                proj
                            )
                        );
                    }
                    DrawCommand::Texture { rect, texture } => {
                        {
                            renderpass.set_bind_group(0, &self.material.bind_group, &[]);
                            self.buffer_pool.draw_instances(gpu, &mut renderpass);
                        }

                        let uv0 = Vec2::new(0.0, 0.0);
                        let uv1 = Vec2::new(1.0, 0.0);
                        let uv2 = Vec2::new(1.0, 1.0);
                        let uv3 = Vec2::new(0.0, 1.0);
                        self.buffer_pool.push(InstanceData::new_uv_2(
                            uv0,
                            uv1,
                            uv2,
                            uv3,
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
                        font: font_name,
                        text,
                        color,
                    } => {
                        let font = content.get_font(font_name);
                        let set = self.fonts.entry(font_name.to_string()).or_default();
                        let atlas = set.inner.entry(*size as u32).or_insert(FontAtlas::new());

                        let mut position_x = 100.0;
                        let mut position_y = 100.0;
                        text.chars().for_each(|char| {
                            let data = atlas.get_or_add_glyph(char, *size as u32, font);
                            position_y = 100.0 + data.ymin as f32;
                            self.buffer_pool.push(InstanceData::new_uv_4(data.uv, Vec2::new(position_x + data.xmin as f32, position_y), Vec2::new(data.width, data.height), color, proj));
                            position_x += data.advance_width;
                        });

                        let material = atlas.get_or_add_material(gpu);
                        renderpass.set_bind_group(0, &material.bind_group, &[]);
                        self.buffer_pool.draw_instances(gpu, &mut renderpass);
                        return;
                    }
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

#[derive(Clone)]
struct GlyphData {
    uv: Vec4,
    width: f32,
    height: f32,
    advance_width: f32,
    advance_height: f32,
    xmin: i32,
    ymin: i32,
}

struct FontAtlas {
    inner: HashMap<char, GlyphData>,
    allocator: AtlasAllocator,
    texture: Vec<u8>,
    size: u32,
    material: Option<Material>
}

impl FontAtlas {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
            allocator: AtlasAllocator::new((512, 512).into()),
            texture: vec![0; 512 * 512 * 4],
            size: 512,
            material: None,
        }
    }

    fn get_or_add_glyph(
        &mut self,
        char: char,
        size: u32,
        font: &Font
    ) -> GlyphData {
        if let Some(glyph) = self.inner.get(&char) {
            return glyph.clone();
        }

        let (metrics, bitmap) = font.rasterize(char, size as f32);
        if metrics.width == 0 || metrics.height == 0 {
            let glyph = GlyphData {
                uv: Vec4::ZERO,
                width: 0.0,
                height: 0.0,
                advance_width: metrics.advance_width,
                advance_height: metrics.advance_height,
                xmin: metrics.xmin,
                ymin: metrics.ymin,
            };

            self.inner.insert(char, glyph.clone());

            return glyph;
        }

        let rectangle = self.allocator.allocate((metrics.width as i32, metrics.height as i32).into()).unwrap().rectangle;

        for y in 0..metrics.height {
            for x in 0..metrics.width {
                let alpha = bitmap[y * metrics.width + x];
                let dst_index = (((rectangle.min.x + x as i32) as u32)
                    + ((rectangle.min.y + y as i32) as u32) * self.size) as usize * 4;
        
                self.texture[dst_index] = 255;       // R
                self.texture[dst_index + 1] = 255;   // G
                self.texture[dst_index + 2] = 255;   // B
                self.texture[dst_index + 3] = alpha; // A
            }
        }


        // Вычисляем UV
        let u0 = rectangle.min.x as f32 / self.size as f32;
        let v0 = rectangle.min.y as f32 / self.size as f32;
        let u1 = rectangle.max.x as f32 / self.size as f32;
        let v1 = rectangle.max.y as f32 / self.size as f32;

        let data = GlyphData {
            uv: Vec4::new(u0, v0, u1, v1),
            xmin: metrics.xmin,
            ymin: metrics.ymin,
            width: metrics.width as f32,
            height: metrics.height as f32,
            advance_width: metrics.advance_width,
            advance_height: metrics.advance_height,
        };

        self.inner.insert(char, data.clone());

        data
    }

    fn get_or_add_material(&mut self, gpu: &Gpu) -> &Material {
        if self.material.is_none() {
            self.save_rgba_image();
            self.material = Some(Material::from_pixels("Glyph atlas", &self.texture, (self.size, self.size), TextureFormat::Rgba8Unorm, &gpu.device, &gpu.queue));
        }

        self.material.as_ref().unwrap()
    }

    fn save_rgba_image(&self) {
        use image::*;
    
        let mut img = RgbaImage::new(self.size, self.size);
    
        for y in 0..self.size {
            for x in 0..self.size {
                let idx = ((y * self.size + x) * 4) as usize;
                let r = self.texture[idx];
                let g = self.texture[idx + 1];
                let b = self.texture[idx + 2];
                let a = self.texture[idx + 3];
                img.put_pixel(x, y, Rgba([r, g, b, a]));
            }
        }
    
        img.save("/home/irisu/image.png").unwrap();
    }
}

#[derive(Default)]
struct FontAtlasSet {
    inner: HashMap<u32, FontAtlas>
}
