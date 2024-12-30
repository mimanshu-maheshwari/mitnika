use commons::Mitnika;
use iced::{
    widget::{column, container, text, text_input},
    Element, Theme,
};

fn main() -> iced::Result {
    iced::application("Mitnika", State::update, State::view)
        .theme(State::theme)
        .run()
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct State {
    mitnika: Mitnika,
    screen: Screen,
    value: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct ProjectScreen;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Message {
    SearchChanged(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Screen {
    Project(ProjectScreen),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Project(ProjectScreen)
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            screen: Screen::Project(ProjectScreen),
            mitnika: Mitnika::new(),
            value: String::from("default"),
        }
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SearchChanged(value) => {
                self.value = value;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let search = text_input("Search Project", &self.value)
            .size(20)
            .padding(10)
            .on_input(Message::SearchChanged);
        let value = text(self.value.clone());
        container(column![search, value].spacing(10))
            .padding(10)
            .into()
    }
}
