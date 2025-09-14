use std::ffi::c_void;
use std::ptr::NonNull;
use wgpu::{Surface, SurfaceConfiguration};
use wl_client::WindowBackend;
use wl_client::window::{DesktopOptions, SpecialOptions, WindowLayer};
use wgpu::rwh::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
    RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle, WindowHandle,
};

use crate::rendering::Renderer;
use crate::widget::{Container, Widget};
use crate::{GUI, UserWindow};

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

    #[must_use]
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    #[must_use]
    pub fn desktop(mut self, options: DesktopOptions) -> Self {
        self.layer = WindowLayer::Desktop(options);
        self
    }

    #[must_use]
    pub fn top(mut self, options: SpecialOptions) -> Self {
        self.layer = WindowLayer::Top(options);
        self
    }

    #[must_use]
    pub fn bottom(mut self, options: SpecialOptions) -> Self {
        self.layer = WindowLayer::Bottom(options);
        self
    }

    #[must_use]
    pub fn overlay(mut self, options: SpecialOptions) -> Self {
        self.layer = WindowLayer::Overlay(options);
        self
    }

    #[must_use]
    pub fn background(mut self, options: SpecialOptions) -> Self {
        self.layer = WindowLayer::Background(options);
        self
    }
}

pub struct Window<W: Widget, R: Container<W>, T: GUI<W, R>> {
    pub(crate) frontend: R,
    pub(crate) backend: WindowBackend,
    pub(crate) surface: Surface<'static>,
    pub(crate) configuration: SurfaceConfiguration,
    pub(crate) handle: Box<dyn UserWindow<W, R, T>>,
    pub(crate) renderer: Renderer,
    //_phantom: std::marker::PhantomData<W>
}

impl<W: Widget, R: Container<W>, T: GUI<W, R>> Window<W, R, T> {
    pub(crate) const fn new(
        frontend: R,
        backend: WindowBackend,
        surface: Surface<'static>,
        configuration: SurfaceConfiguration,
        handle: Box<dyn UserWindow<W, R ,T>>,
        renderer: Renderer,
    ) -> Self {
        Self {
            frontend,
            backend,
            surface,
            configuration,
            handle,
            renderer,
        }
    }
}

pub struct WindowPointer {
    display_ptr: NonNull<c_void>,
    surface_ptr: NonNull<c_void>,
}

impl WindowPointer {
    #[must_use]
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
