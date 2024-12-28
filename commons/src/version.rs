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
