use image::GenericImageView;
use image::*;

use anyhow::*;

use crate::texture;

pub struct Texture {
    #[allow(unused)]
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: [&u8],
        label: &str,
    ) -> Result<str> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &DynamicImage,
        label: Option<&str>,
    ) -> Result<Self> {
        let dimention = img.dimensions();
        let rgba = img.to_rgba8();
        let size = wgpu::Extent3d {
            width: img.width(),
            height: img.height(),
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture();
        queue.write_texture(texture, data, data_layout, size);
        let view = texture.create_view()
        let sampler = device.create_sampler()
        todo!();
    }
}
