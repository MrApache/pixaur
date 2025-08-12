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
    Fill,
}

pub trait Widget: Any {
    fn id(&self) -> &str;
    fn desired_size(&self) -> DesiredSize;
    fn as_container(&self) -> Option<&dyn Container> {
        None
    }
    fn as_container_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>);
    fn layout(&mut self, bounds: Rect);
}

pub trait Container: Widget {
    fn add_child(&mut self, child: Box<dyn Widget>);
    fn children(&self) -> &[Box<dyn Widget>];
    fn children_mut(&mut self) -> &mut [Box<dyn Widget>];
}
