use crate::app::{App, Message, Screen};
use iced::{Task};
use crate::installer::{self};

impl App {
    pub fn message_continue(&mut self) -> Task<Message> {
        if let Some(folder) = self.folder.clone() {
            self.progress = 0;
            self.install_rx = Some(
                installer::start_installation(folder)
            );
            self.screen = Screen::Installing;
        }

        Task::none()

    }
}