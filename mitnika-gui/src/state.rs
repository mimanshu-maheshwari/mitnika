use commons::Storage;

use iced::{
    widget::{button, column, container, row, text, text_input},
    window, Alignment, Element, Length, Task, Theme,
};

use crate::{
    view::{FileView, ProjectAddScreen, ProjectShowScreen, ProjectView},
    MitnikaMessageKind, MitnikaScreen, MitnikaView, ProjectMessage,
};

#[derive(Debug, Default, Clone)]
pub struct MitnikaState {
    screen: MitnikaScreen,
    data: Storage,
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

    pub fn new() -> (Self, Task<MitnikaMessageKind>) {
        let data = Storage::default();
        let screen = MitnikaScreen::Project(ProjectView::default());
        let project_search = String::new();
        let view = MitnikaView::default();
        (
            Self {
                data,
                screen,
                project_search,
                view,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: MitnikaMessageKind) {
        match message {
            MitnikaMessageKind::Project(project_message) => match project_message {
                ProjectMessage::Select(project) => {
                    let mut screen = ProjectShowScreen::default();
                    screen.set_selected_project(project.clone());
                    self.screen = MitnikaScreen::Project(ProjectView::Show(screen));
                }
                ProjectMessage::Search(search_value) => {
                    self.project_search = search_value.clone();
                    self.data.search_projects(&search_value, false);
                }
                ProjectMessage::Create(project_name) => {
                    self.data.add_project(&project_name);
                }
                ProjectMessage::SwitchToAddScreen => {
                    self.screen =
                        MitnikaScreen::Project(ProjectView::Add(ProjectAddScreen::default()));
                }
            },
            MitnikaMessageKind::File(_file_message) => {}
        }
    }

    pub fn view(&self) -> Element<MitnikaMessageKind> {
        // let search = text_input("Search Project", &self.project_search)
        //     .on_input(|value| MitnikaMessageKind::Project(ProjectMessage::Search(value)));
        // let mut project_row = row![search];
        // let projects = self.data.projects();
        // for project in projects {
        //     project_row = project_row.push(button(text(project.name().to_owned())).on_press(
        //         MitnikaMessageKind::Project(ProjectMessage::Select(project.clone())),
        //     ));
        // }

        let left_part = self.sidebar(); // container(project_row);

        let right_part = match &self.screen {
            MitnikaScreen::File(file_screen) => match file_screen {
                FileView::Empty => {
                    todo!()
                }
                FileView::Show(show_screen) => show_screen.view(),
            },
            MitnikaScreen::Project(project_screen) => match project_screen {
                ProjectView::Add(add_screen) => add_screen.view(),
                ProjectView::Show(show_screen) => show_screen.view(),
                ProjectView::Edit(edit_screen) => edit_screen.view(),
            },
        };

        column![left_part, right_part]
            .spacing(10)
            .padding(10)
            .into()
    }

    pub fn sidebar(&self) -> Element<MitnikaMessageKind> {
        let search = text_input("Search Project", &self.project_search)
            .on_input(|value| MitnikaMessageKind::Project(ProjectMessage::Search(value)));
        let mut project_row = row![search];
        let projects = self.data.projects();
        for project in projects {
            project_row = project_row.push(button(text(project.name().to_owned())).on_press(
                MitnikaMessageKind::Project(ProjectMessage::Select(project.clone())),
            ));
        }
        let add_new_project_button = button(text(String::from("Add Project"))).on_press(
            MitnikaMessageKind::Project(ProjectMessage::SwitchToAddScreen),
        );
        project_row = project_row.push(add_new_project_button);
        project_row = project_row.padding(10).spacing(10);

        let left_part = container(project_row)
            .padding(10)
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .align_x(Alignment::Start)
            .align_y(Alignment::End);
        left_part.into()
    }
}
