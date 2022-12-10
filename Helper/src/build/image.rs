use clap::{Arg, ArgMatches, Command};

use crate::build::{folder_build, folder_build_image_source};
use crate::process_main_from_iter;

pub fn command_build_image() -> Command {
    Command::new("image")
        .about("Build the image from the image source.")
        .arg(
            Arg::new("SECTOR_COUNT")
                .help("The count of sectors.")
                .short('c')
                .long("sector_count")
                .default_value("512")
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

pub fn process_build_image(matches: &ArgMatches) {
    let from = folder_build_image_source();
    let to = folder_build().join("Image.img");

    let mut params = vec![
        "image", "create",
        from.to_str().unwrap(),
        to.to_str().unwrap(),
    ];
    matches.get_one::<String>("SECTOR_SIZE").map(|sector_size| {
        params.push("-s");
        params.push(sector_size)
    });
    matches.get_one::<String>("SECTOR_COUNT").map(|sector_count| {
        params.push("-c");
        params.push(sector_count)
    });
    matches.get_one::<String>("FILESYSTEM").map(|filesystem_name| {
        params.push("-f");
        params.push(filesystem_name)
    });

    process_main_from_iter(params);
}
