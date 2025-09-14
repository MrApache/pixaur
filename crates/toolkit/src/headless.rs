use crate::{commands::CommandBuffer, types::Rect, WindowRoot, GUI};
use glam::Vec2;

struct HeadlessWindow<G: GUI> {
    frontend: G::Window,
    _phantom: std::marker::PhantomData<G>,
}

pub struct HeadlessEventLoop<G: GUI> {
    gui: G,
    windows: Vec<HeadlessWindow<G>>,
}

impl<G: GUI> HeadlessEventLoop<G> {
    pub fn new(mut app: G) -> Self {
        let windows = app
            .setup_windows()
            .into_iter()
            .map(|frontend| HeadlessWindow {
                frontend,
                _phantom: std::marker::PhantomData,
            })
            .collect::<Vec<_>>();
        Self { gui: app, windows }
    }

    pub fn run_logic(&mut self) {
        for window in &mut self.windows {
            window.frontend.update(&mut self.gui);
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
