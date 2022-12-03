use clap::Command;

use crate::image::{command_image, process_image};

mod image;

fn main() {
    let command = command_main();
    let matches = command.get_matches();

    let (subcommand_name, subcommand_matches) = matches.subcommand().unwrap();
    match subcommand_name {
        "image" => process_image(subcommand_matches),
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
}
