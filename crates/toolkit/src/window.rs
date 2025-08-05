use wl_client::{Anchor, WindowHandle};
use wl_client::window::{DesktopOptions, SpecialOptions};

use crate::widget::{Container, Widget};

#[derive(Clone, Copy)]
pub(crate) enum WindowLayer {
    Desktop(DesktopOptions),
    Top(SpecialOptions),
    Bottom(SpecialOptions),
    Overlay(SpecialOptions),
    Background(SpecialOptions)
}

pub struct WindowRequest {
    pub(crate) id: String,
    pub(crate) layer:  WindowLayer,

    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl WindowRequest {
    pub fn desktop(
        id: impl Into<String>,
        width: u32,
        height: u32,
        resizable: bool,
        decorations: bool,
    ) ->Self {
        Self {
            id: id.into(),
            layer: WindowLayer::Desktop(DesktopOptions {
                resizable,
                decorations,
            }),
            width,
            height,
        }
    }

    pub fn top(
        id: impl Into<String>,
        width: u32,
        height: u32,
        anchor: Anchor,
        exclusive_zone: u32,
    ) ->Self {
        Self {
            id: id.into(),
            layer: WindowLayer::Top(
                SpecialOptions {
                    anchor,
                    exclusive_zone,
            }),
            width,
            height,
        }
    }

    pub fn bottom(
        id: impl Into<String>,
        width: u32,
        height: u32,
        anchor: Anchor,
        exclusive_zone: u32,
    ) ->Self {
        Self {
            id: id.into(),
            layer: WindowLayer::Bottom(
                SpecialOptions {
                    anchor,
                    exclusive_zone,
            }),
            width,
            height,
        }
    }

    pub fn overlay(
        id: impl Into<String>,
        width: u32,
        height: u32,
        anchor: Anchor,
        exclusive_zone: u32,
    ) ->Self {
        Self {
            id: id.into(),
            layer: WindowLayer::Overlay(
                SpecialOptions {
                    anchor,
                    exclusive_zone,
            }),
            width,
            height,
        }
    }

    pub fn background(
        id: impl Into<String>,
        width: u32,
        height: u32,
        anchor: Anchor,
        exclusive_zone: u32,
    ) ->Self {
        Self {
            id: id.into(),
            layer: WindowLayer::Background(
                SpecialOptions {
                    anchor,
                    exclusive_zone,
            }),
            width,
            height,
        }
    }
}

pub struct Window {
    pub(crate) root:   Box<dyn Container>,
    pub(crate) handle: WindowHandle,
}

impl Window {
    pub(crate) fn new(
        root: Box<dyn Container>,
        handle: WindowHandle,
    ) -> Self {
        Self {
            root,
            handle,
        }
    }

    fn internal_get_by_id<'a, T: Widget>(container: &'a dyn Container, id: &str) -> Option<&'a T> {
        for w in container.children() {
            if w.id().eq(id) {
                return w.as_any().downcast_ref::<T>();
            }

            if let Some(container) = w.as_container() {
                return Self::internal_get_by_id(container, id);
            }
        }

        None
    }

    fn internal_get_mut_by_id<'a, T: Widget>(container: &'a mut dyn Container, id: &str) -> Option<&'a mut T> {
        for w in container.children_mut() {
            if w.id().eq(id) {
                return w.as_any_mut().downcast_mut::<T>();
            }

            if let Some(container) = w.as_container_mut() {
                return Self::internal_get_mut_by_id(container, id);
            }
        }

        None
    }

    pub fn get_by_id<T: Widget>(&self, id: &str) -> Option<&T> {
        Self::internal_get_by_id(self.root.as_ref(), id)
    }

    pub fn get_mut_by_id<T: Widget>(&mut self, id: &str) -> Option<&mut T> {
        Self::internal_get_mut_by_id(self.root.as_mut(), id)
    }
}
