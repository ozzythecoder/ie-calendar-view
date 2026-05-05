#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use iced::window;

mod app;
mod calendar;
mod csv;
mod screen;
mod dialog;
mod platform;
mod converter;

fn main() -> iced::Result {
    let position = if cfg!(debug_assertions) {
        window::Position::Specific((800.00, 320.00).into())
    } else {
        window::Position::Default
    };

    iced::application(app::Application::new, app::Application::update, app::Application::view)
        .window(window::Settings {
            size: (512.00, 360.00).into(),
            position: position,
            ..Default::default()
        })
        .theme(iced::Theme::CatppuccinMocha)
        .title("CSV to Calendar Converter")
        .run()
}
