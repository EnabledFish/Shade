use crate::std::*;

#[derive(Clone, Default)]
pub struct ShadeConfigPath {
    pub elements: Vec<String>,
}

impl ShadeConfigPath {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_elements(elements: Vec<String>) -> Self {
        Self {
            elements
        }
    }

    pub fn parse(source: String) -> Option<Self> {
        let mut path = Self::new();
        if source.trim().is_empty() {
            return None;
        }
        for element in source.split(".") {
            let trim = element.trim().to_string();
            if trim.is_empty(){
                return None;
            }
            if !trim.is_ascii() {
                return None;
            }
            if trim.contains(" ") {
                return None;
            }
            path.push(trim.to_string());
        }
        Some(path)
    }

    pub fn push(&mut self, element: String) -> &mut Self {
        self.elements.push(element);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn pop_front(&mut self) -> String {
        self.elements.remove(0)
    }

    pub fn pop(&mut self) -> Option<String> {
        self.elements.pop()
    }
}

impl TryFrom<String> for ShadeConfigPath{
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value).ok_or("Invalid path expression.")
    }
}
