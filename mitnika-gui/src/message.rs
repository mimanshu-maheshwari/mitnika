use std::sync::{Arc, Mutex};

use commons::Project;

#[derive(Debug, Clone)]
pub enum MitnikaMessageKind {
    Project(ProjectMessage),
    File(FileMessage),
}

#[derive(Debug, Clone)]
pub enum ProjectMessage {
    SelectProject(Arc<Mutex<Project>>),
    SearchProject(String),
}

#[derive(Debug, Clone)]
pub enum FileMessage {
    None,
}
