use std::str::FromStr;
use clap::{Arg, ArgMatches, Command};
use fs_err::read_to_string;
use crate::commands::process_main_from_iter;
use crate::configs::{DEFAULT_BATCH_FILE, DEFAULT_BATCH_RUN};
use crate::folders::file_root;


pub fn command_batch() -> Command {
    Command::new("batch")
        .about("Run Shade Helper batch file.")
        .arg(
            Arg::new("FILE")
                .short('f')
                .long("file")
                .help("The path of the batch file.")
                .default_value(DEFAULT_BATCH_FILE)
        )
        .arg(
            Arg::new("RUN")
                .short('r')
                .long("run")
                .help("Execute the \"run\" command")
                .default_value(DEFAULT_BATCH_RUN)
        )
}

pub fn process_batch(matches: &ArgMatches) {
    let file_name = matches.get_one::<String>("FILE").unwrap();
    let run_string = matches.get_one::<String>("RUN").unwrap();

    let file_path = file_root(file_name);
    let run = bool::from_str(
        run_string.to_lowercase().as_str()
    ).unwrap();

    println!("[ShadeHelper] Running batch file: \"{}\".", file_path.to_str().unwrap());
    println!("[ShadeHelper] If the \"run\" command will be executed: {}.", run);

    if file_path.is_file() {
        let buffer = read_to_string(file_path).unwrap();
        let lines = buffer.lines();
        for line in lines {
            if !line.starts_with("#") && !line.trim().is_empty()
                && (!line.starts_with("run") || run) {
                println!("[ShadeHelper] Running command: \"ShadeHelper {}\".", line);
                let params = line.split(" ");
                process_main_from_iter(params);
            }
        }
    } else {
        panic!("The file does not exist: {}.", file_path.to_str().unwrap());
    }
}
