use std::thread::sleep;
use std::time::Duration;
use std::path::PathBuf;


extern crate rand;

#[macro_use]
extern crate clap;

mod files;
mod image;
mod args;
mod post;

fn main() {
    let args = args::build();
    let input_dir = PathBuf::from(args.value_of("directory").unwrap());
    let sleep_duration =
        Duration::new(args.value_of("minutes").unwrap().parse::<u64>().unwrap() * 60, 0);
    println!(
        "Selecting background from {} every {} minutes",
        input_dir.display(),
        sleep_duration.as_secs() / 60
    );
    loop {
        if image::set_background_from(&input_dir, &args).is_ok() {
            sleep(sleep_duration);
        }
    }
}
