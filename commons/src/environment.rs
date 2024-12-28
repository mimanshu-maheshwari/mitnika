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
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_version(&self, version_name: Option<&str>) -> Option<&Version> {
        if let Some(name) = version_name {
            self.version.get(name)
        } else {
            self.version.get("default")
        }
    }
}
