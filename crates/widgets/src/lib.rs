#![allow(clippy::cast_precision_loss)]

pub mod button;
pub mod image;
pub mod panel;
pub mod rectangle;
pub mod text;
pub mod timer;

#[macro_export]
macro_rules! impl_empty_widget {
    ($name:ident) => {
        #[derive(Default)]
        pub struct $name;
        impl<C: toolkit::widget::Context> toolkit::widget::WidgetQuery<C> for $name {
        }
        impl<Ctx: toolkit::widget::Context> toolkit::widget::Widget<Ctx> for $name {
            fn id(&self) -> Option<&str> {
                None
            }

            fn desired_size(&self) -> toolkit::widget::DesiredSize {
                toolkit::widget::DesiredSize::Min(toolkit::glam::Vec2::ZERO)
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }

            fn draw<'frame>(&'frame self, _: &mut toolkit::commands::CommandBuffer<'frame>) {}

            fn layout(&mut self, _: toolkit::types::Rect) {}

            fn update(
                &mut self,
                _: &toolkit::widget::FrameContext,
                _: &mut toolkit::widget::Sender<Ctx>,
            ) {
            }
        }
    };
}

#[macro_export]
macro_rules! impl_proxy_widget {
    ($name:ident, $ctx:ident) => {
        impl toolkit::widget::WidgetQuery<$ctx> for $name {
            fn get_element<QW: toolkit::widget::Widget<$ctx>>(
                &self,
                id: &str,
            ) -> Option<&QW> {
                self.0.get_element(id)
            }

            fn get_mut_element<QW: toolkit::widget::Widget<$ctx>>(
                &mut self,
                id: &str,
            ) -> Option<&mut QW> {
                self.0.get_mut_element(id)
            }
        }

        impl toolkit::widget::Widget<$ctx> for $name {
            fn id(&self) -> Option<&str> {
                self.0.id()
            }

            fn desired_size(&self) -> toolkit::widget::DesiredSize {
                self.0.desired_size()
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self.0.as_any()
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self.0.as_any_mut()
            }

            fn draw<'frame>(&'frame self, out: &mut toolkit::commands::CommandBuffer<'frame>) {
                self.0.draw(out);
            }

            fn layout(&mut self, bounds: toolkit::types::Rect) {
                self.0.layout(bounds);
            }

            fn update(
                &mut self,
                ctx: &toolkit::widget::FrameContext,
                sender: &mut toolkit::widget::Sender<$ctx>,
            ) {
                self.0.update(ctx, sender);
            }
        }
    };
}
