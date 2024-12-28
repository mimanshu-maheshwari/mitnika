use crate::FileHandler;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub name: String,
    pub files: HashMap<String, FileHandler>,
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
    use std::collections::HashMap;

    use crate::{FileHandlerBuilder, Project, ProjectBuilder};

    #[test]
    fn create_project_using_builder() {
        let file = FileHandlerBuilder::new("secret.yaml")
            .creation_path("/etc/secrets")
            .content("name: mintika")
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
