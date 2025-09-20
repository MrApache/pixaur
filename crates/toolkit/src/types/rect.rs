use glam::Vec2;

use crate::widget::Spacing;

#[derive(Default, Debug, Clone)]
pub struct Bounds {
    pub position: Vec2,
    pub size: Vec2,
}

impl Bounds {
    pub const ZERO: Bounds = Self {
        position: Vec2::ZERO,
        size: Vec2::ZERO,
    };

    #[must_use]
    pub const fn new(position: Vec2, size: Vec2) -> Self {
        Self {
            position,
            size,
        }
    }

    #[must_use]
    pub const fn from_size(size: Vec2) -> Self {
        Self {
            position: Vec2::ZERO,
            size,
        }
    }

    #[must_use]
    pub const fn contains(&self, point: Vec2) -> bool {
        point.x > self.position.x
            && point.x <= self.size.x + self.position.x
            && point.y > self.position.y
            && point.y <= self.size.y + self.position.y
    }

    #[must_use]
    pub const fn shrink(&self, value: &Spacing) -> Self {
        let min_x = self.position.x + value.left;
        let min_y = self.position.y + value.top;
        let max_x = self.size.x - value.right - value.left;
        let max_y = self.size.y - value.bottom - value.top;
        Self {
            position: Vec2::new(min_x, min_y),
            size: Vec2::new(max_x, max_y),
        }
    }

    #[must_use]
    pub const fn extend(&self, value: &Spacing) -> Self {
        let min_x = self.position.x - value.left;
        let min_y = self.position.y - value.top;
        let max_x = self.size.x + value.right;
        let max_y = self.size.y + value.bottom;
        Self {
            position: Vec2::new(min_x, min_y),
            size: Vec2::new(max_x, max_y),
        }
    }
}
