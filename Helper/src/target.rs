use std::str::FromStr;

#[derive(Debug)]
pub enum Target {
    X86_64
}

impl Target {
    pub fn as_str(&self) -> &str {
        match self {
            Target::X86_64 => "x86_64",
        }
    }

    pub fn as_uefi_boot_file_name(&self) -> &str {
        match self {
            Target::X86_64 => "Bootx64.efi"
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
