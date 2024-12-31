mod environment;
mod error;
mod file_handler;
mod project;
mod version;

pub type Result<T> = std::result::Result<T, MitnikaError>;
pub const DEFAULT_STRING: &str = "default";

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub use environment::Environment;
pub use error::MitnikaError;
pub use file_handler::{FileHandler, FileHandlerBuilder};
pub use project::{Project, ProjectBuilder};
pub use version::Version;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mitnika {
    projects: HashMap<String, Project>,
}

impl Default for Mitnika {
    fn default() -> Self {
        let mut map = HashMap::new();
        // TODO: Create random name instead of default. Something like from dictionary of funny
        // words and combine them to create creative names.
        let name = DEFAULT_STRING;
        map.insert(String::from(name), ProjectBuilder::new(name).build());
        Self { projects: map }
    }
}

impl Mitnika {
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
        }
    }

    pub fn add_project(&mut self, name: &str) -> &mut Project {
        let project = ProjectBuilder::new(name).build();
        self.projects.insert(name.to_owned(), project);
        // Safety: in previous lines we have inserted the project in map
        self.projects.get_mut(name).unwrap()
    }

    pub fn get_project(&self, name: &str) -> Option<&Project> {
        self.projects.get(name)
    }

    pub fn get_project_mut(&mut self, name: &str) -> Option<&mut Project> {
        self.projects.get_mut(name)
    }

    pub fn get_project_names(&self) -> Vec<String> {
        self.projects.keys().cloned().collect()
    }
    pub fn get_projects(&self) -> Vec<Arc<Mutex<Project>>> {
        self.projects
            .values()
            .map(|v| Arc::new(Mutex::new(v.clone())))
            .collect()
    }

    pub fn search_projects(&self, search: &str) -> Vec<Arc<Mutex<Project>>> {
        let mut pattern = String::new();
        pattern.push_str(".*");
        pattern.push_str(search);
        pattern.push_str(".*");
        let keys: Vec<_> = self
            .projects
            .keys()
            .filter(|key| key.contains(&pattern))
            .collect();
        self.projects
            .iter()
            .filter(|(key, _)| keys.contains(key))
            .map(|(_, val)| Arc::new(Mutex::new(val.clone())))
            .collect()
    }
}
