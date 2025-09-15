use toolkit::{
    commands::{DrawRectCommand, DrawTextureCommand},
    types::{styling::BackgroundStyle, Rect, Stroke},
    widget::{DesiredSize, Widget},
};

#[derive(Default)]
pub struct Rectangle {
    pub background: BackgroundStyle,
    pub stroke: Stroke,

    id: Option<String>,
    rect: Rect,
}

impl Rectangle {
    #[must_use]
    pub const fn rect(&self) -> &Rect {
        &self.rect
    }
}

impl Widget for Rectangle {
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

    fn update(&mut self, _: &toolkit::widget::FrameContext) {}
}
