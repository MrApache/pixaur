use crate::widget::DesiredSize;
use bevy_ecs::{component::HookContext, prelude::*, world::DeferredWorld};
use glam::Vec2;

#[derive(Default, Clone, Component)]
pub struct Text {
    value: String,
    size: u32,
}

#[derive(Default, Clone, Component)]
#[component(on_add = on_add_z_order)]
pub(crate) struct ZOrder {
    pub z: u16,
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
    pub position: Vec2,
    pub size: Vec2,
}

#[derive(Default, Clone, Bundle)]
pub struct WidgetBundle {
    order: ZOrder,
    transform: Transform,
    pub desired_size: DesiredSize,
}
