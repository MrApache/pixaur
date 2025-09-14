pub mod window;
pub use smithay_client_toolkit::reexports::protocols_wlr::layer_shell::v1::client::zwlr_layer_surface_v1::Anchor;
pub use smithay_client_toolkit::reexports::protocols_wlr::layer_shell::v1::client::zwlr_layer_shell_v1::Layer;

use std::{
    collections::HashMap,
    process::exit,
    sync::{Arc, Mutex},
};

use smithay_client_toolkit::reexports::protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{Event as ZwlrLayerShellV1Event, ZwlrLayerShellV1},
    zwlr_layer_surface_v1::{Event as ZwlrLayerSurfaceV1Event, ZwlrLayerSurfaceV1},
};

use wayland_client::{
    protocol::{
        wl_buffer::{Event as WlBufferEvent, WlBuffer},
        wl_callback::{Event as WlCallbackEvent, WlCallback},
        wl_compositor::{Event as WlCompositorEvent, WlCompositor},
        wl_output::{Event as WlOutputEvent, WlOutput},
        wl_registry::{Event as WlRegistryEvent, WlRegistry},
        wl_shm::{Event as WlShmEvent, WlShm},
        wl_shm_pool::{Event as WlShmPoolEvent, WlShmPool},
        wl_surface::{Event as WlSurfaceEvent, WlSurface},
    },
    Connection, Dispatch, Proxy, QueueHandle,
};

use wayland_protocols::xdg::shell::client::{
    xdg_surface::{Event as XdgSurfaceEvent, XdgSurface},
    xdg_toplevel::{Event as XdgTopLevelEvent, XdgToplevel},
    xdg_wm_base::{Event as XdgWmBaseEvent, XdgWmBase},
};

use crate::window::{ShmPool, Window, WindowId, WindowLayer};

const DESKTOP_DEFAULT_WIDTH: i32 = 600;
const DESKTOP_DEFAULT_HEIGHT: i32 = 400;

#[derive(Default)]
pub struct WlClient {
    compositor: Option<WlCompositor>,
    xdg_wm_base: Option<XdgWmBase>,
    layer_shell: Option<ZwlrLayerShellV1>,
    shm: Option<WlShm>,

    outputs: HashMap<String, WlOutput>,
    windows: HashMap<String, WindowBackend>,
}

impl WlClient {
    fn create_surface(
        &mut self,
        qh: &QueueHandle<WlClient>,
        id: &Arc<String>,
        width: i32,
        height: i32,
    ) -> (WlSurface, ShmPool, WlBuffer) {
        let compositor = self.compositor.as_ref().expect("unreachable");
        let shm = self.shm.as_ref().expect("unreachable");

        let surface = compositor.create_surface(qh, id.clone());
        let pool = ShmPool::new((width as u64 * 4) * height as u64, id, shm, qh);
        let buffer = pool.create_buffer(0, width, height, qh, id);

        (surface, pool, buffer)
    }

    pub fn create_window_backend(
        &mut self,
        qh: QueueHandle<WlClient>,
        id: impl Into<String>,
        width: u32,
        height: u32,
        layer: WindowLayer,
    ) -> Arc<Mutex<Window>> {
        let width = width as i32;
        let height = height as i32;

        let id = id.into();
        let arc_id = Arc::new(id.clone());
        let (surface, pool, buffer) = self.create_surface(&qh, &arc_id, width, height);

        let window = Arc::new(Mutex::new(Window::new(
            Some(self.layer_shell.as_ref().expect("unreachable")),
            Some(self.xdg_wm_base.as_ref().expect("unreachable")),
            qh,
            arc_id,
            surface,
            pool,
            buffer,
            width,
            height,
            layer,
        )));

        self.windows.insert(id, window.clone());
        window
    }

    pub fn destroy_window_backend(&mut self, window_id: &str) {
        let window = self.windows.remove(window_id).unwrap();
        let window = Arc::try_unwrap(window)
            .expect("Arc has other references")
            .into_inner()
            .expect("Mutex poisoned");

        window.destroy();
    }
}

impl Dispatch<WlRegistry, WindowId> for WlClient {
    fn event(
        state: &mut Self,
        registry: &WlRegistry,
        event: WlRegistryEvent,
        id: &WindowId,
        _: &Connection,
        qh: &QueueHandle<WlClient>,
    ) {
        if let WlRegistryEvent::Global {
            name,
            interface,
            version,
        } = event
        {
            //println!("[{name}] {interface} (v{version})");

            match interface.as_ref() {
                "wl_compositor" => {
                    state.compositor =
                        Some(registry.bind::<WlCompositor, _, _>(name, version, qh, id.clone()));
                }
                "wl_shm" => {
                    state.shm = Some(registry.bind::<WlShm, _, _>(name, version, qh, id.clone()));
                }
                "xdg_wm_base" => {
                    state.xdg_wm_base =
                        Some(registry.bind::<XdgWmBase, _, _>(name, version, qh, id.clone()));
                }
                "zwlr_layer_shell_v1" => {
                    state.layer_shell =
                        Some(registry.bind::<ZwlrLayerShellV1, _, _>(name, version, qh, id.clone()));
                }
                "wl_output" => {
                    let output = registry.bind::<WlOutput, _, _>(name, version, qh, id.clone());
                    state.outputs.insert(output.id().to_string(), output);
                }
                _ => {}
            }
        }
    }
}

#[allow(unused)]
impl Dispatch<WlOutput, WindowId> for WlClient {
    fn event(
        state: &mut Self,
        output: &WlOutput,
        event: WlOutputEvent,
        _: &WindowId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            WlOutputEvent::Geometry {
                x,
                y,
                physical_width,
                physical_height,
                subpixel,
                make,
                model,
                transform,
            } => {}
            WlOutputEvent::Mode {
                flags,
                width,
                height,
                refresh,
            } => {}
            WlOutputEvent::Scale { factor } => {}
            WlOutputEvent::Name { name } => {
                let id = output.id().to_string();
                let output = state.outputs.remove(&id).unwrap();
                state.outputs.insert(name, output);
            }
            WlOutputEvent::Description { description } => {}
            WlOutputEvent::Done |
            _ => {}
        }
    }
}

impl Dispatch<WlCompositor, WindowId> for WlClient {
    fn event(
        _: &mut Self,
        _: &WlCompositor,
        _: WlCompositorEvent,
        _: &WindowId,
        _: &Connection,
        _: &QueueHandle<WlClient>,
    ) {
    }
}

impl Dispatch<WlSurface, WindowId> for WlClient {
    fn event(
        state: &mut Self,
        _: &WlSurface,
        event: WlSurfaceEvent,
        id: &WindowId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            WlSurfaceEvent::Enter { output: _ } => println!("Enter"),
            WlSurfaceEvent::Leave { output: _ } => println!("Leave"),

            WlSurfaceEvent::PreferredBufferScale { factor } => {
                let mut window = state.windows.get_mut(id.as_str()).unwrap().lock().unwrap();
                window.scale = factor;
            }

            WlSurfaceEvent::PreferredBufferTransform { transform } => {
                let mut window = state.windows.get_mut(id.as_str()).unwrap().lock().unwrap();
                window.transform = transform.into();
            }

            _ => {}
        }
    }
}

impl Dispatch<WlShmPool, WindowId> for WlClient {
    fn event(
        _: &mut Self,
        _: &WlShmPool,
        _: WlShmPoolEvent,
        _: &WindowId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlShm, WindowId> for WlClient {
    fn event(
        _: &mut Self,
        _: &WlShm,
        _: WlShmEvent,
        _: &WindowId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlBuffer, WindowId> for WlClient {
    fn event(
        _: &mut Self,
        _: &WlBuffer,
        _: WlBufferEvent,
        _: &WindowId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<XdgWmBase, WindowId> for WlClient {
    fn event(
        _: &mut Self,
        xdg_wm_base: &XdgWmBase,
        event: XdgWmBaseEvent,
        _: &WindowId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let XdgWmBaseEvent::Ping { serial } = event {
            xdg_wm_base.pong(serial);
        }
    }
}

impl Dispatch<XdgSurface, WindowId> for WlClient {
    fn event(
        state: &mut Self,
        surface: &XdgSurface,
        event: XdgSurfaceEvent,
        id: &WindowId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let XdgSurfaceEvent::Configure { serial } = event {
            surface.ack_configure(serial);
            let mut window = state.windows.get_mut(id.as_str()).unwrap().lock().unwrap();
            window.resize_pool_if_needed();
            window.resize_buffer_if_needed();
            window.commit();
        }
    }
}

impl Dispatch<XdgToplevel, WindowId> for WlClient {
    fn event(
        state: &mut Self,
        _: &XdgToplevel,
        event: XdgTopLevelEvent,
        id: &WindowId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            XdgTopLevelEvent::Configure {
                mut width,
                mut height,
                states: _, //TODO
            } => {
                let mut window = state.windows.get_mut(id.as_str()).unwrap().lock().unwrap();
                if let WindowLayer::Desktop(opts) = &window.layer {
                    if !opts.resizable {
                        return;
                    }
                } else {
                    unreachable!("{:#?}", window.layer);
                }

                if width == 0 || height == 0 {
                    width = DESKTOP_DEFAULT_WIDTH;
                    height = DESKTOP_DEFAULT_HEIGHT;
                }

                window.width = width;
                window.height = height;

                window.can_resize = true;
            }
            XdgTopLevelEvent::Close => exit(0), //TODO
            //XdgTopLevelEvent::ConfigureBounds { width, height } => todo!(),
            //XdgTopLevelEvent::WmCapabilities { capabilities } => todo!(),
            _ => {}
        }
    }
}

impl Dispatch<ZwlrLayerShellV1, WindowId> for WlClient {
    fn event(
        _: &mut Self,
        _: &ZwlrLayerShellV1,
        _: ZwlrLayerShellV1Event,
        _: &WindowId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwlrLayerSurfaceV1, WindowId> for WlClient {
    fn event(
        state: &mut Self,
        surface: &ZwlrLayerSurfaceV1,
        event: ZwlrLayerSurfaceV1Event,
        id: &WindowId,
        _: &Connection,
        _: &QueueHandle<WlClient>,
    ) {
        match event {
            ZwlrLayerSurfaceV1Event::Configure {
                serial,
                width: _,  //TODO
                height: _, //TODO
            } => {
                surface.ack_configure(serial);
                let mut window = state.windows.get_mut(id.as_str()).unwrap().lock().unwrap();
                window.resize_buffer_if_needed();
                window.draw();
            }
            ZwlrLayerSurfaceV1Event::Closed => {
                println!("Layer surface event 'closed'");
                //TODO
            }
            _ => {}
        }
    }
}

impl Dispatch<WlCallback, WindowId> for WlClient {
    fn event(
        state: &mut Self,
        _: &WlCallback,
        _: WlCallbackEvent,
        id: &WindowId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let result = state.windows.get_mut(id.as_str());
        if let Some(window) = result {
            let mut window = window.lock().unwrap();
            window.can_draw = true;
        }
    }
}

pub type WindowBackend = Arc<Mutex<Window>>;
