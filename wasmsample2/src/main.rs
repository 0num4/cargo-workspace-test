// mod cargo_workspaces_sample;
extern crate add_one;

fn main() {
    println!("Hello, world!");
    let mut a = add_one::add_one(1);
    println!("Hello, world! {}", a);
}
