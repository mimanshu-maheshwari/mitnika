use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MitnikaError {
    EmptyFileCreationPath,
    InvalidFileCreationPath,
}

impl Error for MitnikaError {}

impl Display for MitnikaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyFileCreationPath => writeln!(f, "Creation path for file is empty."),
            Self::InvalidFileCreationPath => writeln!(f, "Creation path for file is Invalid."),
        }
    }
}
