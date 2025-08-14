use bevy_ecs::{component::HookContext, prelude::*, world::DeferredWorld};
use glam::Vec2;

use crate::{
    types::{Color, Stroke},
    TextureHandle,
};

#[derive(Default, Clone, Component)]
struct Text {
    value: String,
    size: u32,
}

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

#[derive(Default, Clone, Component)]
#[component(on_add = on_add_z_order)]
pub struct ZOrder {
    z: u32,
}

fn on_add_z_order(mut world: DeferredWorld, context: HookContext) {
    let entity = world.get_entity(context.entity).unwrap();
    if let Some(child_of) = entity.get::<ChildOf>() {
        let parent = world.get_entity_mut(child_of.0).unwrap();
        let z = parent
            .get::<ZOrder>()
            .expect("Parent does not have the required ZOrder component")
            .z;

        let mut entity = world.get_entity_mut(context.entity).unwrap();
        let mut current_component = entity.get_mut::<ZOrder>().unwrap();
        current_component.z = z + 1;
    }
}

#[derive(Default, Clone, Component)]
pub struct Transform {
    position: Vec2,
    size: Vec2,
}

#[derive(Default, Clone, Bundle)]
pub struct WidgetBundle {
    order: ZOrder,
    transform: Transform,
}
