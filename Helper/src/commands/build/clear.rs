use clap::{ArgMatches, Command};

use crate::folders::folder_build;

pub fn command_build_clear() -> Command {
    Command::new("clear")
        .about("Clear the building temporary folder.")
}

pub fn process_build_clear(_matches: &ArgMatches) {
    let path = folder_build();
    fs_err::remove_dir_all(&path).unwrap();
    println!("[ShadeHelper] Cleared one folder: \"{}\".", path.to_str().unwrap());
}
