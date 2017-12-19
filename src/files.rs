use std::path::PathBuf;
use image::is_valid_image;

pub fn get_files(input_dir: &PathBuf) -> Vec<PathBuf> {
    let input_files = input_dir.read_dir().expect("Failed to read dir");
    return input_files
        .map(|d| d.unwrap())
        .filter(|d| d.path().is_file())
        .map(|d| d.path())
        .filter(is_valid_image)
        .collect();
}
