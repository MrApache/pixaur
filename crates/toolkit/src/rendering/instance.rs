use crate::{
    rendering::Gpu,
    types::{self, Argb8888, Stroke},
};
use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use wgpu::*;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData {
    uv: Vec4,

    size: Vec2, 
    _padding0: [u32; 2],

    model: Mat4,

    color: Vec4,

    stroke_color: Vec4,

    stroke_width: f32,
    _padding1: [u32; 3],

    stroke_corners: Vec4,

    color_end: Vec4,

    degree: f32,
    use_gradient: u32,
    support_stroke: u32,
    _padding3: u32,
}

impl InstanceData {
    pub fn new_uv_4(
        uv: Vec4,
        position: Vec2,
        size: Vec2,
        color: &types::Color,
        stroke: Option<Stroke>,
        proj: Mat4,
    ) -> Self {
        let model = proj
            * Mat4::from_scale_rotation_translation(
                Vec3::new(size.x, size.y, 0.0),
                Quat::IDENTITY,
                Vec3::new(position.x, position.y, 0.0),
            );

        let (color, color_end, degree, use_gradient): (Vec4, Vec4, f32, u32) = match color {
            types::Color::Simple(argb8888) => {
                (argb8888.into(), Argb8888::TRANSPARENT.into(), 0.0, 0)
            }
            types::Color::LinearGradient(linear_gradient) => (
                (&linear_gradient.from).into(),
                (&linear_gradient.to).into(),
                linear_gradient.degree,
                1,
            ),
        };

        let (stroke_color, stroke_width, stroke_corners, support_stroke) = {
            if let Some(stroke) = stroke {
                (stroke.color.into(), stroke.width, stroke.corners.into(), 1)
            }
            else {
                Default::default()
            }
        };

        Self {
            uv,
            size,

            model,
            color,

            stroke_color,
            stroke_width,
            stroke_corners,
            support_stroke,

            color_end,
            degree,
            use_gradient,

            _padding0: Default::default(),
            _padding1: Default::default(),
            _padding3: Default::default(),
        }
    }

    pub fn new_uv_2(
        uv0: Vec2,
        uv1: Vec2,
        uv2: Vec2,
        uv3: Vec2,
        position: Vec2,
        size: Vec2,
        color: &types::Color,
        stroke: Option<Stroke>,
        proj: Mat4,
    ) -> Self {
        let u_min = uv0.x.min(uv1.x).min(uv2.x).min(uv3.x);
        let v_min = uv0.y.min(uv1.y).min(uv2.y).min(uv3.y);
        let u_max = uv0.x.max(uv1.x).max(uv2.x).max(uv3.x);
        let v_max = uv0.y.max(uv1.y).max(uv2.y).max(uv3.y);

        let uv_rect = Vec4::new(u_min, v_min, u_max, v_max);

        Self::new_uv_4(uv_rect, position, size, color, stroke, proj)
    }


    pub fn get_layout() -> VertexBufferLayout<'static> {
        use wgpu::{VertexAttribute, VertexFormat::*, VertexStepMode};
    
        const ATTRIBUTES: &[VertexAttribute] = &[
            // location, offset, format
            VertexAttribute { offset: 0,   shader_location: 1, format: Float32x4 }, // uv
            VertexAttribute { offset: 16,  shader_location: 2, format: Float32x2 }, // size
    
            VertexAttribute { offset: 32,  shader_location: 3, format: Float32x4 }, // model_matrix_0
            VertexAttribute { offset: 48,  shader_location: 4, format: Float32x4 }, // model_matrix_1
            VertexAttribute { offset: 64,  shader_location: 5, format: Float32x4 }, // model_matrix_2
            VertexAttribute { offset: 80,  shader_location: 6, format: Float32x4 }, // model_matrix_3
    
            VertexAttribute { offset: 96,  shader_location: 7, format: Float32x4 }, // color
    
            VertexAttribute { offset: 112, shader_location: 8,  format: Float32x4 }, // stroke_color
            VertexAttribute { offset: 128, shader_location: 9,  format: Float32 },   // stroke_width
            VertexAttribute { offset: 144, shader_location: 10, format: Float32x4 }, // stroke_corners
    
            VertexAttribute { offset: 160, shader_location: 11, format: Float32x4 }, // color_end
            VertexAttribute { offset: 176, shader_location: 12, format: Float32 },   // degree
            VertexAttribute { offset: 180, shader_location: 13, format: Uint32 },    // use_gradient
            VertexAttribute { offset: 184, shader_location: 14, format: Uint32 },    // support_stroke
        ];
    
        VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceData>() as wgpu::BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: ATTRIBUTES,
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
