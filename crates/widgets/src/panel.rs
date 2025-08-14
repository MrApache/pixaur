use toolkit_macros::define_widget;
use std::slice::Iter;
use bevy_ecs::{
    change_detection::DetectChangesMut,
    component::Component,
    entity::Entity,
    hierarchy::Children,
    query::{Changed, Or},
    system::Query,
};
use toolkit::{
    commands::{CommandBuffer, DrawCommand, DrawRectCommand},
    glam::{Vec2, Vec4},
    types::*,
    widget::{DesiredSize, Widget},
    Transform,
};

#[derive(Copy, Clone, Debug, Default)]
pub enum LayoutMode {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Copy, Clone, Debug, Default)]
pub enum HorizontalAlign {
    #[default]
    Start,
    Center,
    End,
}

#[derive(Copy, Clone, Debug, Default)]
pub enum VerticalAlign {
    #[default]
    Start,
    Center,
    End,
}

#[derive(Component)]
pub struct Panel {
    pub padding: Vec4,
    pub spacing: f32,
    pub mode: LayoutMode,
    pub vertical_align: VerticalAlign,
    pub horizontal_align: HorizontalAlign,
}

impl Default for Panel {
    fn default() -> Self {
        Self::new()
    }
}

impl Panel {
    pub fn new() -> Self {
        Self {
            padding: Vec4::new(4.0, 4.0, 4.0, 4.0),
            spacing: 4.0,
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Center,
            mode: LayoutMode::Vertical,
        }
    }

    fn layout(
        &mut self,
        transform: &mut Transform,
        stroke: &Stroke,
        content: Iter<'_, Entity>,
        transforms: &mut Query<(&mut Transform, &DesiredSize)>,
    ) {
        transform.position.x += stroke.width;
        transform.position.y += stroke.width;
        transform.size.x -= stroke.width;
        transform.size.y -= stroke.width;

        // Учитываем padding с обеих сторон для вычисления внутренних границ
        let min_x = transform.position.x + self.padding.x; // left
        let min_y = transform.position.y + self.padding.y; // bottom
        let max_x = transform.size.x - self.padding.z; // right
        let max_y = transform.size.y - self.padding.w; // top

        let len = content.len();
        let available_width = max_x;
        let available_height = max_y - min_y;

        let mut cursor_x = match self.horizontal_align {
            HorizontalAlign::Start => min_x,
            HorizontalAlign::Center => min_x + available_width / 2.0,
            HorizontalAlign::End => max_x - available_width,
        };

        let mut total_min_width = 0.0;
        let mut fill_count = 0;

        content.clone().for_each(|entity| {
            let (_, desired_size) = transforms.get_mut(*entity).unwrap();
            match desired_size {
                DesiredSize::Min(size) => total_min_width += size.x,
                DesiredSize::Fill => fill_count += 1,
                DesiredSize::FillMinY(_) => fill_count += 1,
            }
        });

        let total_spacing = self.spacing * len.saturating_sub(1) as f32;
        let total_available_width = max_x - total_spacing - total_min_width - self.padding.z;
        let fill_width = total_available_width / fill_count as f32;

        for (i, entity) in content.enumerate() {
            let (mut child, desired_size) = transforms.get_mut(*entity).unwrap();

            let (width, height) = match desired_size {
                DesiredSize::Min(vec2) => (vec2.x, vec2.y.min(available_height)),
                DesiredSize::Fill => (fill_width, available_height),
                DesiredSize::FillMinY(y) => (fill_width, y.min(available_height)),
            };

            let offset_y = match self.vertical_align {
                VerticalAlign::Start => 0.0,
                VerticalAlign::Center => (available_height - height) / 2.0,
                VerticalAlign::End => available_height - height,
            };

            let offset_x = match self.horizontal_align {
                HorizontalAlign::Start => 0.0,
                HorizontalAlign::Center => width - (width / 2.0),
                HorizontalAlign::End => 0.0,
            };

            let position = Vec2::new(cursor_x - offset_x, min_y + offset_y);
            let size = Vec2::new(width, height);

            child.position = position;
            child.size = size;

            cursor_x += width;

            if i != len - 1 {
                cursor_x += self.spacing;
            }

            if cursor_x >= max_x {
                break;
            }
        }
    }
}

pub fn panel_layout(
    mut query: Query<
        (&mut Panel, &mut Transform, &Stroke, Option<&Children>),
        Or<(Changed<Panel>, Changed<Transform>, Changed<Children>)>,
    >,
    mut widgets: Query<(&mut Transform, &DesiredSize)>,
) {
    query
        .iter_mut()
        .for_each(|(mut panel, mut transform, stroke, content)| {
            let content = if let Some(content) = content {
                content.iter()
            } else {
                Default::default()
            };

            panel.layout(
                transform.bypass_change_detection(),
                stroke,
                content,
                &mut widgets,
            );
        });
}

define_widget! {
    Panel,
    Color,
    Texture,
}

#[derive(Default)]
pub struct TestPanelLayoutWidget {
    pub min: Vec2,
    rect: Rect,
    pub stroke: Stroke,
}

impl Widget for TestPanelLayoutWidget {
    fn id(&self) -> Option<&str> {
        Some("test_widget")
    }

    fn desired_size(&self) -> DesiredSize {
        DesiredSize::Min(self.min)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>) {
        out.push(DrawCommand::Rect(DrawRectCommand::new(
            self.rect.clone(),
            Argb8888::CYAN,
            self.stroke.clone(),
        )));
    }

    fn layout(&mut self, bounds: Rect) {
        self.rect = bounds;
    }
}
