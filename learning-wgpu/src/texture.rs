use image::GenericImageView;
use image::*;

use anyhow::*;

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
        bytes: &[u8],
        label: &str,
    ) -> Result<Self> {
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
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("diffuse_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // unormのnormはnormalizationを意味する。Srgbのsはstandardを意味する
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // ImageCopyTextureやImageCopyBufferなどがある
        // webgpuではテクスチャを使用してシェーダーに画像データを提供して、レンダリング時にそのデータを使用してピクセルの色を決定します。テクスチャはシェーダーに渡す。
        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0, // mipmap_level0は最も高解像度のmipmaplevelを指す。
                origin: wgpu::Origin3d::ZERO, // 3次元空間の0(0,0,0)を表す
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * img.width()),
                rows_per_image: Some(img.height()),
            },
            size,
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        // アドレスモードはU、V、W軸すべてで繰り返し（Repeat）。
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("nya"),
            // clamptoEdgeはtextureが範囲外(0~1以外)にある場合にもっとも近いedgeの色を使用することを意味します。
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            // magfilterはテクスチャの拡大に関わる。テクスチャがシェーダーで拡大される際にどのように
            mag_filter: wgpu::FilterMode::Linear, //より滑らかで計算コストは中
            min_filter: wgpu::FilterMode::Nearest, //最も計算コストが低い
            mipmap_filter: wgpu::FilterMode::Nearest, //ピクセル化された見た目になることがある
            ..Default::default()
        });
        Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}
