use bevy_ecs::component::Component;
use crate::TextureHandle;

#[derive(Default, Debug, Clone, Component)]
pub struct Texture {
    pub handle: TextureHandle,
}

impl Texture {
    pub const fn new(handle: TextureHandle) -> Self {
        Self {
            handle,
        }
    }
}
