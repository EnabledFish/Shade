use std::path::PathBuf;

use clap::{ArgMatches, Command};
use fs_extra::dir::CopyOptions;

use crate::build::bootloader::{command_build_bootloader, process_build_bootloader};
use crate::build::clear::{command_build_clear, process_build_clear};
use crate::build::image::{command_build_image, process_build_image};
use crate::build::kernel::{command_build_kernel, process_build_kernel};
use crate::folder_project_root;

pub mod bootloader;
pub mod image;
pub mod clear;
pub mod kernel;

pub fn command_build() -> Command {
    Command::new("build")
        .about("Build something like the bootloader from the source files.")
        .subcommand_required(true)
        .subcommand(
            command_build_clear()
        )
        .subcommand(
            command_build_image()
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
        "clear" => process_build_clear(subcommand_matches),
        "image" => process_build_image(subcommand_matches),

        "bootloader" => process_build_bootloader(subcommand_matches),
        "kernel" => process_build_kernel(subcommand_matches),
        _ => unreachable!()
    }
}

pub fn folder_build() -> PathBuf {
    let path = folder_project_root().join("Build");
    if !path.exists() {
        fs_err::create_dir(&path).unwrap();
    }
    path
}

pub fn folder_build_object() -> PathBuf {
    let path = folder_build().join("Object");
    if !path.exists() {
        fs_err::create_dir(&path).unwrap();
    }
    path
}

pub fn folder_build_image_source() -> PathBuf {
    let path = folder_build().join("ImageSource");
    if !path.exists() {
        fs_err::create_dir(&path).unwrap();
        fs_extra::dir::copy(
            folder_project_root()
                .join("Base"),
            &path,
            &CopyOptions{
                overwrite: true,
                skip_exist: false,
                buffer_size: 64000,
                copy_inside: false,
                content_only: true,
                depth: 0
            }
        ).unwrap();
    }
    path
}
