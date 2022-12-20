use std::ffi::OsString;
use std::str::FromStr;
use clap::{ArgMatches, Command};
use crate::commands::build::{command_build, process_build};

pub mod build;
pub mod run;

pub fn command_main() -> Command {
    Command::new("ShadeHelper")
        .about("Shade Helper is helper for operating system engineering, which is created for Shade System originally.")
        .subcommand_required(true)
        .subcommand(command_build())
}

pub fn process_main(matches: &ArgMatches) {
    let (subcommand_name, subcommand_matches) = matches.subcommand().unwrap();
    match subcommand_name {
        "build" => process_build(subcommand_matches),
        _ => unreachable!()
    }
}

pub fn process_main_from_argv() {
    let command = command_main();
    let matches = command.get_matches();
    process_main(&matches);
}

pub fn process_main_from_iter<I, T>(iter: I)
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

