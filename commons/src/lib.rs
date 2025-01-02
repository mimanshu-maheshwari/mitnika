mod environment;
mod error;
mod file_handler;
mod project;
mod storage;
mod version;

pub use environment::Environment;
pub use error::MitnikaError;
pub use file_handler::{FileHandler, FileHandlerBuilder};
pub use project::{Project, ProjectBuilder};
pub use storage::{MitnikaData, Storage};
pub use version::Version;

pub type Result<T> = std::result::Result<T, MitnikaError>;
// Mitnika default strings
pub const DEFAULT_STRING: &str = "default";

pub const MITNIKA_LOWER: &str = "mitnika";
pub const MITNIKA_UPPER: &str = "MITNIKA";
pub const MITNIKA_TITLE: &str = "Mitnika";

pub const AVIDVIVARTA_LOWER: &str = "avidvivarta";
pub const AVIDVIVARTA_UPPER: &str = "AVIDVIVARTA";
pub const AVIDVIVARTA_TITLE: &str = "AvidVivarta";

pub const MM: &str = "mm";

const DATA_FILE: &str = "projects.json";
const EXTENSION: &str = "json";
