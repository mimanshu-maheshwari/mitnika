use std::fs;

use directories::ProjectDirs;

use crate::{
    MitnikaError, AVIDVIVARTA_LOWER, MITNIKA_LOWER, MM, SQLITE_FILE, SQLITE_FILE_EXTENSION,
};

pub(super) fn get_dir() -> Result<std::path::PathBuf, MitnikaError> {
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
        return Ok(file_path);
    } else {
        return Err(MitnikaError::ProjectUserDirectoryNotCreated);
    }
}
