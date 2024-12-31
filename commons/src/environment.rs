use std::collections::HashMap;

use crate::{Version, DEFAULT_STRING};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Environment {
    name: String,
    // will contain the version name and version as map
    version: HashMap<String, Version>,
}

impl Default for Environment {
    fn default() -> Self {
        let mut version_map = HashMap::new();
        version_map.insert(String::from("default"), Version::default());
        Self {
            name: String::from("default"),
            version: version_map,
        }
    }
}

impl Environment {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            version: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn version(&mut self, version: Version) {
        self.version
            .entry(version.get_name().to_owned())
            .and_modify(|v| *v = version.clone())
            .or_insert(version);
    }

    pub fn add_version(&mut self, version: &str) {
        self.version
            .entry(version.to_owned())
            .or_insert(Version::new(version));
    }

    pub fn get_version(&self, version_name: &str) -> Option<&Version> {
        self.version.get(version_name)
    }

    pub fn get_version_mut(&mut self, version_name: &str) -> Option<&mut Version> {
        self.version.get_mut(version_name)
    }

    pub fn get_default_version_mut(&mut self) -> Option<&mut Version> {
        self.get_version_mut(DEFAULT_STRING)
    }

    pub fn delete_version(&mut self, version_name: &str) {
        self.version.remove(version_name);
    }

    pub fn get_version_names(&self) -> Vec<String> {
        self.version.keys().cloned().collect()
    }
}
