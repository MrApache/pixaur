use glam::Vec2;

use crate::{commands::CommandBuffer, types::Rect, widget::{Container, Widget}, Context, UserWindow, GUI};

struct HeadlessWindow<W: Widget, R: Container<W>, T: GUI<W, R>> {
    frontend: R,
    handle: Box<dyn UserWindow<W, R, T>>,
}

pub struct HeadlessEventLoop<W: Widget, R: Container<W>, T: GUI<W, R>> {
    gui: T,
    windows: Vec<HeadlessWindow<W, R, T>>,
}

impl<W: Widget, R: Container<W>, T: GUI<W, R>> HeadlessEventLoop<W, R, T> {
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
                _phantom: std::marker::PhantomData,
            };
            window.handle.update(&mut self.gui, &mut context);

            window
                .frontend
                .layout(Rect::new(Vec2::ZERO, Vec2::new(1920.0, 1080.0)));
        }
    }

    pub fn run_draw(&mut self) {
        for window in &mut self.windows {
            let mut commands = CommandBuffer::default();
            window.frontend.draw(&mut commands);
            commands.pack_active_group();
        }
    }
}
