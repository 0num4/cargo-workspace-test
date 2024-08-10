
use std::panic;

use cfg_if::cfg_if;

use web_sys::window;
use winit::{
    event::{self, *},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: &'a Window
}

impl <'a> State<'a> {
    async fn new(window: &'a Window) -> State<'a> {
        
        let size = window.inner_size();
        // wgpu::Instance::newが一番重要。
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { 
            #[cfg(not(target_arch="wasm32"))]
            backends: wgpu::Backends::PRIMARY, 
            // flags: (), 
            #[cfg(target_arch="wasm32")]
            backends: wgpu::Backends::GL,
            // dx12_shader_compiler: (), 
            // gles_minor_version: () 
            ..Default::default()
        });
        let surface = instance.create_surface(window).unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase { 
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface)
        }).await.unwrap();

        let (device, queue) = adapter.request_device(
            // &wgpu::Features::empty(),
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: if cfg!(target_arch = "wasm32"){
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
                memory_hints: Default::default()
            },
            None  
        ).await.unwrap();
        let surface_caps =  surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter().find(|f| f.is_srgb()).copied().unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        return Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
        };
    }
    
    pub fn window(&self) -> &Window {
        &self.window
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool{
        return false;
    }

    fn update(&mut self) {}

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>){
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
            panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else{
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
        let _ = window.request_inner_size(PhysicalSize::new(450, 400));
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
            if !state.input(event){
                // UPDATED!
                match event {
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput { 
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
                    WindowEvent::RedrawRequested => {
                        state.window().request_redraw();
                        if surface_configured == false {
                            return;
                        }
                    }
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
        },
        _ => {}
    });
}
