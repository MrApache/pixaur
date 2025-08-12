pub struct Corners {
    left_top: f32,
    left_bottom: f32,
    right_top: f32,
    right_bottom: f32,
}

impl Default for Corners {
    fn default() -> Self {
        Self {
            left_top: 2.0,
            left_bottom: 2.0,
            right_top: 2.0,
            right_bottom: 2.0,
        }
    }
}
