use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum Target {
    X86_64
}

impl Target {
    pub fn qemu_command_name(&self) -> String {
        format!("qemu-system-{}", self)
    }

    pub fn uefi_target_fullname(&self) -> String {
        format!("{}-unknown-uefi", self)
    }

    pub fn none_target_fullname(&self) -> String {
        format!("{}-unknown-none", self)
    }

    pub fn uefi_boot_file_name(&self) -> &str {
        match self {
            Target::X86_64 => "Bootx64.efi",
        }
    }
}

impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "x86_64" => Ok(Target::X86_64),
            _ => Err(format!("Illegal target name."))
        }
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}
