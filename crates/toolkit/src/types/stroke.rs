use super::{Argb8888, Corners};

#[derive(Clone)]
pub struct Stroke {
    pub color: Argb8888,
    pub width: f32,
    pub corners: Corners,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: Argb8888::GRAY,
            width: 2.0,
            corners: Corners::default(),
        }
    }
}
