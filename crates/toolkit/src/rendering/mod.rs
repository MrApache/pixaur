#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2,  // position
        1 => Float32x3   // color
    ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    // triangle 1
    Vertex { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0]}, // bottom-left red
    Vertex { position: [0.5, -0.5], color: [0.0, 1.0, 0.0]},  // bottom-right green
    Vertex { position: [-0.5, 0.5], color: [0.0, 0.0, 1.0]},  // top-left blue

    // triangle 2
    Vertex { position: [0.5, -0.5], color: [0.0, 1.0, 0.0]},  // bottom-right green
    Vertex { position: [0.5, 0.5], color: [1.0, 1.0, 0.0]},   // top-right yellow
    Vertex { position: [-0.5, 0.5], color: [0.0, 0.0, 1.0]},  // top-left blue
];

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectUniform {
    pub pos: [f32; 2],      // x, y позиция (например, нормализованная или пиксели)
    pub size: [f32; 2],     // ширина и высота
    pub color: [f32; 4],    // RGBA (0..1)
}
