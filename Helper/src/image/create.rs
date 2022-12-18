use std::path::PathBuf;
use std::str::FromStr;

use clap::{Arg, ArgMatches, Command};

use crate::image::filesystem::fat::image_create_fat;
use crate::image::filesystem::Filesystem;

pub fn command_image_create() -> Command {
    Command::new("create")
        .about("Create an image file from a folder.")
        .arg(
            Arg::new("FROM_FOLDER")
                .help("The source folder path.")
                .required(true)
        )
        .arg(
            Arg::new("TO_FILE")
                .help("The generated image file path.")
                .required(true)
        )
        .arg(
            Arg::new("SECTOR_COUNT")
                .help("The count of sectors.")
                .short('c')
                .long("sector_count")
                .default_value("4096")
        )
        .arg(
            Arg::new("SECTOR_SIZE")
                .help("The size of each sectors. (Byte)")
                .short('s')
                .long("sector_size")
                .default_value("512")
        )
        .arg(
            Arg::new("FILESYSTEM")
                .help("The filesystem of the generated image. [Fat]")
                .short('f')
                .long("filesystem")
                .default_value("Fat")
        )
}

pub fn process_image_create(matches: &ArgMatches) {
    let from_folder = matches.get_one::<String>("FROM_FOLDER").unwrap();
    let to_file = matches.get_one::<String>("TO_FILE").unwrap();
    let sector_size = matches.get_one::<String>("SECTOR_SIZE").unwrap();
    let sector_count = matches.get_one::<String>("SECTOR_COUNT").unwrap();
    let filesystem_name = matches.get_one::<String>("FILESYSTEM").unwrap();

    println!("Creating one image from source folder: \"{}\".", from_folder);
    println!("The count of the sectors: {}.", sector_count);
    println!("The size of each sectors: {}.", sector_size);
    let sector_count = usize::from_str(sector_count.as_str()).unwrap();
    let sector_size = usize::from_str(sector_size.as_str()).unwrap();
    println!("The size of the total image: {}.", sector_count * sector_size);

    let filesystem = Filesystem::from_str(filesystem_name).unwrap();
    println!("The filesystem of the image: {}.", filesystem.as_str());

    image_create(
        &from_folder.into(),
        &to_file.into(),
        sector_size,
        sector_count,
        filesystem,
    );
    println!("Created image file: \"{}\".", to_file);
}

pub fn image_create(
    from_folder: &PathBuf,
    to_file: &PathBuf,
    sector_size: usize,
    sector_count: usize,
    filesystem: Filesystem,
) {
    match filesystem {
        Filesystem::Fat => image_create_fat(from_folder, to_file, sector_size, sector_count)
    }
}