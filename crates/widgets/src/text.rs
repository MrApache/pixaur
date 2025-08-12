use toolkit::{
    fontdue::layout::{
        Layout,
        LayoutSettings,
        TextStyle,
    },
    glam::Vec2,
        widget::{
            DesiredSize,
            Widget
    },
    Color,
    Argb8888,
    DrawCommand,
    FontHandle
};

pub struct Text {
    id: String,
    font: FontHandle,

    pub value: String,
    pub size: u32,
    pub color: Color,

    layout: Layout,
    position: Vec2,
}

impl Text {
    pub fn new(font: FontHandle, id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            value: String::new(),
            font,
            size: 12,
            color: Color::Simple(Argb8888::WHITE),
            layout: Layout::new(toolkit::fontdue::layout::CoordinateSystem::PositiveYDown),
            position: Vec2::ZERO,
        }
    }
}

impl Widget for Text {
    fn id(&self) -> &str {
        &self.id
    }

    fn desired_size(&self) -> DesiredSize {
        DesiredSize::Fill
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw<'frame>(&'frame self, out: &mut toolkit::CommandBuffer<'frame>) {
        out.push(DrawCommand::Text {
            font: &self.font,
            size: self.size,
            color: self.color.clone(),
            position: self.position,
            layout: &self.layout,
        });
    }

    fn layout(&mut self, bounds: toolkit::widget::Rect) {
        self.layout.reset(&LayoutSettings {
            max_width: Some(bounds.max.x),
            max_height: Some(bounds.max.y),
            ..LayoutSettings::default()
        });

        self.layout.append(&[self.font.as_ref()], &TextStyle {
            text: &self.value,
            px: self.size as f32,
            font_index: 0,
            user_data: (),
        });

        self.position = bounds.min;
    }
}
