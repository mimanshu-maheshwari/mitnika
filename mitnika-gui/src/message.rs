use core::{FileDetails, ProjectDetails};

#[derive(Debug, Clone)]
pub enum MitnikaMessageKind {
    Project(ProjectMessage),
    File(FileMessage),
}

#[derive(Debug, Clone)]
pub enum ProjectMessage {
    Select(ProjectDetails),
    Search(String),
    Create(String),
    SwitchToAddScreen,
    NewProjectName(String),
}

#[derive(Debug, Clone)]
pub enum FileMessage {
    Select(FileDetails),
    Search(String),
    Create(String),
    SwitchToAddScreen,
    NewFileName(String),
}
