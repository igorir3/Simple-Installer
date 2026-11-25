mod app;
mod views;
mod messages;
mod installer;
fn main() -> iced::Result {
    iced::application(app::App::default, app::App::update, app::App::view)
        .title(|_: &app::App| {format!("Installer {}", env!("APP_NAME"))})
        .subscription(app::App::subscription)
        .window_size((650.0, 450.0))
        .run()
}
