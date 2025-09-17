use toolkit::{
    commands::{DrawRectCommand, DrawTextureCommand},
    glam::Vec2,
    types::{Argb8888, Rect, Stroke, Texture},
    widget::{Context, DesiredSize, Sender, Widget},
    Handle, WidgetQuery,
};

#[derive(WidgetQuery)]
pub struct Image<C>
where
    C: Context,
{
    pub size: Vec2,
    pub rect: Rect,
    pub handle: Option<Handle>,

    id: Option<String>,
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

    pub fn with_id(id: impl Into<String>) -> Self {
        Self::new_with_id(Some(id.into()))
    }

    fn new_with_id(id: Option<String>) -> Self {
        Self {
            size: Vec2::ZERO,
            rect: Rect::ZERO,
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
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn desired_size(&self) -> DesiredSize {
        DesiredSize::Min(self.size)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw<'frame>(&'frame self, out: &mut toolkit::commands::CommandBuffer<'frame>) {
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

    fn layout(&mut self, bounds: Rect) {
        self.rect = bounds.clone();
    }

    fn update(&mut self, _: &toolkit::widget::FrameContext, _: &mut Sender<C>) {}
}
