use std::env;
use std::ffi::OsString;
use std::process::Command;

fn sanitized_path(orig_path: OsString) -> OsString {
    let paths = env::split_paths(&orig_path);
    let sanitized_paths = paths.filter(|path| {
        !path
            .components()
            .any(|component| component.as_os_str() == ".rustup")
    });

    env::join_paths(sanitized_paths).expect("Invalid PATH.")
}

pub fn fix_nested_cargo_env(cmd: &mut Command) {
    cmd.env_remove("RUSTC");
    cmd.env_remove("RUSTDOC");
    let orig_path = env::var_os("PATH").unwrap_or_default();
    cmd.env("PATH", sanitized_path(orig_path));
}

pub fn cargo_command(action: &str) -> Command {
    let mut command = Command::new("cargo");
    fix_nested_cargo_env(&mut command);
    command.arg(action);
    command
}
