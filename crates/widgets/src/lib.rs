#![allow(clippy::cast_precision_loss)]

pub mod panel;
pub mod text;

#[macro_export]
macro_rules! impl_empty_widget {
    ($name:ident) => {
        pub struct $name;
        impl toolkit::widget::Widget for $name {
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

            fn draw<'frame>(&'frame self, _: &mut toolkit::commands::CommandBuffer<'frame>) {
            }

            fn layout(&mut self, _: toolkit::types::Rect) {
            }
        }
    };
}

#[macro_export]
macro_rules! impl_window_root_enum {
    (
        $enum_name:ident<$gui_ty:ty> {
            $( $variant:ident ( $inner_ty:ty ) ),* $(,)?
        }
    ) => {
        impl WindowRoot for $enum_name {
            type Gui = $gui_ty;

            fn request(&self) -> WindowRequest {
                match self {
                    $(
                        $enum_name::$variant(inner) => inner.request(),
                    )*
                }
            }

            fn setup(&mut self, gui: &mut $gui_ty) {
                match self {
                    $(
                        $enum_name::$variant(inner) => inner.setup(gui),
                    )*
                }
            }

            fn draw<'frame>(&'frame self, out: &mut toolkit::commands::CommandBuffer<'frame>) {
                match self {
                    $(
                        $enum_name::$variant(inner) => inner.draw(out),
                    )*
                }
            }

            fn layout(&mut self, bounds: toolkit::types::Rect) {
                match self {
                    $(
                        $enum_name::$variant(inner) => inner.layout(bounds),
                    )*
                }
            }

            fn update(&mut self, gui: &mut $gui_ty) {
                match self {
                    $(
                        $enum_name::$variant(inner) => inner.update(gui),
                    )*
                }
            }
        }
    };
}
