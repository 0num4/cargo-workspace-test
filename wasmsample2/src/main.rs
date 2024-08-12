// mod cargo_workspaces_sample;
extern crate add_one;
mod lib;
fn main() {
    println!("Hello, world!");
    lib::Test::new();
    let mut a = add_one::add_one(1);
    println!("Hello, world! {}", a);
}
