use crate::{
    commands::CommandBuffer,
    types::{Bounds, Color},
    ContentManager, WindowRoot,
};
use bitflags::bitflags;
use glam::Vec2;
use std::any::Any;
use wl_client::ButtonState;

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Anchor: u8 {
        const Left   = 1 << 0;
        const Right  = 1 << 1;
        const Top    = 1 << 2;
        const Bottom = 1 << 3;
        const Center = 1 << 4;

        const VerticalCenter   = 1 << 5;
        const HorizontalCenter = 1 << 6;
    }
}

#[derive(Default, Clone, Copy, Debug)]
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

#[derive(Default, Clone, Copy, Debug)]
pub enum DesiredSize {
    Exact(Vec2),
    ExactY(f32),
    ExactX(f32),
    #[default]
    Fill,
    Ignore,
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
    fn desired_size(&self) -> DesiredSize;
    fn anchor(&self) -> Anchor;
    fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>);
    fn layout(&mut self, bounds: Bounds);
    fn update(&mut self, ctx: &FrameContext, sender: &mut Sender<C>);
}

pub trait Container<C: Context, W: Widget<C>>: Widget<C> {
    fn add_child(&mut self, child: W);
    fn children(&self) -> &[W];
    fn children_mut(&mut self) -> &mut [W];
}

pub trait Context: Send + Sync + Default + Sized + 'static {
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
    pub(crate) frontends: &'a mut [C::WindowRoot],
}

impl<C> Tree<'_, C>
where
    C: Context,
{
    #[must_use]
    pub fn get_element<QW: Widget<C>>(&self, id: &str) -> Option<&QW> {
        for frontend in self.frontends.iter() {
            let element = frontend.root().get_element(id);
            if element.is_some() {
                return element;
            }
        }

        None
    }

    #[must_use]
    pub fn get_mut_element<QW: Widget<C>>(&mut self, id: &str) -> Option<&mut QW> {
        for frontend in self.frontends.iter_mut() {
            let element = frontend.root_mut().get_mut_element(id);
            if element.is_some() {
                return element;
            }
        }

        None
    }
}

pub trait WidgetID: Send + Sync + Default + 'static {
    type IdType: Send + Sync + Default;
    fn as_option(id: &Self::IdType) -> Option<&str>;
    fn eq_id(id: &Self::IdType, other: &str) -> bool;
}

#[derive(Default)]
pub struct StaticID;
impl WidgetID for StaticID {
    type IdType = &'static str;

    fn as_option(id: &Self::IdType) -> Option<&str> {
        Some(id)
    }

    fn eq_id(id: &Self::IdType, other: &str) -> bool {
        (*id).eq(other)
    }
}

#[derive(Default)]
pub struct NoID;
impl WidgetID for NoID {
    type IdType = ();

    fn as_option((): &Self::IdType) -> Option<&str> {
        None
    }

    fn eq_id((): &Self::IdType, _: &str) -> bool {
        false
    }
}

#[derive(Default)]
pub struct DefaultID;
impl WidgetID for DefaultID {
    type IdType = Option<String>;

    fn as_option(id: &Self::IdType) -> Option<&str> {
        id.as_deref()
    }

    fn eq_id(id: &Self::IdType, other: &str) -> bool {
        if let Some(id) = id {
            id.eq(other)
        } else {
            false
        }
    }
}

#[allow(unused_variables)]
pub trait WidgetQuery<C>
where
    C: Context,
{
    fn id(&self) -> Option<&str>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_element<QW: Widget<C>>(&self, id: &str) -> Option<&QW> {
        None
    }
    fn get_mut_element<QW: Widget<C>>(&mut self, id: &str) -> Option<&mut QW> {
        None
    }
}

impl<C, W> WidgetQuery<C> for Vec<W>
where
    C: Context,
    W: Widget<C>,
{
    fn id(&self) -> Option<&str> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_element<QW: Widget<C>>(&self, id: &str) -> Option<&QW> {
        for element in self {
            let element = element.get_element(id);
            if element.is_some() {
                return element;
            }
        }

        None
    }

    fn get_mut_element<QW: Widget<C>>(&mut self, id: &str) -> Option<&mut QW> {
        for element in self.iter_mut() {
            let element = element.get_mut_element(id);
            if element.is_some() {
                return element;
            }
        }

        None
    }
}

#[derive(Default)]
pub struct Empty;

impl<C> Widget<C> for Empty
where
    C: Context,
{
    fn desired_size(&self) -> DesiredSize {
        DesiredSize::Ignore
    }

    fn anchor(&self) -> Anchor {
        Anchor::Left
    }

    fn draw<'frame>(&'frame self, _: &mut CommandBuffer<'frame>) {}
    fn layout(&mut self, _: Bounds) {}
    fn update(&mut self, _: &FrameContext, _: &mut Sender<C>) {}
}

impl<C> WidgetQuery<C> for Empty
where
    C: Context,
{
    fn id(&self) -> Option<&str> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
