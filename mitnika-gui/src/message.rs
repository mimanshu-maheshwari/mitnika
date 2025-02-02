use commons::{FileHandler, Project};

#[derive(Debug, Clone)]
pub enum MitnikaMessageKind {
    Project(ProjectMessage),
    File(FileMessage),
}

#[derive(Debug, Clone)]
pub enum ProjectMessage {
    Select(Project),
    Search(String),
    Create(String),
    SwitchToAddScreen,
    NewProjectName(String),
}

#[derive(Debug, Clone)]
pub enum FileMessage {
    Select(FileHandler),
    Search(String),
    Create(String),
    SwitchToAddScreen,
    NewFileName(String),
}
