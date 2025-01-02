use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Version, DEFAULT_STRING};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Environment {
    #[serde(with = "uuid::serde::simple")]
    id: uuid::Uuid,
    name: String,
    versions: HashMap<String, Version>,
}

impl Default for Environment {
    fn default() -> Self {
        let mut map = HashMap::new();
        let version = Version::default();
        map.insert(version.id().to_string(), version);
        Self {
            id: uuid::Uuid::new_v4(),
            name: String::from(DEFAULT_STRING),
            versions: map,
        }
    }
}

impl Environment {
    pub fn new(name: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            versions: HashMap::new(),
            name: name.to_owned(),
        }
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn versions(&self) -> Vec<&Version> {
        self.versions.values().collect()
    }

    pub fn add_version(&mut self, name: &str) {
        if self.versions.values().filter(|v| v.name() == name).count() == 0 {
            let version = Version::new(name);
            self.versions.insert(version.id().to_string(), version);
        }
    }

    pub fn version(&self, id: &uuid::Uuid) -> Option<&Version> {
        self.versions.get(&id.to_string())
    }

    pub fn get_default_version(&mut self) -> Option<&mut Version> {
        self.versions
            .values_mut()
            .find(|v| v.name() == DEFAULT_STRING)
    }

    pub fn delete_version(&mut self, id: &uuid::Uuid) {
        self.versions.remove(&id.to_string());
    }
}
