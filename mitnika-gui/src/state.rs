use commons::Storage;

use iced::{
    widget::{button, column, container, row, scrollable, text, text_input},
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
        let screen = MitnikaScreen::default();
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
                }
                ProjectMessage::Create(project_name) => {
                    self.data.add_project(&project_name);
                    if let MitnikaScreen::Project(ProjectView::Add(add_screen)) = &mut self.screen {
                        add_screen.update_project_name("");
                    }
                }
                ProjectMessage::SwitchToAddScreen => {
                    self.screen =
                        MitnikaScreen::Project(ProjectView::Add(ProjectAddScreen::default()));
                }
                ProjectMessage::NewProjectName(name) => {
                    if let MitnikaScreen::Project(ProjectView::Add(add_screen)) = &mut self.screen {
                        add_screen.update_project_name(&name);
                    }
                }
            },
            MitnikaMessageKind::File(_file_message) => {}
        }
    }

    pub fn view(&self) -> Element<MitnikaMessageKind> {
        let left_part = self.sidebar();

        let right_part = match &self.screen {
            MitnikaScreen::File(file_screen) => match file_screen {
                FileView::Empty => {
                    todo!()
                }
                FileView::Show(show_screen) => show_screen.view(),
            },
            MitnikaScreen::Project(project_screen) => match project_screen {
                ProjectView::Empty(empty_screen) => empty_screen.view(),
                ProjectView::Add(add_screen) => add_screen.view(),
                ProjectView::Show(show_screen) => show_screen.view(),
                ProjectView::Edit(edit_screen) => edit_screen.view(),
            },
        };
        let right_part = container(right_part)
            .padding(10)
            .width(Length::FillPortion(5))
            .height(Length::Fill)
            .align_x(Alignment::Start)
            .align_y(Alignment::Start);

        let view_row = container(row![left_part, right_part]);
        view_row.into()
    }

    pub fn sidebar(&self) -> Element<MitnikaMessageKind> {
        // search bar for projects
        let search = container(
            text_input("Search Project", &self.project_search)
                .on_input(|value| MitnikaMessageKind::Project(ProjectMessage::Search(value))),
        )
        .height(Length::FillPortion(1))
        .align_x(Alignment::Start);

        let mut project_column = column![search];

        // projects selection buttons
        let mut buttons = column![];
        let filterd_projects = self.data.search_projects(&self.project_search, false);
        for project in filterd_projects {
            buttons = buttons.push(
                button(text(project.name().to_owned()))
                    .on_press(MitnikaMessageKind::Project(ProjectMessage::Select(
                        project.clone(),
                    )))
                    .padding(10)
                    .width(Length::Fill),
            );
        }
        project_column = project_column.push(
            container(scrollable(buttons).spacing(10))
                .width(Length::Fill)
                .align_x(Alignment::Start)
                .align_y(Alignment::Start)
                .height(Length::FillPortion(8)),
        );

        // button to add projects
        let add_new_project_button = container(
            button(text(String::from("Add Project")))
                .on_press(MitnikaMessageKind::Project(
                    ProjectMessage::SwitchToAddScreen,
                ))
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::FillPortion(1))
        .align_x(Alignment::Start)
        .align_y(Alignment::End);
        project_column = project_column.push(add_new_project_button);
        project_column = project_column.padding(10).spacing(10);

        // group data
        let sidebar = container(project_column)
            .padding(10)
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .align_x(Alignment::Start)
            .align_y(Alignment::Start);
        sidebar.into()
    }
}
