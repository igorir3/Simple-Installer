use crate::app::{App, Message};
use iced::{Task};

impl App {
    pub fn message_folder_selected(&mut self, folder: Option<std::path::PathBuf>) -> Task<Message> {
        if let Some(folder) = folder {
            let install_path = folder.join(env!("APP_NAME"));
            self.folder = Some(install_path);
        }
        Task::none()
    }
}