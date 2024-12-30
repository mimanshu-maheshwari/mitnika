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
        }
    }
}
