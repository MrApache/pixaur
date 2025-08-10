use wgpu::*;
use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use crate::Argb8888;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    model: Mat4,
    color_start: Vec4,
    color_end: Vec4,
    use_gradient: u32,
    _padding: [u32; 3]
}

impl InstanceData {
    pub fn new(position: Vec2, size: Vec2, color: &crate::Color, proj: Mat4) -> Self {
        let model = proj * Mat4::from_scale_rotation_translation(Vec3::new(size.x, size.y, 0.0), Quat::IDENTITY, Vec3::new(position.x, position.y, 0.0));
        let (color_start, color_end, use_gradient) : (Vec4, Vec4, u32) = match color {
            crate::Color::Simple(argb8888) => (argb8888.into(), Argb8888::TRANSPARENT.into(), 0),
            crate::Color::LinearGradient(linear_gradient) => ((&linear_gradient.from).into(), (&linear_gradient.to).into(), 1),
        };

        Self {
            model,
            color_start,
            color_end,
            use_gradient,
            _padding: [0, 0, 0],
        }
    }

    pub fn get_layout() -> VertexBufferLayout<'static> {
        use std::mem;
        VertexBufferLayout {
            array_stride: mem::size_of::<InstanceData>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as BufferAddress,
                    shader_location: 6,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as BufferAddress,
                    shader_location: 7,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as BufferAddress,
                    shader_location: 8,
                    format: VertexFormat::Float32x4,
                },

                VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as BufferAddress,
                    shader_location: 9,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 20]>() as BufferAddress,
                    shader_location: 10,
                    format: VertexFormat::Float32x4,
                },
                VertexAttribute {
                    offset: 96,
                    shader_location: 11,
                    format: VertexFormat::Uint32,
                },
            ],
        }
    }
}
