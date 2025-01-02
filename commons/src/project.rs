use serde::{Deserialize, Serialize};

use crate::FileHandler;

use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
pub struct Project {
    #[serde(with = "uuid::serde::simple")]
    pub id: uuid::Uuid,
    pub name: String,
    pub files: HashMap<String, FileHandler>,
}

impl Hash for Project {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Project {
    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn update_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    pub fn update_file(&mut self, new_file: FileHandler) {
        if let Some(f) = self.files.get_mut(&new_file.id().to_string()) {
            *f = new_file;
        }
    }

    pub fn add_file(&mut self, file_handler: FileHandler) {
        self.files
            .entry(file_handler.id().to_string())
            .and_modify(|f| *f = file_handler.clone())
            .or_insert(file_handler);
    }

    pub fn file(&self, id: &uuid::Uuid) -> Option<&FileHandler> {
        self.files.get(&id.to_string())
    }

    pub fn file_mut(&mut self, id: &uuid::Uuid) -> Option<&mut FileHandler> {
        self.files.get_mut(&id.to_string())
    }

    pub fn delete_file(&mut self, id: &uuid::Uuid) {
        self.files.remove(&id.to_string());
    }
}

pub struct ProjectBuilder {
    name: String,
    files: HashMap<String, FileHandler>,
}

impl ProjectBuilder {
    pub fn new(name: &str) -> Self {
        ProjectBuilder {
            name: name.to_owned(),
            files: Default::default(),
        }
    }

    pub fn file(mut self, file: FileHandler) -> Self {
        self.files.insert(file.id().to_string(), file);
        self
    }

    pub fn build(self) -> Project {
        Project {
            id: uuid::Uuid::new_v4(),
            name: self.name,
            files: self.files,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::{Environment, FileHandlerBuilder, Project, ProjectBuilder};

    #[test]
    fn create_project_using_builder() {
        let file = FileHandlerBuilder::new("secret.yaml")
            .creation_path("/etc/secrets")
            .environment(Environment::default())
            .build();
        let builder: ProjectBuilder = ProjectBuilder::new("mitnika").file(file.clone());
        let actual_project: Project = builder.build();
        let mut expected_project: Project = Project {
            id: actual_project.id().to_owned(),
            name: String::from("mitnika"),
            files: HashMap::new(),
        };
        expected_project.add_file(file.clone());
        assert_eq!(expected_project, actual_project);
    }
}
