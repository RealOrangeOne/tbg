use std::env;
use std::path::PathBuf;
use std::fs::DirEntry;
use std::thread::sleep;
use std::time::Duration;

extern crate rand;

use rand::Rng;

fn select_file(input_dir: &PathBuf) -> DirEntry {
    let input_files = input_dir.read_dir().expect("Failed to read dir");
    let mut collected_input_files : Vec<DirEntry> = input_files
        .map(|d| d.unwrap())
        .filter(|d| d.path().is_file())
        .collect();
    rand::thread_rng().shuffle(&mut collected_input_files);
    return collected_input_files.pop().expect("Failed to get input file");
}

fn main() {
    let input_dir = PathBuf::from(env::var("BG_DIR").expect("Missing environment variable BG_DIR"));
    let sleep_duration = Duration::new(10, 0);
    loop {
        println!("{:?}", select_file(&input_dir));
        sleep(sleep_duration);
    }
}
