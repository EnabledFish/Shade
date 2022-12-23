use clap::{ArgMatches, Command};
use crate::commands::run::qemu::{command_run_qemu, process_run_qemu};

pub mod qemu;

pub fn command_run() -> Command {
    Command::new("run")
        .about("Run the emulator.")
        .subcommand_required(true)
        .subcommand(
            command_run_qemu()
        )
}

pub fn process_run(matches: &ArgMatches) {
    let (subcommand_name, subcommand_matches) = matches.subcommand().unwrap();
    match subcommand_name {
        "qemu" => process_run_qemu(subcommand_matches),
        _ => unreachable!(),
    }
}
