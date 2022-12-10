use std::ffi::OsString;
use std::path::PathBuf;
use std::str::FromStr;

use clap::{ArgMatches, Command};

use crate::build::{command_build, process_build};
use crate::image::{command_image, process_image};

mod target;
mod image;
mod build;
mod run;
mod cargo;

#[cfg(test)]
mod test;

fn main() {
    process_main_from_argv();
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

fn process_main(matches: &ArgMatches) {
    let (subcommand_name, subcommand_matches) = matches.subcommand().unwrap();
    match subcommand_name {
        "image" => process_image(subcommand_matches),
        "build" => process_build(subcommand_matches),
        _ => unreachable!()
    }
}

fn process_main_from_argv() {
    let command = command_main();
    let matches = command.get_matches();
    process_main(&matches);
}

fn process_main_from_iter<I, T>(iter: I)
    where I: IntoIterator<Item=T>,
          T: Into<OsString> + Clone,
{
    let command = command_main();
    let mut params = vec![OsString::from_str("ShadeHelper").unwrap()];
    params.extend(
        iter.into_iter().map(|param| param.into())
    );
    let matches = command.get_matches_from(params);
    process_main(&matches);
}

pub fn folder_project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
}
