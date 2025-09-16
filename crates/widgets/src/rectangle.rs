use toolkit::{
    commands::{DrawRectCommand, DrawTextureCommand},
    types::{styling::BackgroundStyle, Rect, Stroke},
    widget::{Context, DesiredSize, Sender, Widget},
};

pub struct Rectangle<Ctx: Context> {
    pub background: BackgroundStyle,
    pub stroke: Stroke,

    id: Option<String>,
    rect: Rect,
    _phantom: std::marker::PhantomData<Ctx>,
}

impl<Ctx: Context> Default for Rectangle<Ctx> {
    fn default() -> Self {
        Self {
            background: BackgroundStyle::default(),
            stroke: Stroke::default(),
            id: None,
            rect: Rect::default(),
            _phantom: std::marker::PhantomData
        }
    }
}

impl<Ctx: Context> Rectangle<Ctx> {
    #[must_use]
    pub const fn rect(&self) -> &Rect {
        &self.rect
    }
}

impl<Ctx: Context> Widget<Ctx> for Rectangle<Ctx> {
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn desired_size(&self) -> toolkit::widget::DesiredSize {
        DesiredSize::Fill
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw<'frame>(&'frame self, out: &mut toolkit::commands::CommandBuffer<'frame>) {
        match &self.background {
            BackgroundStyle::Color(color) => out.push(DrawRectCommand::new(
                self.rect.clone(),
                color.clone(),
                self.stroke.clone(),
            )),
            BackgroundStyle::Texture(texture) => out.push(DrawTextureCommand::new(
                self.rect.clone(),
                texture.clone(),
                self.stroke.clone(),
            )),
        }
    }

    fn layout(&mut self, bounds: Rect) {
        self.rect = bounds;
    }

    fn update(&mut self, _: &toolkit::widget::FrameContext, _: &mut Sender<Ctx>) {}
}
