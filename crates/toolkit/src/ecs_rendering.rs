use crate::{
    ecs::{Text, ZOrder},
    types::{Color, Stroke},
    widget::Widget,
    CollectDrawCommands, Render, TextureHandle, Transform,
};
use bevy_ecs::prelude::*;

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
    pub buffer: Vec<(u32, DrawRequest)>,
}

fn collect_draw_rect(
    mut commands: ResMut<Commands>,
    rects: Query<(
        &Transform,
        &Color,
        &ZOrder,
        Option<&TextureHandle>,
        Option<&Stroke>,
    )>,
) {
    rects
        .iter()
        .for_each(|(transform, color, z_order, texture, stroke)| {
            let command = DrawRect {
                transform: transform.clone(),
                color: color.clone(),
                stroke: stroke.cloned(),
                texture: texture.cloned(),
            };
            commands
                .buffer
                .push((z_order.z, DrawRequest::Rect(command)));
        });
}

fn collect_draw_text(
    mut commands: ResMut<Commands>,
    texts: Query<(&Transform, &Color, &ZOrder, &Text)>,
) {
    texts.iter().for_each(|(transform, color, z_order, text)| {
        let command = DrawText {
            transform: transform.clone(),
            color: color.clone(),
            text: text.clone(),
        };

        commands
            .buffer
            .push((z_order.z, DrawRequest::Text(command)));
    });
}

fn sort_commands(mut commands: ResMut<Commands>) {
    commands.buffer.sort_unstable_by_key(|&(key, _)| key);
}

pub(crate) struct Renderer;

impl Widget for Renderer {
    fn init(&self, app: &mut crate::App) {
        app.insert_resource(Commands::default());
        app.add_systems(CollectDrawCommands, (collect_draw_text, collect_draw_rect));
        app.add_systems(Render, sort_commands); //TODO add renderer itself
    }
}
