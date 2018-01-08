use std::path::PathBuf;
use std::process::Command;
use std::fs::copy;

use clap::ArgMatches;

const BG_POINTER_PATH: &str = "/tmp/.wallpaper";
const CMD_REPLACEMENT: &str = "/_";

pub fn handle_post_image(new_bg: PathBuf, args: &ArgMatches) {
    copy(&new_bg, BG_POINTER_PATH).expect("Failed to copy background pointer");
    let cmd_result = args.value_of("command");
    if cmd_result.is_some() {
        let cmd = cmd_result.unwrap().replace(CMD_REPLACEMENT, &new_bg.display().to_string());
        println!("Executing '{}'.", cmd);
        Command::new("bash").arg("-c").arg(cmd).output().expect("Failed to run post command");
    }
}
