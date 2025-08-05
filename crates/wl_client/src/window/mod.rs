mod transform;
mod pool;

use crate::WlClient;
use transform::Transform;
pub(crate) use pool::ShmPool;

use std::{
    ffi::c_void,
    ptr::NonNull,
    sync::Arc
};

use wayland_client::{
    protocol::{
        wl_buffer::WlBuffer,
        wl_surface::WlSurface
    },
    Proxy,
    QueueHandle
};

use wayland_protocols::xdg::shell::client::{
    xdg_wm_base::XdgWmBase,
};

use smithay_client_toolkit::reexports::protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{
        Layer,
        ZwlrLayerShellV1,
    },
    zwlr_layer_surface_v1::{
        Anchor,
    }
};

use wgpu::rwh::{
    DisplayHandle,
    WaylandDisplayHandle,
    WaylandWindowHandle,
    WindowHandle
};


pub type WindowId = Arc<String>;

#[derive(Default, Clone)]
pub struct DesktopOptions {
    pub title: String,
    pub resizable: bool,
    pub decorations: bool,
}

#[derive(Clone)]
pub struct SpecialOptions {
    pub anchor: Anchor,
    pub exclusive_zone: u32,
    pub target: TargetMonitor
}

#[derive(Clone)]
pub enum WindowLayer {
    Desktop(DesktopOptions),
    Top(SpecialOptions),
    Bottom(SpecialOptions),
    Overlay(SpecialOptions),
    Background(SpecialOptions)
}

impl Default for WindowLayer {
    fn default() -> Self {
        Self::Desktop(DesktopOptions::default())
    }
}

#[derive(Default, Clone)]
pub enum TargetMonitor {
    #[default]
    Primary,
    Name(String),
    Index(usize),
    All,
}

pub struct Window {
    surface: WlSurface,
    buffer: WlBuffer,
    pool: ShmPool,
    qh: QueueHandle<WlClient>,
    pub id: Arc<String>,

    pub layer: WindowLayer,

    //Window size
    pub width: i32,
    pub height: i32,

    //Window transformation
    pub scale: i32,
    pub transform: Transform,

    pub(crate) can_draw: bool,
    pub(crate) display_ptr: NonNull<c_void>,
}

impl Window {
    pub fn resize_buffer(&mut self) {
        self.buffer = self.pool.create_buffer(0, self.width, self.height, &self.qh, &self.id);
    }

    pub fn new(
        ls: Option<&ZwlrLayerShellV1>, // 'Some' when WindowLayer is not a WindowLayer::Desktop
        xdg_wm_base: Option<&XdgWmBase>, // 'Some' when WindowLayer is a WindowLayer::Desktop

        qh: QueueHandle<WlClient>,
        id: WindowId,

        surface: WlSurface,
        pool: ShmPool,
        buffer: WlBuffer,

        display_ptr: NonNull<c_void>,

        width: i32,
        height: i32,
        layer: WindowLayer,
    ) -> Self {
        let mut instance = Self {
            surface,
            buffer,
            pool,
            qh,
            id,
            layer,
            width,
            height,
            scale: 1,
            transform: Transform::Normal0,
            can_draw: false,
            display_ptr,
        };

        instance.init(ls, xdg_wm_base);
        instance.draw();
        instance.frame();
        instance
    }

    fn init(&self,
        ls: Option<&ZwlrLayerShellV1>,
        xdg_wm_base: Option<&XdgWmBase>,
        ) {
        match &self.layer {
            WindowLayer::Desktop(_) => self.init_desktop(xdg_wm_base.unwrap()),
            WindowLayer::Top(options) => self.init_layer_shell(ls.unwrap(), Layer::Top, options),
            WindowLayer::Bottom(options) => self.init_layer_shell(ls.unwrap(), Layer::Bottom, options),
            WindowLayer::Overlay(options) => self.init_layer_shell(ls.unwrap(), Layer::Overlay, options),
            WindowLayer::Background(options) => self.init_layer_shell(ls.unwrap(), Layer::Background, options),
        };
    }

    fn init_layer_shell(
        &self,
        ls: &ZwlrLayerShellV1,
        layer: Layer,
        options: &SpecialOptions,
    ) {
        let layer_surface = ls.get_layer_surface(
            &self.surface,
            None, //TODO fix
            layer,
            self.id.as_ref().into(),
            &self.qh,
            self.id.clone()
        );

        layer_surface.set_size(self.width as u32, self.height as u32);
        layer_surface.set_anchor(options.anchor);
        layer_surface.set_exclusive_zone(options.exclusive_zone as i32);
    }

    fn init_desktop(
        &self,
        xdg_wm_base: &XdgWmBase,
    ) {
        let xdg_surface = xdg_wm_base.get_xdg_surface(&self.surface, &self.qh, self.id.clone());
        let _xdg_toplevel = xdg_surface.get_toplevel(&self.qh, self.id.clone());
    }

    pub fn can_draw(&self) -> bool {
        self.can_draw
    }

    pub fn frame(&self) {
        self.surface.frame(&self.qh, self.id.clone());
    }

    pub fn commit(&mut self) {
        self.surface.damage_buffer(0, 0, self.width, self.height);
        self.surface.commit();
        self.can_draw = false;
    }

    pub fn draw(&mut self) {
        self.surface.attach(Some(&self.buffer), 0, 0);
        self.commit();
    }

    pub fn resize_pool_if_needed(&mut self) {
        let size = (self.width as u64 * 4) * self.height as u64;
        if self.pool.need_resize(size) {
            self.pool.resize(size); 
        }
    }

    pub fn clear(&mut self) {
        self.pool.clear();
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, width: usize, pixel: (u8, u8, u8, u8)) {
        self.pool.write_pixel(x, y, width, pixel);
    }

    pub fn draw_text_at(&mut self, x: usize, y: usize, coverage: f32) {
        self.pool.draw_text_at(x, y, self.width as usize, self.height as usize, coverage);
    }
}

impl wgpu::rwh::HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<wgpu::rwh::DisplayHandle<'_>, wgpu::rwh::HandleError> {
        unsafe {
            Ok(
                DisplayHandle::borrow_raw(
                    wgpu::rwh::RawDisplayHandle::Wayland(
                        WaylandDisplayHandle::new(self.display_ptr)
                    )
                )
            )
        }
    }
}

impl wgpu::rwh::HasWindowHandle for Window {
    fn window_handle(&self) -> Result<wgpu::rwh::WindowHandle<'_>, wgpu::rwh::HandleError> {
        let proxy_ptr = self.surface.id().as_ptr() as *mut c_void;
        let ptr = NonNull::new(proxy_ptr).unwrap();
        unsafe {
            Ok(
                WindowHandle::borrow_raw(
                    wgpu::rwh::RawWindowHandle::Wayland(
                        WaylandWindowHandle::new(ptr)
                    )
                )
            )
        }
    }
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}
