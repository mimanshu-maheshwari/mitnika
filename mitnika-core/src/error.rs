use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MitnikaError {
    EmptyFileCreationPath,
    InvalidFileCreationPath,
    ProjectNotFound(String),
    FileNotFound(String),
    FileCreationPathRequired(String),
    EnvironmentNotFound(String),
    VersionNotFound(String),
    LocalUserDirectoryNotFound(String),
    UnableToReadDataFile(String),
    UnableToParseDataFile(String),
    PoolNotCreated,
    ProjectUserDirectoryNotCreated,
    SQLiteFileNotCreated,
    SQLiteDBError(String),
    RuntimeCreationError(String),
    ProjectAlreadyExists,
    FileAlreadyExists,
}

impl Error for MitnikaError {}

impl Display for MitnikaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyFileCreationPath => writeln!(f, "Creation path for file is empty."),
            Self::InvalidFileCreationPath => writeln!(f, "Creation path for file is Invalid."),
            Self::ProjectNotFound(project_name) => {
                writeln!(f, "Project with name [{project_name}] not found.")
            }
            Self::FileNotFound(file_name) => {
                writeln!(f, "File with name [{file_name}] not found.")
            }
            Self::FileCreationPathRequired(file_name) => {
                writeln!(
                    f,
                    "File creation path is required for file with name [{file_name}]."
                )
            }
            Self::EnvironmentNotFound(env_name) => {
                writeln!(f, "Environment not found [{env_name}].")
            }
            Self::VersionNotFound(ver_name) => {
                writeln!(f, "Version not found [{ver_name}].")
            }
            Self::LocalUserDirectoryNotFound(err) => {
                writeln!(f, "Unable to find local user directory: [{err}]")
            }
            Self::UnableToReadDataFile(err) => {
                writeln!(f, "Unable to read data file: [{err}]")
            }
            Self::UnableToParseDataFile(err) => {
                writeln!(f, "Unable to parse data file: [{err}]")
            }
            Self::PoolNotCreated => writeln!(f, "Error creating DB connection Pool"),
            Self::ProjectUserDirectoryNotCreated => {
                writeln!(f, "Unable to create project user directories")
            }
            Self::SQLiteDBError(e) => writeln!(f, "Error occured while db operation: {}", e),
            Self::SQLiteFileNotCreated => writeln!(f, "Unable to create SQLite file"),
            Self::RuntimeCreationError(e) => writeln!(f, "Unable to create a runtime: {}", e),
            Self::ProjectAlreadyExists => writeln!(f, "Project already exists"),
            Self::FileAlreadyExists => writeln!(f, "File already exists"),
        }
    }
}
