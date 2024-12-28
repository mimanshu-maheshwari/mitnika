use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileHandler {
    file_name: String,
    content: String,
    creation_path: String,
}

impl FileHandler {
    pub fn get_name(&self) -> &str {
        &self.file_name
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
    pub fn get_creation_path(&self) -> &str {
        &self.creation_path
    }
}

pub struct FileHandlerBuilder<T> {
    file_name: String,
    content: Option<String>,
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
