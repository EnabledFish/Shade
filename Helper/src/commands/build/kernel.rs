use std::str::FromStr;

use clap::{Arg, ArgMatches, Command};

use crate::cargo::cargo_command;
use crate::configs::{DEFAULT_DEBUG, DEFAULT_TARGET};
use crate::folders::{file_build_base_shade_kernel, file_build_objects, folder_build_objects};
use crate::targets::Target;

pub fn command_build_kernel() -> Command {
    Command::new("kernel")
        .about("Build the kernel target to elf.")
        .arg(
            Arg::new("TARGET")
                .help("The target platform of the kernel. [x86_64]")
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

pub fn process_build_kernel(matches: &ArgMatches) {
    let target_name = matches.get_one::<String>("TARGET").unwrap();
    let debug_name = matches.get_one::<String>("DEBUG").unwrap();

    let target = Target::from_str(target_name.as_str()).unwrap();
    let debug = bool::from_str(
        debug_name.to_lowercase().as_str()
    ).unwrap();

    println!("[ShadeHelper] The target platform of the kernel: {}.", target.as_str());
    println!("[ShadeHelper] If the debug mode is enabled: {}.", debug);

    build_kernel(target, debug);

    println!("[ShadeHelper] Built one project: Shade-Kernel.");
}

pub fn build_kernel(target: Target, debug: bool) {
    let target_fullname = format!("{}-unknown-none", target.as_str());
    let mut command = cargo_command("build");
    command.args(["--bin", "shade-kernel"]);
    command.args(["--target", target_fullname.as_str()]);
    command.arg("-Zbuild-std=core");
    if !debug {
        command.arg("--release");
    }
    command.arg("-Zunstable-options");
    command.args(["--out-dir", folder_build_objects().to_str().unwrap()]);
    command.spawn().unwrap().wait().unwrap();
    fs_err::copy(
        file_build_objects("shade-kernel"),
        file_build_base_shade_kernel("Kernel.sk"),
    ).unwrap();
}
