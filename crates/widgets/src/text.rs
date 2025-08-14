use bevy_ecs::{
    prelude::Component,
    query::{Changed, Or},
    system::Query,
};
use toolkit::{
    fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle},
    glam::Vec2,
    types::*,
    widget::DesiredSize,
    FontHandle, Transform,
};
use toolkit_macros::define_widget;

#[derive(Component)]
pub struct Text {
    font: FontHandle,
    layout: Layout,
    value: String,
    size: u32,
}

impl Default for Text {
    fn default() -> Self {
        let mut instance = Self {
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

    pub fn set_size(&mut self, value: u32) {
        self.size = value;
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

pub fn text_layout(
    mut query: Query<(&mut Text, &Transform), Or<(Changed<Text>, Changed<Transform>)>>,
) {
    query.iter_mut().for_each(|(mut text, transform)| {
        text.layout.reset(&LayoutSettings {
            max_width: Some(transform.size.x),
            max_height: Some(transform.size.y),
            ..LayoutSettings::default()
        });

        text.refresh_layout();
    });
}

pub fn text_desired_size(mut query: Query<(&Text, &mut DesiredSize), Changed<Text>>) {
    query.iter_mut().for_each(|(text, mut desired_size)| {
        let font = text.font.as_ref();
        let mut x = 0.0;

        text.layout.glyphs().iter().for_each(|c| {
            let metrics = font.metrics(c.parent, text.size as f32);
            x += metrics.advance_width;
            x += metrics.bounds.xmin;
        });

        let y = text.layout.height();
        *desired_size = DesiredSize::Min(Vec2::new(x.floor(), y));
    });
}

define_widget! {
    Text,
    Color
}
