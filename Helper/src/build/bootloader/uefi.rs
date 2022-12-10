use std::str::FromStr;

use clap::{Arg, ArgMatches, Command};

use crate::build::{folder_build_image_source, folder_build_object};
use crate::cargo::cargo_command;
use crate::folder_project_root;
use crate::target::Target;

pub fn command_build_bootloader_uefi() -> Command {
    Command::new("uefi")
        .about("Build the bootloader based on uefi.")
        .arg(
            Arg::new("TARGET")
                .help("The target platform of the uefi bootloader. [x86_64]")
                .short('t')
                .long("target")
                .default_value("x86_64")
        )
}

pub fn process_build_bootloader_uefi(matches: &ArgMatches) {
    let target_name = matches.get_one::<String>("TARGET").unwrap();
    let target = Target::from_str(target_name.as_str()).unwrap();
    println!("The target platform of the uefi bootloader: {}.", target.as_str());
    build_bootloader_uefi(target);
}

pub fn build_bootloader_uefi(target: Target) {
    let target_fullname = format!("{}-unknown-uefi", target.as_str());
    let mut command = cargo_command("build");
    command.args(["--bin", "shade-bootloader-uefi"]);
    command.args(["--target", target_fullname.as_str()]);
    command.arg("-Zbuild-std=core,alloc");
    command.arg("-Zunstable-options");
    command.args(["--out-dir", folder_build_object().to_str().unwrap()]);
    command.current_dir(folder_project_root());
    command.spawn().unwrap().wait().unwrap();
    fs_err::copy(
        folder_build_object()
            .join("shade-bootloader-uefi.efi"),
        folder_build_image_source()
            .join("EFI")
            .join("Boot")
            .join(target.as_uefi_boot_file_name()),
    ).unwrap();
}
