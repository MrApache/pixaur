use bevy_ecs::{prelude::Component, query::{Changed, Or}, system::Query};
use toolkit::{
    commands::CommandBuffer,
    fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle},
    glam::Vec2,
    types::*,
    widget::{DesiredSize, Widget},
    FontHandle, Transform,
};
use toolkit_macros::define_widget;

#[derive(Component)]
pub struct Text {
    id: Option<String>,
    font: FontHandle,

    value: String,
    pub size: u32,

    layout: Layout,
}

impl Default for Text {
    fn default() -> Self {
        let mut instance = Self {
            id: None,
            value: String::new(),
            font: FontHandle::default(),
            size: 12,
            layout: Layout::new(CoordinateSystem::PositiveYDown),
        };

        instance.refresh_layout();
        instance
    }
}

impl Text {
    pub fn new(font: FontHandle) -> Self {
        let mut instance = Self {
            id: None,
            value: String::new(),
            font,
            size: 12,
            layout: Layout::new(CoordinateSystem::PositiveYDown),
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
            layout: Layout::new(CoordinateSystem::PositiveYDown),
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

    #[allow(clippy::eq_op)]
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
                .fold(0.0 / 0.0, |m, v| v.max(m)),
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

    fn draw<'frame>(&'frame self, _out: &mut CommandBuffer<'frame>) {}

    fn layout(&mut self, bounds: Rect) {
    }
}

pub fn text_layout(mut query: Query<(&mut Text, &Transform), Or<(Changed<Text>, Changed<Transform>)>>) {
    query.iter_mut().for_each(|(mut text, transform)| {
        text.layout.reset(&LayoutSettings {
            max_width: Some(transform.size.x),
            max_height: Some(transform.size.y),
            ..LayoutSettings::default()
        });

        text.refresh_layout();
    });
}

define_widget! {
    Text,
    Color
}
