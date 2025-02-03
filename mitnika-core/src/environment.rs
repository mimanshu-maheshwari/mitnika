use serde::{Deserialize, Serialize};

use crate::DEFAULT_STRING;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, sqlx::FromRow)]
pub struct Environment {
    id: String,
    name: String,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::from(DEFAULT_STRING),
        }
    }
}

impl Environment {
    pub fn new(name: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_owned(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
