use lcs_image_diff::compare;

pub fn diff_two_images(first_file_path: &str, second_file_path: &str, output_dir: &str) {
    let file_name: Vec<&str> = first_file_path.split("/").collect();
    let file_name = file_name[file_name.len() - 1];

    let mut before = image::open(&first_file_path).unwrap();
    let mut after = image::open(&second_file_path).unwrap();

    let diff = compare(&mut before, &mut after, 100.0 / 256.0).unwrap();

    before
        .save(format!("{}/{}_marked_before.png", &output_dir, &file_name))
        .unwrap();
    after
        .save(format!("{}/{}_marked_after.png", &output_dir, &file_name))
        .unwrap();
    diff.save(format!("{}/{}_diff.png", &output_dir, &file_name))
        .unwrap();
}
