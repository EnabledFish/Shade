use clap::{ArgMatches, Command};
use crate::commands::build::bootloader::uefi::{command_build_bootloader_uefi, process_build_bootloader_uefi};

pub mod uefi;

pub fn command_build_bootloader() -> Command {
    Command::new("bootloader")
        .about("Build the bootloader into the required target.")
        .subcommand_required(true)
        .subcommand(
            command_build_bootloader_uefi()
        )
}

pub fn process_build_bootloader(matches: &ArgMatches) {
    let (subcommand_name, subcommand_matches) = matches.subcommand().unwrap();
    match subcommand_name {
        "uefi" => process_build_bootloader_uefi(subcommand_matches),
        _ => unreachable!()
    }
}

