use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct FileDetails {
    id: String,
    name: String,
    creation_path: String,
}

impl FileDetails {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn creation_path(&self) -> &str {
        &self.creation_path
    }
}
