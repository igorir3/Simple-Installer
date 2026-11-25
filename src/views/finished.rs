use crate::app::{App, Message};
use iced::{widget::{button, column, text, Space, row}, Element, Length};

impl App {
    pub fn view_finished(&self) -> Element<'_, Message> {
        let folder: String = self.folder.as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_default();

        column![
            text("Готово!").size(26),
            column![
                text("Программа установленна в: "),
                text(folder)
            ].height(Length::Fill),
            row![
                Space::new().width(Length::Fill),
                button("Закрыть").on_press(Message::Close)
            ]
            
        ]
        .spacing(20)
        .into()
    }
}