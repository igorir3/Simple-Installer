use crate::app::{App, Message};
use iced::{Task};

impl App {
    pub fn message_pick_folder(&mut self) -> Task<Message> {
        Task::perform(
            async {
                rfd::AsyncFileDialog::new()
                    .pick_folder()
                    .await
                    .map(|folder| folder.path().to_path_buf())
            },
            Message::FolderSelected,
        )
    }
}