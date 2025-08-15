use crate::{widget::DesiredSize, FontHandle};
use bevy_ecs::{component::HookContext, prelude::*, world::DeferredWorld};
use fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use glam::Vec2;

#[derive(Component)]
pub struct Text {
    pub(crate) font: FontHandle,
    pub(crate) layout: Layout,
    pub(crate) value: String,
    pub(crate) size: u32,
}

impl Default for Text {
    fn default() -> Self {
        let mut instance = Self {
            value: String::new(),
            font: FontHandle::default(),
            size: 12,
            layout: Layout::new(CoordinateSystem::PositiveYDown),
        };

        instance.refresh_layout();
        instance
    }
}

impl Text {
    pub fn new(font: FontHandle) -> Self {
        let mut instance = Self {
            value: String::new(),
            font,
            size: 12,
            layout: Layout::new(CoordinateSystem::PositiveYDown),
        };

        instance.refresh_layout();
        instance
    }

    pub fn set_text(&mut self, value: &str) {
        self.value.clear();
        self.value.insert_str(0, value);
        self.refresh_layout();
    }

    pub fn set_size(&mut self, value: u32) {
        self.size = value;
        self.refresh_layout();
    }

    fn refresh_layout(&mut self) {
        self.layout.clear();
        self.layout.append(
            &[self.font.as_ref()],
            &TextStyle {
                text: &self.value,
                px: self.size as f32,
                font_index: 0,
                user_data: (),
            },
        );
    }

    pub fn clone_layout(&self) -> Layout {
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.append(
            &[self.font.as_ref()],
            &TextStyle {
                text: &self.value,
                px: self.size as f32,
                font_index: 0,
                user_data: (),
            },
        );

        layout
    }
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
