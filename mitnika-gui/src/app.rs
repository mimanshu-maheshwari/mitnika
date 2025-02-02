use super::MitnikaState;

pub fn run() -> iced::Result {
    iced::application(
        MitnikaState::title,
        MitnikaState::update,
        MitnikaState::view,
    )
    .window(iced::window::Settings {
        position: iced::window::Position::Centered,
        size: iced::Size::new(1280.0, 720.0),
        icon: Some(
            iced::window::icon::from_file_data(include_bytes!("../res/favicon.ico"), None)
                .expect("Unable to load icon"),
        ),
        ..Default::default()
    })
    .theme(MitnikaState::theme)
    .run_with(MitnikaState::new)
}
