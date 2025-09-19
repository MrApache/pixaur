use toolkit::{
    commands::{CommandBuffer, DrawRectCommand, DrawTextureCommand},
    glam::Vec2,
    types::{Argb8888, Bounds, Stroke, Texture},
    widget::{Anchor, Context, DesiredSize, FrameContext, Sender, Widget},
    Handle, WidgetQuery,
};

#[derive(WidgetQuery)]
pub struct Image<C>
where
    C: Context,
{
    pub handle: Option<Handle>,
    pub anchor: Anchor,
    pub size: Vec2,

    id: Option<String>,
    rect: Bounds,
    _phantom: std::marker::PhantomData<C>,
}

impl<C> Default for Image<C>
where
    C: Context,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C> Image<C>
where
    C: Context,
{
    #[must_use]
    pub fn new() -> Self {
        Self::new_with_id(None)
    }

    #[must_use]
    pub fn with_id(id: impl Into<String>) -> Self {
        Self::new_with_id(Some(id.into()))
    }

    const fn new_with_id(id: Option<String>) -> Self {
        Self {
            size: Vec2::ZERO,
            rect: Bounds::ZERO,
            anchor: Anchor::Left,
            handle: None,
            id,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<C> Widget<C> for Image<C>
where
    C: Context,
{
    fn desired_size(&self) -> DesiredSize {
        DesiredSize::Exact(self.size)
    }

    fn anchor(&self) -> Anchor {
        self.anchor
    }

    fn draw<'frame>(&'frame self, out: &mut CommandBuffer<'frame>) {
        //use toolkit::commands::DrawCommand;
        //use toolkit::types::Color;
        //out.push(DrawCommand::Rect(DrawRectCommand::new(
        //    self.rect.clone(),
        //    Color::Simple(Argb8888::BLUE),
        //    Stroke::NONE,
        //)));
        if let Some(handle) = &self.handle {
            out.push(DrawTextureCommand::new(
                self.rect.clone(),
                Texture {
                    color: Argb8888::WHITE.into(),
                    handle: handle.clone(),
                },
                Stroke::NONE,
            ));
        } else {
            out.push(DrawRectCommand::new(
                self.rect.clone(),
                Argb8888::WHITE,
                Stroke::NONE,
            ));
        }
    }

    fn layout(&mut self, bounds: Bounds) {
        self.rect = bounds;
    }

    fn update(&mut self, _: &FrameContext, _: &mut Sender<C>) {}
}
