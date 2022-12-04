use std::path::PathBuf;

use clap::{ArgMatches, Command};

use crate::build::bootloader::{command_build_bootloader, process_build_bootloader};
use crate::folder_project_root;

pub mod bootloader;
pub mod image;

pub fn command_build() -> Command {
    Command::new("build")
        .about("Build something like the bootloader from the source files.")
        .subcommand(
            command_build_bootloader()
        )
}

pub fn process_build(matches: &ArgMatches) {
    let (subcommand_name, subcommand_matches) = matches.subcommand().unwrap();
    match subcommand_name {
        "bootloader" => process_build_bootloader(subcommand_matches),
        _ => unreachable!()
    }
}

pub fn folder_build() -> PathBuf {
    folder_project_root().join("Build")
}

pub fn folder_build_object() -> PathBuf {
    folder_build().join("Object")
}

pub fn folder_build_image_source() -> PathBuf {
    folder_build().join("ImageSource")
}
