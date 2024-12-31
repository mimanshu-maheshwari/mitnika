use std::sync::{Arc, Mutex};

use commons::Project;
use iced::{
    widget::{row, text},
    window, Element, Theme,
};

use crate::{MitnikaMessageKind, MitnikaState, ProjectMessage};

#[derive(Debug, Clone)]
pub struct MitnikaView {
    title: String,
    theme: Theme,
}

impl Default for MitnikaView {
    fn default() -> Self {
        Self {
            title: String::from("Mitnika"),
            theme: Theme::TokyoNight,
        }
    }
}

impl MitnikaView {
    pub fn title(&self) -> String {
        self.title.to_owned()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
    pub fn window_settings(&self) -> window::Settings {
        window::Settings::default()
    }
}

#[derive(Debug, Clone)]
pub enum MitnikaScreen {
    Project(ProjectScreen),
    File(FileScreen),
}

impl Default for MitnikaScreen {
    fn default() -> Self {
        Self::Project(ProjectScreen::default())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProjectScreen {
    selected_project: Arc<Mutex<Project>>,
}

impl ProjectScreen {
    pub fn update(&mut self, message: MitnikaMessageKind) {
        todo!();
    }

    pub fn view(&self) -> Element<MitnikaMessageKind> {
        let selected_project = self
            .selected_project
            .lock()
            .expect("Unable to get lock on selected prject");
        row![text(selected_project.get_name().to_owned())]
            .spacing(20)
            .into()
    }

    pub fn handle_message(&mut self, message: MitnikaMessageKind) {
        match message {
            MitnikaMessageKind::Project(project_mesage) => match project_mesage {
                ProjectMessage::SelectProject(project_ref) => {
                    todo!();
                }
                ProjectMessage::SearchProject(value) => {
                    println!("{value}");
                }
            },
            MitnikaMessageKind::File(file_mesage) => {
                todo!();
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FileScreen;
impl FileScreen {
    pub fn update(&mut self, message: MitnikaMessageKind) {
        todo!();
    }

    pub fn view(&self) -> Element<MitnikaMessageKind> {
        todo!();
    }

    pub fn handle_message(&mut self, message: MitnikaMessageKind) {
        todo!()
    }
}
