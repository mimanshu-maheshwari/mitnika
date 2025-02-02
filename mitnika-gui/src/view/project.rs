use commons::Project;
use iced::{
    widget::{button, column, container, row, text, text_input},
    Alignment, Element, Length,
};

use crate::{FileMessage, MitnikaMessageKind, ProjectMessage};

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
        let edit_button = container(button(text("Edit".to_owned())));
        let header_text = container(text(self.selected_project.name().to_owned()));
        let header_row = row![header_text, edit_button].spacing(10);
        let files_name = self.selected_project.files().values();
        let mut file_buttons = column![].spacing(10);
        for file_handler in files_name {
            file_buttons = file_buttons.push(
                button(text(file_handler.name().to_owned()))
                    .on_press(MitnikaMessageKind::File(FileMessage::Select(
                        file_handler.clone(),
                    )))
                    // .padding(10)
                    .width(Length::Fill),
            );
        }
        let add_file_button = container(
            button(text(String::from("Add File")))
                .on_press(MitnikaMessageKind::File(FileMessage::SwitchToAddScreen))
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::FillPortion(1))
        .align_x(Alignment::Start)
        .align_y(Alignment::End);
        file_buttons = file_buttons.push(add_file_button);
        let data_row = row![file_buttons].spacing(10);
        let cols = column![header_row, data_row].spacing(10);
        container(cols).into()
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
