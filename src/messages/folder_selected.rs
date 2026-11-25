use crate::app::{App, Message};
use iced::{Task};

impl App {
    pub fn message_folder_selected(&mut self, folder: Option<std::path::PathBuf>) -> Task<Message> {
        if let Some(folder) = folder {
            self.folder = Some(folder);
        }
        Task::none()
    }
}