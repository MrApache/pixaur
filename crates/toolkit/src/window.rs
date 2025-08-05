use wl_client::WindowBackend;
use wl_client::window::{DesktopOptions, SpecialOptions, WindowLayer};

use crate::widget::{Container, Widget};

pub struct WindowRequest {
    pub(crate) id: String,
    pub(crate) layer: WindowLayer,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl WindowRequest {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            layer: WindowLayer::default(),
            width: 600,
            height: 400,
        }
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn desktop(mut self, options: DesktopOptions) -> Self {
        self.layer = WindowLayer::Desktop(options);
        self
    }

    pub fn top(mut self, options: SpecialOptions) -> Self {
        self.layer = WindowLayer::Top(options);
        self
    }

    pub fn bottom(mut self, options: SpecialOptions) -> Self {
        self.layer = WindowLayer::Bottom(options);
        self
    }

    pub fn overlay(mut self, options: SpecialOptions) -> Self {
        self.layer = WindowLayer::Overlay(options);
        self
    }

    pub fn background(mut self, options: SpecialOptions) -> Self {
        self.layer = WindowLayer::Background(options);
        self
    }
}

pub struct Window {
    pub(crate) root:   Box<dyn Container>,
    pub(crate) handle: WindowBackend,
}

impl Window {
    pub(crate) fn new(
        root: Box<dyn Container>,
        handle: WindowBackend,
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
