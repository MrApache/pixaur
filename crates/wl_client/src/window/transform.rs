use wayland_client::{
    protocol::wl_output::Transform as WTransform,
    WEnum
};

pub enum Transform {
    Normal0,
    Normal90,
    Normal180,
    Normal270,
    Flipped0,
    Flipped90,
    Flipped180,
    Flipped270,
    Custom(u32),
}

impl Into<Transform> for WEnum<WTransform> {
    fn into(self) -> Transform {
        match self {
            WEnum::Value(t) => match t {
                WTransform::Normal => Transform::Normal0,
                WTransform::_90 => Transform::Normal90,
                WTransform::_180 => Transform::Normal180,
                WTransform::_270 => Transform::Normal270,
                WTransform::Flipped => Transform::Flipped0,
                WTransform::Flipped90 => Transform::Flipped90,
                WTransform::Flipped180 => Transform::Flipped180,
                WTransform::Flipped270 => Transform::Flipped270,
                _ => Transform::Normal0,
            }
            WEnum::Unknown(v) => Transform::Custom(v)
        }
    }
}
