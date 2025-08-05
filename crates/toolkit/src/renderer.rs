use wgpu::*;
use wl_client::window::Window;
use crate::{error::Error, Argb8888};

pub struct GPU {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

impl GPU {
    pub fn new() -> Result<Self, Error> {
        let instance = Instance::new(&InstanceDescriptor::default());
        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            //compatible_surface: Some(&surface),
            compatible_surface: None,
        }))?;
        let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor::default()))?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }

    pub fn create_surface<'window>(&self, window: &'window Window) -> Result<Surface<'window>, Error>{
        let surface = self.instance.create_surface(window)?;
        let surface_caps = surface.get_capabilities(&self.adapter);
        let surface_format = surface_caps.formats[0];
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.width as u32,
            height: window.height as u32,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&self.device, &config);

        Ok(surface)
    }
}

fn start(window: &Window) -> Result<(), Error> {
    // 3. Создание цветного рендера
    let frame = surface.get_current_texture()?;
    let view = frame.texture.create_view(&TextureViewDescriptor::default());

    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    {
        let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Clear Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Argb8888::BLACK.into()),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            ..Default::default()
        });
    }

    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}
