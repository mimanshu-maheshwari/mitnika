use iced::Element;

use crate::MitnikaMessageKind;

#[derive(Debug, Clone, Default)]
pub enum FileView {
    #[default]
    Empty,
    Show(FileShowScreen),
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
