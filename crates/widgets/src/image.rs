use toolkit::{
    commands::{DrawRectCommand, DrawTextureCommand},
    glam::Vec2,
    types::{Argb8888, Rect, Stroke, Texture},
    widget::{DesiredSize, Widget},
    Handle,
};

#[derive(Default)]
pub struct Image {
    pub size: Vec2,
    pub rect: Rect,
    pub handle: Option<Handle>,
}

impl Widget for Image {
    fn id(&self) -> Option<&str> {
        None
    }

    fn desired_size(&self) -> toolkit::widget::DesiredSize {
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
                Stroke::none(),
            ));
        }
        else {
            out.push(DrawRectCommand::new(
                self.rect.clone(),
                Argb8888::WHITE,
                Stroke::none(),
            ));
        }
    }

    fn layout(&mut self, bounds: toolkit::types::Rect) {
        self.rect = bounds.clone();
    }

    fn update(&mut self, _: &toolkit::widget::FrameContext) {}
}
