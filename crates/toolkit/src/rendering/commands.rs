use bevy_ecs::prelude::*;
use enum_dispatch::enum_dispatch;
use fontdue::layout::Layout;
use glam::{Mat4, Vec2};
use std::{collections::HashMap, slice::IterMut};
use wgpu::RenderPass;

use crate::{
    ecs::{Text, ZOrder},
    rendering::{instance::InstanceData, Gpu, Renderer},
    types::{Color, Stroke},
    ContentManager, FontHandle, TextureHandle, Transform, WindowId, Windows,
};

#[enum_dispatch(DrawCommand)]
pub(crate) trait DrawDispatcher {
    fn start(
        &mut self,
        pipeline: &mut Renderer,
        content: &ContentManager,
        renderpass: &mut RenderPass,
    );
    fn prepare(&mut self, pipeline: &mut Renderer, renderpass: &mut RenderPass, projection: Mat4);
    fn finish(&self, pipeline: &mut Renderer, gpu: &Gpu, renderpass: &mut RenderPass);
}

pub struct DrawRectCommand {
    pub transform: Transform,
    pub color: Color,
    pub stroke: Option<Stroke>,
}

impl DrawDispatcher for DrawRectCommand {
    fn start(&mut self, pipeline: &mut Renderer, _: &ContentManager, renderpass: &mut RenderPass) {
        renderpass.set_bind_group(0, &pipeline.material.bind_group, &[]);
    }

    fn prepare(&mut self, pipeline: &mut Renderer, _: &mut RenderPass, projection: Mat4) {
        const UV0: Vec2 = Vec2::new(0.0, 0.0);
        const UV1: Vec2 = Vec2::new(1.0, 0.0);
        const UV2: Vec2 = Vec2::new(1.0, 1.0);
        const UV3: Vec2 = Vec2::new(0.0, 1.0);
        pipeline.buffer_pool.push(InstanceData::new_uv_2(
            UV0,
            UV1,
            UV2,
            UV3,
            self.transform.position,
            self.transform.size,
            &self.color,
            self.stroke.clone(),
            projection,
        ));
    }

    fn finish(&self, pipeline: &mut Renderer, gpu: &Gpu, renderpass: &mut RenderPass) {
        pipeline.buffer_pool.draw_instances(gpu, renderpass);
    }
}

pub struct DrawTextureCommand {
    pub transform: Transform,
    pub color: Color,
    pub texture: TextureHandle,
    pub stroke: Option<Stroke>,
}

impl DrawDispatcher for DrawTextureCommand {
    fn start(
        &mut self,
        _pipeline: &mut Renderer,
        content: &ContentManager,
        renderpass: &mut RenderPass,
    ) {
        let material = content.get_texture(self.texture);
        renderpass.set_bind_group(0, &material.bind_group, &[]);
    }

    fn prepare(&mut self, pipeline: &mut Renderer, _: &mut RenderPass, projection: Mat4) {
        const UV0: Vec2 = Vec2::new(0.0, 0.0);
        const UV1: Vec2 = Vec2::new(1.0, 0.0);
        const UV2: Vec2 = Vec2::new(1.0, 1.0);
        const UV3: Vec2 = Vec2::new(0.0, 1.0);
        pipeline.buffer_pool.push(InstanceData::new_uv_2(
            UV0,
            UV1,
            UV2,
            UV3,
            self.transform.position,
            self.transform.size,
            &self.color,
            self.stroke.clone(),
            projection,
        ));
    }

    fn finish(&self, pipeline: &mut Renderer, gpu: &Gpu, renderpass: &mut RenderPass) {
        pipeline.buffer_pool.draw_instances(gpu, renderpass);
    }
}

pub struct DrawTextCommand {
    pub(crate) size: u32,
    pub(crate) position: Vec2,
    pub(crate) color: Color,
    pub(crate) font: FontHandle,
    pub(crate) layout: Layout,
}

impl DrawDispatcher for DrawTextCommand {
    fn start(&mut self, _: &mut Renderer, _: &ContentManager, _: &mut RenderPass) {}

    fn prepare(&mut self, pipeline: &mut Renderer, _: &mut RenderPass, projection: Mat4) {
        let set = pipeline
            .fonts
            .entry(self.font.inner.name().unwrap().to_string())
            .or_default();
        let atlas = set.get_atlas(self.size);

        self.layout.glyphs().iter().for_each(|glyph| {
            match glyph.parent {
                ' ' | '\t' | '\n' | '\r' | '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{FEFF}' => {
                    return;
                }
                c if c.is_control() => return,
                _ => {}
            }

            let data = atlas.get_or_add_glyph(glyph.parent, self.size, &self.font.inner);
            pipeline.buffer_pool.push(InstanceData::new_uv_4(
                data.uv,
                Vec2::new(self.position.x + glyph.x, self.position.y + glyph.y),
                Vec2::new(data.metrics.width as f32, data.metrics.height as f32),
                &self.color,
                None,
                projection,
            ));
        });
    }

    fn finish(&self, pipeline: &mut Renderer, gpu: &Gpu, renderpass: &mut RenderPass) {
        let set = pipeline
            .fonts
            .entry(self.font.inner.name().unwrap().to_string())
            .or_default();
        let atlas = set.get_atlas(self.size);

        let material = atlas.get_or_add_material(gpu);
        renderpass.set_bind_group(0, &material.bind_group, &[]);
        pipeline.buffer_pool.draw_instances(gpu, renderpass);
    }
}

#[enum_dispatch]
pub enum DrawCommand {
    Rect(DrawRectCommand),
    Texture(DrawTextureCommand),
    Text(DrawTextCommand),
}

impl DrawCommand {
    fn is_same_type(&self, other: &DrawCommand) -> bool {
        use DrawCommand::*;
        matches!(
            (self, other),
            (Rect(_), Rect(_)) | (Texture(_), Texture(_)) | (Text(_), Text(_))
        )
    }
}

#[derive(Default)]
pub struct PackedGroup {
    inner: Vec<DrawCommand>,
}

impl PackedGroup {
    pub fn prepare_frame(
        &mut self,
        pipeline: &mut Renderer,
        content: &ContentManager,
        gpu: &Gpu,
        renderpass: &mut RenderPass,
        projection: Mat4,
    ) {
        let len = self.inner.len();
        self.inner.iter_mut().enumerate().for_each(|(i, command)| {
            if len == 1 {
                command.start(pipeline, content, renderpass);
                command.prepare(pipeline, renderpass, projection);
                command.finish(pipeline, gpu, renderpass);
            } else if i == 0 {
                command.start(pipeline, content, renderpass);
                command.prepare(pipeline, renderpass, projection);
            } else if i == len - 1 {
                command.prepare(pipeline, renderpass, projection);
                command.finish(pipeline, gpu, renderpass);
            } else {
                command.prepare(pipeline, renderpass, projection);
            }
        })
    }
}

#[derive(Default, Resource)]
pub struct CommandBuffer {
    packed_groups: HashMap<WindowId, Vec<PackedGroup>>,
    active_group: Vec<DrawCommand>,
}

impl CommandBuffer {
    pub fn push(&mut self, window_id: &WindowId, command: impl Into<DrawCommand>) {
        let command = command.into();
        let last = self.active_group.last();
        if let Some(last) = last {
            if !last.is_same_type(&command) {
                self.pack_active_group(window_id);
            }
        }
        self.active_group.push(command);
    }

    pub fn pack_active_group(&mut self, window_id: &WindowId) {
        let group = std::mem::take(&mut self.active_group);
        self.get_packed_group(window_id).push(PackedGroup {
            inner: group,
        });
    }

    pub fn iter_mut(&mut self, window_id: &WindowId) -> CommandBufferIter<'_> {
        CommandBufferIter {
            iter: self.packed_groups.get_mut(window_id).unwrap().iter_mut()
        }
    }

    pub fn clear(&mut self) {
        self.packed_groups.clear();
        self.active_group.clear();
    }

    fn get_packed_group(&mut self, id: &WindowId) -> &mut Vec<PackedGroup> {
        if !self.packed_groups.contains_key(id) {
            self.packed_groups.insert(id.clone(), vec![]);
        }
        self.packed_groups.get_mut(id).unwrap()
    }
}

pub struct CommandBufferIter<'a> {
    iter: IterMut<'a, PackedGroup>,
}

impl<'a> Iterator for CommandBufferIter<'a> {
    type Item = &'a mut PackedGroup;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[derive(Default, Resource)]
pub struct UnsortedCommandBuffer {
    pub inner: HashMap<WindowId, Vec<(u16, DrawCommand)>>,
}

impl UnsortedCommandBuffer {
    fn get_mut_or_insert(&mut self, id: &WindowId) -> &mut Vec<(u16, DrawCommand)> {
        if !self.inner.contains_key(id) {
            self.inner.insert(id.clone(), vec![]);
        }
        self.inner.get_mut(id).unwrap()
    }

    pub fn clear(&mut self) {
        self.inner.values_mut().for_each(|values| values.clear());
    }
}

pub(super) fn collect_draw_rect(
    windows: Res<Windows>,
    mut commands: ResMut<UnsortedCommandBuffer>,
    rects: Query<(
        &Transform,
        &Color,
        &WindowId,
        &ZOrder,
        Option<&TextureHandle>,
        Option<&Stroke>,
    )>,
) {
    rects
        .iter()
        .for_each(|(transform, color, window_id, z_order, texture, stroke)| {
            if !windows.can_draw(window_id) {
                return;
            }

            let command = if let Some(texture) = texture {
                DrawTextureCommand {
                    transform: transform.clone(),
                    color: color.clone(),
                    stroke: stroke.cloned(),
                    texture: texture.clone(),
                }
                .into()
            } else {
                DrawRectCommand {
                    transform: transform.clone(),
                    color: color.clone(),
                    stroke: stroke.cloned(),
                }
                .into()
            };

            commands
                .get_mut_or_insert(window_id)
                .push((z_order.z, command));
        });
}

pub(super) fn collect_draw_text(
    windows: Res<Windows>,
    mut commands: ResMut<UnsortedCommandBuffer>,
    texts: Query<(&Transform, &Color, &WindowId, &ZOrder, &Text)>,
) {
    texts
        .iter()
        .for_each(|(transform, color, window_id, z_order, text)| {
            if !windows.can_draw(window_id) {
                return;
            }

            let command = DrawTextCommand {
                size: text.size,
                position: transform.position,
                color: color.clone(),
                font: text.font.clone(),
                layout: text.clone_layout(),
            };
            commands
                .get_mut_or_insert(window_id)
                .push((z_order.z, command.into()));
        });
}

pub(super) fn sort_commands(
    mut unsorted: ResMut<UnsortedCommandBuffer>,
    mut sorted: ResMut<CommandBuffer>,
) {
    let mut last_window_id = None;
    unsorted.inner.drain().for_each(|(window_id, mut buffer)| {
        buffer.sort_unstable_by_key(|&(key, _)| key);
        buffer.drain(..).for_each(|command| {
            sorted.push(&window_id, command.1);
        });
        last_window_id = Some(window_id);
    });

    if let Some(window_id) = last_window_id {
        sorted.pack_active_group(&window_id);
    }
}
