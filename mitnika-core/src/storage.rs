use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

use directories::ProjectDirs;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Sqlite, SqlitePool};

use crate::{
    MitnikaError, Project, ProjectBuilder, Result, AVIDVIVARTA_LOWER, DATA_FILE, DEFAULT_STRING,
    EXTENSION, MITNIKA_LOWER, MM, SQLITE_FILE, SQLITE_FILE_EXTENSION,
};

#[derive(Debug, Clone)]
pub struct Storage {
    // data: MitnikaData,
    pool: Arc<SqlitePool>,
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage {
    pub fn new() -> Self {
        // let mut file = Self::get_data_file(false).unwrap();
        let pool = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(Self::get_db_pool())
            .map_err(|err| {
                eprintln!("{err}");
                err
            })
            .expect("Unable to create a db pool.");
        Self {
            // data: match MitnikaData::load(&mut file) {
            //     Ok(data) => data,
            //     Err(_err) => MitnikaData::new(),
            // },
            pool: Arc::new(pool),
        }
    }

    pub fn add_project(&mut self, project_name: &str) {
        todo!()
        //     // self.data.add_project(project_name);
        //     let mut file = Self::get_data_file(true).unwrap();
        //     let _ = file.write_all(
        //         serde_json::to_string::<MitnikaData>(&self.data)
        //             .unwrap()
        //             .as_bytes(),
        //     );
    }

    pub fn projects(&self) -> Vec<&Project> {
        // self.data.projects.values().collect()
        todo!();
    }

    pub fn search_projects(&self, search: &str, exact: bool) -> Vec<&Project> {
        // self.data.search_projects(search, exact)
        todo!();
    }

    async fn get_db_pool() -> std::result::Result<SqlitePool, sqlx::Error> {
        let dir_path = Self::get_dir();
        // println!("{dir_path:?}");
        // let canonical_path = Self::convert_windows_to_linux_path(&dir_path);
        let mut db_url = format!(
            "sqlite:///{}",
            dir_path.to_str().expect("Unable to load data file.")
        );
        // println!("{db_url:?}");
        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            Sqlite::create_database(&db_url).await.unwrap();
            match Self::create_schema(&db_url).await {
                Ok(_) => println!("Database created successfully."),
                Err(err) => panic!("{}", err),
            }
        }
        SqlitePool::connect(&db_url).await
    }

    // fn _convert_windows_to_linux_path(windows_path: &Path) -> PathBuf {
    //     let windows_path_str = windows_path.to_str().unwrap_or("");
    //     let mut linux_path = windows_path_str.replace("\\", "/");
    //
    //     // If the path starts with a drive letter like C:\, convert it to /mnt/c
    //     if windows_path_str.len() > 1 && windows_path_str[1..2] == *":" {
    //         linux_path = format!("{}", windows_path_str[..1].to_lowercase()) + &linux_path[2..];
    //     }
    //
    //     PathBuf::from(linux_path)
    // }

    pub async fn create_schema(
        db_url: &str,
    ) -> std::result::Result<SqliteQueryResult, sqlx::Error> {
        let pool = SqlitePool::connect(db_url).await?;
        let qry = "PRAGMA foreign_keys = ON;
    
    CREATE TABLE IF NOT EXISTS Project (
        id TEXT PRIMARY KEY NOT NULL,
        name TEXT NOT NULL
    );
    
    CREATE TABLE IF NOT EXISTS File (
        id TEXT PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        project_id TEXT NOT NULL,
        FOREIGN KEY (project_id) REFERENCES Project(id) ON DELETE CASCADE
    );
    
    CREATE TABLE IF NOT EXISTS Environment (
        id TEXT PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        file_id TEXT NOT NULL,
        FOREIGN KEY (file_id) REFERENCES File(id) ON DELETE CASCADE
    );
    
    CREATE TABLE IF NOT EXISTS Version (
        id TEXT PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        environment_id TEXT NOT NULL,
        FOREIGN KEY (environment_id) REFERENCES Environment(id) ON DELETE CASCADE
    );";

        let result = sqlx::query(qry).execute(&pool).await;
        pool.close().await;
        result
    }

    fn get_dir() -> std::path::PathBuf {
        if let Some(project_dir) = ProjectDirs::from(MM, AVIDVIVARTA_LOWER, MITNIKA_LOWER) {
            let data_dir_path = project_dir.data_local_dir();
            if !data_dir_path.exists() {
                match fs::create_dir_all(data_dir_path) {
                    Ok(_) => {
                        println!("Created directories: {:?}", data_dir_path);
                    }
                    Err(err) => {
                        panic!("Creation failed for directories: {:?} {err}", data_dir_path);
                    }
                }
            }
            let file_path = data_dir_path
                .join(SQLITE_FILE)
                .with_extension(SQLITE_FILE_EXTENSION);
            return file_path;
        } else {
            panic!("Something wrong with file directory. Unable to read the data.")
        }
    }

    // fn get_data_file(truncate: bool) -> Result<File> {
    //     if let Some(project_dir) = ProjectDirs::from(MM, AVIDVIVARTA_LOWER, MITNIKA_LOWER) {
    //         let data_dir_path = project_dir.data_local_dir();
    //         if !data_dir_path.exists() {
    //             match fs::create_dir_all(data_dir_path) {
    //                 Ok(_) => {
    //                     println!("Created directories: {:?}", data_dir_path);
    //                 }
    //                 Err(err) => {
    //                     println!("Creation failed for directories: {:?} {err}", data_dir_path);
    //                 }
    //             }
    //         }
    //         let file_path = data_dir_path.join(DATA_FILE).with_extension(EXTENSION);
    //         println!("{:?}", file_path.clone());
    //         File::options()
    //             .read(true)
    //             .create(!file_path.clone().exists())
    //             .write(true)
    //             .truncate(truncate)
    //             .open(file_path.clone())
    //             .map_err(|err| {
    //                 MitnikaError::UnableToReadDataFile(format!(
    //                     "{err}: {}",
    //                     file_path
    //                         .to_str()
    //                         .expect("Unable to find file path")
    //                         .to_owned(),
    //                 ))
    //             })
    //     } else {
    //         Err(MitnikaError::LocalUserDirectoryNotFound(String::from(
    //             "Unable to find the directory",
    //         )))
    //     }
    // }
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct MitnikaData {
//     projects: HashMap<String, Project>,
// }

// impl Default for MitnikaData {
//     fn default() -> Self {
//         // TODO: Create random name instead of default. Something like from dictionary of funny
//         // words and combine them to create creative names.
//         let name = DEFAULT_STRING;
//         let mut map = HashMap::new();
//         let project = ProjectBuilder::new(name).build();
//         map.insert(project.id().to_string(), project);
//         Self { projects: map }
//     }
// }
//
// impl MitnikaData {
//     pub fn new() -> Self {
//         Self {
//             projects: Default::default(),
//         }
//     }
//
//     pub fn load(file: &mut File) -> Result<Self> {
//         let mut reader = BufReader::new(file);
//         let mut buf = String::new();
//         match reader.read_to_string(&mut buf) {
//             Ok(bytes_read) => {
//                 if bytes_read > 0 {
//                     serde_json::from_str::<Self>(&buf)
//                         .map_err(|err| MitnikaError::UnableToParseDataFile(format!("{err}")))
//                 } else {
//                     Ok(Self::new())
//                 }
//             }
//             Err(err) => Err(MitnikaError::UnableToReadDataFile(format!("{err}"))),
//         }
//     }
//
//     pub fn add_project(&mut self, name: &str) {
//         if !name.is_empty() && !self.projects().into_iter().any(|p| p.name() == name) {
//             let project = ProjectBuilder::new(name).build();
//             self.projects.insert(project.id().to_string(), project);
//         }
//     }
//
//     pub fn projects(&self) -> Vec<&Project> {
//         self.projects.values().collect()
//     }
//
//     pub fn search_projects(&self, search: &str, exact: bool) -> Vec<&Project> {
//         if search.trim().is_empty() {
//             return self.projects();
//         }
//         let mut pat = String::new();
//         if !exact {
//             pat.push_str(".*");
//         }
//         pat.push_str(&search.to_lowercase());
//         if !exact {
//             pat.push_str(".*");
//         }
//
//         let re = Regex::new(&pat).expect("Unable to create regex for search");
//         self.projects()
//             .into_iter()
//             .filter(|p| re.is_match(&p.name().to_lowercase()))
//             .collect()
//     }
// }
