use crate::{
    TextureHandle,
    types::{Argb8888, Color},
};

#[derive(Debug, Clone)]
pub struct Texture {
    pub color: Color,
    pub handle: TextureHandle,
}

impl Texture {
    pub const fn new(handle: TextureHandle) -> Self {
        Self {
            color: Color::Simple(Argb8888::WHITE),
            handle,
        }
    }

    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self
    }
}
