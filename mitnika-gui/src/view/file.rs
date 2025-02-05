use core::ProjectDetails;
use iced::{
    widget::{row, text_input},
    Element,
};

use crate::MitnikaMessageKind;

#[derive(Debug, Clone, Default)]
pub enum FileView {
    #[default]
    Empty,
    Show(FileShowScreen),
    Add(FileAddScreen),
}

// impl Default for FileView {
//     fn default() -> Self {
//         Self::Show(FileShowScreen::default())
//     }
// }

#[derive(Debug, Clone, Default)]
pub struct FileShowScreen;
impl FileShowScreen {
    pub fn _update(&mut self, _message: MitnikaMessageKind) {
        todo!();
    }

    pub fn view(&self) -> Element<MitnikaMessageKind> {
        todo!();
    }
}

#[derive(Debug, Clone, Default)]
pub struct FileAddScreen {
    file_name: String,
    _project: ProjectDetails,
}

impl FileAddScreen {
    pub fn update(&mut self, _message: MitnikaMessageKind) {
        todo!();
    }

    pub fn update_project_name(&mut self, name: &str) {
        self.file_name = name.to_owned();
    }

    pub fn view(&self) -> Element<MitnikaMessageKind> {
        row![text_input("Enter name for File", &self.file_name)]
            .spacing(20)
            .into()
    }
}
