use crate::{Argb8888, rendering::Gpu};
use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use wgpu::*;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    model: Mat4,
    color_start: Vec4,
    color_end: Vec4,
    use_gradient: u32,
    _padding: [u32; 3],
}

impl InstanceData {
    pub fn new(position: Vec2, size: Vec2, color: &crate::Color, proj: Mat4) -> Self {
        let model = proj
            * Mat4::from_scale_rotation_translation(
                Vec3::new(size.x, size.y, 0.0),
                Quat::IDENTITY,
                Vec3::new(position.x, position.y, 0.0),
            );
        let (color_start, color_end, use_gradient): (Vec4, Vec4, u32) = match color {
            crate::Color::Simple(argb8888) => (argb8888.into(), Argb8888::TRANSPARENT.into(), 0),
            crate::Color::LinearGradient(linear_gradient) => (
                (&linear_gradient.from).into(),
                (&linear_gradient.to).into(),
                1,
            ),
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

struct InstanceBuffer {
    instances: Vec<InstanceData>,
    instance_buffer: Buffer,
    instance_buffer_len: usize,
}

impl InstanceBuffer {
    pub fn new(gpu: &Gpu, instance_buffer_size: usize) -> Self {
        let instance_buffer = gpu.device.create_buffer(&BufferDescriptor {
            label: Some("Instance buffer"),
            size: (instance_buffer_size * std::mem::size_of::<InstanceData>()) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            instances: Vec::with_capacity(instance_buffer_size),
            instance_buffer,
            instance_buffer_len: instance_buffer_size,
        }
    }

    fn create_instance_buffer(&mut self, gpu: &Gpu, size: usize) {
        let instance_buffer = gpu.device.create_buffer(&BufferDescriptor {
            label: Some("Instance buffer"),
            size: (size * std::mem::size_of::<InstanceData>()) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        self.instance_buffer = instance_buffer;
        self.instance_buffer_len = size;
    }

    fn resize_buffer_if_needed(&mut self, gpu: &Gpu, renderpass: &mut RenderPass) {
        if self.instances.capacity() > self.instance_buffer_len {
            self.create_instance_buffer(gpu, self.instances.capacity());
            renderpass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        }
    }

    fn write_instance_buffer(&self, gpu: &Gpu) {
        gpu.queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&self.instances),
        );
    }

    fn draw_instances(&mut self, gpu: &Gpu, renderpass: &mut RenderPass) {
        self.resize_buffer_if_needed(gpu, renderpass);
        self.write_instance_buffer(gpu);
        renderpass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        renderpass.draw_indexed(0..6, 0, 0..self.instances.len() as u32);
    }

    fn clear(&mut self) {
        self.instances.clear();
    }
}

pub struct InstancingPool {
    available: Vec<InstanceBuffer>,
    in_use: Vec<InstanceBuffer>,
    current: Option<InstanceBuffer>,
}

const INSTANCE_BUFFER_SIZE: usize = 2;
impl InstancingPool {
    pub fn new(gpu: &Gpu) -> Self {
        Self {
            available: vec![],
            in_use: vec![],
            current: Some(InstanceBuffer::new(gpu, INSTANCE_BUFFER_SIZE)),
        }
    }

    fn take(&mut self, gpu: &Gpu) {
        if self.available.is_empty() {
            self.current = Some(InstanceBuffer::new(gpu, INSTANCE_BUFFER_SIZE));
        } else {
            self.current = self.available.pop();
        }
    }

    fn complete(&mut self) {
        let buffer = self.current.take().unwrap();
        self.in_use.push(buffer);
    }

    pub fn clear(&mut self) {
        self.in_use.iter_mut().for_each(|buffer| {
            buffer.clear();
        });

        self.available.append(&mut self.in_use);
    }

    pub fn push(&mut self, data: InstanceData) {
        let buffer = self.current.as_mut().unwrap();
        buffer.instances.push(data);
    }

    pub fn draw_instances(&mut self, gpu: &Gpu, renderpass: &mut RenderPass) {
        let buffer = self.current.as_mut().unwrap();
        buffer.draw_instances(gpu, renderpass);
        self.complete();
        self.take(gpu);
    }
}
