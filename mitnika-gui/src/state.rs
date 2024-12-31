use std::sync::Arc;

use commons::Mitnika;
use iced::{
    widget::{button, column, container, text, text_input, Row},
    window, Element, Theme,
};

use crate::{MitnikaMessageKind, MitnikaScreen, MitnikaView, ProjectMessage};

#[derive(Debug, Default, Clone)]
pub struct MitnikaState {
    screen: MitnikaScreen,
    data: Mitnika,
    project_search: String,
    view: MitnikaView,
}

impl MitnikaState {
    pub fn theme(&self) -> Theme {
        self.view.theme()
    }

    pub fn title(&self) -> String {
        self.view.title()
    }

    pub fn window_settings(&self) -> window::Settings {
        self.view.window_settings()
    }

    pub fn update(&mut self, message: MitnikaMessageKind) {
        match message {
            MitnikaMessageKind::Project(project_message) => match project_message {
                ProjectMessage::SelectProject(_project) => {
                    todo!();
                }
                ProjectMessage::SearchProject(search_value) => {
                    self.project_search = search_value.clone();
                    self.data.search_projects(&search_value);
                }
            },
            MitnikaMessageKind::File(_file_message) => {}
        }
    }

    pub fn view(&self) -> Element<MitnikaMessageKind> {
        let mut project_row = Row::new();
        project_row =
            project_row.push(text_input("Search Project", &self.project_search).on_input(
                |value| MitnikaMessageKind::Project(ProjectMessage::SearchProject(value)),
            ));
        let projects = self.data.get_projects();
        for project in projects {
            let project_mut = project
                .lock()
                .expect("Unable to aquire lock on project for list");
            project_row =
                project_row.push(button(text(project_mut.get_name().to_owned())).on_press(
                    MitnikaMessageKind::Project(ProjectMessage::SelectProject(Arc::clone(
                        &project,
                    ))),
                ));
        }
        let left_part = container(project_row);
        let right_part = match &self.screen {
            MitnikaScreen::File(file_screen) => file_screen.view(),
            MitnikaScreen::Project(project_screen) => project_screen.view(),
        };
        column![left_part, right_part].spacing(10).into()
    }
}
