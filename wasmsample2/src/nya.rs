pub struct Test {
    pub v: i32,
}

impl Test {
    pub fn new() -> Self {
        println!("hello");
        Test { v: 4 }
    }
}
