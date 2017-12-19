use std::thread::sleep;
use std::time::Duration;
use std::path::PathBuf;

extern crate rand;

#[macro_use]
extern crate clap;

mod files;
mod image;
mod args;

fn main() {
    let args = args::build();
    let input_dir = PathBuf::from(args.value_of("directory").unwrap());
    let input_files = files::get_files(&input_dir);
    if input_files.is_empty() {
        panic!("Could not find any valid files in directory {}", input_dir.display());
    }
    let sleep_duration = Duration::new(10, 0);

    loop {
        if image::set_background_from(&input_dir).is_ok() {
            sleep(sleep_duration);
        }
    }
}
