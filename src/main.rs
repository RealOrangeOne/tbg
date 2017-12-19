use std::env;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use std::process::{Command, Stdio};

extern crate rand;

use rand::Rng;

fn is_valid_image(img: &PathBuf) -> bool {
    return Command::new("feh")
        .arg("--list")
        .arg(img.display().to_string())
        .stdout(Stdio::null())
        .output()
        .expect("Failed to get output")
        .status
        .success();
}

fn get_files(input_dir: &PathBuf) -> Vec<PathBuf> {
    let input_files = input_dir.read_dir().expect("Failed to read dir");
    return input_files
        .map(|d| d.unwrap())
        .filter(|d| d.path().is_file())
        .map(|d| d.path())
        .filter(is_valid_image)
        .collect();
}

fn set_background_from(input_dir: &PathBuf) -> Result<(), ()> {
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

fn main() {
    let input_dir = PathBuf::from(env::var("BG_DIR").expect("Missing environment variable BG_DIR"));
    let input_files = get_files(&input_dir);
    if input_files.is_empty() {
        panic!("Could not find any valid files in directory {}", input_dir.display());
    }
    let sleep_duration = Duration::new(10, 0);

    loop {
        if set_background_from(&input_dir).is_ok() {
            sleep(sleep_duration);
        }
    }
}
