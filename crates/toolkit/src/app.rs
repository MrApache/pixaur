use crate::{
    commands::CommandBuffer,
    rendering::Gpu,
    types::Rect,
    widget::{Context, FrameContext, Sender, Tree, Widget},
    ContentManager, Error, WindowRoot,
};
use glam::Vec2;

pub struct App<C, W, WR>
where
    C: Context<Widget = W, WindowRoot = WR>,
    W: Widget<C>,
    WR: WindowRoot<C, W>,
{
    pub(crate) frontends: Vec<WR>,
    pub(crate) requested_frontends: Vec<WR>,

    content: ContentManager,

    _phantom0: std::marker::PhantomData<C>,
    _phantom1: std::marker::PhantomData<W>,
}

impl<C, W, WR> Default for App<C, W, WR>
where
    C: Context<Widget = W, WindowRoot = WR>,
    W: Widget<C>,
    WR: WindowRoot<C, W>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C, W, WR> App<C, W, WR>
where
    C: Context<Widget = W, WindowRoot = WR>,
    W: Widget<C>,
    WR: WindowRoot<C, W>,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            frontends: vec![],
            requested_frontends: vec![],
            content: ContentManager::default(),
            _phantom0: std::marker::PhantomData,
            _phantom1: std::marker::PhantomData,
        }
    }

    pub(crate) fn dispatch_queue(&mut self, gpu: &Gpu) -> Result<(), Error> {
        self.content.dispatch_queue(gpu)
    }

    pub fn content_manager(&mut self) -> &mut ContentManager {
        &mut self.content
    }

    pub fn add_window(&mut self, mut window: WR) {
        window.setup(self);
        self.requested_frontends.push(window);
    }

    pub(crate) fn tick_logic_frontend(
        &mut self,
        index: usize,
        window_width: f32,
        window_height: f32,
        frame: &FrameContext,
    ) {
        let frontend = &mut self.frontends[index];
        let root = frontend.root_mut();
        let mut sender = Sender::<C>::default();
        root.update(frame, &mut sender);
        root.layout(Rect::new(
            Vec2::ZERO,
            Vec2::new(window_width, window_height),
        ));

        sender.execute(&mut self.content, Tree {
            frontends: self.frontends.as_mut_slice(),
        });
    }

    pub(crate) fn tick_render_frontend(&mut self, index: usize) -> CommandBuffer {
        let frontend = &mut self.frontends[index];
        let root = frontend.root_mut();
        let mut commands = CommandBuffer::new(&self.content);
        root.draw(&mut commands);
        commands.pack_active_group();
        commands
    }
}
