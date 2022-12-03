use clap::{ArgMatches, Command};

use crate::image::create::{command_image_create, process_image_create};

pub mod create;
pub mod filesystem;

pub fn command_image() -> Command {
    Command::new("image")
        .about("Manage .img files, like creating one image from one folder.")
        .subcommand_required(true)
        .subcommand(
            command_image_create()
        )
}

pub fn process_image(matches: &ArgMatches) {
    let (subcommand_name, subcommand_matches) = matches.subcommand().unwrap();
    match subcommand_name {
        "create" => process_image_create(subcommand_matches),
        _ => unreachable!()
    }
}
