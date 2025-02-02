use std::{collections::HashMap, marker::PhantomData};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::Environment;

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
pub struct FileHandler {
    #[serde(with = "uuid::serde::simple")]
    id: uuid::Uuid,
    file_name: String,
    creation_path: String,
    environments: HashMap<String, Environment>,
}

impl FileHandler {
    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.file_name
    }

    pub fn creation_path(&self) -> &str {
        &self.creation_path
    }

    pub fn environment(&self, id: &uuid::Uuid) -> Option<&Environment> {
        self.environments.get(&id.to_string())
    }

    pub fn environment_mut(&mut self, id: &uuid::Uuid) -> Option<&mut Environment> {
        self.environments.get_mut(&id.to_string())
    }

    pub fn add_environment(&mut self, name: &str) {
        let env = Environment::new(name);
        self.environments.insert(env.id().to_string(), env);
    }

    pub fn update_environment(&mut self, env: Environment) {
        if let Some(old_env) = self.environments.get_mut(env.id().to_string().as_str()) {
            *old_env = env;
        }
    }

    pub fn delete_environment(&mut self, id: &uuid::Uuid) {
        self.environments.remove(&id.to_string());
    }

    pub fn search_environments(&self, search: &str, exact: bool) -> Vec<&Environment> {
        let mut pat = String::new();
        if !exact {
            pat.push_str(".*");
        }
        pat.push_str(&search.to_lowercase());
        if !exact {
            pat.push_str(".*");
        }
        let re = Regex::new(&pat).expect("Unable to create regex for search");
        self.environments
            .values()
            .filter(|env| re.is_match(&env.name().to_lowercase()))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileHandlerBuilder<T> {
    file_name: String,
    environments: HashMap<String, Environment>,
    creation_path: Option<String>,
    _marker: PhantomData<T>,
}

pub struct FileHandlerBuilderStateInit;
pub struct FileHandlerBuilderStateName;
pub struct FileHandlerBuilderStateCreationPath;

impl FileHandlerBuilder<FileHandlerBuilderStateInit> {
    pub fn new(name: &str) -> FileHandlerBuilder<FileHandlerBuilderStateName> {
        FileHandlerBuilder::<FileHandlerBuilderStateName> {
            file_name: name.to_owned(),
            environments: Default::default(),
            creation_path: None,
            _marker: PhantomData,
        }
    }
}

impl FileHandlerBuilder<FileHandlerBuilderStateName> {
    pub fn creation_path(
        self,
        path: &str,
    ) -> FileHandlerBuilder<FileHandlerBuilderStateCreationPath> {
        FileHandlerBuilder::<FileHandlerBuilderStateCreationPath> {
            file_name: self.file_name,
            environments: self.environments,
            creation_path: Some(path.to_owned()),
            _marker: PhantomData,
        }
    }

    pub fn environment(mut self, environment: Environment) -> Self {
        self.environments
            .insert(environment.id().to_string(), environment);
        self
    }
}

impl FileHandlerBuilder<FileHandlerBuilderStateCreationPath> {
    pub fn environment(mut self, environment: Environment) -> Self {
        self.environments
            .insert(environment.id().to_string(), environment);
        self
    }

    pub fn build(self) -> FileHandler {
        FileHandler {
            id: uuid::Uuid::new_v4(),
            file_name: self.file_name,
            environments: self.environments,
            // Safety: can unwrap because we are using builder, if builder doesn't reach the
            // creation path state then we can't build
            creation_path: self.creation_path.unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::file_handler::{
        FileHandler, FileHandlerBuilder, FileHandlerBuilderStateCreationPath,
        FileHandlerBuilderStateName,
    };

    #[test]
    fn file_creation_test() {
        let file_handler_builder: FileHandlerBuilder<FileHandlerBuilderStateName> =
            FileHandlerBuilder::new("secrets.yml");
        let file_handler_builder: FileHandlerBuilder<FileHandlerBuilderStateCreationPath> =
            file_handler_builder.creation_path("/etc/secrets");
        let actual_file_handler = file_handler_builder.build();
        let expected_file_handler = FileHandler {
            id: actual_file_handler.id().to_owned(),
            file_name: String::from("secrets.yml"),
            environments: Default::default(),
            creation_path: String::from("/etc/secrets"),
        };
        assert_eq!(expected_file_handler, actual_file_handler);
    }
}
