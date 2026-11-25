use crate::app::{App, Message};
use iced::{widget::{button, column, text, row, Space}, Element, Length};

impl App {
    pub fn view_select_folder(&self) -> Element<'_, Message> {
        let folder_name = self.folder.as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "Папка не выбрана".into());

        column![
            text("Выберите папку").size(26),
            row![
                text(folder_name).width(Length::Fill),
                button("Обзор...").on_press(Message::PickFolder)
            ].height(Length::Fill),

            row![
                Space::new().width(Length::Fill),
                button("Продолжить").on_press_maybe((self.license_accepted && self.folder.is_some()).then_some(Message::Continue))
            ]

        ]
        .spacing(20)
        .padding(10)
        .into()
    }
}