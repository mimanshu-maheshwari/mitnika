use std::collections::HashSet;

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

pub struct FileHandler {
    file_name: String,
    content: String,
    creation_path: String,
}

pub struct FileHandlerBuilder {
    file_name: String,
    content: Option<String>,
    creation_path: Option<String>,
}

impl FileHandlerBuilder {
    pub fn new(name: &str) -> Self {
        FileHandlerBuilder {
            file_name: name.to_owned(),
            content: None,
            creation_path: None,
        }
    }

    pub fn creation_path(mut self, path: &str) -> Self {
        self.creation_path = Some(path.to_owned());
        self
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = Some(content.to_owned());
        self
    }
    pub fn build(self) -> FileHandler {
        FileHandler {
            file_name: self.file_name,
            content: self.content.unwrap(),
            creation_path: self.creation_path.unwrap(),
        }
    }
}

pub struct Environment;
pub struct Version;
