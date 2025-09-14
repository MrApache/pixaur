use crate::{error::Error, rendering::bind_group::BindGroupBuilder};
use image::GenericImageView;
use wgpu::{
    AddressMode, BindGroup, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Device,
    Extent3d, FilterMode, Origin3d, Queue, SamplerBindingType, SamplerDescriptor, ShaderStages,
    TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect, TextureDescriptor,
    TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureViewDescriptor,
    TextureViewDimension,
};

pub struct Material {
    pub bind_group: BindGroup,
}

impl Material {
    pub(crate) fn from_pixels(
        label: &'static str,
        pixels: &[u8],
        size: (u32, u32),
        format: TextureFormat,
        mag_filter: FilterMode,
        min_filter: FilterMode,
        device: &Device,
        queue: &Queue,
    ) -> Self {
        let texture_size = Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        };

        let texture_descriptor = TextureDescriptor {
            label: Some(label),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[format],
        };

        let texture = device.create_texture(&texture_descriptor);
        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            pixels,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(size.0 * 4),
                rows_per_image: Some(size.1),
            },
            texture_size,
        );
        let view = texture.create_view(&TextureViewDescriptor::default());

        let sampler_descriptor = SamplerDescriptor {
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            mag_filter,
            min_filter,
            ..Default::default()
        };
        let sampler = device.create_sampler(&sampler_descriptor);

        let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Material"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let mut builder = BindGroupBuilder::new(device);
        builder.set_layout(&layout);
        builder.add_material(&view, &sampler);
        let bind_group = builder.build(label);

        Material { bind_group }
    }

    pub(crate) fn from_rgba_pixels(
        label: &'static str,
        pixels: &[u8],
        size: (u32, u32),
        device: &Device,
        queue: &Queue,
    ) -> Self {
        Self::from_pixels(
            label,
            pixels,
            size,
            TextureFormat::Rgba8Unorm,
            FilterMode::Linear,
            FilterMode::Nearest,
            device,
            queue,
        )
    }

    pub fn default(device: &Device, queue: &Queue) -> Self {
        Self::from_rgba_pixels("Default", &[255, 255, 255, 255], (1, 1), device, queue)
    }

    pub fn from_bytes(bytes: &[u8], device: &Device, queue: &Queue) -> Result<Self, Error> {
        let image = image::load_from_memory(bytes)?;
        let converted = image.to_rgba8();
        let size = image.dimensions();
        Ok(Self::from_rgba_pixels(
            "texture", &converted, size, device, queue,
        ))
    }
}
