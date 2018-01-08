use std::path::PathBuf;
use std::process::Command;
use std::fs::copy;

use clap::ArgMatches;

const BG_POINTER_PATH: &str = "/tmp/.wallpaper";

pub fn handle_post_image(new_bg: PathBuf, args: &ArgMatches) {
    copy(new_bg, BG_POINTER_PATH).expect("Failed to copy background pointer");
    let cmd_result = args.value_of("command");
    if cmd_result.is_some() {
        Command::new("bash").arg("-c").arg(cmd_result.unwrap()).output().expect(
            "Failed to run post command"
        );
    }
}
