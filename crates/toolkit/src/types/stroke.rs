use super::{Argb8888, Corners};

#[derive(Clone)]
pub struct Stroke {
    pub color: [Argb8888; 4],
    pub width: f32,
    pub corners: Corners,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: [Argb8888::GRAY; 4],
            width: 1.0,
            corners: Corners::default(),
        }
    }
}

impl Stroke {
    #[must_use]
    pub fn none() -> Self {
        Self {
            color: [Argb8888::TRANSPARENT; 4],
            width: 0.0,
            corners: Corners::none(),
        }
    }
}
