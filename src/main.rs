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
    let sleep_duration =
        Duration::new(args.value_of("seconds").unwrap().parse::<u64>().unwrap(), 0);
    println!(
        "Setting background from {} every {} seconds",
        input_dir.display(),
        sleep_duration.as_secs()
    );
    loop {
        if image::set_background_from(&input_dir).is_ok() {
            sleep(sleep_duration);
        }
    }
}
