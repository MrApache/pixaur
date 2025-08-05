use std::ffi::c_void;
use std::ptr::NonNull;

use wgpu::rwh::{
    DisplayHandle,
    HasDisplayHandle,
    HasWindowHandle,
    RawDisplayHandle,
    RawWindowHandle,
    WaylandDisplayHandle,
    WaylandWindowHandle,
    WindowHandle
};

use wgpu::Surface;
use wl_client::WindowBackend;
use wl_client::window::{DesktopOptions, SpecialOptions, WindowLayer};

use crate::widget::{Container, Widget};
use crate::{UserWindow, GUI};

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

pub struct Window<T: GUI> {
    pub(crate) frontend: Box<dyn Container>,
    pub(crate) backend:  WindowBackend,
    pub(crate) handle: Box<dyn UserWindow<T>>,
    pub(crate) surface: Surface<'static>,
}

//Crate API
impl<T: GUI> Window<T> {
    pub(crate) const fn new(
        frontend: Box<dyn Container>,
        backend: WindowBackend,
        surface: Surface<'static>,
        handle: Box<dyn UserWindow<T>>,

    ) -> Self {
        Self {
            frontend,
            backend,
            handle,
            surface,
        }
    }
}

//Public API
impl<T: GUI> Window<T> {
    fn internal_get_by_id<'a, W: Widget>(container: &'a dyn Container, id: &str) -> Option<&'a W> {
        for w in container.children() {
            if w.id().eq(id) {
                return w.as_any().downcast_ref::<W>();
            }

            if let Some(container) = w.as_container() {
                return Self::internal_get_by_id(container, id);
            }
        }

        None
    }

    fn internal_get_mut_by_id<'a, W: Widget>(container: &'a mut dyn Container, id: &str) -> Option<&'a mut W> {
        for w in container.children_mut() {
            if w.id().eq(id) {
                return w.as_any_mut().downcast_mut::<W>();
            }

            if let Some(container) = w.as_container_mut() {
                return Self::internal_get_mut_by_id(container, id);
            }
        }

        None
    }

    pub fn get_by_id<W: Widget>(&self, id: &str) -> Option<&W> {
        Self::internal_get_by_id(self.frontend.as_ref(), id)
    }

    pub fn get_mut_by_id<W: Widget>(&mut self, id: &str) -> Option<&mut W> {
        Self::internal_get_mut_by_id(self.frontend.as_mut(), id)
    }
}

pub struct WindowPointer {
    display_ptr: NonNull<c_void>,
    surface_ptr: NonNull<c_void>,
}

impl WindowPointer {
    pub const fn new(display_ptr: NonNull<c_void>, surface_ptr: NonNull<c_void>) -> Self {
        Self {
            display_ptr,
            surface_ptr,
        }
    }
}

impl HasDisplayHandle for WindowPointer {
    fn display_handle(&self) -> Result<wgpu::rwh::DisplayHandle<'_>, wgpu::rwh::HandleError> {
        unsafe {
            Ok(
                DisplayHandle::borrow_raw(
                    RawDisplayHandle::Wayland(
                        WaylandDisplayHandle::new(self.display_ptr)
                    )
                )
            )
        }
    }
}

impl HasWindowHandle for WindowPointer {
    fn window_handle(&self) -> Result<wgpu::rwh::WindowHandle<'_>, wgpu::rwh::HandleError> {
        unsafe {
            Ok(
                WindowHandle::borrow_raw(
                    RawWindowHandle::Wayland(
                        WaylandWindowHandle::new(self.surface_ptr)
                    )
                )
            )
        }
    }
}

unsafe impl Send for WindowPointer {}
unsafe impl Sync for WindowPointer {}
