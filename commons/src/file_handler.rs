use std::{collections::HashMap, marker::PhantomData};

use crate::Environment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileHandler {
    file_name: String,
    environment: HashMap<String, Environment>,
    creation_path: String,
}

impl FileHandler {
    pub fn get_name(&self) -> &str {
        &self.file_name
    }

    pub fn get_creation_path(&self) -> &str {
        &self.creation_path
    }
    pub fn set_environment(&mut self, env: Option<Environment>) {
        if let Some(env) = env {
            self.environment
                .entry(env.get_name().to_owned())
                .and_modify(|v| *v = env.clone())
                .or_insert(env);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileHandlerBuilder<T> {
    file_name: String,
    environment: HashMap<String, Environment>,
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
            environment: HashMap::default(),
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
            environment: self.environment,
            creation_path: Some(path.to_owned()),
            _marker: PhantomData,
        }
    }

    pub fn environment(mut self, environment: Environment) -> Self {
        self.environment
            .insert(environment.get_name().to_owned(), environment);
        self
    }
}

impl FileHandlerBuilder<FileHandlerBuilderStateCreationPath> {
    pub fn environment(mut self, environment: Environment) -> Self {
        self.environment
            .insert(environment.get_name().to_owned(), environment);
        self
    }

    pub fn build(self) -> FileHandler {
        FileHandler {
            file_name: self.file_name,
            environment: self.environment,
            // Safety: can unwrap because we are using builder, if builder doesn't reach the
            // creation path state then we can't build
            creation_path: self.creation_path.unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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
            file_name: String::from("secrets.yml"),
            environment: HashMap::default(),
            creation_path: String::from("/etc/secrets"),
        };
        assert_eq!(expected_file_handler, actual_file_handler);
    }
}
