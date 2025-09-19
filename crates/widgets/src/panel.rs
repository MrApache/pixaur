use crate::rectangle::Rectangle;
use toolkit::{
    commands::CommandBuffer,
    glam::Vec2,
    types::Bounds,
    widget::{Anchor, Container, Context, DesiredSize, FrameContext, Spacing, Sender, Widget},
    WidgetQuery,
};
/*

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

#[derive(WidgetQuery)]
pub struct Panel<C, W>
where
    C: Context,
    W: Widget<C>,
{
    pub padding: Padding,
    pub spacing: f32,
    pub mode: LayoutMode,
    pub vertical_align: VerticalAlign,
    pub horizontal_align: HorizontalAlign,
    pub rectangle: Rectangle<C>,
    pub anchor: Anchor,

    id: Option<String>,
    rect: Bounds,

    #[content]
    content: Vec<W>,

    _phantom: std::marker::PhantomData<C>,
}

impl<C: Context, W: Widget<C>> Default for Panel<C, W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Context, W: Widget<C>> Panel<C, W> {
    #[must_use]
    pub fn new() -> Self {
        Self::with_id(String::new())
    }

    pub fn with_id(id: impl Into<String>) -> Self {
        Self {
            padding: Padding::new(4.0, 4.0, 4.0, 4.0),
            spacing: 4.0,
            mode: LayoutMode::Vertical,
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Center,
            rectangle: Rectangle::default(),
            anchor: Anchor::Left,

            id: Some(id.into()),
            rect: Bounds::default(),
            content: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<C: Context, W: Widget<C>> Widget<C> for Panel<C, W> {
    fn anchor(&self) -> Anchor {
        self.anchor
    }

    fn desired_size(&self) -> DesiredSize {
        DesiredSize::Fill
    }

    fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>) {
        self.rectangle.draw(out);
        self.content.iter().for_each(|w| {
            w.draw(out);
        });
    }

    fn layout(&mut self, bounds: Bounds) {
        self.rectangle.layout(bounds.clone());
        self.rect = bounds;

        let min_x = self.rect.position.x + self.padding.left;
        let min_y = self.rect.position.y + self.padding.top;
        let max_x = self.rect.size.x - self.padding.right;
        let max_y = self.rect.size.y - self.padding.bottom;

        let len = self.content.len();
        let available_width = max_x;
        let available_height = max_y - min_y;

        let mut cursor_x = match self.horizontal_align {
            HorizontalAlign::Start => min_x,
            HorizontalAlign::Center => min_x + available_width / 2.0,
            HorizontalAlign::End => max_x - available_width,
        };

        let mut total_min_width = 0.0;
        let mut fill_count = 0;

        /*
        self.content
            .iter()
            .for_each(|widget| match widget.desired_size() {
                DesiredSize::Exact(size) => total_min_width += size.x,
                DesiredSize::Fill | DesiredSize::FillXExactY(_) => fill_count += 1,
                DesiredSize::Ignore => {},
            });

        let total_spacing = self.spacing * len.saturating_sub(1) as f32;
        let total_available_width = max_x - total_spacing - total_min_width - self.padding.right;
        let fill_width = total_available_width / fill_count as f32;

        for (i, child) in self.content.iter_mut().enumerate() {
            let (width, height) = match child.desired_size() {
                DesiredSize::Exact(vec2) => (vec2.x, vec2.y.min(available_height)),
                DesiredSize::Fill => (fill_width, available_height),
                DesiredSize::FillXExactY(y) => (fill_width, y.min(available_height)),
                DesiredSize::Ignore => continue,
            };

            let offset_y = match self.vertical_align {
                VerticalAlign::Start => 0.0,
                VerticalAlign::Center => (available_height - height) / 2.0,
                VerticalAlign::End => available_height - height,
            };

            let offset_x = match self.horizontal_align {
                HorizontalAlign::Center => width - (width / 2.0),
                HorizontalAlign::Start | HorizontalAlign::End => 0.0,
            };

            let child_bounds = Bounds {
                position: Vec2::new(cursor_x - offset_x, min_y + offset_y),
                size: Vec2::new(width, height),
            };

            //println!("Offset: {offset_x}x{offset_y}");

            child.layout(child_bounds);

            cursor_x += width;

            if i != len - 1 {
                cursor_x += self.spacing;
            }

            if cursor_x >= max_x + min_x {
                break;
            }
        }
        */
    }

    fn update(&mut self, ctx: &FrameContext, sender: &mut Sender<C>) {
        self.content.iter_mut().for_each(|w| {
            w.update(ctx, sender);
        });
    }
}

impl<C: Context, W: Widget<C>> Container<C, W> for Panel<C, W> {
    fn add_child(&mut self, child: W) {
        self.content.push(child);
    }

    fn children(&self) -> &[W] {
        &self.content
    }

    fn children_mut(&mut self) -> &mut [W] {
        &mut self.content
    }
}

*/
