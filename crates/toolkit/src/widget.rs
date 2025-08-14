use std::any::Any;

use crate::App;
use bevy_ecs::component::Component;
use glam::Vec2;

#[derive(Default)]
pub struct Spacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Default, Component, Clone)]
pub enum DesiredSize {
    Min(Vec2),
    FillMinY(f32),
    #[default]
    Fill,
}

pub trait Widget: Send + Sync + 'static {
    fn init(&self, app: &mut App);
}

pub trait Container: Any + Send + Sync + 'static {
    fn layout(&self);
    fn draw(&self);
}
