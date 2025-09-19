use toolkit::{
    commands::{CommandBuffer, DrawRectCommand, DrawTextureCommand},
    glam::Vec2,
    types::{styling::BackgroundStyle, Bounds, Stroke},
    widget::{Anchor, Context, DesiredSize, FrameContext, Sender, Widget},
    WidgetQuery,
};

#[derive(WidgetQuery)]
pub struct Rectangle<C, W>
where
    C: Context,
    W: Widget<C>,
{
    pub background: BackgroundStyle,
    pub stroke: Stroke,
    pub anchor: Anchor,
    pub width: Option<f32>,
    pub height: Option<f32>,
    id: Option<String>,
    bounds: Bounds,

    #[content]
    content: W,

    _phantom: std::marker::PhantomData<C>,
}

impl<C, W> Default for Rectangle<C, W>
where
    C: Context,
    W: Widget<C>,
{
    fn default() -> Self {
        Self {
            background: BackgroundStyle::WHITE,
            stroke: Stroke::default(),
            anchor: Anchor::Left,
            bounds: Bounds::default(),
            width: None,
            height: None,
            id: None,
            content: W::default(),

            _phantom: std::marker::PhantomData,
        }
    }
}

impl<C, W> Rectangle<C, W>
where
    C: Context,
    W: Widget<C>,
{
    pub fn content_mut(&mut self) -> &mut W {
        &mut self.content
    }

    pub fn content(&self) -> &W {
        &self.content
    }
}

impl<C, W> Widget<C> for Rectangle<C, W>
where
    C: Context,
    W: Widget<C>,
{
    fn anchor(&self) -> Anchor {
        self.anchor
    }

    fn desired_size(&self) -> DesiredSize {
        match (self.width, self.height) {
            (Some(width), Some(height)) => DesiredSize::Exact(Vec2::new(width, height)),
            (Some(width), None) => DesiredSize::ExactX(width),
            (None, Some(height)) => DesiredSize::ExactY(height),
            (None, None) => DesiredSize::Fill,
        }
    }

    fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>) {
        match &self.background {
            BackgroundStyle::Color(color) => out.push(DrawRectCommand::new(
                self.bounds.clone(),
                color.clone(),
                self.stroke.clone(),
            )),
            BackgroundStyle::Texture(texture) => out.push(DrawTextureCommand::new(
                self.bounds.clone(),
                texture.clone(),
                self.stroke.clone(),
            )),
        }

        self.content.draw(out);
    }

    fn layout(&mut self, bounds: Bounds) {
        self.bounds = bounds.clone();

        let size = match self.content.desired_size() {
            DesiredSize::Exact(size) => size,
            DesiredSize::ExactY(height) => Vec2::new(bounds.size.x, height),
            DesiredSize::ExactX(width) => Vec2::new(width, bounds.size.y),
            DesiredSize::Fill => bounds.size,
            DesiredSize::Ignore => return,
        };

        let mut position = Vec2::ZERO;
        self.content
            .anchor()
            .iter()
            .for_each(|anchor| match anchor {
                Anchor::Left => position = bounds.position,
                Anchor::Right => position = Vec2::new(bounds.size.x - size.x, bounds.position.y),
                Anchor::Top => position.y = bounds.position.y,
                Anchor::Bottom => position.y = bounds.size.y - size.y,
                Anchor::Center => position = bounds.position + (bounds.size - size) / 2.0,
                Anchor::VerticalCenter => {
                    position.y = bounds.position.y + (bounds.size.y - size.y) / 2.0;
                }
                Anchor::HorizontalCenter => {
                    position.x = bounds.position.x + (bounds.size.x - size.x) / 2.0;
                }
                _ => {}
            });

        self.content.layout(Bounds::new(position, size));
    }

    fn update(&mut self, frame: &FrameContext, sender: &mut Sender<C>) {
        self.content.update(frame, sender);
    }
}
