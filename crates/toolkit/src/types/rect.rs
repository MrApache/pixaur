use glam::Vec2;

#[derive(Default, Debug, Clone)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    #[must_use]
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self {
            min: position,
            max: position + size,
        }
    }

    #[must_use]
    pub const fn from_size(size: Vec2) -> Self {
        Self {
            min: Vec2::ZERO,
            max: size,
        }
    }

    #[must_use]
    pub const fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    #[must_use]
    pub const fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    #[must_use]
    pub const fn contains(&self, point: Vec2) -> bool {
        point.x > self.min.x
            && point.x <= self.max.x + self.min.x
            && point.y > self.min.y
            && point.y <= self.max.y + self.min.y
    }
}
