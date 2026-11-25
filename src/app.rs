use iced::{
    widget::{center},
    Element, Length, Subscription, Task,
};
use std::{path::PathBuf, time::Duration};
use toml;
use std::sync::mpsc::Receiver;
use crate::installer::{InstallEvent};

const LICENSE_FILE: &str = include_str!("assets/LICENSE");
const DESCRIPTION_FILE: &str = include_str!("assets/description.toml");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Hello,
    License,
    SelectFolder,
    Installing,
    Finished,
    Error
}

#[derive(Debug, Clone)]
pub enum Message {
    CloseHello,
    CloseLicense,
    LicenseAccepted(bool),
    PickFolder,
    FolderSelected(Option<PathBuf>),
    Continue,
    Tick,
    Close,
}

pub struct App {
    pub screen: Screen,
    pub folder: Option<PathBuf>,
    pub progress: usize,
    pub install_rx: Option<Receiver<InstallEvent>>,
    pub total_files: usize,
    pub error: Option<String>,
    pub license_str: String,
    pub description: toml::Table,
    pub license_accepted: bool
}

impl Default for App {
    fn default() -> Self {
        let desc_vec = toml::from_str(DESCRIPTION_FILE).unwrap();
        
        Self {
            screen: Screen::Hello,
            folder: None,
            install_rx: None,
            progress: 0,
            total_files: 0,
            error: None,
            license_str: LICENSE_FILE.to_string(),
            description: desc_vec,
            license_accepted: false
        }
    }
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CloseHello => {
                    self.screen = Screen::License;
                    Task::none()
                }
            Message::LicenseAccepted(value) => {
                self.license_accepted = value;
                Task::none()
            },
            Message::CloseLicense => {
                self.screen = Screen::SelectFolder;
                Task::none()
            },
            Message::PickFolder => self.message_pick_folder(),
            Message::FolderSelected(folder) => self.message_folder_selected(folder),
            Message::Continue => self.message_continue(),
            Message::Tick => {
                if let Some(rx) = &self.install_rx {
                    while let Ok(event) = rx.try_recv() {
                        match event {
                            InstallEvent::Progress(count) => {
                                self.progress = count;
                            }

                            InstallEvent::Finished => {
                                self.screen = Screen::Finished;
                            }

                            InstallEvent::Error(error) => {
                                self.error = Some(error);
                                self.screen = Screen::Error;
                            }

                            InstallEvent::TotalSize(total_size) => {
                                self.total_files = total_size;
                            }
                        }
                    }

                    if self.screen != Screen::Installing {
                        self.install_rx = None;
                    }
            }

            Task::none()

            }

            Message::Close => iced::exit(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content: Element<'_, Message> = match self.screen {
            Screen::Hello => self.view_hello(),
            Screen::License => self.view_license(),
            Screen::SelectFolder => self.view_select_folder(),
            Screen::Installing => self.view_installing(),
            Screen::Finished => self.view_finished(),
            Screen::Error => self.view_error()
        };

        center(content)
            .padding(5)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.screen == Screen::Installing {
            iced::time::every(Duration::from_millis(10))
                .map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }
}