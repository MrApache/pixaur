use glam::Vec2;

use crate::{commands::CommandBuffer, types::Rect, widget::Container, Context, UserWindow, GUI};

struct HeadlessWindow<T: GUI> {
    frontend: Box<dyn Container>,
    handle: Box<dyn UserWindow<T>>,
}

pub struct HeadlessEventLoop<T: GUI> {
    gui: T,
    windows: Vec<HeadlessWindow<T>>,
}

impl<T: GUI> HeadlessEventLoop<T> {
    pub fn new(mut app: T) -> Self {
        let windows = app
            .setup_windows()
            .into_iter()
            .map(|w| HeadlessWindow {
                frontend: w.setup(&mut app),
                handle: w,
            })
            .collect::<Vec<_>>();
        Self { gui: app, windows }
    }

    pub fn run_logic(&mut self) {
        for window in &mut self.windows {
            let mut context = Context {
                root: &mut window.frontend,
            };

            window.handle.update(&mut self.gui, &mut context);
        }
    }

    pub fn run_draw(&mut self) {
        for window in &mut self.windows {
            let mut commands = CommandBuffer::default();
            window
                .frontend
                .layout(Rect::new(Vec2::ZERO, Vec2::new(1920.0, 1080.0)));
            window.frontend.draw(&mut commands);
            commands.pack_active_group();
        }
    }
}
