use crate::{CommandBuffer, types::Rect};
use glam::Vec2;
use wl_client::ButtonState;
use std::any::Any;

#[derive(Default)]
pub struct Spacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Default, Clone, Copy)]
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
