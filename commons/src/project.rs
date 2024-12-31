use crate::FileHandler;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Project {
    pub name: String,
    pub files: HashMap<String, FileHandler>,
}

impl Project {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_file(&mut self, file_handler: FileHandler) -> Option<FileHandler> {
        self.files
            .insert(file_handler.get_name().to_owned(), file_handler)
    }

    pub fn get_file(&self, file_name: &str) -> Option<&FileHandler> {
        self.files.get(file_name)
    }

    pub fn get_file_mut(&mut self, file_name: &str) -> Option<&mut FileHandler> {
        self.files.get_mut(file_name)
    }

    pub fn delete_file(&mut self, file_name: &str) {
        self.files.remove(file_name);
    }
    pub fn get_file_names(&self) -> Vec<String> {
        self.files.keys().cloned().collect()
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
            files: HashMap::new(),
        }
    }

    pub fn file(mut self, file: FileHandler) -> Self {
        self.files.insert(file.get_name().to_owned(), file);
        self
    }

    pub fn build(self) -> Project {
        Project {
            name: self.name,
            files: self.files,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{Environment, FileHandlerBuilder, Project, ProjectBuilder};
    use std::collections::HashMap;

    #[test]
    fn create_project_using_builder() {
        let file = FileHandlerBuilder::new("secret.yaml")
            .creation_path("/etc/secrets")
            .environment(Environment::default())
            .build();
        let builder: ProjectBuilder = ProjectBuilder::new("mitnika").file(file.clone());
        let actual_project: Project = builder.build();
        let mut hash_map = HashMap::new();
        hash_map.insert(file.get_name().to_owned(), file.clone());
        let expected_project: Project = Project {
            name: String::from("mitnika"),
            files: hash_map,
        };
        assert_eq!(expected_project, actual_project);
    }
}
