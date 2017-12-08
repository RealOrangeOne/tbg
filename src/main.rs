use std::env;
use std::path::PathBuf;
use std::fs::DirEntry;
use std::thread::sleep;
use std::time::Duration;
use std::process::Command;

extern crate rand;

use rand::Rng;

fn select_file(input_dir: &PathBuf) -> PathBuf {
    let input_files = input_dir.read_dir().expect("Failed to read dir");
    let mut collected_input_files: Vec<DirEntry> =
        input_files.map(|d| d.unwrap()).filter(|d| d.path().is_file()).collect();
    rand::thread_rng().shuffle(&mut collected_input_files);
    return collected_input_files.pop().expect("Failed to get input file").path();
}

fn main() {
    let input_dir = PathBuf::from(env::var("BG_DIR").expect("Missing environment variable BG_DIR"));
    let sleep_duration = Duration::new(60, 0);
    loop {
        let input_file = select_file(&input_dir);
        println!("Setting background to {}...", input_file.display());
        let output = Command::new("feh")
            .arg("--bg-fill")
            .arg(input_file.display().to_string())
            .output()
            .expect("Command failed");
        if !output.status.success() {
            eprintln!("{}", String::from_utf8(output.stderr).unwrap());
            eprintln!("Execution failed, Retrying...");
            continue;
        }
        sleep(sleep_duration);
    }
}
