use lcs_image_diff::compare;
use std::env;

#[derive(Debug)]
struct CustomError(String);

fn main() {
    for item in env::args() {
        println!("{}", item);
    }
    let mut before = image::open("before.png").unwrap();
    let mut after = image::open("after.png").unwrap();

    let diff = compare(&mut before, &mut after, 100.0 / 256.0).unwrap();

    before.save("marked_before.png").unwrap();
    after.save("marked_after.png").unwrap();
    diff.save("diff.png").unwrap();
}
