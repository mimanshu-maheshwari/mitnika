use commons::Project;
use iced::{
    widget::{row, text, text_input},
    Element,
};

use crate::{MitnikaMessageKind, ProjectMessage};

#[derive(Debug, Clone)]
pub enum ProjectView {
    Show(ProjectShowScreen),
    Edit(ProjectEditScreen),
    Add(ProjectAddScreen),
}

impl Default for ProjectView {
    fn default() -> Self {
        Self::Show(ProjectShowScreen::default())
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

    pub fn view(&self) -> Element<MitnikaMessageKind> {
        row![
            text_input("Enter name for project", &self.project_name).on_submit(
                MitnikaMessageKind::Project(ProjectMessage::Create(self.project_name.clone()))
            ),
            text(&self.project_name)
        ]
        .spacing(20)
        .into()
    }
}
