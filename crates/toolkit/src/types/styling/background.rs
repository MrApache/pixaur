use crate::{
    types::{Argb8888, Color, Texture}, TextureHandle
};

#[derive(Debug, Clone)]
pub enum BackgroundStyle {
    Color(Color),
    Texture(Texture),
}

impl Default for BackgroundStyle {
    fn default() -> Self {
        Self::Color(Color::Simple(Argb8888::WHITE))
    }
}

impl From<Color> for BackgroundStyle {
    fn from(value: Color) -> Self {
        BackgroundStyle::Color(value)
    }
}

impl From<TextureHandle> for BackgroundStyle {
    fn from(value: TextureHandle) -> Self {
        BackgroundStyle::Texture(Texture {
            //color: Argb8888::WHITE.into(),
            handle: value,
        })
    }
}

impl From<Texture> for BackgroundStyle {
    fn from(value: Texture) -> Self {
        BackgroundStyle::Texture(value)
    }
}
