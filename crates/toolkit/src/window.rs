use std::ffi::c_void;
use std::ptr::NonNull;

use wgpu::rwh::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
    RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle, WindowHandle,
};

use wgpu::{Surface, SurfaceConfiguration};
use wl_client::window::{DesktopOptions, SpecialOptions, WindowLayer};
use wl_client::WindowBackend;

use crate::UserWindow;

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
    pub(crate) backend: WindowBackend,
    pub(crate) surface: Surface<'static>,
    pub(crate) configuration: SurfaceConfiguration,
    pub(crate) _handle: Box<dyn UserWindow>,
}

impl Window {
    pub(crate) const fn new(
        backend: WindowBackend,
        surface: Surface<'static>,
        configuration: SurfaceConfiguration,
        _handle: Box<dyn UserWindow>,
    ) -> Self {
        Self {
            backend,
            _handle,
            surface,
            configuration,
        }
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
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        unsafe {
            Ok(DisplayHandle::borrow_raw(RawDisplayHandle::Wayland(
                WaylandDisplayHandle::new(self.display_ptr),
            )))
        }
    }
}

impl HasWindowHandle for WindowPointer {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        unsafe {
            Ok(WindowHandle::borrow_raw(RawWindowHandle::Wayland(
                WaylandWindowHandle::new(self.surface_ptr),
            )))
        }
    }
}

unsafe impl Send for WindowPointer {}
unsafe impl Sync for WindowPointer {}
