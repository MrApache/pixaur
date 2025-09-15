use derive_more::From;
use crate::{
    TextureHandle,
    types::{Argb8888, Color, Texture},
};

#[derive(Debug, Clone)]
#[derive(Debug, Clone, From)]
pub enum BackgroundStyle {
    Color(Color),
    Texture(Texture),
}

impl From<Color> for BackgroundStyle {
    fn from(value: Color) -> Self {
        BackgroundStyle::Color(value)
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

impl From<Texture> for BackgroundStyle {
    fn from(value: Texture) -> Self {
        BackgroundStyle::Texture(value)
impl From<Argb8888> for BackgroundStyle {
    fn from(value: Argb8888) -> Self {
        Self::Color(value.into())
    }
}
