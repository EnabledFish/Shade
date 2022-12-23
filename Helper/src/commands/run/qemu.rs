use std::str::FromStr;
use clap::{Arg, ArgMatches, Command};
use crate::configs::{DEFAULT_OVMF_ENABLED, DEFAULT_TARGET};
use crate::emulators::qemu::Qemu;
use crate::folders::{file_build, file_resources};
use crate::targets::Target;

pub fn command_run_qemu() -> Command {
    Command::new("qemu")
        .about("Run the qemu emulator.")
        .arg(
            Arg::new("TARGET")
                .help("The target of the image. [x86_64]")
                .short('t')
                .long("target")
                .default_value(DEFAULT_TARGET)
        )
        .arg(
            Arg::new("OVMF")
                .short('o')
                .long("ovmf")
                .help("Enable the Ovmf firmware to support Uefi. [True/False]")
                .default_value(DEFAULT_OVMF_ENABLED)
        )
}

pub fn process_run_qemu(matches: &ArgMatches) {
    let target_name = matches.get_one::<String>("TARGET").unwrap();
    let ovmf_string = matches.get_one::<String>("OVMF").unwrap();

    let target = Target::from_str(target_name.as_str()).unwrap();
    let ovmf = bool::from_str(
        ovmf_string.to_lowercase().as_str()
    ).unwrap();

    println!("[ShadeHelper] The target platform of the emulator: {}.", target);
    println!("[ShadeHelper] If the ovmf firmware is enabled: {}.", ovmf);

    run_run_qemu(target, ovmf);

    println!("[ShadeHelper] Executed the emulator: {}.", target.qemu_command_name());
}

pub fn run_run_qemu(target: Target, ovmf: bool) {
    let mut command = Qemu::from_target(target);
    if ovmf {
        command.bios(file_resources("Ovmf.fd"));
    }
    command.disk_img(file_build("Image.img")).spawn();
}
