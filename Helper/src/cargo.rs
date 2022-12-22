use std::env;
use std::ffi::OsString;
use std::process::Command;
use crate::folders::folder_root;

fn cargo_sanitized_path(orig_path: OsString) -> OsString {
    let paths = env::split_paths(&orig_path);
    let sanitized_paths = paths.filter(|path| {
        !path
            .components()
            .any(|component| component.as_os_str() == ".rustup")
    });

    env::join_paths(sanitized_paths).unwrap()
}

fn cargo_fix_env(cmd: &mut Command) {
    cmd.env_remove("RUSTC");
    cmd.env_remove("RUSTDOC");
    let orig_path = env::var_os("PATH").unwrap_or_default();
    cmd.env("PATH", cargo_sanitized_path(orig_path));
}

pub fn cargo_command(action: &str) -> Command {
    let mut command = Command::new("cargo");
    cargo_fix_env(&mut command);
    command.arg("+nightly");
    command.arg(action);
    command.current_dir(folder_root());
    command
}
