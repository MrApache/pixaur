use toolkit::{
    commands::{CommandBuffer, DrawCommand, DrawRectCommand, DrawTextureCommand},
    glam::Vec2,
    types::{styling::BackgroundStyle, Bounds, Stroke},
    widget::{Anchor, Context, DesiredSize, Empty, FrameContext, Spacing, Sender, Widget},
    WidgetQuery,
};

#[derive(Default)]
struct LayoutData {
    right_current_x: f32,
    right_total_min_width: f32,
    right_total_flex: f32,
    right_flex_unit: f32,

    left_current_x: f32,
    left_total_min_width: f32,
    left_total_flex: f32,
    left_flex_unit: f32,
}

#[derive(WidgetQuery)]
pub struct Row<C, W = Empty>
where
    C: Context,
    W: Widget<C>,
{
    pub padding: Spacing,

    pub background: BackgroundStyle,
    pub stroke: Stroke,

    pub anchor: Anchor,

    pub spacing: f32,

    pub width: Option<f32>,
    pub height: Option<f32>,

    bounds: Bounds,
    id: Option<String>,

    #[content]
    content: Vec<W>,

    _phantom: std::marker::PhantomData<C>,
}

impl<C, W> Default for Row<C, W>
where
    C: Context,
    W: Widget<C>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C, W> Row<C, W>
where
    C: Context,
    W: Widget<C>,
{
    #[must_use]
    pub const fn new() -> Self {
        Self::new_with_id(None)
    }

    #[must_use]
    pub fn with_id(id: impl Into<String>) -> Self {
        Self::new_with_id(Some(id.into()))
    }

    const fn new_with_id(id: Option<String>) -> Self {
        Self {
            padding: Spacing::ZERO,
            spacing: 0.0,
            bounds: Bounds::ZERO,
            background: BackgroundStyle::WHITE,
            stroke: Stroke::NONE,
            anchor: Anchor::Left,
            width: None,
            height: None,
            id,

            content: vec![],

            _phantom: std::marker::PhantomData,
        }
    }

    #[must_use]
    pub fn content(&self) -> &[W] {
        &self.content
    }

    #[must_use]
    pub fn content_mut(&mut self) -> &mut Vec<W> {
        &mut self.content
    }

    fn get_layout_data(&self, inner_bounds: &Bounds) -> LayoutData {
        let mut data = LayoutData {
            left_current_x: inner_bounds.position.x,
            right_current_x: inner_bounds.size.x,
            ..Default::default()
        };

        for child in &self.content {
            match child.desired_size() {
                DesiredSize::Exact(size) => {
                    if child.anchor().contains(Anchor::Right) {
                        data.right_total_min_width += size.x;
                    } else {
                        data.left_total_min_width += size.x;
                    }
                }
                DesiredSize::ExactY(_) | DesiredSize::Fill => {
                    if child.anchor().contains(Anchor::Right) {
                        data.right_total_flex += 1.0;
                    } else {
                        data.left_total_flex += 1.0;
                    }
                }
                DesiredSize::ExactX(_) => {
                    if child.anchor().contains(Anchor::Right) {
                        data.right_total_min_width += 1.0;
                    } else {
                        data.left_total_min_width += 1.0;
                    }
                }
                DesiredSize::Ignore => {}
            }
        }

        let left_available_width = inner_bounds.size.x
            - data.left_total_min_width
            - self.spacing * (self.content.len() - 1) as f32;

        data.left_flex_unit = if data.left_total_flex > 0.0 {
            left_available_width / data.left_total_flex
        } else {
            0.0
        };

        let right_available_width = inner_bounds.size.x
            - data.right_total_min_width
            - self.spacing * (self.content.len() - 1) as f32;

        data.right_flex_unit = if data.right_total_flex > 0.0 {
            right_available_width / data.right_total_flex
        } else {
            0.0
        };

        data
    }

    fn get_child_size(data: &LayoutData, inner_bounds: &Bounds, desired_size: DesiredSize, anchor: Anchor) -> Vec2 {
        match desired_size {
            DesiredSize::Exact(size) => size,
            DesiredSize::ExactY(height) => {
                if anchor.contains(Anchor::Right) {
                    Vec2::new(data.right_flex_unit, height)
                } else {
                    Vec2::new(data.left_flex_unit, height)
                }
            }
            DesiredSize::ExactX(width) => Vec2::new(width, inner_bounds.size.y),
            DesiredSize::Fill => {
                if anchor.contains(Anchor::Right) {
                    Vec2::new(data.right_flex_unit, inner_bounds.size.y)
                } else {
                    Vec2::new(data.left_flex_unit, inner_bounds.size.y)
                }
            }
            DesiredSize::Ignore => unreachable!(),
        }
    }

    fn get_child_position(
        child: &W,
        child_size: Vec2,
        spacing: f32,
        data: &mut LayoutData,
        inner_bounds: &Bounds,
    ) -> Vec2 {
        let mut position = Vec2::ZERO;
        child.anchor().iter().for_each(|anchor| match anchor {
            Anchor::Left => {
                position = Vec2::new(data.left_current_x, inner_bounds.position.y);
                data.left_current_x += child_size.x + spacing;
            }
            Anchor::Right => {
                data.right_current_x -= child_size.x;
                position = Vec2::new(data.right_current_x, inner_bounds.position.y);
                data.right_current_x -= spacing;
            }
            Anchor::Top => {
                position.y = inner_bounds.position.y;
            }
            Anchor::Bottom => {
                position.y = inner_bounds.size.y - child_size.y;
            }
            Anchor::Center => {
                position = inner_bounds.position + (inner_bounds.size - child_size) / 2.0;
            }
            Anchor::VerticalCenter => {
                position.y = inner_bounds.position.y + (inner_bounds.size.y - child_size.y) / 2.0;
            }
            Anchor::HorizontalCenter => {
                position.x = inner_bounds.position.x + (inner_bounds.size.x - child_size.x) / 2.0;
            }
            _ => {}
        });
        
        position
    }
}

impl<C, W> Widget<C> for Row<C, W>
where
    C: Context,
    W: Widget<C>,
{
    fn desired_size(&self) -> DesiredSize {
        match (self.width, self.height) {
            (Some(width), Some(height)) => DesiredSize::Exact(Vec2::new(width, height)),
            (Some(width), None) => DesiredSize::ExactX(width),
            (None, Some(height)) => DesiredSize::ExactY(height),
            (None, None) => DesiredSize::Fill,
        }
    }

    fn anchor(&self) -> Anchor {
        self.anchor
    }

    fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>) {
        let command = match &self.background {
            BackgroundStyle::Color(color) => DrawCommand::Rect(DrawRectCommand::new(
                self.bounds.clone(),
                color.clone(),
                self.stroke.clone(),
            )),
            BackgroundStyle::Texture(texture) => DrawCommand::Texture(DrawTextureCommand::new(
                self.bounds.clone(),
                texture.clone(),
                self.stroke.clone(),
            )),
        };

        out.push(command);

        self.content.iter().for_each(|f| {
            f.draw(out);
        });
    }

    fn layout(&mut self, bounds: Bounds) {
        self.bounds = bounds;
        if self.content.is_empty() {
            return;
        }
        let inner_bounds = self.bounds.shrink(&self.padding);
        let mut data = self.get_layout_data(&inner_bounds);

        for child in &mut self.content {
            let desired_size = child.desired_size();
            if let DesiredSize::Ignore = desired_size {
                continue;
            }
            let child_size = Self::get_child_size(&data, &inner_bounds, desired_size, child.anchor());
            let mut child_position = Self::get_child_position(child, child_size, self.spacing, &mut data, &inner_bounds);
            child_position.x += self.bounds.position.x;
            let child_rect = Bounds::new(child_position, child_size);
            child.layout(child_rect);
        }
    }

    fn update(&mut self, ctx: &FrameContext, sender: &mut Sender<C>) {
        self.content.iter_mut().for_each(|e| {
            e.update(ctx, sender);
        });
    }
}
