use toolkit::{
    commands::{CommandBuffer, DrawCommand, DrawRectCommand, DrawTextCommand},
    fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle},
    types::{Argb8888, Color, Rect, Stroke},
    widget::{DesiredSize, Widget},
    glam::Vec2,
    FontHandle,
};

pub struct Text {
    pub size: u32,
    pub color: Color,
    font: FontHandle,

    id: Option<String>,
    value: String,
    layout: Layout,
    //position: Vec2,
    rect: Rect,
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}

impl Text {
    #[must_use]
    pub fn new() -> Self {
        Self::new_with_id(None)
    }

    #[must_use]
    pub fn with_id(id: impl Into<String>) -> Self {
        Self::new_with_id(Some(id.into()))
    }

    fn new_with_id(id: Option<String>) -> Self {
        let mut instance = Self {
            id,
            value: String::new(),
            font: FontHandle::default(),
            size: 12,
            color: Argb8888::WHITE.into(),
            layout: Layout::new(CoordinateSystem::PositiveYDown),
            rect: Rect::ZERO,
            //position: Vec2::ZERO,
        };

        instance.refresh_layout();
        instance
    }

    pub fn set_font(&mut self, font: FontHandle) {
        self.font = font;
        self.refresh_layout();
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
        self.id.as_deref()
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
        DesiredSize::Min(Vec2::new(text_width, y))
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
            self.rect.min,
            &self.font,
            &self.layout,
        )));
    }

    fn layout(&mut self, bounds: Rect) {
        self.layout.reset(&LayoutSettings {
            max_width: Some(bounds.max.x + bounds.min.x),
            max_height: Some(bounds.max.y),
            ..LayoutSettings::default()
        });

        self.refresh_layout();

        self.rect = bounds;
    }

    fn update(&mut self, _: &toolkit::widget::FrameContext) {}
}
