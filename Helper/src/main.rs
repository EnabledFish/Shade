#![allow(dead_code)]

use crate::commands::process_main_from_argv;

mod tests;
mod commands;
mod filesystems;
mod folders;
mod configs;
mod cargo;
mod emulators;
mod targets;

fn main() {
    process_main_from_argv();
}
