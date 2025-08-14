mod app;
pub mod components;
mod content;
mod debug;
mod ecs;
mod ecs_rendering;
mod error;
mod rendering;

use bevy_ecs::resource::Resource;
use bevy_ecs::system::Commands;
pub use ecs::Monitor;
pub use ecs::WidgetBundle;

pub use app::*;
pub mod types;

pub use content::*;
pub use fontdue;
pub use glam;

pub mod widget;
pub mod window;

pub use rendering::commands;

pub use error::*;
use wayland_client::QueueHandle;
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
    widget::Plugin,
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
    pub fn run(&mut self) -> Result<(), Error> {
        self.gui.load_content(&mut self.content);
        let mut windows = self.init_windows_backends()?;
        let mut counter = FpsCounter::new(144);

        loop {
            //self.content.dispath_queue(&self.gpu)?;

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
                    //window.renderer.render(
                    //    &self.gpu,
                    //    &window.surface,
                    //    &mut commands,
                    //    &self.content,
                    //    window.configuration.width as f32,
                    //    window.configuration.height as f32,
                    //)?;
                    backend.commit();

                    Ok(())
                })?;
            self.event_queue.blocking_dispatch(&mut self.client)?;
        }
    }

    fn init_windows_backends(&mut self) -> Result<Vec<Window>, Error> {
        Ok(vec![])
    }
}

pub trait UserWindow: Send + Sync + 'static {
    fn request(&self) -> WindowRequest;
    fn setup(&self, commands: &mut Commands);
}

#[derive(Resource)]
pub struct Windows {
    handle: QueueHandle<WlClient>,
    active: Vec<Window>,
    not_initalized: Vec<Box<dyn UserWindow>>,
}

#[derive(Resource)]
pub(crate) struct Client {
    pub inner: WlClient,
    pub display_ptr: NonNull<c_void>,
}

unsafe impl Sync for Client {}
unsafe impl Send for Client {}
