use crate::{
    commands::CommandBuffer,
    types::Rect,
    widget::{Context, FrameContext, Sender, Widget},
    WindowRoot, GUI,
};
use glam::Vec2;

struct HeadlessWindow<CTX: Context, W: Widget<CTX>, G: GUI<CTX, W>> {
    frontend: G::Window,
    _phantom: std::marker::PhantomData<G>,
}

pub struct HeadlessEventLoop<CTX: Context, W: Widget<CTX>, G: GUI<CTX, W>> {
    gui: G,
    windows: Vec<HeadlessWindow<CTX, W, G>>,
}

impl<CTX: Context, W: Widget<CTX>, G: GUI<CTX, W>> HeadlessEventLoop<CTX, W, G> {
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
        let frame_context = FrameContext::default();
        for window in &mut self.windows {
            let root = window.frontend.root();
            let mut sender = Sender::<CTX>::default();
            root.update(&frame_context, &mut sender);
            root.layout(Rect::new(Vec2::ZERO, Vec2::new(1920.0, 1080.0)));
        }
    }

    pub fn run_draw(&mut self) {
        for window in &mut self.windows {
            let mut commands = CommandBuffer::default();
            window.frontend.root().draw(&mut commands);
            commands.pack_active_group();
        }
    }
}
