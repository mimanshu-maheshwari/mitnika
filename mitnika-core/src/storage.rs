mod file_handler;
mod sqlite_handler;

use sqlite_handler::SQLiteDB;

use crate::{FileDetails, MitnikaError, ProjectDetails};

#[derive(Debug, Clone)]
pub struct Storage {
    db: SQLiteDB,
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage {
    pub fn new() -> Self {
        Self {
            // Unwrap because if we find a error then it should be mitnika error and that would be
            // breaking error we don't handle it.
            db: SQLiteDB::new().unwrap(),
        }
    }

    pub fn add_project(&self, name: &str) -> Option<ProjectDetails> {
        if let Ok(rt) = tokio::runtime::Runtime::new()
            .map_err(|err| MitnikaError::RuntimeCreationError(err.to_string()))
        {
            rt.block_on(self.db.create_project(name)).unwrap_or(None)
        } else {
            None
        }
    }

    pub fn projects(&self) -> Vec<ProjectDetails> {
        match tokio::runtime::Runtime::new()
            .map_err(|err| MitnikaError::RuntimeCreationError(err.to_string()))
        {
            Ok(rt) => rt.block_on(self.db.get_all_projects()).unwrap_or_default(),
            Err(_) => vec![],
        }
    }

    pub fn project_by_id(&self, id: &str) -> Option<ProjectDetails> {
        match tokio::runtime::Runtime::new()
            .map_err(|err| MitnikaError::RuntimeCreationError(err.to_string()))
        {
            Ok(rt) => rt.block_on(self.db.find_project_by_id(id)).unwrap_or(None),
            Err(_) => None,
        }
    }

    pub fn search_projects(&self, search: &str, exact: bool) -> Vec<ProjectDetails> {
        if !search.is_empty() {
            match tokio::runtime::Runtime::new()
                .map_err(|err| MitnikaError::RuntimeCreationError(err.to_string()))
            {
                Ok(rt) => rt
                    .block_on(self.db.search_project(search, exact))
                    .unwrap_or_default(),
                Err(_) => vec![],
            }
        } else {
            self.projects()
        }
    }

    pub fn get_files_by_project_id(&self, project_id: &str) -> Vec<FileDetails> {
        if !project_id.is_empty() {
            match tokio::runtime::Runtime::new()
                .map_err(|err| MitnikaError::RuntimeCreationError(err.to_string()))
            {
                Ok(rt) => rt
                    .block_on(self.db.find_file_for_project(project_id))
                    .unwrap_or_default(),
                Err(_) => Default::default(),
            }
        } else {
            Default::default()
        }
    }
    pub fn add_file_to_project(&self, file_name: &str, creation_path: &str, project_id: &str) {
        //validate if project is present
        //validate if file is present
        //validate creation_path
    }
}
