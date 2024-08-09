
use std::panic;

use cfg_if::cfg_if;
use wasm_bindgen::prelude;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};


pub fn run() {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            panic::set_hook(Box::new(console_error_panic_hook::hook))
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger")
        } else{
            env_logger::init() // env_logger, useしなくても使えてる:thinking_face:
        }
    }
    let event_loop = EventLoop::new().unwrap(); // event_loopはEventLoop型
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let _ = event_loop.run(move |event, control_flow| match event {
        // eventはバリアント。
        // バリアントって一般的なプログラミング言語のフィールドという意味ですか？
        // いいえ、バリアントは一般的なプログラミング言語のフィールドとは異なる概念です。この違いを明確にするために、もう少し詳しく説明しましょう。
        Event::WindowEvent {
            ref event, // デフォルトではeventは参照型。
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                .. //パターンマッチの記法で、..は残りのフィールドを無視する。
            } => control_flow.exit(),
            _ => {}
        },
        _ => {}
    });
}
