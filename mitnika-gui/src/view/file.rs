use iced::Element;

use crate::MitnikaMessageKind;

#[derive(Debug, Clone, Default)]
pub enum FileView {
    #[default]
    Empty,
    Show(FileScreen),
}

#[derive(Debug, Clone, Default)]
pub struct FileScreen;
impl FileScreen {
    pub fn _update(&mut self, _message: MitnikaMessageKind) {
        todo!();
    }

    pub fn view(&self) -> Element<MitnikaMessageKind> {
        todo!();
    }
}
