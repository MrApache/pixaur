use crate::{CommandBuffer, types::Rect};
use glam::Vec2;
use std::any::Any;

#[derive(Default)]
pub struct Spacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

pub enum DesiredSize {
    Min(Vec2),
    FillMinY(f32),
    Fill,
}

pub trait Widget: Any + Sync + Send {
    fn id(&self) -> Option<&str>;
    fn desired_size(&self) -> DesiredSize;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>);
    fn layout(&mut self, bounds: Rect);
}

pub trait Container<T: Widget>: Widget {
    fn add_child(&mut self, child: T);
    fn children(&self) -> &[T];
    fn children_mut(&mut self) -> &mut [T];
}
