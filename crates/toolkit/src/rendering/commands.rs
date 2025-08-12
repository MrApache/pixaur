use std::slice::IterMut;
use enum_dispatch::enum_dispatch;
use fontdue::layout::Layout;
use glam::Vec2;
use wgpu::RenderPass;

use crate::{
    rendering::{
        instance::InstanceData,
        Gpu,
        Renderer
    },
    style::Texture,
    widget::Rect,
    Color,
    ContentManager,
    FontHandle
};

#[enum_dispatch(DrawCommand)]
pub(crate) trait DrawDispatcher {
    fn start(&mut self, pipeline: &mut Renderer, content: &ContentManager, renderpass: &mut RenderPass);
    fn prepare(&mut self, pipeline: &mut Renderer, renderpass: &mut RenderPass);
    fn finish(&self, pipeline: &mut Renderer, gpu: &Gpu, renderpass: &mut RenderPass);
}

pub struct DrawRectCommand {
    rect: Rect,
    color: Color,
}

impl DrawRectCommand {
    pub fn new(rect: Rect, color: impl Into<Color>) -> Self {
        Self {
            rect,
            color: color.into(),
        }
    }
}

impl DrawDispatcher for DrawRectCommand {
    fn start(&mut self, pipeline: &mut Renderer, _content: &ContentManager, renderpass: &mut RenderPass) {
        renderpass.set_bind_group(0, &pipeline.material.bind_group, &[]);
    }

    fn prepare(&mut self, pipeline: &mut Renderer, _renderpass: &mut RenderPass) {
        const UV0: Vec2 = Vec2::new(0.0, 0.0);
        const UV1: Vec2 = Vec2::new(1.0, 0.0);
        const UV2: Vec2 = Vec2::new(1.0, 1.0);
        const UV3: Vec2 = Vec2::new(0.0, 1.0);
        pipeline.buffer_pool.push(
            InstanceData::new_uv_2(
                UV0,
                UV1,
                UV2,
                UV3,
                self.rect.min,
                self.rect.max,
                &self.color,
                pipeline.projection
            )
        );
    }

    fn finish(&self, pipeline: &mut Renderer, gpu: &Gpu, renderpass: &mut RenderPass) {
        pipeline.buffer_pool.draw_instances(gpu, renderpass);
    }
}

pub struct DrawTextureCommand {
    rect: Rect,
    texture: Texture,
}

impl DrawTextureCommand {
    pub fn new(rect: Rect, texture: Texture) -> Self {
        Self {
            rect,
            texture,
        }
    }
}

impl DrawDispatcher for DrawTextureCommand {
    fn start(&mut self, _pipeline: &mut Renderer, content: &ContentManager, renderpass: &mut RenderPass) {
        let material = content.get_texture(self.texture.handle);
        renderpass.set_bind_group(0, &material.bind_group, &[]);
    }

    fn prepare(&mut self, pipeline: &mut Renderer, _renderpass: &mut RenderPass) {
        const UV0: Vec2 = Vec2::new(0.0, 0.0);
        const UV1: Vec2 = Vec2::new(1.0, 0.0);
        const UV2: Vec2 = Vec2::new(1.0, 1.0);
        const UV3: Vec2 = Vec2::new(0.0, 1.0);
        pipeline.buffer_pool.push(InstanceData::new_uv_2(
            UV0,
            UV1,
            UV2,
            UV3,
            self.rect.min,
            self.rect.max,
            &self.texture.color,
            pipeline.projection,
        ));

    }

    fn finish(&self, pipeline: &mut Renderer, gpu: &Gpu, renderpass: &mut RenderPass) {
        pipeline.buffer_pool.draw_instances(gpu, renderpass);
    }
}

pub struct DrawTextCommand<'frame> {
    size: u32,
    color: Color,
    position: Vec2,
    font: &'frame FontHandle,
    layout: &'frame Layout,
}

impl<'frame> DrawTextCommand<'frame> {
    pub fn new(size: u32, color: impl Into<Color>, position: Vec2, font: &'frame FontHandle, layout: &'frame Layout) -> Self {
        DrawTextCommand {
            size,
            color: color.into(),
            position,
            font,
            layout,
        }
    }
}

impl<'frame> DrawDispatcher for DrawTextCommand<'frame> {
    fn start(&mut self, _: &mut Renderer, _: &ContentManager, _: &mut RenderPass) {
    }

    fn prepare(&mut self, pipeline: &mut Renderer, _: &mut RenderPass) {
        let set = pipeline.fonts.entry(self.font.inner.name().unwrap().to_string()).or_default();
        let atlas = set.get_atlas(self.size);

        self.layout.glyphs().iter().for_each(|glyph| {
            match glyph.parent {
                ' '
                    | '\t'
                    | '\n'
                    | '\r'
                    | '\u{200B}'
                    | '\u{200C}'
                    | '\u{200D}'
                    | '\u{FEFF}' => return,
                c if c.is_control() => return,
                _ => {}
            }

            let data = atlas.get_or_add_glyph(glyph.parent, self.size, &self.font.inner);
            pipeline.buffer_pool.push(
                InstanceData::new_uv_4(
                    data.uv,
                    Vec2::new(
                        self.position.x + glyph.x,
                        self.position.y + glyph.y,
                    ),
                    Vec2::new(data.metrics.width as f32, data.metrics.height as f32),
                    &self.color,
                    pipeline.projection
                )
            );
        });
    }

    fn finish(&self, pipeline: &mut Renderer, gpu: &Gpu, renderpass: &mut RenderPass) {
        let set = pipeline.fonts.entry(self.font.inner.name().unwrap().to_string()).or_default();
        let atlas = set.get_atlas(self.size);

        let material = atlas.get_or_add_material(gpu);
        renderpass.set_bind_group(0, &material.bind_group, &[]);
        pipeline.buffer_pool.draw_instances(gpu, renderpass);
    }
}

#[enum_dispatch]
pub enum DrawCommand<'frame> {
    Rect(DrawRectCommand),
    Texture(DrawTextureCommand),
    Text(DrawTextCommand<'frame>),
}

impl DrawCommand<'_> {
    fn is_same_type(&self, other: &DrawCommand) -> bool {
        use DrawCommand::*;
        matches!(
            (self, other),

            (Rect(_), Rect(_))
            | (Texture(_), Texture(_))
            | (Text(_), Text(_)))
    }
}

#[derive(Default)]
pub struct PackedGroup<'frame> {
    inner: Vec<DrawCommand<'frame>>,
}

impl<'frame> PackedGroup<'frame> {
    pub fn prepare_frame(&mut self, pipeline: &mut Renderer, content: &ContentManager, gpu: &Gpu, renderpass: &mut RenderPass) {
        let len = self.inner.len();

        for (i, command) in self.inner.iter_mut().enumerate() {
            if len == 1 {
                command.start(pipeline, content, renderpass);
                command.prepare(pipeline, renderpass);
                command.finish(pipeline, gpu, renderpass);
            } else if i == 0 {
                command.start(pipeline, content, renderpass);
                command.prepare(pipeline, renderpass);
            } else if i == len - 1 {
                command.prepare(pipeline, renderpass);
                command.finish(pipeline, gpu, renderpass);
            } else {
                command.prepare(pipeline, renderpass);
            }
        }
    }
}

#[derive(Default)]
pub struct CommandBuffer<'frame> {
    packed_groups: Vec<PackedGroup<'frame>>,
    active_group: Vec<DrawCommand<'frame>>,
}

impl<'frame> CommandBuffer<'frame> {
    pub fn push(&mut self, command: impl Into<DrawCommand<'frame>>) {
        let command = command.into();
        let last = self.active_group.last();
        if let Some(last) = last {
            if !last.is_same_type(&command) {
                self.pack_active_group();
            }
        }
        self.active_group.push(command);
    }

    pub fn pack_active_group(&mut self) {
        let group = std::mem::take(&mut self.active_group);
        self.packed_groups.push(PackedGroup { inner: group });
    }

    pub fn iter_mut(&mut self) -> CommandBufferIter<'_, 'frame> {
        CommandBufferIter {
            iter: self.packed_groups.iter_mut()
        }
    }
}

pub struct CommandBufferIter<'a, 'frame> {
    iter: IterMut<'a, PackedGroup<'frame>>
}

impl<'a, 'frame> Iterator for CommandBufferIter<'a, 'frame> {
    type Item = &'a mut PackedGroup<'frame>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
