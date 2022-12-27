use crate::std::*;
use hashbrown::HashMap;
use crate::object::ShadeConfigObject;
use crate::path::ShadeConfigPath;

#[derive(Default, Clone)]
pub struct ShadeConfigFields {
    fields: HashMap<String, ShadeConfigObject>,
}

impl ShadeConfigFields {
    pub fn new() -> Self{
        Self{
            fields: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: &String) -> &mut ShadeConfigObject {
        if !self.fields.contains_key(key) {
            self.init(key.clone());
        }
        self.fields.get_mut(key).unwrap()
    }

    pub fn set(&mut self, key: String, value: ShadeConfigObject) -> Option<ShadeConfigObject> {
        self.fields.insert(key, value)
    }

    pub fn set_path(&mut self, mut path: ShadeConfigPath, value: ShadeConfigObject) -> Option<Option<ShadeConfigObject>> {
        if path.len() == 1 {
            Some(self.set(path.pop().unwrap(), value))
        } else {
            self.get(&path.pop_front())
                .field_set_path(path,value)
        }
    }

    pub fn get_path(&mut self, mut path: ShadeConfigPath) -> Option<&mut ShadeConfigObject> {
        if path.len() == 1 {
            Some(self.get(&path.pop().unwrap()))
        } else {
            self.get(&path.pop_front())
                .field_get_path(path)
        }
    }

    pub fn init(&mut self, key: String) -> Option<ShadeConfigObject> {
        self.set(key, ShadeConfigObject::Null)
    }
}

impl Display for ShadeConfigFields{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<Fields>")
    }
}

impl Debug for ShadeConfigFields{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (key, value) in &self.fields {
            write!(f, "{}: {:?}, ", key, value)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
