use crate::app::{App, Message};
use iced::{widget::{button, column, text}, Element};

impl App {
    pub fn view_error(&self) -> Element<'_, Message> {
        let folder: String = self.folder.as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        
        column![
            text("Ошибка!").size(26),
            text(self.error.as_deref().unwrap_or("Неизвестная ошибка")),
            text(folder),
            button("Закрыть").on_press(Message::Close),
        ]
        .spacing(20)
        .into()
    }
}