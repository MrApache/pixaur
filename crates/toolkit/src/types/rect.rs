use glam::Vec2;

#[derive(Default, Debug, Clone)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self {
            min: position,
            max: position + size,
        }
    }

    pub const fn from_size(size: Vec2) -> Self {
        Self {
            min: Vec2::ZERO,
            max: size,
        }
    }

    pub const fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub const fn height(&self) -> f32 {
        self.max.y - self.min.y
    }
}
