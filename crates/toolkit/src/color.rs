use glam::Vec4;

#[derive(Debug, Clone)]
pub enum Color {
    Simple(Argb8888),
    LinearGradient(LinearGradient),
}

#[derive(Debug, Clone)]
pub struct Argb8888 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for Argb8888 {
    fn default() -> Self {
        Self::WHITE
    }
}

impl Argb8888 {
    pub const BLACK: Self   = Self::new(0, 0, 0, 255);
    pub const WHITE: Self   = Self::new(255, 255, 255, 255);
    pub const RED: Self     = Self::new(255, 0, 0, 255);
    pub const GREEN: Self   = Self::new(0, 255, 0, 255);
    pub const BLUE: Self    = Self::new(0, 0, 255, 255);
    pub const YELLOW: Self  = Self::new(255, 255, 0, 255);
    pub const CYAN: Self    = Self::new(0, 255, 255, 255);
    pub const MAGENTA: Self = Self::new(255, 0, 255, 255);
    pub const GRAY: Self    = Self::new(128, 128, 128, 255);
    pub const LIGHT_GRAY: Self = Self::new(192, 192, 192, 255);
    pub const DARK_GRAY: Self = Self::new(64, 64, 64, 255);
    pub const ORANGE: Self  = Self::new(255, 165, 0, 255);
    pub const PURPLE: Self  = Self::new(128, 0, 128, 255);
    pub const BROWN: Self   = Self::new(139, 69, 19, 255);
    pub const PINK: Self    = Self::new(255, 192, 203, 255);
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
            ((a as f32) + (b as f32 - a as f32) * t).round() as u8
        }

        Self {
            a: lerp_u8(self.a, other.a, t),
            r: lerp_u8(self.r, other.r, t),
            g: lerp_u8(self.g, other.g, t),
            b: lerp_u8(self.b, other.b, t),
        }
    }
}

impl From<Argb8888> for wgpu::Color {
    fn from(value: Argb8888) -> wgpu::Color {
        wgpu::Color {
            r: value.r as f64 / 255.0,
            g: value.g as f64 / 255.0,
            b: value.b as f64 / 255.0,
            a: value.a as f64 / 255.0,
        }
    }
}

impl From<Argb8888> for Vec4 {
    fn from(value: Argb8888) -> Self {
        Vec4::from(&value)
    }
}

impl From<&Argb8888> for Vec4 {
    fn from(value: &Argb8888) -> Self {
        Vec4::new(value.r as f32 / 255.0, value.g as f32 / 255.0, value.b as f32 / 255.0, value.a as f32 / 255.0)
    }
}

#[derive(Debug, Clone)]
pub struct LinearGradient {
    pub from: Argb8888,
    pub to: Argb8888,
}

impl LinearGradient {
    pub const fn new(from: Argb8888, to: Argb8888) -> Self {
        Self {
            from,
            to,
        }
    }

    pub fn color_at(&self, t: f32) -> Argb8888 {
        self.from.lerp(&self.to, t.clamp(0.0, 1.0))
    }
}
