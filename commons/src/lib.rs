mod error;
mod file_handler;
mod project;

pub use file_handler::{FileHandler, FileHandlerBuilder};
pub use project::{Project, ProjectBuilder};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Environment;
pub struct Version;
