use std::collections::HashMap;
use bevy_ecs::prelude::*;
use crate::{
    ecs::{Text, ZOrder},
    types::{Color, Stroke},
    widget::Plugin,
    CollectDrawCommands, Render, TextureHandle, Transform, WindowId, Windows,
};

pub struct DrawRect {
    pub transform: Transform,
    pub color: Color,
    pub stroke: Option<Stroke>,
    pub texture: Option<TextureHandle>,
}

pub struct DrawText {
    pub transform: Transform,
    pub color: Color,
    pub text: Text,
}

pub enum DrawRequest {
    Rect(DrawRect),
    Text(DrawText),
}

#[derive(Default, Resource)]
pub struct Commands {
    pub inner: HashMap<WindowId, Vec<(u16, DrawRequest)>>,
}

impl Commands {
    fn get_mut_or_insert(&mut self, id: &WindowId) -> &mut Vec<(u16, DrawRequest)> {
        if self.inner.contains_key(id) {
            self.inner.insert(id.clone(), vec![]);
        }
        self.inner.get_mut(id).unwrap()
    }

    fn clear(&mut self) {
        self.inner.values_mut().for_each(|values| values.clear());
    }
}

fn collect_draw_rect(
    windows: Res<Windows>,
    mut commands: ResMut<Commands>,
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

            let command = DrawRect {
                transform: transform.clone(),
                color: color.clone(),
                stroke: stroke.cloned(),
                texture: texture.cloned(),
            };
            commands
                .get_mut_or_insert(window_id)
                .push((z_order.z, DrawRequest::Rect(command)));
        });
}

fn collect_draw_text(
    windows: Res<Windows>,
    mut commands: ResMut<Commands>,
    texts: Query<(&Transform, &Color, &WindowId, &ZOrder, &Text)>,
) {
    texts
        .iter()
        .for_each(|(transform, color, window_id, z_order, text)| {
            if !windows.can_draw(window_id) {
                return;
            }

            let command = DrawText {
                transform: transform.clone(),
                color: color.clone(),
                text: text.clone(),
            };
            commands
                .get_mut_or_insert(window_id)
                .push((z_order.z, DrawRequest::Text(command)));
        });
}

fn sort_commands(mut commands: ResMut<Commands>) {
    commands.inner.values_mut().for_each(|window| {
        window.sort_unstable_by_key(|&(key, _)| key);
    });
}

pub(crate) struct RendererPlugin;

impl Plugin for RendererPlugin {
    fn init(&self, app: &mut crate::App) {
        app.insert_resource(Commands::default());
        app.add_systems(CollectDrawCommands, (collect_draw_text, collect_draw_rect));
        app.add_systems(Render, sort_commands); //TODO add renderer itself
    }
}
