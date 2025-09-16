use crate::{types::Rect, CommandBuffer};
use glam::Vec2;
use std::any::Any;
use wl_client::ButtonState;

#[derive(Default, Clone, Copy, Debug)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Padding {
    pub const ZERO: Padding = Self {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    };

    #[must_use]
    pub const fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    #[must_use]
    pub const fn all(value: f32) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Spacing {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Spacing {
    pub const ZERO: Spacing = Self {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    };
}

#[derive(Default, Clone, Copy, Debug)]
pub enum DesiredSize {
    Min(Vec2),
    FillMinY(f32),
    #[default]
    Fill,
}

#[derive(Default)]
pub struct FrameContext {
    pub(crate) delta_time: f32,
    pub(crate) position: Vec2,
    pub(crate) buttons: ButtonState,
}

impl FrameContext {
    #[must_use]
    pub const fn delta_time(&self) -> f32 {
        self.delta_time
    }

    #[must_use]
    pub const fn position(&self) -> Vec2 {
        self.position
    }

    #[must_use]
    pub const fn buttons(&self) -> ButtonState {
        self.buttons
    }
}

pub trait Widget: Any + Sync + Send + Default {
    fn id(&self) -> Option<&str>;
    fn desired_size(&self) -> DesiredSize;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>);
    fn layout(&mut self, bounds: Rect);
    fn update(&mut self, ctx: &FrameContext);
}

pub trait Container<T: Widget>: Widget {
    fn add_child(&mut self, child: T);
    fn children(&self) -> &[T];
    fn children_mut(&mut self) -> &mut [T];
}
