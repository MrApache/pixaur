use toolkit::{
    commands::{CommandBuffer, DrawCommand, DrawTextCommand},
    fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle},
    glam::Vec2,
    types::{Argb8888, Color, Rect},
    widget::{DesiredSize, Widget},
    FontHandle,
};

pub struct Text {
    id: Option<String>,
    font: FontHandle,

    value: String,
    pub size: u32,
    pub color: Color,

    layout: Layout,
    position: Vec2,
}

impl Default for Text {
    fn default() -> Self {
        let mut instance = Self {
            id: None,
            value: String::new(),
            font: FontHandle::default(),
            size: 12,
            color: Color::Simple(Argb8888::WHITE),
            layout: Layout::new(CoordinateSystem::PositiveYDown),
            position: Vec2::ZERO,
        };

        instance.refresh_layout();
        instance
    }
}

impl Text {
    #[must_use]
    pub fn new(font: FontHandle) -> Self {
        let mut instance = Self {
            id: None,
            value: String::new(),
            font,
            size: 12,
            color: Color::Simple(Argb8888::WHITE),
            layout: Layout::new(CoordinateSystem::PositiveYDown),
            position: Vec2::ZERO,
        };

        instance.refresh_layout();
        instance
    }

    pub fn with_id(font: FontHandle, id: impl Into<String>) -> Self {
        let mut instance = Self {
            id: Some(id.into()),
            value: String::new(),
            font,
            size: 12,
            color: Color::Simple(Argb8888::WHITE),
            layout: Layout::new(CoordinateSystem::PositiveYDown),
            position: Vec2::ZERO,
        };

        instance.refresh_layout();
        instance
    }

    pub fn set_text(&mut self, value: &str) {
        self.value.clear();
        self.value.insert_str(0, value);
        self.refresh_layout();
    }

    fn refresh_layout(&mut self) {
        self.layout.clear();
        self.layout.append(
            &[self.font.as_ref()],
            &TextStyle {
                text: &self.value,
                px: self.size as f32,
                font_index: 0,
                user_data: (),
            },
        );
    }
}

impl Widget for Text {
    fn id(&self) -> Option<&str> {
        if let Some(id) = &self.id {
            Some(id)
        } else {
            None
        }
    }

    fn desired_size(&self) -> DesiredSize {
        let font = self.font.as_ref();
        let mut x = 0.0;

        let glyphs = self.layout.glyphs();
        let text_width = match self.layout.lines() {
            Some(lines) => lines
                .iter()
                .map(|ln| {
                    let glyph = &glyphs[ln.glyph_end];
                    glyph.x + glyph.width as f32
                })
                .fold(f32::NAN, |m, v| v.max(m)),
            None => 0.0,
        };

        self.layout.glyphs().iter().for_each(|c| {
            let metrics = font.metrics(c.parent, self.size as f32);
            x += metrics.advance_width;
            x += metrics.bounds.xmin;
        });

        let y = self.layout.height();
        DesiredSize::Min(Vec2::new(x.floor(), y))
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

        self.refresh_layout();

        self.position = bounds.min;
    }
}
