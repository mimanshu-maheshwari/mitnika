use std::collections::HashMap;

use crate::Version;

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

    pub fn set_version(&mut self, version: Option<Version>) {
        if let Some(version) = version {
            self.version
                .entry(version.get_name().to_owned())
                .and_modify(|ve| *ve = version.clone())
                .or_insert(version);
        }
    }

    pub fn get_version(&self, version_name: Option<&str>) -> Option<&Version> {
        if let Some(name) = version_name {
            self.version.get(name)
        } else {
            self.version.get("default")
        }
    }
}
