use clap::{App, AppSettings, Arg, ArgMatches};


pub fn build() -> ArgMatches<'static> {
    return App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::StrictUtf8)
        .arg(
            Arg::with_name("directory")
                .required(true)
        ).get_matches();
}
