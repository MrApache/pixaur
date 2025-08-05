use wgpu::*;
use crate::{error::Error, window::WindowPointer};

pub struct GPU {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,
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

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }

    pub fn create_surface<'window>(&self, ptr: WindowPointer, width: u32, height: u32) -> Result<Surface<'window>, Error> {
        let surface = self.instance.create_surface(ptr)?;
        let surface_caps = surface.get_capabilities(&self.adapter);
        let surface_format = surface_caps.formats[0];
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&self.device, &config);

        Ok(surface)
    }
}
