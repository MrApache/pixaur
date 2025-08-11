use toolkit::{
    glam::Vec2, widget::{DesiredSize, Widget}, Argb8888, Color, DrawCommand, DEFAULT_FONT
};

pub struct Text {
    id: String,
    font: String,

    pub value: String,
    pub size: f32,
    pub color: Color,
}

impl Text {
    pub fn empty(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            value: String::new(),
            font: DEFAULT_FONT.to_string(),
            size: 12.0,
            color: Color::Simple(Argb8888::WHITE),
        }
    }

    pub fn new(value: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
            font: DEFAULT_FONT.to_string(),
            size: 12.0,
            color: Color::Simple(Argb8888::WHITE),
        }
    }
}

impl Widget for Text {
    fn id(&self) -> &str {
        &self.id
    }

    fn desired_size(&self) -> DesiredSize {
        DesiredSize::Min(Vec2::default())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw<'frame>(&'frame self, out: &mut toolkit::CommandBuffer<'frame>) {
        out.push(DrawCommand::Text {
            text: &self.value,
            font: &self.font,
            size: self.size,
            color: Color::Simple(Argb8888::RED),
        });
    }

    fn layout(&mut self, bounds: toolkit::widget::Rect) {
    }
}
