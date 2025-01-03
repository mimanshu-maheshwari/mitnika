use iced::{window, Size};
use mitnika_gui::MitnikaState;

fn main() -> iced::Result {
    iced::application(
        MitnikaState::title,
        MitnikaState::update,
        MitnikaState::view,
    )
    .window(iced::window::Settings {
        position: window::Position::Centered,
        size: Size::new(1280.0, 720.0),
        icon: Some(
            window::icon::from_file_data(include_bytes!("../res/favicon.ico"), None)
                .expect("Unable to load icon"),
        ),
        ..Default::default()
    })
    .theme(MitnikaState::theme)
    .run_with(MitnikaState::new)
}
