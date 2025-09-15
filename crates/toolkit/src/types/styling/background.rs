use derive_more::From;
use crate::{
    TextureHandle,
    types::{Argb8888, Color, Texture},
};

#[derive(Debug, Clone, From)]
pub enum BackgroundStyle {
    Color(Color),
    Texture(Texture),
}

impl Default for BackgroundStyle {
    fn default() -> Self {
        Self::Color(Argb8888::WHITE.into())
    }
}

impl From<TextureHandle> for BackgroundStyle {
    fn from(value: TextureHandle) -> Self {
        BackgroundStyle::Texture(Texture {
            color: Argb8888::WHITE.into(),
            handle: value,
        })
    }
}

impl From<Argb8888> for BackgroundStyle {
    fn from(value: Argb8888) -> Self {
        Self::Color(value.into())
    }
}
