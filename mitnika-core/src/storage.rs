mod file_handler;
mod sqlite_handler;

use sqlite_handler::SQLiteDB;
// use regex::Regex;

use crate::Project;

#[derive(Debug)]
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

    pub fn add_project(&mut self, _project_name: &str) {
        unimplemented!("implement add project")
    }

    pub fn projects(&self) -> Vec<&Project> {
        unimplemented!("implement get all projects")
    }

    pub fn search_projects(&self, _search: &str, _exact: bool) -> Vec<&Project> {
        unimplemented!("implemented Search Projects");
    }
}
