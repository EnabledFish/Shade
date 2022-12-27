use crate::std::*;
use crate::fields::ShadeConfigFields;
use crate::object::ShadeConfigObject;
use crate::path::ShadeConfigPath;

#[derive(Clone, Default)]
pub struct ShadeConfigFile {
    root: ShadeConfigFields,
}

impl ShadeConfigFile {
    pub fn parse(&mut self, content: String) -> Result<(), usize> {
        for (line_count, line) in content.lines().into_iter().enumerate() {
            let line_count = line_count + 1;
            let line = line.trim();
            if line.starts_with("#") || line.is_empty() {
                continue;
            }
            if line.contains("=") {
                let mut path_and_value = line.splitn(2, '=');
                let path_string = path_and_value.next().unwrap();
                let value_string = path_and_value.next().unwrap();
                let path = ShadeConfigPath::parse(path_string.to_string())
                    .ok_or(line_count)?;
                let value = ShadeConfigObject::parse(value_string.to_string())
                    .ok_or(line_count)?;
                self.root.set_path(path, value).ok_or(line_count)?;
            } else {
                return Err(line_count);
            }
        }
        Ok(())
    }

    pub fn get_root(&mut self) -> &mut ShadeConfigFields {
        &mut self.root
    }
}

impl Display for ShadeConfigFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "File {:?}", self.root)
    }
}

impl Debug for ShadeConfigFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "File {:?}", self.root)
    }
}
