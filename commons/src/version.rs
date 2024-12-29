#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    name: String,
    content: String,
}

impl Default for Version {
    fn default() -> Self {
        Self {
            name: String::from("default"),
            content: String::new(),
        }
    }
}
impl Version {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            content: String::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_owned();
    }
}
