pub mod components;
mod content;
mod debug;
mod ecs;
mod error;
mod rendering;
mod app;

pub use app::*;
pub mod types;

pub use content::*;
pub use ecs::WidgetBundle;
pub use fontdue;
pub use glam;

pub mod widget;
pub mod window;

pub use rendering::commands;

pub use error::*;
pub use wl_client::window::TargetMonitor;
pub use wl_client::{
    window::{DesktopOptions, SpecialOptions},
    Anchor,
};

pub use ecs::Transform;

use crate::{
    debug::FpsCounter,
    rendering::{commands::CommandBuffer, Gpu, Renderer},
    types::Rect,
    widget::Widget,
    window::{Window, WindowPointer, WindowRequest},
};

use glam::Vec2;
use std::{ffi::c_void, ptr::NonNull, sync::Arc};
use wayland_client::{Connection, EventQueue, Proxy};
use wl_client::{window::WindowLayer, WlClient};

#[allow(unused)]
pub trait GUI {
    fn load_content(&mut self, content: &mut ContentManager) {}
    //fn setup_windows(&mut self) -> Vec<Box<dyn UserWindow<Self>>>;
}

pub struct EventLoop<T: GUI> {
    gui: T,
    content: ContentManager,

    client: WlClient,
    event_queue: EventQueue<WlClient>,
    display_ptr: NonNull<c_void>,

    gpu: Gpu,
}

impl<T: GUI> EventLoop<T> {
    pub fn new(app: T) -> Result<Self, Error> {
        let conn = Connection::connect_to_env()?;

        let display = conn.display();
        let mut event_queue = conn.new_event_queue();
        let qh = event_queue.handle();

        let _registry = display.get_registry(&qh, Arc::new("".to_string()));

        let mut client = WlClient::default();

        event_queue.roundtrip(&mut client)?; //Register objects
        event_queue.roundtrip(&mut client)?; //Register outputs

        let content = ContentManager::default();

        //Fix egl error: BadDisplay
        let (display_ptr, gpu) = {
            let display_ptr = NonNull::new(display.id().as_ptr() as *mut c_void).unwrap();
            let dummy = client.create_window_backend(qh, "dummy", 1, 1, WindowLayer::default());
            event_queue.roundtrip(&mut client)?; //Init dummy

            let dummy_ptr = dummy.lock().unwrap().as_ptr();
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
        let mut counter = FpsCounter::new(144);

        loop {
            self.content.dispath_queue(&self.gpu)?;

            windows
                .iter_mut()
                .try_for_each(|window| -> Result<(), Error> {
                    let mut backend = window.backend.lock().unwrap();
                    if backend.can_resize() {
                        window.configuration.width = backend.width as u32;
                        window.configuration.height = backend.height as u32;
                        self.gpu
                            .confugure_surface(&window.surface, &window.configuration);
                        backend.set_resized();
                    }

                    //let mut context = Context {
                    //    root: &mut window.frontend,
                    //};

                    //window.handle.update(&mut self.gui, &mut context);

                    backend.frame();
                    if !backend.can_draw() {
                        return Ok(());
                    }

                    let fps = counter.tick();
                    println!("FPS: {fps:.1}");

                    let mut commands = CommandBuffer::default();
                    //window.frontend.layout(Rect::new(
                    //    Vec2::ZERO,
                    //    Vec2::new(
                    //        window.configuration.width as f32,
                    //        window.configuration.height as f32,
                    //    ),
                    //));
                    //window.frontend.draw(&mut commands);
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

    fn init_windows_backends(&mut self) -> Result<Vec<Window<T>>, Error> {
        Ok(vec![])
        //let user_windows = self.gui.setup_windows();
        //let mut backends = Vec::with_capacity(user_windows.len());
        //let qh = self.event_queue.handle();

        //user_windows.into_iter().try_for_each(|handle| {
        //    let request = handle.request();
        //    let backend = self.client.create_window_backend(
        //        qh.clone(),
        //        request.id,
        //        request.width,
        //        request.height,
        //        request.layer,
        //    );

        //    let (width, height, surface_ptr) = {
        //        let guard = backend.lock().unwrap();
        //        (guard.width as u32, guard.height as u32, guard.as_ptr())
        //    };

        //    let window_ptr = WindowPointer::new(self.display_ptr, surface_ptr);
        //    let (surface, configuration) = self.gpu.create_surface(window_ptr, width, height)?;
        //    let frontend = handle.setup(&mut self.gui);
        //    let renderer = Renderer::new(&self.gpu, None, &surface)?;
        //    let window = Window::new(frontend, backend, surface, configuration, handle, renderer);

        //    backends.push(window);

        //    Ok::<(), Error>(())
        //})?;

        //Ok(backends)
    }
}

pub trait UserWindow<T: GUI> {
    fn request(&self) -> WindowRequest;
    //fn setup(&self, gui: &mut T) -> Box<dyn Container>;
    //fn update<'ctx>(&mut self, gui: &mut T, context: &'ctx mut Context<'ctx>);
}
