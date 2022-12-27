use crate::std::*;
use crate::fields::ShadeConfigFields;
use crate::path::ShadeConfigPath;

#[derive(Clone)]
pub enum ShadeConfigObject {
    Null,
    Integer(u64),
    Boolean(bool),
    String(String),
    Fields(ShadeConfigFields),
}

impl ShadeConfigObject {
    pub const fn null() -> Self {
        Self::Null
    }

    pub const fn integer(value: u64) -> Self {
        Self::Integer(value)
    }

    pub const fn boolean(value: bool) -> Self {
        ShadeConfigObject::Boolean(value)
    }

    pub const fn string(value: String) -> Self {
        ShadeConfigObject::String(value)
    }

    pub const fn fields(value: ShadeConfigFields) -> Self {
        ShadeConfigObject::Fields(value)
    }

    pub const fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false,
        }
    }

    pub fn parse(string: String) -> Option<Self> {
        let trim = string.trim();
        let lowercase = trim.to_lowercase();
        if lowercase == "null" {
            Some(Self::null())
        } else if lowercase == "true" {
            Some(Self::boolean(true))
        } else if lowercase == "false" {
            Some(Self::boolean(false))
        } else if let Ok(value) = u64::from_str(lowercase.as_str()) {
            Some(Self::integer(value))
        } else if trim.len() >= 2
            && trim.starts_with("\"")
            && trim.ends_with("\"") {
            let mut string = trim.to_string();
            string.pop();
            string.remove(0);
            Some(Self::String(string))
        } else {
            None
        }
    }

    pub fn self_replace(&mut self, value: Self) -> Self {
        replace(self, value)
    }

    pub fn self_replace_fields(&mut self) -> Self {
        self.self_replace(
            Self::Fields(
                ShadeConfigFields::new()
            )
        )
    }

    pub fn field_set(&mut self, key: String, value: Self) -> Option<Option<Self>> {
        if let Self::Fields(fields) = self {
            Some(fields.set(key, value))
        } else if self.is_null() {
            self.self_replace_fields();
            self.field_set(key, value)
        } else {
            None
        }
    }

    pub fn field_get(&mut self, key: &String) -> Option<&mut Self> {
        if let Self::Fields(fields) = self {
            Some(fields.get(key))
        } else if self.is_null() {
            self.self_replace_fields();
            self.field_get(key)
        } else {
            None
        }
    }

    pub fn field_set_path(&mut self, path: ShadeConfigPath, value: Self) -> Option<Option<Self>> {
        if let Self::Fields(fields) = self {
            fields.set_path(path, value)
        } else if self.is_null() {
            self.self_replace_fields();
            self.field_set_path(path, value)
        } else {
            None
        }
    }

    pub fn field_get_path(&mut self, path: ShadeConfigPath) -> Option<&mut Self> {
        if let Self::Fields(fields) = self {
           fields.get_path(path)
        } else if self.is_null() {
            self.self_replace_fields();
            self.field_get_path(path)
        } else {
            None
        }
    }
}

impl Default for ShadeConfigObject {
    fn default() -> Self {
        Self::null()
    }
}

impl Display for ShadeConfigObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Integer(value) => write!(f, "{}", value),
            Self::Boolean(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{}", value),
            Self::Fields(_) => write!(f, "<Fields>"),
        }
    }
}

impl Debug for ShadeConfigObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "Null(null)"),
            Self::Integer(value) => write!(f, "Integer({:?})", value),
            Self::Boolean(value) => write!(f, "Boolean({:?})", value),
            Self::String(value) => write!(f, "String({:?})", value),
            Self::Fields(value) => write!(f, "Fields {:?}", value),
        }
    }
}
