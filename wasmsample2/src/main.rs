// mod cargo_workspaces_sample;
extern crate add_one;
extern crate wasmsample2;

fn main() {
    println!("Hello, world!");
    wasmsample2::run();
    let mut a = add_one::add_one(1);
    println!("Hello, world! {}", a);
}
