use serde::{Deserialize, Serialize};

use crate::DEFAULT_STRING;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, sqlx::FromRow)]
pub struct Version {
    id: String,
    name: String,
    content: String,
}

impl Default for Version {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::from(DEFAULT_STRING),
            content: String::new(),
        }
    }
}
impl Version {
    pub fn new(name: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_owned(),
            content: String::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn update_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_owned();
    }
}
