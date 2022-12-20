use std::str::FromStr;
use clap::{Arg, ArgMatches, Command};
use crate::configs::{DEFAULT_FILESYSTEM, DEFAULT_SECTOR_COUNT, DEFAULT_SECTOR_SIZE};
use crate::filesystems::{Filesystem, filesystem_create_image_from_folder};
use crate::folders::{folder_build, folder_build_base};

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

pub fn process_build_image(matches: &ArgMatches) {
    let sector_size = matches.get_one::<String>("SECTOR_SIZE").unwrap();
    let sector_count = matches.get_one::<String>("SECTOR_COUNT").unwrap();
    let filesystem_name = matches.get_one::<String>("FILESYSTEM").unwrap();

    let from_folder = folder_build_base();
    let to_file = folder_build().join("Image.img");

    println!("[ShadeHelper] Creating one image from source folder: \"{}\".", from_folder.to_str().unwrap());
    println!("[ShadeHelper] The count of the sectors: {}.", sector_count);
    println!("[ShadeHelper] The size of each sectors: {}.", sector_size);
    let sector_count = usize::from_str(sector_count.as_str()).unwrap();
    let sector_size = usize::from_str(sector_size.as_str()).unwrap();
    let total_size_kb = sector_count * sector_size / 1024;
    let total_size_string;
    if total_size_kb < 4096 {
        total_size_string = format!("{} KB", total_size_kb)
    } else {
        total_size_string = format!("{} MB", total_size_kb / 1024);
    }
    println!("[ShadeHelper] The size of the total image: {} KB.", total_size_string);

    let filesystem = Filesystem::from_str(filesystem_name).unwrap();
    println!("[ShadeHelper] The filesystem of the image: {}.", filesystem.as_str());

    filesystem_create_image_from_folder(
        &from_folder,
        &to_file,
        sector_size,
        sector_count,
        filesystem,
    );
    println!("[ShadeHelper] Created image file: \"{}\".", to_file.to_str().unwrap());
}