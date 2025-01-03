use commons::Project;
use iced::{
    widget::{container, row, text, text_input},
    Element,
};

use crate::{MitnikaMessageKind, ProjectMessage};

#[derive(Debug, Clone)]
pub enum ProjectView {
    Empty(ProjectEmptyScreen),
    Show(ProjectShowScreen),
    Edit(ProjectEditScreen),
    Add(ProjectAddScreen),
}

impl Default for ProjectView {
    fn default() -> Self {
        Self::Empty(ProjectEmptyScreen)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProjectEmptyScreen;
impl ProjectEmptyScreen {
    pub fn update(&mut self, _message: MitnikaMessageKind) {
        todo!();
    }

    pub fn view(&self) -> Element<MitnikaMessageKind> {
        container(text("Select/Modify/Add Project")).into()
    }
}
#[derive(Debug, Clone, Default)]
pub struct ProjectShowScreen {
    selected_project: Project,
}

impl ProjectShowScreen {
    pub fn update(&mut self, _message: MitnikaMessageKind) {
        todo!();
    }
    pub fn set_selected_project(&mut self, project: Project) {
        self.selected_project = project;
    }

    pub fn view(&self) -> Element<MitnikaMessageKind> {
        row![text(self.selected_project.name().to_owned())]
            .spacing(20)
            .into()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProjectEditScreen {
    selected_project: Project,
}

impl ProjectEditScreen {
    pub fn update(&mut self, _message: MitnikaMessageKind) {
        todo!();
    }
    pub fn set_selected_project(&mut self, project: Project) {
        self.selected_project = project;
    }

    pub fn view(&self) -> Element<MitnikaMessageKind> {
        row![text(self.selected_project.name().to_owned())]
            .spacing(20)
            .into()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProjectAddScreen {
    project_name: String,
}

impl ProjectAddScreen {
    pub fn update(&mut self, _message: MitnikaMessageKind) {
        todo!();
    }

    pub fn update_project_name(&mut self, name: &str) {
        self.project_name = name.to_owned();
    }

    pub fn view(&self) -> Element<MitnikaMessageKind> {
        row![
            text_input("Enter name for project", &self.project_name)
                .on_input(
                    |value| MitnikaMessageKind::Project(ProjectMessage::NewProjectName(value))
                )
                .on_submit(MitnikaMessageKind::Project(ProjectMessage::Create(
                    self.project_name.clone()
                ))),
            text(&self.project_name)
        ]
        .spacing(20)
        .into()
    }
}
