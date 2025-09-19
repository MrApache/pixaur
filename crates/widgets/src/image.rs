use toolkit::{
    commands::{CommandBuffer, DrawRectCommand, DrawTextureCommand},
    glam::Vec2,
    types::{Argb8888, Bounds, Stroke, Texture},
    widget::{
        Anchor, Context, DefaultID, DesiredSize, FrameContext, NoID, Sender, StaticID, Widget,
        WidgetID,
    },
    Handle, WidgetQuery,
};

#[derive(WidgetQuery)]
pub struct Image<C, ID = DefaultID>
where
    C: Context,
    ID: WidgetID,
{
    pub handle: Option<Handle>,
    pub anchor: Anchor,
    pub size: Vec2,

    id: ID::IdType,
    rect: Bounds,
    _phantom: std::marker::PhantomData<C>,
}

impl<C> Image<C, NoID>
where
    C: Context,
{
    #[must_use]
    pub const fn new() -> Self {
        Self::new_with_id(())
    }
}

impl<C> Image<C, DefaultID>
where
    C: Context,
{
    #[must_use]
    pub fn new_default() -> Self {
        Self::new_with_id(None)
    }

    #[must_use]
    pub fn new_id(id: impl Into<String>) -> Self {
        Self::new_with_id(Some(id.into()))
    }
}

impl<C> Image<C, StaticID>
where
    C: Context,
{
    #[must_use]
    pub const fn new_static(id: &'static str) -> Self {
        Self::new_with_id(id)
    }
}

impl<C, ID> Default for Image<C, ID>
where
    C: Context,
    ID: WidgetID,
{
    fn default() -> Self {
        Self::new_with_id(ID::IdType::default())
    }
}

impl<C, ID> Image<C, ID>
where
    C: Context,
    ID: WidgetID,
{
    const fn new_with_id(id: ID::IdType) -> Self {
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

impl<C, ID> Widget<C> for Image<C, ID>
where
    C: Context,
    ID: WidgetID,
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
