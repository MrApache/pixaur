use derive_more::From;
use glam::Vec4;

#[derive(Debug, Clone, From)]
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
    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const RED: Self = Self::new(255, 0, 0, 255);
    pub const GREEN: Self = Self::new(0, 255, 0, 255);
    pub const BLUE: Self = Self::new(0, 0, 255, 255);
    pub const YELLOW: Self = Self::new(255, 255, 0, 255);
    pub const CYAN: Self = Self::new(0, 255, 255, 255);
    pub const MAGENTA: Self = Self::new(255, 0, 255, 255);
    pub const GRAY: Self = Self::new(128, 128, 128, 255);
    pub const LIGHT_GRAY: Self = Self::new(192, 192, 192, 255);
    pub const DARK_GRAY: Self = Self::new(64, 64, 64, 255);
    pub const ORANGE: Self = Self::new(255, 165, 0, 255);
    pub const PURPLE: Self = Self::new(128, 0, 128, 255);
    pub const BROWN: Self = Self::new(139, 69, 19, 255);
    pub const PINK: Self = Self::new(255, 192, 203, 255);
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);

    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Argb8888> for wgpu::Color {
    fn from(value: Argb8888) -> wgpu::Color {
        wgpu::Color {
            r: f64::from(value.r) / 255.0,
            g: f64::from(value.g) / 255.0,
            b: f64::from(value.b) / 255.0,
            a: f64::from(value.a) / 255.0,
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
        Vec4::new(
            f32::from(value.r) / 255.0,
            f32::from(value.g) / 255.0,
            f32::from(value.b) / 255.0,
            f32::from(value.a) / 255.0,
        )
    }
}

#[derive(Debug, Clone)]
pub struct LinearGradient {
    pub from: Argb8888,
    pub to: Argb8888,
    pub degree: f32,
}

impl LinearGradient {
    #[must_use]
    pub const fn new(from: Argb8888, to: Argb8888, degree: f32) -> Self {
        Self { from, to, degree }
    }
}
