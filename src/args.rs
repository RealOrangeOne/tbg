use clap::{App, AppSettings, Arg, ArgMatches};
use std::path::PathBuf;
use files::get_files;

fn validate_path(input: String) -> Result<(), String> {
    let path = PathBuf::from(input);
    if !path.is_dir() {
        return Err(format!("Could not find directory at {}", path.display()));
    }
    let files = get_files(&path);
    if files.is_empty() {
        return Err(format!("Failed to find valid images in {}", path.display()));
    }
    return Ok(());
}

pub fn build() -> ArgMatches<'static> {
    return App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::StrictUtf8)
        .arg(
            Arg::with_name("directory")
                .required(true)
                .validator(validate_path)
        ).get_matches();
}
