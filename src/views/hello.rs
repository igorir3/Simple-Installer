use crate::app::{App, Message};
use iced::{widget::{button, column, text, container, row, Space}, Element, Length};

impl App {
    pub fn view_hello(&self) -> Element<'_, Message> {
        column![
            text(self.description["title"].as_str().unwrap_or("")).size(26),
            container(
                text(self.description["description"].as_str().unwrap_or("")))
                .padding(10)
                .width(Length::Fill)
                .height(Length::Fill),
            row![
                Space::new().width(Length::Fill),
                button("Продолжить").on_press(Message::CloseHello),
            ]
        ]
        .spacing(20)
        .padding(10)
        .into()
    }
}