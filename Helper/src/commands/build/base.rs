use clap::{Arg, ArgMatches, Command};
use fs_err::remove_dir_all;

use crate::folders::{folder_build, folder_build_base};

pub fn command_build_base() -> Command {
    Command::new("base")
        .about("Build the image source base.")
        .arg(
            Arg::new("REMOVE")
            .short('f')
            .long("remove")
            .help("Remove the old image source base.")
            .num_args(0..0)
        )
}

pub fn process_build_base(matches: &ArgMatches) {
    let build_base = folder_build().join("Base");
    if build_base.exists() {
        if matches.contains_id("REMOVE") {
            remove_dir_all(&build_base).unwrap();
            println!("[ShadeHelper] One folder has been removed: {}", build_base.to_str().unwrap());
            folder_build_base();
        } else {
            panic!("The image source base is already built, use -r or --remove to remove it and build then.");
        }
    } else {
        folder_build_base();
    }
}
