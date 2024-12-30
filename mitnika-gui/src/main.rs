use iced::widget::{button, column, text};
use iced::Element;
use iced::Theme;

pub fn main() -> iced::Result {
    iced::application("Mitnika", update, view)
        .theme(theme)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}
#[derive(Default, Debug, Clone)]
struct Counter {
    value: i64,
}

fn theme(_state: &Counter) -> Theme {
    Theme::TokyoNight
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
        Message::Decrement => counter.value -= 1,
    }
}

fn view(counter: &Counter) -> Element<Message> {
    column![
        button("Decrement").on_press(Message::Decrement),
        text(counter.value).size(20),
        button("Increment").on_press(Message::Increment),
    ]
    .spacing(10)
    .into()
}
