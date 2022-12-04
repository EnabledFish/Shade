use std::str::FromStr;

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
