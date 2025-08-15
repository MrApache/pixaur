use bevy_ecs::{
    change_detection::DetectChangesMut,
    component::Component,
    entity::Entity,
    hierarchy::Children,
    query::{Changed, Or, With},
    schedule::IntoScheduleConfigs,
    system::{Commands, Query},
};
use std::slice::Iter;
use toolkit::{
    glam::{Vec2, Vec4},
    types::*,
    widget::{DesiredSize, Plugin},
    Transform, Update,
};
use toolkit_macros::define_widget;

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

#[derive(Default, Component)]
struct PanelLayoutCache {
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
    total_min_width: f32,
    fill_count: usize,
    len: usize
}

#[derive(Component)]
#[require(PanelLayoutCache)]
pub struct Panel {
    pub padding: Vec4,
    pub spacing: f32,
    pub mode: LayoutMode,
    pub v_align: VerticalAlign,
    pub h_align: HorizontalAlign,
}

impl Default for Panel {
    fn default() -> Self {
        Self::new(
            Vec4::new(4.0, 4.0, 4.0, 4.0),
            4.0,
            HorizontalAlign::Center,
            VerticalAlign::Center,
            LayoutMode::Vertical,
        )
    }
}

impl Panel {
    pub fn new(
        padding: Vec4,
        spacing: f32,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
        mode: LayoutMode,
    ) -> Self {
        Self {
            padding,
            spacing,
            h_align,
            v_align,
            mode,
        }
    }

    pub fn with_padding(mut self, padding: Vec4) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn with_h_align(mut self, h_align: HorizontalAlign) -> Self {
        self.h_align = h_align;
        self
    }

    pub fn with_v_align(mut self, v_align: VerticalAlign) -> Self {
        self.v_align = v_align;
        self
    }
}

fn setup_panels_transforms(
    mut query: Query<
        (&mut Transform, &Stroke),
        (Or<(Changed<Panel>, Changed<Transform>, Changed<Children>)>, With<Panel>),
    >,
) {
    query
        .iter_mut()
        .for_each(|(mut transform, stroke)| {
            let transform = transform.bypass_change_detection();
            transform.position.x += stroke.width;
            transform.position.y += stroke.width;
            transform.size.x -= stroke.width;
            transform.size.y -= stroke.width;
        });
}

fn compute_panel_layout_cache(
    mut panels: Query<(&mut PanelLayoutCache, &Panel, &Transform, Option<&Children>)>,
    widgets: Query<&DesiredSize>,
) {

    panels.iter_mut().for_each(|(mut cache, panel, transform, children)| {
        let mut total_min_width = 0.0;
        let mut fill_count = 0;
        let mut len = 0;
        if let Some(children) = children {
            for &child in children.iter() {
                len += 1;
                if let Ok(desired_size) = widgets.get(child) {
                    match desired_size {
                        DesiredSize::Min(size) => total_min_width += size.x,
                        DesiredSize::Fill => fill_count += 1,
                        DesiredSize::FillMinY(_) => fill_count += 1,
                    }
                }
            }
        }
        cache.total_min_width = total_min_width;
        cache.fill_count = fill_count;
        cache.len = len;

        // Учитываем padding с обеих сторон для вычисления внутренних границ
        cache.min_x = transform.position.x + panel.padding.x; // left
        cache.min_y = transform.position.y + panel.padding.y; // bottom
        cache.max_x = transform.size.x - panel.padding.z; // right
        cache.max_y = transform.size.y - panel.padding.w; // top
    });
}

fn apply_panel_layout(
    panels: Query<(&Panel, &PanelLayoutCache, Option<&Children>)>,
    mut widgets: Query<(&mut Transform, &DesiredSize)>,
) {
    panels.iter().for_each(|(panel, cache, content)| {
        let available_width = cache.max_x;
        let available_height = cache.max_y - cache.min_y;

        let mut cursor_x = match panel.h_align {
            HorizontalAlign::Start => cache.min_x,
            HorizontalAlign::Center => cache.min_x + available_width / 2.0,
            HorizontalAlign::End => cache.max_x - available_width,
        };

        let total_spacing = panel.spacing * cache.len.saturating_sub(1) as f32;
        let total_available_width = cache.max_x - total_spacing - cache.total_min_width - panel.padding.z;
        let fill_width = total_available_width / cache.fill_count as f32;

        let content = if let Some(content) = content {
            content.iter()
        } else {
            Default::default()
        };

        for (i, entity) in content.enumerate() {
            println!("Update content '{i}'");
            let (mut child, desired_size) = widgets.get_mut(*entity).unwrap();

            let (width, height) = match desired_size {
                DesiredSize::Min(vec2) => (vec2.x, vec2.y.min(available_height)),
                DesiredSize::Fill => (fill_width, available_height),
                DesiredSize::FillMinY(y) => (fill_width, y.min(available_height)),
            };

            let offset_y = match panel.v_align {
                VerticalAlign::Start => 0.0,
                VerticalAlign::Center => (available_height - height) / 2.0,
                VerticalAlign::End => available_height - height,
            };

            let offset_x = match panel.h_align {
                HorizontalAlign::Start => 0.0,
                HorizontalAlign::Center => width - (width / 2.0),
                HorizontalAlign::End => 0.0,
            };

            let position = Vec2::new(cursor_x - offset_x, cache.min_y + offset_y);
            let size = Vec2::new(width, height);

            child.position = position;
            child.size = size;

            cursor_x += width;

            if i != cache.len - 1 {
                cursor_x += panel.spacing;
            }

            if cursor_x >= cache.max_x {
                break;
            }
        }
    });
}

define_widget! {
    Panel,
    Color,
    Texture,
    Stroke,

    default: {
        desired_size: DesiredSize::Fill,
    }
}

#[derive(Default, Component)]
pub struct TestPanelLayoutWidget {
    pub min: Vec2,
}

define_widget! {
    TestPanelLayoutWidget,
    Stroke,

    default: {
        desired_size: DesiredSize::Min(Vec2::ZERO),
    }
}

pub fn test_panel_set_size(
    mut query: Query<(&TestPanelLayoutWidget, &mut DesiredSize), Changed<DesiredSize>>,
) {
    query
        .iter_mut()
        .for_each(|(widget, mut size)| *size = DesiredSize::Min(widget.min));
}

pub struct PanelWidgetPlugin;
impl Plugin for PanelWidgetPlugin {
    fn init(&self, app: &mut toolkit::App) {
        app.add_systems(Update, test_panel_set_size);
        app.add_systems(
            Update,
            (
                setup_panels_transforms,
                compute_panel_layout_cache.after(setup_panels_transforms),
                apply_panel_layout.after(compute_panel_layout_cache)
            ),
        );
    }
}
