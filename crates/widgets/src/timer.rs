use toolkit::{
    commands::CommandBuffer,
    types::Bounds,
    widget::{Anchor, Callbacks, Context, DesiredSize, FrameContext, Sender, Widget},
    WidgetQuery,
};

#[allow(dead_code, unused_variables)]
pub trait TimerCallback<C: Context>: Callbacks {
    fn on_triggered(&self, sender: &mut Sender<C>) {}
}

#[derive(Default)]
pub struct TimerMock;
impl<C: Context> TimerCallback<C> for TimerMock {}
impl Callbacks for TimerMock {}

#[derive(WidgetQuery)]
pub struct Timer<C: Context, CB: TimerCallback<C>> {
    pub interval: f64,
    pub running: bool,
    pub repeat: bool,

    elapsed_time: f64,
    id: Option<String>,
    callbacks: CB,
    _phantom: std::marker::PhantomData<C>,
}

impl<C: Context, CB: TimerCallback<C>> Default for Timer<C, CB> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Context, CB: TimerCallback<C>> Timer<C, CB> {
    #[must_use]
    pub fn new() -> Self {
        Self::new_with_id(None)
    }

    pub fn with_id(id: impl Into<String>) -> Self {
        Self::new_with_id(Some(id.into()))
    }

    fn new_with_id(id: Option<String>) -> Self {
        Self {
            interval: 0.0,
            running: false,
            repeat: false,
            elapsed_time: f64::MAX,
            id,
            callbacks: CB::default(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<C: Context, CB: TimerCallback<C>> Widget<C> for Timer<C, CB> {
    fn desired_size(&self) -> DesiredSize {
        DesiredSize::Ignore
    }

    fn draw<'frame>(&'frame self, _: &mut CommandBuffer<'frame>) {}

    fn layout(&mut self, _: Bounds) {}

    fn update(&mut self, ctx: &FrameContext, sender: &mut Sender<C>) {
        if !self.running {
            return;
        }

        if self.elapsed_time < self.interval {
            self.elapsed_time += ctx.delta_time();
            return;
        }

        self.elapsed_time = 0.0;
        self.callbacks.on_triggered(sender);

        if !self.repeat {
            self.running = false;
        }
    }

    fn anchor(&self) -> Anchor {
        Anchor::Left
    }
}
