mod lib;
pub fn main() {
    lib::render_image();
    lib::run();
    // use_variant_sample(); // extern crateじゃなくてもいいんだ…

    println!("Hello, world!");
}
