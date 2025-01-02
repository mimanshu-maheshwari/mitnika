use commons::MITNIKA_TITLE;
use iced::{window, Theme};

pub mod file;
pub mod project;
pub use file::{FileScreen, FileView};
pub use project::{ProjectAddScreen, ProjectEditScreen, ProjectShowScreen, ProjectView};

#[derive(Debug, Clone)]
pub struct MitnikaView {
    title: String,
    theme: Theme,
}

impl Default for MitnikaView {
    fn default() -> Self {
        Self {
            title: String::from(MITNIKA_TITLE),
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
    Project(ProjectView),
    File(FileView),
}

impl Default for MitnikaScreen {
    fn default() -> Self {
        Self::Project(ProjectView::default())
    }
}
