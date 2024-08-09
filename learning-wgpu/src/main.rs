use winit::{
    event::*,
    event_loop::{self, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
}

pub fn main() {
    env_logger::init(); // env_logger, useしなくても使えてる:thinking_face:
    let event_loop = EventLoop::new().unwrap(); // event_loopはEventLoop型
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, contol_flow| match event {

        // eventはバリアント。
        // バリアントって一般的なプログラミング言語のフィールドという意味ですか？
        // いいえ、バリアントは一般的なプログラミング言語のフィールドとは異なる概念です。この違いを明確にするために、もう少し詳しく説明しましょう。
        Event::WindowEvent { 
            ref: event,
            window_id
         } if window_id == window.id() => match event {
            WindowEvent::
             
         }
    })
    println!("Hello, world!");
}
