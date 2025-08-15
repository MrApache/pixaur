mod app;
pub mod components;
mod content;
mod debug;
mod ecs;
mod ecs_rendering;
mod error;
mod rendering;

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;
use bevy_ecs::system::Commands;
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

use crate::window::{Window, WindowRequest};

use std::collections::HashMap;
use std::collections::HashSet;
use std::{ffi::c_void, ptr::NonNull};
use wl_client::WlClient;

//Load content
//Resize
//Set root size
//Layout(Update)
//Draw
//Sort
//Render
//Commit
//Command buffers cleanup
//Window event queue

pub trait UserWindow: Send + Sync + 'static {
    fn request(&self) -> WindowRequest;
    fn setup(&self, commands: &mut Commands, window_id: WindowId);
}

#[derive(Resource)]
pub struct Windows {
    handle: QueueHandle<WlClient>,
    active: HashMap<WindowId, Window>,
    not_initalized: Vec<Box<dyn UserWindow>>,
}

impl Windows {
    pub(crate) fn can_draw(&self, id: &WindowId) -> bool {
        self.active.get(id).unwrap().can_draw
    }
}

#[derive(Resource)]
pub(crate) struct Client {
    pub inner: WlClient,
    pub display_ptr: NonNull<c_void>,
}

unsafe impl Sync for Client {}
unsafe impl Send for Client {}

#[derive(Component, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct WindowId(pub(crate) u16);
