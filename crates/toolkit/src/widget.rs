use crate::{types::Rect, CommandBuffer, ContentManager, WindowRoot};
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
    pub(crate) delta_time: f64,
    pub(crate) position: Vec2,
    pub(crate) buttons: ButtonState,
}

impl FrameContext {
    #[must_use]
    pub const fn delta_time(&self) -> f64 {
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

pub trait Widget<C: Context>: WidgetQuery<C> + Any + Sync + Send + Default {
    fn id(&self) -> Option<&str>;
    fn desired_size(&self) -> DesiredSize;
    fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>);
    fn layout(&mut self, bounds: Rect);
    fn update(&mut self, ctx: &FrameContext, sender: &mut Sender<C>);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Container<C: Context, W: Widget<C>>: Widget<C> {
    fn add_child(&mut self, child: W);
    fn children(&self) -> &[W];
    fn children_mut(&mut self) -> &mut [W];
}

pub trait Context: Send + Sync + Sized + 'static {
    type Widget: Widget<Self>;
    type WindowRoot: WindowRoot<Self, Self::Widget>;
    fn execute(&self, content: &mut ContentManager, tree: &mut Tree<Self>);
}

pub struct Sender<C: Context> {
    inner: Vec<C>,
}

impl<C: Context> Default for Sender<C> {
    fn default() -> Self {
        Self {
            inner: Vec::with_capacity(32),
        }
    }
}

impl<C: Context> Sender<C> {
    pub(crate) fn execute(&mut self, content: &mut ContentManager, mut tree: Tree<C>) {
        self.inner.retain(|ctx| {
            ctx.execute(content, &mut tree);
            false
        });
    }

    pub fn create_event(&mut self, event: C) {
        self.inner.push(event);
    }
}

pub trait Callbacks: Send + Sync + Default + 'static {}

pub struct Tree<'a, C: Context> {
    pub(crate) frontends: &'a mut [C::WindowRoot]
}

impl<C: Context> WidgetQuery<C> for Tree<'_, C> {
    fn get_element<QW: Widget<C>>(&self, id: &str) -> Option<&QW> { 
        for frontend in self.frontends.iter() {
            let element = frontend.root().get_element(id);
            if element.is_some() {
                return element;
            }
        }

        None
    }

    fn get_mut_element<QW: Widget<C>>(&mut self, id: &str) -> Option<&mut QW> {
        for frontend in self.frontends.iter_mut() {
            let element = frontend.root_mut().get_mut_element(id);
            if element.is_some() {
                return element;
            }
        }

        None
    }
}

#[allow(unused_variables)]
pub trait WidgetQuery<C: Context> {
    fn get_element<QW: Widget<C>>(&self, id: &str) -> Option<&QW> { None }
    fn get_mut_element<QW: Widget<C>>(&mut self, id: &str) -> Option<&mut QW> { None }
}
