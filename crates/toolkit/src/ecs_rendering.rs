use std::collections::HashMap;

use crate::{
    ecs::{Text, ZOrder},
    types::{Color, Stroke},
    widget::Plugin,
    CollectDrawCommands, Monitor, Render, TextureHandle, Transform,
};
use bevy_ecs::prelude::*;
use wl_client::window::TargetMonitor;

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
    pub inner: HashMap<TargetMonitor, Vec<(u32, DrawRequest)>>,
}

impl Commands {
    fn get_mut_or_insert(&mut self, monitor: &TargetMonitor) -> &mut Vec<(u32, DrawRequest)> {
        if self.inner.contains_key(monitor) {
            self.inner.insert(monitor.clone(), vec![]);
        }
        self.inner.get_mut(monitor).unwrap()
    }

    fn clear(&mut self) {
        self.inner.values_mut().for_each(|values| values.clear());
    }
}

fn collect_draw_rect(
    mut commands: ResMut<Commands>,
    rects: Query<(
        &Transform,
        &Color,
        &Monitor,
        &ZOrder,
        Option<&TextureHandle>,
        Option<&Stroke>,
    )>,
) {
    rects
        .iter()
        .for_each(|(transform, color, monitor, z_order, texture, stroke)| {
            let command = DrawRect {
                transform: transform.clone(),
                color: color.clone(),
                stroke: stroke.cloned(),
                texture: texture.cloned(),
            };
            commands
                .get_mut_or_insert(&monitor.0)
                .push((z_order.z, DrawRequest::Rect(command)));
        });
}

fn collect_draw_text(
    mut commands: ResMut<Commands>,
    texts: Query<(&Transform, &Color, &Monitor, &ZOrder, &Text)>,
) {
    texts
        .iter()
        .for_each(|(transform, color, monitor, z_order, text)| {
            let command = DrawText {
                transform: transform.clone(),
                color: color.clone(),
                text: text.clone(),
            };
            commands
                .get_mut_or_insert(&monitor.0)
                .push((z_order.z, DrawRequest::Text(command)));
        });
}

fn sort_commands(mut commands: ResMut<Commands>) {
    commands.inner.values_mut().for_each(|window| {
        window.sort_unstable_by_key(|&(key, _)| key);
    });
}

pub(crate) struct Renderer;

impl Plugin for Renderer {
    fn init(&self, app: &mut crate::App) {
        app.insert_resource(Commands::default());
        app.add_systems(CollectDrawCommands, (collect_draw_text, collect_draw_rect));
        app.add_systems(Render, sort_commands); //TODO add renderer itself
    }
}
