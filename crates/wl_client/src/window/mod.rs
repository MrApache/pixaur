mod transform;
mod pool;

use std::{ffi::c_void, ptr::NonNull, sync::Arc};
use wayland_client::{
    protocol::{
        wl_buffer::WlBuffer, wl_compositor::WlCompositor, wl_output::WlOutput, wl_shm::WlShm, wl_surface::WlSurface
    }, Proxy, QueueHandle
};

use wayland_protocols::xdg::shell::client::{
    xdg_toplevel::XdgToplevel,
    xdg_surface::XdgSurface,
    xdg_wm_base::XdgWmBase,
};

use smithay_client_toolkit::reexports::protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{
        Layer,
        ZwlrLayerShellV1,
    },
    zwlr_layer_surface_v1::{
        Anchor,
        ZwlrLayerSurfaceV1,
    }
};


pub(crate) use pool::ShmPool;
use transform::Transform;
use wgpu::{rwh::{DisplayHandle, WaylandDisplayHandle, WaylandWindowHandle, WindowHandle}, Surface};
use crate::WlClient;

pub type WindowId = Arc<String>;

enum WindowLayer {
    Desktop(Desktop),
    LayerShell(ZwlrLayerSurfaceV1),
}

struct Desktop {
    xdg_surface: XdgSurface,
    xdg_toplevel: XdgToplevel,
}

pub struct Window {
    surface: WlSurface,
    buffer: WlBuffer,
    layer: WindowLayer,
    pool: ShmPool,

    pub id: Arc<String>,

    pub width: i32,
    pub height: i32,
    pub scale: i32,
    pub transform: Transform,

    pub(crate) can_draw: bool,

    pub(crate) display_ptr: NonNull<c_void>,

    pub(crate) gpu_surface: Surface<'static>
}

impl Window {
    pub fn resize_buffer(&mut self, id: &WindowId, qh: &QueueHandle<WlClient>) {
        self.buffer = self.pool.create_buffer(0, self.width, self.height, qh, id);
    }

    pub fn layer_shell(
        ls: &ZwlrLayerShellV1,
        qh: &QueueHandle<WlClient>,
        id: WindowId,

        surface: WlSurface,
        pool: ShmPool,
        buffer: WlBuffer,

        output: Option<&WlOutput>,
        width: i32,
        height: i32,
        layer: Layer,
        anchor: Anchor,
        exclusive_zone: i32,

        display_ptr: NonNull<c_void>,
        gpu_surface: Surface<'static>

    ) -> Self {
        let layer_surface = ls.get_layer_surface(
            &surface,
            output,
            layer,
            "".into(),
            qh,
            id.clone()
        );

        layer_surface.set_size(width as u32, height as u32);
        layer_surface.set_anchor(anchor);
        layer_surface.set_exclusive_zone(exclusive_zone);

        let layer = WindowLayer::LayerShell(layer_surface);

        surface.attach(Some(&buffer), 0, 0);
        surface.damage_buffer(0, 0, width, height);
        surface.commit();
        surface.frame(qh, id.clone());

        Self {
            surface,
            buffer,
            pool,
            id,
            layer,

            width,
            height,
            scale: 1,
            transform: Transform::Normal0,

            can_draw: false,
            display_ptr,
            gpu_surface
        }
    }

    pub fn desktop_window(
        compositor: &WlCompositor,
        shm: &WlShm,
        xdg_wm_base: &XdgWmBase,
        qh: &QueueHandle<WlClient>,

        id: WindowId,
        width: i32,
        height: i32,

        display_ptr: NonNull<c_void>,
        gpu_surface: Surface<'static>
    ) -> Self {
        let surface = compositor.create_surface(qh, id.clone());
        let pool = ShmPool::new((width as u64 * 4) * height as u64, &id, shm, qh);
        let buffer = pool.create_buffer(0, width, height, qh, &id);

        let xdg_surface = xdg_wm_base.get_xdg_surface(&surface, qh, id.clone());
        let xdg_toplevel = xdg_surface.get_toplevel(qh, id.clone());
        let layer = WindowLayer::Desktop(Desktop { xdg_surface, xdg_toplevel });

        surface.attach(Some(&buffer), 0, 0);
        surface.damage_buffer(0, 0, width, height);
        surface.commit();
        surface.frame(qh, id.clone());
        Self {
            surface,
            buffer,
            pool,
            id,
            layer,

            width,
            height,
            scale: 1,
            transform: Transform::Normal0,

            can_draw: false,
            display_ptr,
            gpu_surface
        }
    }

    pub fn can_draw(&self) -> bool {
        self.can_draw
    }

    pub fn frame(&self, qh: &QueueHandle<WlClient>) {
        self.surface.frame(qh, self.id.clone());
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
//impl<'window> Into<SurfaceTarget<'window>> for Window {
//    fn into(self) -> SurfaceTarget<'window> {
//        SurfaceTarget::Window(Box::new(self))
//    }
//}
