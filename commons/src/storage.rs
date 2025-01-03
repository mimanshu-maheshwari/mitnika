use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, Read, Write},
};

use directories::ProjectDirs;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    MitnikaError, Project, ProjectBuilder, Result, AVIDVIVARTA_LOWER, DATA_FILE, DEFAULT_STRING,
    EXTENSION, MITNIKA_LOWER, MM,
};

#[derive(Debug, Clone)]
pub struct Storage {
    data: MitnikaData,
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage {
    pub fn new() -> Self {
        let mut file = Self::get_data_file(false).unwrap();
        Self {
            data: match MitnikaData::load(&mut file) {
                Ok(data) => data,
                Err(_err) => MitnikaData::new(),
            },
        }
    }

    pub fn add_project(&mut self, project_name: &str) {
        self.data.add_project(project_name);
        let mut file = Self::get_data_file(true).unwrap();
        let _ = file.write_all(
            serde_json::to_string::<MitnikaData>(&self.data)
                .unwrap()
                .as_bytes(),
        );
    }

    pub fn projects(&self) -> Vec<&Project> {
        self.data.projects.values().collect()
    }

    pub fn search_projects(&self, search: &str, exact: bool) -> Vec<&Project> {
        self.data.search_projects(search, exact)
    }

    fn get_data_file(truncate: bool) -> Result<File> {
        if let Some(project_dir) = ProjectDirs::from(MM, AVIDVIVARTA_LOWER, MITNIKA_LOWER) {
            let data_dir_path = project_dir.data_local_dir();
            if !data_dir_path.exists() {
                match fs::create_dir_all(data_dir_path) {
                    Ok(_) => {
                        println!("Created directories: {:?}", data_dir_path);
                    }
                    Err(err) => {
                        println!("Creation failed for directories: {:?} {err}", data_dir_path);
                    }
                }
            }
            let file_path = data_dir_path.join(DATA_FILE).with_extension(EXTENSION);
            println!("{:?}", file_path.clone());
            File::options()
                .read(true)
                .create(!file_path.clone().exists())
                .write(true)
                .truncate(truncate)
                .open(file_path.clone())
                .map_err(|err| {
                    MitnikaError::UnableToReadDataFile(format!(
                        "{err}: {}",
                        file_path
                            .to_str()
                            .expect("Unable to find file path")
                            .to_owned(),
                    ))
                })
        } else {
            Err(MitnikaError::LocalUserDirectoryNotFound(String::from(
                "Unable to find the directory",
            )))
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MitnikaData {
    projects: HashMap<String, Project>,
}

impl Default for MitnikaData {
    fn default() -> Self {
        // TODO: Create random name instead of default. Something like from dictionary of funny
        // words and combine them to create creative names.
        let name = DEFAULT_STRING;
        let mut map = HashMap::new();
        let project = ProjectBuilder::new(name).build();
        map.insert(project.id().to_string(), project);
        Self { projects: map }
    }
}

impl MitnikaData {
    pub fn new() -> Self {
        Self {
            projects: Default::default(),
        }
    }

    pub fn load(file: &mut File) -> Result<Self> {
        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        match reader.read_to_string(&mut buf) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    serde_json::from_str::<Self>(&buf)
                        .map_err(|err| MitnikaError::UnableToParseDataFile(format!("{err}")))
                } else {
                    Ok(Self::new())
                }
            }
            Err(err) => Err(MitnikaError::UnableToReadDataFile(format!("{err}"))),
        }
    }

    pub fn add_project(&mut self, name: &str) {
        if self
            .projects()
            .into_iter()
            .find(|p| p.name() == name)
            .is_none()
        {
            let project = ProjectBuilder::new(name).build();
            self.projects.insert(project.id().to_string(), project);
        }
    }

    pub fn projects(&self) -> Vec<&Project> {
        self.projects.values().collect()
    }

    pub fn search_projects(&self, search: &str, exact: bool) -> Vec<&Project> {
        if search.trim().is_empty() {
            return self.projects();
        }
        let mut pat = String::new();
        if !exact {
            pat.push_str(".*");
        }
        pat.push_str(&search.to_lowercase());
        if !exact {
            pat.push_str(".*");
        }

        let re = Regex::new(&pat).expect("Unable to create regex for search");
        self.projects()
            .into_iter()
            .filter(|p| re.is_match(&p.name().to_lowercase()))
            .collect()
    }
}
