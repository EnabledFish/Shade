use std::str::FromStr;
use clap::{Arg, ArgMatches, Command};
use crate::cargo::cargo_command;
use crate::configs::{DEFAULT_DEBUG, DEFAULT_TARGET};
use crate::folders::{file_build_base_efi_boot_for_uefi, file_build_objects, folder_build_objects};
use crate::targets::Target;

pub fn command_build_bootloader_uefi() -> Command {
    Command::new("uefi")
        .about("Build the bootloader based on uefi.")
        .arg(
            Arg::new("TARGET")
                .help("The target platform of the uefi bootloader. [x86_64]")
                .short('t')
                .long("target")
                .default_value(DEFAULT_TARGET)
        )
        .arg(
            Arg::new("DEBUG")
                .short('d')
                .long("debug")
                .help("Enable the debug mode. [True/False]")
                .default_value(DEFAULT_DEBUG)
        )
}

pub fn process_build_bootloader_uefi(matches: &ArgMatches) {
    let target_name = matches.get_one::<String>("TARGET").unwrap();
    let debug_name = matches.get_one::<String>("DEBUG").unwrap();

    let target = Target::from_str(target_name.as_str()).unwrap();
    let debug = bool::from_str(
        debug_name.to_lowercase().as_str()
    ).unwrap();

    println!("[ShadeHelper] The target platform of the uefi bootloader: {}.", target.as_str());
    println!("[ShadeHelper] If the debug mode is enabled: {}.", debug);

    build_bootloader_uefi(target, debug);
}

pub fn build_bootloader_uefi(target: Target, debug: bool) {
    let target_fullname = format!("{}-unknown-uefi", target.as_str());
    let mut command = cargo_command("build");
    command.args(["--bin", "shade-bootloader-uefi"]);
    command.args(["--target", target_fullname.as_str()]);
    command.arg("-Zbuild-std=core,alloc");
    if !debug {
        command.arg("--release");
    }
    command.arg("-Zunstable-options");
    command.args(["--out-dir", folder_build_objects().to_str().unwrap()]);
    command.spawn().unwrap().wait().unwrap();
    fs_err::copy(
        file_build_objects("shade-bootloader-uefi.efi"),
        file_build_base_efi_boot_for_uefi(target.as_uefi_boot_file_name()),
    ).unwrap();
}
