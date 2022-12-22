use clap::{ArgMatches, Command};
use crate::commands::build::base::{command_build_base, process_build_base};
use crate::commands::build::bootloader::{command_build_bootloader, process_build_bootloader};
use crate::commands::build::clear::{command_build_clear, process_build_clear};
use crate::commands::build::image::{command_build_image, process_build_image};
use crate::commands::build::kernel::{command_build_kernel, process_build_kernel};

pub mod clear;
pub mod bootloader;
pub mod kernel;
pub mod driver;
pub mod userlands;
pub mod image;
pub mod base;

pub fn command_build() -> Command {
    Command::new("build")
        .about("Build something like the bootloader from the source files.")
        .subcommand_required(true)
        .subcommand(
            command_build_image()
        )
        .subcommand(
            command_build_base()
        )
        .subcommand(
        command_build_clear()
        )
        .subcommand(
            command_build_bootloader()
        )
        .subcommand(
            command_build_kernel()
        )
}

pub fn process_build(matches: &ArgMatches) {
    let (subcommand_name, subcommand_matches) = matches.subcommand().unwrap();
    match subcommand_name {
        "image" => process_build_image(subcommand_matches),
        "base" => process_build_base(subcommand_matches),
        "clear" => process_build_clear(subcommand_matches),
        "bootloader" => process_build_bootloader(subcommand_matches),
        "kernel" => process_build_kernel(subcommand_matches),
        _ => unreachable!()
    }
}
