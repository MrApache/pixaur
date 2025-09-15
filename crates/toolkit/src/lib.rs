#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]

#[cfg(feature = "derive")]
pub use toolkit_derive::*;

pub mod headless;

mod content;
mod debug;
mod error;
mod rendering;
pub mod types;

pub use content::*;
pub use fontdue;
pub use glam;

pub mod widget;
pub mod window;

use crate::{
    rendering::{commands::CommandBuffer, Gpu, Renderer},
    types::Rect,
    widget::FrameContext,
    window::{Window, WindowPointer, WindowRequest},
};
pub use error::*;
use glam::Vec2;
pub use rendering::commands;
use std::{ffi::c_void, ptr::NonNull, sync::Arc};
use wayland_client::{Connection, EventQueue, Proxy};
pub use wl_client::window::TargetMonitor;
use wl_client::{window::WindowLayer, WlClient};
pub use wl_client::{
    window::{DesktopOptions, SpecialOptions},
    Anchor,
};

pub struct EventLoop<G: GUI> {
    gui: G,
    content: ContentManager,

    client: WlClient,
    event_queue: EventQueue<WlClient>,
    display_ptr: NonNull<c_void>,

    gpu: Gpu,
}

impl<G: GUI> EventLoop<G> {
    pub fn new(app: G) -> Result<Self, Error> {
        let conn = Connection::connect_to_env()?;

        let display = conn.display();
        let mut event_queue = conn.new_event_queue();
        let qh = event_queue.handle();

        let _registry = display.get_registry(&qh, Arc::new(String::new()));

        let mut client = WlClient::default();

        event_queue.roundtrip(&mut client)?; //Register objects
        event_queue.roundtrip(&mut client)?; //Register outputs

        let content = ContentManager::default();

        //Fix egl error: BadDisplay
        let (display_ptr, gpu) = {
            let display_ptr = NonNull::new(display.id().as_ptr().cast::<c_void>())
                .ok_or(Error::DisplayNullPointer)?;
            let dummy = client.create_window_backend(qh, "dummy", 1, 1, WindowLayer::default());
            event_queue.roundtrip(&mut client)?; //Init dummy

            let dummy_ptr = dummy
                .lock()
                .map_err(|e| Error::LockFailed(e.to_string()))?
                .as_ptr();
            let ptr = WindowPointer::new(display_ptr, dummy_ptr);
            let gpu = Gpu::new(ptr)?;

            drop(dummy);

            client.destroy_window_backend("dummy");
            event_queue.roundtrip(&mut client)?; //Destroy dummy

            (display_ptr, gpu)
        };

        Ok(Self {
            gui: app,
            content,

            client,
            event_queue,
            display_ptr,

            gpu,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        self.gui.load_content(&mut self.content);
        let mut windows = self.init_windows_backends()?;

        let mut frame_ctx = FrameContext::default();

        loop {
            frame_ctx.position = self.client.pointer().position();
            frame_ctx.buttons = self.client.pointer().buttons();

            self.content.dispath_queue(&self.gpu)?;

            windows
                .iter_mut()
                .try_for_each(|window| -> Result<(), Error> {
                    let mut backend = window
                        .backend
                        .lock()
                        .map_err(|e| Error::LockFailed(e.to_string()))?;
                    if backend.can_resize() {
                        window.configuration.width = backend
                            .width
                            .try_into()
                            .map_err(|_| Error::NegativeWidth(backend.width))?;

                        window.configuration.height = backend
                            .height
                            .try_into()
                            .map_err(|_| Error::NegativeHeight(backend.height))?;

                        self.gpu
                            .confugure_surface(&window.surface, &window.configuration);
                        backend.set_resized();
                    }

                    window.frontend.update(&mut self.gui, &frame_ctx);

                    backend.frame();
                    if !backend.can_draw() {
                        return Ok(());
                    }

                    let mut commands = CommandBuffer::default();
                    window.frontend.layout(Rect::new(
                        Vec2::ZERO,
                        Vec2::new(
                            window.configuration.width as f32,
                            window.configuration.height as f32,
                        ),
                    ));
                    window.frontend.draw(&mut commands);
                    commands.pack_active_group();
                    window.renderer.render(
                        &self.gpu,
                        &window.surface,
                        &mut commands,
                        &self.content,
                        window.configuration.width as f32,
                        window.configuration.height as f32,
                    )?;
                    backend.commit();

                    Ok(())
                })?;
            self.event_queue.blocking_dispatch(&mut self.client)?;
        }
    }

    fn init_windows_backends(&mut self) -> Result<Vec<Window<G>>, Error> {
        let user_windows = self.gui.setup_windows();
        let mut backends = Vec::with_capacity(user_windows.len());
        let qh = self.event_queue.handle();

        user_windows.into_iter().try_for_each(|mut frontend| {
            let request = frontend.request();
            let backend = self.client.create_window_backend(
                qh.clone(),
                request.id,
                request.width,
                request.height,
                request.layer,
            );

            let (width, height, surface_ptr) = {
                let guard = backend.lock().unwrap();

                let width: u32 = guard.width.try_into().expect("width must be >= 0");
                let height: u32 = guard.height.try_into().expect("height must be >= 0");
                (width, height, guard.as_ptr())
            };

            let window_ptr = WindowPointer::new(self.display_ptr, surface_ptr);
            let (surface, configuration) = self.gpu.create_surface(window_ptr, width, height)?;
            let renderer = Renderer::new(&self.gpu, None, &surface)?;
            frontend.setup(&mut self.gui);
            let window = Window::new(frontend, backend, surface, configuration, renderer);

            backends.push(window);

            Ok::<(), Error>(())
        })?;

        Ok(backends)
    }
}

#[allow(unused)]
pub trait GUI {
    type Window: WindowRoot<Gui = Self>;
    fn load_content(&mut self, content: &mut ContentManager) {}
    fn setup_windows(&mut self) -> Vec<Self::Window>;
}

pub trait WindowRoot {
    type Gui: GUI<Window = Self>;

    fn request(&self) -> WindowRequest;
    fn setup(&mut self, gui: &mut Self::Gui);
    fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>);
    fn layout(&mut self, bounds: Rect);
    fn update(&mut self, gui: &mut Self::Gui, ctx: &FrameContext);
}

/*

pub struct Context<'a, W: WindowRoot> {
    root: &'a mut W,
}
impl<'a, W: Widget, R: Container<W>> Context<'a, W, R> {
    fn internal_get_by_id(container: &'a R, id: &str) -> Option<&'a W> {
        for w in container.children() {
            if let Some(w_id) = w.id() {
                if w_id.eq(id) {
                    return w.as_any().downcast_ref::<W>();
                }

                if let Some(container) = w.as_container() {
                    return Self::internal_get_by_id(container, id);
                }
            }
        }

        None
    }

    fn internal_get_mut_by_id(
        container: &'a mut R,
        id: &str,
    ) -> Option<&'a mut W> {
        for w in container.children_mut() {
            if let Some(w_id) = w.id() {
                if w_id.eq(id) {
                    return w.as_any_mut().downcast_mut::<W>();
                }

                if let Some(container) = w.as_container_mut() {
                    return Self::internal_get_mut_by_id(container, id);
                }
            }
        }

        None
    }

    #[must_use]
    pub fn get_by_id(&'a self, id: &str) -> Option<&'a W> {
        Self::internal_get_by_id(self.root.as_ref(), id)
    }

    pub fn get_mut_by_id(&'a mut self, id: &str) -> Option<&'a mut W> {
        Self::internal_get_mut_by_id(self.root.as_mut(), id)
    }
}

*/
