use std::str::FromStr;
use clap::{Arg, ArgMatches, Command};
use fs_err::remove_dir_all;

use crate::folders::{folder_build, folder_build_base};

pub fn command_build_base() -> Command {
    Command::new("base")
        .about("Build the image source base.")
        .arg(
            Arg::new("REMOVE")
                .short('r')
                .long("remove")
                .help("Remove the old image source base. [True/False]")
                .default_value("false")
        )
}

pub fn process_build_base(matches: &ArgMatches) {
    let build_base = folder_build().join("Base");
    if build_base.exists() {
        if bool::from_str(
            matches.get_one::<String>("REMOVE")
                .unwrap().to_lowercase().as_str()
        ).unwrap() {
            remove_dir_all(&build_base).unwrap();
            println!("[ShadeHelper] Removed one folder: \"{}\".", build_base.to_str().unwrap());
            folder_build_base();
        } else {
            println!("[ShadeHelper] The image source base is already existed, using '-r true' or '--remove true' to remove it and build then.");
        }
    } else {
        folder_build_base();
    }
}
