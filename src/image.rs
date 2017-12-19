use std::process::{Command, Stdio};
use files::get_files;
use std::path::PathBuf;

use rand;
use rand::Rng;


pub fn is_valid_image(img: &PathBuf) -> bool {
    return Command::new("feh")
        .arg("--list")
        .arg(img.display().to_string())
        .stdout(Stdio::null())
        .output()
        .expect("Failed to get output")
        .status
        .success();
}

pub fn set_background_from(input_dir: &PathBuf) -> Result<(), ()> {
    let mut input_files = get_files(input_dir);
    if input_files.is_empty() {
        panic!("Could not find any valid files in directory {}", input_dir.display());
    }
    rand::thread_rng().shuffle(&mut input_files);
    let input_file = input_files.pop().expect("Failed to get input file");
    println!("Setting background to {}...", input_file.display());
    let output = Command::new("feh")
        .arg("--bg-fill")
        .arg(input_file.display().to_string())
        .output()
        .expect("Command failed");
    if !output.status.success() {
        eprintln!("{}", String::from_utf8(output.stderr).unwrap());
        eprintln!("Execution failed, Retrying...");
        return Err(());
    }
    return Ok(());
}
