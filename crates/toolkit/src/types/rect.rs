use bevy_ecs::component::Component;
use glam::Vec2;

use crate::types::Color;

#[derive(Default, Debug, Clone, Component)]
#[require(Color)]
pub struct Rect {
    pub position: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self {
            position,
            size: position + size,
        }
    }

    pub const fn from_size(size: Vec2) -> Self {
        Self {
            position: Vec2::ZERO,
            size,
        }
    }

    pub const fn width(&self) -> f32 {
        self.size.x - self.position.x
    }

    pub const fn height(&self) -> f32 {
        self.size.y - self.position.y
    }
}
