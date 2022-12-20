use std::path::PathBuf;
use std::str::FromStr;
use crate::filesystems::fat::fat_create_image_from_folder;

pub mod fat;

#[derive(Debug)]
pub enum Filesystem {
    Fat,
}

impl Filesystem {
    pub fn as_str(&self) -> &str {
        match self {
            Filesystem::Fat => "Fat",
        }
    }
}

impl FromStr for Filesystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fat" => Ok(Filesystem::Fat),
            _ => Err(format!("Illegal filesystem name."))
        }
    }
}

pub fn filesystem_create_image_from_folder(
    from_folder: &PathBuf,
    to_file: &PathBuf,
    sector_size: usize,
    sector_count: usize,
    filesystem: Filesystem,
) {
    match filesystem {
        Filesystem::Fat => fat_create_image_from_folder(from_folder, to_file, sector_size, sector_count)
    }
}
