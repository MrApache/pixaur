use super::{Argb8888, Color, Corners};

pub struct Stroke {
    pub color: Color,
    pub width: f32,
    pub corners: Corners,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: Argb8888::GRAY.into(),
            width: 2.0,
            corners: Corners::default(),
        }
    }
}
