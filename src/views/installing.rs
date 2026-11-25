use crate::app::{App, Message};
use iced::{widget::{progress_bar, column, text}, Element};

impl App {
    pub fn view_installing(&self) -> Element<'_, Message> {
        let (progress, total) = (self.progress, self.total_files);
        column![
                    text("Установка...").size(26),
                    progress_bar(0.0..=total as f32, progress as f32),
                    text(format!("{progress} / {total}")),
                ]
                .spacing(20)
                .into()
    }
}