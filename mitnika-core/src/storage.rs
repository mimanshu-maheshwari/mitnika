mod file_handler;
mod sqlite_handler;

use sqlite_handler::SQLiteDB;

use crate::{MitnikaError, Project};

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

    pub fn add_project(&self, name: &str) -> Option<Project> {
        if let Ok(rt) = tokio::runtime::Runtime::new()
            .map_err(|err| MitnikaError::RuntimeCreationError(err.to_string()))
        {
            rt.block_on(self.db.create_project(name)).unwrap_or(None)
        } else {
            None
        }
    }

    pub fn projects(&self) -> Vec<Project> {
        match tokio::runtime::Runtime::new()
            .map_err(|err| MitnikaError::RuntimeCreationError(err.to_string()))
        {
            Ok(rt) => rt.block_on(self.db.get_all_projects()).unwrap_or_default(),
            Err(_) => vec![],
        }
    }

    pub fn project_by_id(&self, id: &str) -> Option<Project> {
        match tokio::runtime::Runtime::new()
            .map_err(|err| MitnikaError::RuntimeCreationError(err.to_string()))
        {
            Ok(rt) => rt.block_on(self.db.find_project_by_id(id)).unwrap_or(None),
            Err(_) => None,
        }
    }

    pub fn search_projects(&self, search: &str, exact: bool) -> Vec<Project> {
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
}
