use wgpu::{util::{BufferInitDescriptor, DeviceExt}, *};
use crate::{
    error::Error,
    window::WindowPointer
};

pub struct GPU {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    render_pipeline: RenderPipeline,
    quad: Mesh,
    //vertex_buffer_layouts: Vec<VertexBufferLayout<'static>>
}

impl GPU {
    pub fn new(dummy: WindowPointer) -> Result<Self, Error> {
        let instance = Instance::new(&InstanceDescriptor::default());
        let surface = instance.create_surface(dummy)?;
        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))?;
        let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor::default()))?;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../assets/shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Vertex::get_layout()],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: surface.get_capabilities(&adapter).formats[0],
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        //let quad = make_quad(&device);
        let quad = make_triangle(&device);

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            render_pipeline,
            quad,
        })
    }

    pub fn confugure_surface(&self, surface: &Surface<'_>, configuration: &SurfaceConfiguration) {
        surface.configure(&self.device, configuration);
    }

    pub fn create_surface<'window>(&self, ptr: WindowPointer, width: u32, height: u32) -> Result<(Surface<'window>, SurfaceConfiguration), Error> {
        let surface = self.instance.create_surface(ptr)?;
        let surface_caps = surface.get_capabilities(&self.adapter);
        let format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&self.device, &config);

        Ok((surface, config))
    }

    pub fn render(&self, surface: &Surface) {
        let drawable = surface.get_current_texture().expect("TODO");
        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        };
        let mut command_encoder = self.device.create_command_encoder(&command_encoder_descriptor);

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0
                }),
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        };

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None
        };

        {
            let mut renderpass = command_encoder.begin_render_pass(&render_pass_descriptor);
            renderpass.set_pipeline(&self.render_pipeline);
            renderpass.set_vertex_buffer(0, self.quad.vertex_buffer.slice(..));
            renderpass.set_index_buffer(self.quad.index_buffer.slice(..), IndexFormat::Uint16);
            renderpass.draw_indexed(0..4, 0, 0..1);
            //renderpass.draw(0..3, 0..1);
        }
        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: Vec3,
    color: Vec3
}

impl Vertex {
    pub fn get_layout() -> VertexBufferLayout<'static> {
        const ATTRIBUTES: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &ATTRIBUTES
        }
    }
}

pub fn make_triangle(device: &Device) -> Mesh {
    let vertices: [Vertex; 3] = [
        Vertex {position: Vec3::new(-1.0, -1.0, 0.0), color: Vec3::new(1.0, 0.0, 0.0)},
        Vertex {position: Vec3::new(1.0, -1.0, 0.0), color: Vec3::new(0.0, 1.0, 0.0)},
        Vertex {position: Vec3::new(1.0, 1.0, 0.0), color: Vec3::new(0.0, 0.0, 1.0)},
    ];

    let vertex_buffer_descriptor = BufferInitDescriptor {
        label: Some("Triangle vertex buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: BufferUsages::VERTEX,
    };

    let indices: [u16; 4] = [0, 1, 2, 0];

    let index_buffer_descriptor = BufferInitDescriptor {
        label: Some("Triangle index buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: BufferUsages::INDEX,
    };

    Mesh {
        vertex_buffer: device.create_buffer_init(&vertex_buffer_descriptor),
        index_buffer: device.create_buffer_init(&index_buffer_descriptor)
    }
}

pub fn make_quad(device: &Device) -> Mesh {
    let vertices: [Vertex; 4] = [
        Vertex {position: Vec3::new(-1.0, -1.0, 0.0), color: Vec3::new(1.0, 0.0, 0.0)},
        Vertex {position: Vec3::new(1.0, -1.0, 0.0), color: Vec3::new(0.0, 1.0, 0.0)},
        Vertex {position: Vec3::new(1.0, 1.0, 0.0), color: Vec3::new(0.0, 0.0, 1.0)},
        Vertex {position: Vec3::new(-1.0, 1.0, 0.0), color: Vec3::new(1.0, 0.0, 0.0)},
    ];

    let vertex_buffer_descriptor = BufferInitDescriptor {
        label: Some("Quad vertex buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: BufferUsages::VERTEX,
    };

    let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];

    let index_buffer_descriptor = BufferInitDescriptor {
        label: Some("Quad index buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: BufferUsages::INDEX,
    };

    Mesh {
        vertex_buffer: device.create_buffer_init(&vertex_buffer_descriptor),
        index_buffer: device.create_buffer_init(&index_buffer_descriptor)
    }
}

pub struct Mesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
}
