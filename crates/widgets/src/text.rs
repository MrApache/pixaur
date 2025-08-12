use toolkit::{
    FontHandle,
    commands::{CommandBuffer, DrawCommand, DrawTextCommand},
    fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle},
    glam::Vec2,
    types::*,
    widget::{DesiredSize, Widget},
};

pub struct Text {
    id: Option<String>,
    font: FontHandle,

    pub value: String,
    pub size: u32,
    pub color: Color,

    layout: Layout,
    position: Vec2,
}

impl Text {
    pub fn new(font: FontHandle, id: Option<impl Into<String>>) -> Self {
        Self {
            id: id.map(|val| val.into()),
            value: String::new(),
            font,
            size: 12,
            color: Color::Simple(Argb8888::WHITE),
            layout: Layout::new(CoordinateSystem::PositiveYDown),
            position: Vec2::ZERO,
        }
    }
}

impl Widget for Text {
    fn id(&self) -> Option<&str> {
        if let Some(id) = &self.id {
            Some(id)
        }
        else {
            None
        }
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

    fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>) {
        out.push(DrawCommand::Text(DrawTextCommand::new(
            self.size,
            self.color.clone(),
            self.position,
            &self.font,
            &self.layout,
        )));
    }

    fn layout(&mut self, bounds: Rect) {
        self.layout.reset(&LayoutSettings {
            max_width: Some(bounds.max.x),
            max_height: Some(bounds.max.y),
            ..LayoutSettings::default()
        });

        self.layout.append(
            &[self.font.as_ref()],
            &TextStyle {
                text: &self.value,
                px: self.size as f32,
                font_index: 0,
                user_data: (),
            },
        );

        self.position = bounds.min;
    }
}
