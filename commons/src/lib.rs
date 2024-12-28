mod environment;
mod error;
mod file_handler;
mod project;
mod version;

pub use environment::Environment;
pub use file_handler::{FileHandler, FileHandlerBuilder};
pub use project::{Project, ProjectBuilder};
pub use version::Version;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
