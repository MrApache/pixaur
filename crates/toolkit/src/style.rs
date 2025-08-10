use crate::{Argb8888, Color, TextureHandle};

#[derive(Debug, Clone)]
pub struct Texture {
    pub color: Color,
    pub handle: TextureHandle,
}

impl Texture {
    pub fn new(handle: TextureHandle) -> Self {
        Self {
            color: Color::Simple(Argb8888::WHITE),
            handle,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

#[derive(Debug, Clone)]
pub enum BackgroundStyle {
    Color(Color),
    Texture(Texture),
}

impl From<Color> for BackgroundStyle {
    fn from(value: Color) -> Self {
        BackgroundStyle::Color(value)
    }
}

impl From<TextureHandle> for BackgroundStyle {
    fn from(value: TextureHandle) -> Self {
        BackgroundStyle::Texture(Texture {
            color: Color::Simple(Argb8888::WHITE),
            handle: value,
        })
    }
}

impl From<Texture> for BackgroundStyle {
    fn from(value: Texture) -> Self {
        BackgroundStyle::Texture(value)
    }
}
