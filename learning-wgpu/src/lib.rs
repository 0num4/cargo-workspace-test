use std::panic;

use cfg_if::cfg_if;
use image::GenericImageView;
use texture::Texture;
// mod texture::Texture;
// pub use texture::Texture;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: &'a Window,
    render_pipeline: wgpu::RenderPipeline,
}

impl<'a> State<'a> {
    async fn new(window: &'a Window) -> State<'a> {
        // let bytes = include_bytes!("happy-tree.png");
        let img = image::open("happy-tree.png").unwrap();
        // let img2 = img.as_bytes();
        let img_rgba = img.to_rgba8();
        let size: winit::dpi::PhysicalSize<u32> = window.inner_size();
        // let diffuse_texture =
        // wgpu::Instance::newが一番重要。

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            // flags: (),
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            // dx12_shader_compiler: (),
            // gles_minor_version: ()
            ..Default::default()
        });
        let surface = instance.create_surface(window).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                // &wgpu::Features::empty(),
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .unwrap();
        let diffuser_texture =
            texture::Texture::from_image(&device, &queue, &img, Some("test nya")).unwrap();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .unwrap_or(&surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("shader"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });
        // webgpu上ですべてのテスクチャは3dとして表現される。そのためdepthは1にする必要がある
        let texture_size = wgpu::Extent3d {
            width: img.width(),
            height: img.height(),
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("diffuse texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            // これは SurfaceConfig の場合と同じです。このテクスチャの TextureView を作成するためにどのテクスチャ形式を使用できるかを指定します。基本テクスチャ形式 (この場合は Rgba8UnormSrgb) が常にサポートされます。異なるテクスチャ形式の使用は、WebGL2 バックエンドではサポートされていないことに注意してください。
            // viewを作るためにことなるwgpu::TextureFormat::を指定する必要があることもあるって感じかな？
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * img.width()),
                rows_per_image: Some(4 * img.height()),
            },
            texture_size,
        );

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("nya"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bingroup_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("nya"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuser_texture.view),
            }],
        });

        return Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
        };
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut command_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
        {
            let mut render_path = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.6,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_path.set_pipeline(&self.render_pipeline);
            render_path.draw(0..3, 0..1);
        }
        self.queue.submit(std::iter::once(command_encoder.finish()));
        output.present();
        Ok(())
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        return false;
    }

    fn update(&mut self) {}

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            // panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_error_panic_hook::set_once();
            console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
        } else{
            println!("not assumed");
            env_logger::init(); // env_logger, useしなくても使えてる:thinking_face:
        }
    }
    let event_loop = EventLoop::new().unwrap(); // event_loopはEventLoop型
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // web_sys::window().and_then(|win| win.document()).and_then(|doc | {
    //     let dist = doc.get_element_by_id("wasm_example")?;
    //     let canvas = web_sys::Element::from(window.canvas()?);
    //     dist.append_child(&canvas).ok();
    //     Some(())
    // }).expect("canvasを追加できない");
    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;
        // winitではcssのサイズ変更ができないのでweb上でやる必要がある
        use winit::platform::web::WindowExtWebSys; //webの補完が効かなかった
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
        let _ = window.request_inner_size(PhysicalSize::new(450, 400));
    }

    let mut state = State::new(&window).await;
    let mut surface_configured = false;
    let _ = event_loop.run(move |event, control_flow| match event {
        // eventはバリアント。
        // バリアントって一般的なプログラミング言語のフィールドという意味ですか？
        // いいえ、バリアントは一般的なプログラミング言語のフィールドとは異なる概念です。この違いを明確にするために、もう少し詳しく説明しましょう。
        Event::WindowEvent {
            ref event, // デフォルトではeventは参照型。
            window_id,
        } if window_id == state.window.id() => {
            if !state.input(event) {
                // UPDATED!
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => control_flow.exit(),
                    WindowEvent::Resized(physical_size) => {
                        log::info!("physical_size: {physical_size:?}");
                        surface_configured = true;
                        state.resize(*physical_size);
                    }
                    WindowEvent::RedrawRequested if window_id == state.window().id() => {
                        // state.window().request_redraw();
                        // if surface_configured == false {
                        //     return;
                        // }
                        state.update();
                        match state.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),
                            Err(e) => println!("{:?}", e),
                        }
                    }
                    WindowEvent::CursorMoved {
                        device_id,
                        position,
                    } => {
                        log::info!("cursor moved{:?}, {:?}", device_id, position);
                        println!("cursor moved{:?}, {:?}", device_id, position);
                        // panic!("cursor moved{:?}, {:?}", device_id, position);
                    }
                    // Event::AboutToWait => {
                    //     state.window().request_redraw();
                    // }
                    _ => {}
                }
            }
            // WindowEvent::CloseRequested
            // | WindowEvent::KeyboardInput {
            //     event:
            //         KeyEvent {
            //             state: ElementState::Pressed,
            //             physical_key: PhysicalKey::Code(KeyCode::Escape),
            //             ..
            //         },
            //     .. //パターンマッチの記法で、..は残りのフィールドを無視する。
            // } => control_flow.exit(),
            // _ => {}
        }
        _ => {}
    });
}

pub fn render_image() -> Option<()> {
    let img = image::open("image.png").unwrap();
    println!("dimensions {:?}", img.dimensions());
    println!("color {:?}", img.color());
    let res = img.save("test.png");
    match res {
        Ok(_) => Some(()),
        Err(e) => panic!("err {:?}", e),
    }
}
