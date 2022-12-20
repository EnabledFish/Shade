use clap::{Arg, ArgMatches, Command};

use crate::folders::folder_build;

pub fn command_build_clear() -> Command {
    Command::new("clear")
        .about("Clear the building temporary folder.")
        .arg(
            Arg::new("RECREATE")
                .short('r')
                .long("recreate")
                .help("Recreate the folder.")
                .num_args(0..0)
        )
}

pub fn process_build_clear(matches: &ArgMatches) {
    fs_err::remove_dir_all(folder_build()).unwrap();

    if matches.contains_id("RECREATE") {
        folder_build();
    }
}
