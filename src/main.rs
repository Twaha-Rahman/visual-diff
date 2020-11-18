use lcs_image_diff::compare;
use std::env;

fn main() {
    for item in env::args() {
        println!("{}", item);
    }
    let mut before = image::open("samples/before.png").unwrap();
    let mut after = image::open("samples/after.png").unwrap();

    let diff = compare(&mut before, &mut after, 100.0 / 256.0).unwrap();

    before.save("output/marked_before.png").unwrap();
    after.save("output/marked_after.png").unwrap();
    diff.save("output/diff.png").unwrap();
}
