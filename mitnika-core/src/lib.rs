mod environment;
mod error;
mod file_details;
mod project;
mod storage;
mod version;

pub use environment::EnvironmentDetails;
pub use error::MitnikaError;
pub use file_details::FileDetails;
pub use project::ProjectDetails;
pub use storage::Storage;
pub use version::VersionDetails;

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

const SQLITE_FILE: &str = "data";
const SQLITE_FILE_EXTENSION: &str = "db";

const _DATA_FILE: &str = "projects.json";
const _EXTENSION: &str = "json";
