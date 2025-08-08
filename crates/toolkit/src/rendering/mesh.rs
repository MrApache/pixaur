use glam::{Vec2, Vec3};
use wgpu::util::DeviceExt;
use wgpu::{util::BufferInitDescriptor};
use wgpu::*;

use crate::rendering::Vertex;

pub struct QuadMesh {
    pub(crate) vertex_buffer: Buffer,
    pub(crate) index_buffer: Buffer,
}

impl QuadMesh {
    pub fn new(device: &Device) -> Self {
        let vertices = [
            Vertex::new(Vec3::new(0.0, 0.0, 0.0), Vec2::new(0.0, 0.0)), // левый верх
            Vertex::new(Vec3::new(1.0, 0.0, 0.0), Vec2::new(1.0, 0.0)), // правый верх
            Vertex::new(Vec3::new(1.0, 1.0, 0.0), Vec2::new(1.0, 1.0)), // правый низ
            Vertex::new(Vec3::new(0.0, 1.0, 0.0), Vec2::new(0.0, 1.0)), // левый низ
        ];

        let vertex_buffer_descriptor = BufferInitDescriptor {
            label: Some("Quad vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        };

        const INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

        let index_buffer_descriptor = BufferInitDescriptor {
            label: Some("Quad index buffer"),
            contents: bytemuck::cast_slice(&INDICES),
            usage: BufferUsages::INDEX,
        };

        Self {
            vertex_buffer: device.create_buffer_init(&vertex_buffer_descriptor),
            index_buffer: device.create_buffer_init(&index_buffer_descriptor),
        }
    }
}
