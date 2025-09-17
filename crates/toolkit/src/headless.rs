use crate::{
    app::App,
    widget::{Context, FrameContext, Widget},
    WindowRoot,
};

pub struct HeadlessEventLoop<C, W, WR>
where
    C: Context<Widget = W, WindowRoot = WR>,
    W: Widget<C>,
    WR: WindowRoot<C, W>,
{
    app: App<C, W, WR>,
}

impl<C, W, WR> HeadlessEventLoop<C, W, WR>
where
    C: Context<Widget = W, WindowRoot = WR>,
    W: Widget<C>,
    WR: WindowRoot<C, W>,
{
    #[must_use]
    pub fn new(mut app: App<C, W, WR>) -> Self {
        let mut windows = std::mem::take(&mut app.requested_frontends);
        windows.iter_mut().for_each(|f| f.setup(&mut app));
        app.frontends = windows;
        Self { app }
    }

    pub fn run_logic(&mut self) {
        let frame = FrameContext::default();
        for i in 0..self.app.frontends.len() {
            self.app.tick_logic_frontend(i, 1920.0, 1080.0, &frame);
        }
    }

    pub fn run_draw(&mut self) {
        for i in 0..self.app.frontends.len() {
            self.app.tick_render_frontend(i);
        }
    }
}
