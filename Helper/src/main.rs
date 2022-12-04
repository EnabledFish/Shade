use std::path::PathBuf;

use clap::Command;

use crate::build::{command_build, process_build};
use crate::image::{command_image, process_image};

mod target;
mod image;
mod build;
mod run;
mod cargo;

fn main() {
    let command = command_main();
    let matches = command.get_matches();

    let (subcommand_name, subcommand_matches) = matches.subcommand().unwrap();
    match subcommand_name {
        "image" => process_image(subcommand_matches),
        "build" => process_build(subcommand_matches),
        _ => unreachable!()
    }
}

fn command_main() -> Command {
    Command::new("ShadeHelper")
        .about("Shade Helper is helper for operating system engineering, which is created for Shade OS originally.")
        .subcommand_required(true)
        .subcommand(
            command_image()
        )
        .subcommand(
            command_build()
        )
}

pub fn folder_project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}
