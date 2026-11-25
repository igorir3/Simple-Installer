use crate::app::{App, Message};
use iced::{widget::{scrollable, text, container, button, column, checkbox, row, Space}, Element, Length};

impl App {
    pub fn view_license(&self) -> Element<'_, Message> {
        column![
            text("Лицензия").size(26),
            scrollable(
                container(
                    text(self.license_str.as_str())
                        .size(14)
                )
                .padding(10)
                .width(Length::Fill)
            )
                .height(Length::Fill)
                .width(Length::Fill),
                checkbox(self.license_accepted).label("Я принимаю условия лицензии").on_toggle(Message::LicenseAccepted),

            row![
                button("Закрыть").on_press(Message::Close),
                Space::new().width(Length::Fill),
                button("Продолжить").on_press_maybe(self.license_accepted.then_some(Message::CloseLicense)),
            ],
            
        ]
        .spacing(20)
        .padding(10)
        .into()
    }
}