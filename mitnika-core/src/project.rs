use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize, sqlx::FromRow, Hash)]
pub struct ProjectDetails {
    id: String,
    name: String,
}

impl ProjectDetails {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {}
