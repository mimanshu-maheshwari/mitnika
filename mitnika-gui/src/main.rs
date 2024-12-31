use mitnika_gui::MitnikaState;

fn main() -> iced::Result {
    iced::application(
        MitnikaState::title,
        MitnikaState::update,
        MitnikaState::view,
    )
    .window(iced::window::Settings::default())
    .theme(MitnikaState::theme)
    .run()
}
