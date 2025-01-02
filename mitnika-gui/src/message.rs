use commons::Project;

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
}

#[derive(Debug, Clone)]
pub enum FileMessage {
    None,
}
