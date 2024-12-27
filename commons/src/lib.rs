use std::{collections::HashSet, error::Error, fmt::Display, marker::PhantomData};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MitnikaError {
    EmptyFilePath,
}

impl Error for MitnikaError {}

impl Display for MitnikaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyFilePath => writeln!(f, "Creation path for file is empty."),
        }
    }
}

pub struct Collection {
    pub name: String,
    pub files: HashSet<String>,
}
pub struct CollectionBuilder {
    name: String,
    files: HashSet<String>,
}
impl CollectionBuilder {
    pub fn new(name: &str) -> Self {
        CollectionBuilder {
            name: name.to_owned(),
            files: HashSet::new(),
        }
    }

    pub fn file(mut self, name: &str) -> Self {
        self.files.insert(name.to_owned());
        self
    }

    pub fn build(self) -> Collection {
        Collection {
            name: self.name,
            files: self.files,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileHandler {
    file_name: String,
    content: String,
    creation_path: String,
}

pub struct FileHandlerBuilder<T> {
    file_name: String,
    content: Option<String>,
    creation_path: Option<String>,
    _marker: PhantomData<T>,
}
struct FileHandlerBuilderStateInit;
struct FileHandlerBuilderStateName;
struct FileHandlerBuilderStateCreationPath;

impl FileHandlerBuilder<FileHandlerBuilderStateInit> {
    pub fn new(name: &str) -> FileHandlerBuilder<FileHandlerBuilderStateName> {
        FileHandlerBuilder::<FileHandlerBuilderStateName> {
            file_name: name.to_owned(),
            content: None,
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
            content: None,
            creation_path: Some(path.to_owned()),
            _marker: PhantomData,
        }
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = Some(content.to_owned());
        self
    }
}

impl FileHandlerBuilder<FileHandlerBuilderStateCreationPath> {
    pub fn build(self) -> FileHandler {
        FileHandler {
            file_name: self.file_name,
            content: self.content.unwrap_or_default(),
            // Safety: can unwrap because we are using builder, if builder doesn't reach the
            // creation path state then we can't build
            creation_path: self.creation_path.unwrap(),
        }
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = Some(content.to_owned());
        self
    }
}

pub struct Environment;
pub struct Version;

#[cfg(test)]
mod tests {
    use crate::{FileHandler, FileHandlerBuilderStateCreationPath};

    use super::{FileHandlerBuilder, FileHandlerBuilderStateName};

    #[test]
    fn file_creation_test() {
        let file_handler_builder: FileHandlerBuilder<FileHandlerBuilderStateName> =
            FileHandlerBuilder::new("secrets.yml");
        let file_handler_builder: FileHandlerBuilder<FileHandlerBuilderStateName> =
            file_handler_builder.content("");
        let file_handler_builder: FileHandlerBuilder<FileHandlerBuilderStateCreationPath> =
            file_handler_builder.creation_path("/etc/secrets");
        let file_handler_builder: FileHandlerBuilder<FileHandlerBuilderStateCreationPath> =
            file_handler_builder.content("");
        let actual_file_handler = file_handler_builder.build();
        let expected_file_handler = FileHandler {
            file_name: String::from("secrets.yml"),
            content: String::from(""),
            creation_path: String::from("/etc/secrets"),
        };
        assert_eq!(expected_file_handler, actual_file_handler);
    }
}
