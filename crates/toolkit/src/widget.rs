use std::any::Any;

use glam::Vec2;

use crate::CommandBuffer;

#[derive(Default, Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const ZERO: Self = Self::new(0.0, 0.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }
}

#[derive(Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Default)]
pub struct Spacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Default, Debug, Clone)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self {
            min: position,
            max: position + size,
        }
    }

    pub const fn from_size(size: Vec2) -> Self {
        Self {
            min: Vec2::ZERO,
            max: size,
        }
    }
}

pub trait Widget: Any {
    fn id(&self) -> &str;
    fn desired_size(&self) -> Size;
    fn spacing(&self) -> Spacing;
    fn as_container(&self) -> Option<&dyn Container> { None }
    fn as_container_mut(&mut self) -> Option<&mut dyn Container> { None }
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>);
}

pub trait Container: Widget {
    fn add_child(&mut self, child: Box<dyn Widget>);
    fn layout(&mut self, bounds: Rect);
    fn children(&self) -> &[Box<dyn Widget>];
    fn children_mut(&mut self) -> &mut [Box<dyn Widget>];
}
