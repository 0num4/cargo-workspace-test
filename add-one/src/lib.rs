enum VariantSample {
    NoDataVariant,
    VecHasXandY { x: i32, y: i32 },
    Write(String),
    Color(i32, i32, i32),
}

pub fn useVariantSample() {
    let variantInstance = VariantSample::VecHasXandY { x: 1, y: 2 };
    match variantInstance {
        // matchの後は変数(instance)を入れる。matchのifはinstanceの元のフィールドを使う。
        VariantSample::NoDataVariant => println!("NoDataVariant"),
        VariantSample::VecHasXandY { x, y } => println!("VecHasXandY x: {}, y: {}", x, y),
        VariantSample::Write(s) => println!("Write {}", s),
        VariantSample::Color(r, g, b) => println!("Color r: {}, g: {}, b: {}", r, g, b),
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
