use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MitnikaError {
    EmptyFilePath,
}

impl Error for MitnikaError {}

impl Display for MitnikaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyFilePath => writeln!(f, "Creation path for file is empty."),
        }
    }
}
