use std::path::Path;
use std::process::Command;
use crate::folders::folder_root;
use crate::targets::Target;

pub struct Qemu {
    command: Command,
}

impl Qemu {
    pub fn from_target(target: Target) -> Self {
        let mut command = Command::new(target.qemu_command_name());
        command.current_dir(folder_root());

        Self {
            command,
        }
    }

    pub fn bios<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.command.args(["--bios", path.as_ref().to_str().unwrap()]);
        self
    }

    pub fn disk_img<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        let arg = format!("format=raw,file={}", path.as_ref().to_str().unwrap());
        self.command.args(["-drive", arg.as_str()]);
        self
    }

    pub fn spawn(&mut self) {
        self.command.spawn().unwrap().wait().unwrap();
    }
}
