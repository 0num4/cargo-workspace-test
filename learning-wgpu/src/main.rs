use add_one::use_variant_sample;
mod lib;
pub fn main() {
    env_logger::init(); // env_logger, useしなくても使えてる:thinking_face:
    lib::run();
    // use_variant_sample(); // extern crateじゃなくてもいいんだ…

    println!("Hello, world!");
}
