use clap::{Arg, Command};
use crate::configs::{DEFAULT_FILESYSTEM, DEFAULT_SECTOR_COUNT, DEFAULT_SECTOR_SIZE};

pub fn command_build_image() -> Command {
    Command::new("image")
        .about("Build the image from the image source.")
        .arg(
            Arg::new("SECTOR_SIZE")
                .help("The size of each sectors. (Byte)")
                .short('s')
                .long("sector_size")
                .default_value(DEFAULT_SECTOR_SIZE)
        )
        .arg(
            Arg::new("SECTOR_COUNT")
                .help("The count of sectors.")
                .short('c')
                .long("sector_count")
                .default_value(DEFAULT_SECTOR_COUNT)
        )
        .arg(
            Arg::new("FILESYSTEM")
                .help("The filesystem of the generated image. [Fat]")
                .short('f')
                .long("filesystem")
                .default_value(DEFAULT_FILESYSTEM)
        )
}
