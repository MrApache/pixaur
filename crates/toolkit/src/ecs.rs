use bevy_ecs::{component::HookContext, prelude::*, world::DeferredWorld};
use glam::Vec2;
use paste::paste;

use crate::{types::{Argb8888, Color, LinearGradient, Stroke, Texture}, TextureHandle};

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
    pub buffer: Vec<(u32, DrawRequest)>
}

fn collect_draw_rect(
    mut commands: ResMut<Commands>,
    rects: Query<(
        &Transform,
        &Color,
        &ZOrder,
        Option<&TextureHandle>,
        Option<&Stroke>)>
    ) {
    rects.iter().for_each(|(transform, color, z_order, texture, stroke)| {
        let command = DrawRect {
            transform: transform.clone(),
            color: color.clone(),
            stroke: stroke.cloned(),
            texture: texture.cloned()
        };
        commands.buffer.push((z_order.z, DrawRequest::Rect(command)));
    });
}

fn collect_draw_text(
    mut commands: ResMut<Commands>,
    texts: Query<(
        &Transform,
        &Color,
        &ZOrder,
        &Text)>
    ) {

    texts.iter().for_each(|(transform, color, z_order, text)| {
        let command = DrawText {
            transform: transform.clone(),
            color: color.clone(),
            text: text.clone() ,
        };

        commands.buffer.push((z_order.z, DrawRequest::Text(command)));
    });
}

fn sort_commands(mut commands: ResMut<Commands>) {
    commands.buffer.sort_unstable_by_key(|&(key, _)| key);
}

#[derive(Default, Clone, Component)]
#[component(on_add = on_add_z_order)]
pub struct ZOrder {
    z: u32
}

fn on_add_z_order(mut world: DeferredWorld, context: HookContext) {
    let entity =  world.get_entity(context.entity).unwrap();
    if let Some(child_of) = entity.get::<ChildOf>() {
        let parent = world.get_entity_mut(child_of.0).unwrap();
        let z = parent.get::<ZOrder>().expect("Parent does not have the required ZOrder component").z;

        let mut entity =  world.get_entity_mut(context.entity).unwrap();
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

macro_rules! define_widget {
    (
        $first:ident
        $(, $rest:ident)* $(,)?
    ) => {
        paste::item! {
            #[derive(Default, bevy_ecs::bundle::Bundle)]
            pub struct [<$first Bundle>] {
                widget_base: WidgetBundle,
                pub [<$first:snake>]: $first,
                $(pub [<$rest:snake>]: $rest,)*
            }

            pub struct [<$first Builder>]<'commands, 'world, 'state> {
                commands: &'commands mut bevy_ecs::prelude::Commands<'world, 'state>,
                bundle: [<$first Bundle>]
            }

            impl<'commands, 'world, 'state> [<$first Builder>]<'commands, 'world, 'state> {
                pub fn new(commands: &'commands mut bevy_ecs::prelude::Commands<'world, 'state>) -> Self {
                    Self {
                        commands,
                        bundle: [<$first Bundle>] {
                            widget_base: Default::default(),
                            [<$first:snake>]: Default::default(),
                            $( [<$rest:snake>]: Default::default(), )*
                        }
                    }
                }

                paste! {
                    $(
                        define_widget!(@method $rest);
                    )*
                }

                pub fn build_as_child_of(self, parent: Entity) -> Entity {
                    self.commands.spawn((self.bundle, bevy_ecs::prelude::ChildOf(parent))).id()
                }

                pub fn build(self) -> Entity {
                    self.commands.spawn(self.bundle).id()
                }
            }
        }
    };

    // вспомогательная ветка для генерации метода
    (@method Color) => {
        paste! {
            pub fn color(mut self, value: impl Into<Color>) -> Self {
                self.bundle.color = value.into();
                self
            }
        }
    };
    
    (@method $name:ident) => {
        paste! {
            pub fn [<$name:snake>](mut self, value: $name) -> Self {
                self.bundle.[<$name:snake>] = value;
                self
            }
        }
    };
}

#[derive(Default, Component)]
struct Panel {
}

define_widget! {
    Panel,
    Color,
    Texture,
}

#[derive(Default, Clone, Component)]
struct Text {
    value: String,
    size: u32,
}

define_widget! {
    Text,
    Color,
}

fn testst(mut commands: bevy_ecs::prelude::Commands) {
    let panel = PanelBuilder::new(&mut commands)
        .color(Argb8888::CYAN)
        .build();

    TextBuilder::new(&mut commands)
        .color(LinearGradient::new(Argb8888::BLACK, Argb8888::WHITE, 45.0))
        .build_as_child_of(panel);
}

