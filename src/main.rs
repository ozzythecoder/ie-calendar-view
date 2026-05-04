#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

pub mod calendar;
pub mod csv;
pub mod ui;

use iced::{
    Color, Font, Length, Task,
    alignment::{Horizontal, Vertical},
    color,
    futures::never,
    never,
    widget::{Column, Space, button, column, rich_text, row, span, text, text_input},
    window,
};
use rfd::AsyncFileDialog;
use std::{
    env,
    error::Error,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::ui::confirmation_dialog;

#[derive(Default)]
struct Application {
    view: View,
    csv_path: PathBuf,
    export_path: PathBuf,
    filename_out: String,
}

#[derive(Default)]
enum View {
    #[default]
    Form,
    ConfirmOverwrite,
}

#[derive(Debug, Clone)]
enum Message {
    OpenFileDialog,
    OpenExportDialog,
    ErrorChanged(String),
    FilenameOutChanged(String),
    FileSelected(Option<PathBuf>),
    ExportPathSelected(Option<PathBuf>),
    Convert,
    ConfirmOverwrite(confirmation_dialog::Message),
}

impl Application {
    pub fn view(&self) -> Column<'_, Message> {
        match self.view  {
            View::Form => {
                column![
                    text("Export a CSV from IntelliEvent, then use this tool to convert it into a calendar file. You can then import this file into a calendar app of your choice.")
                        .size(14)
                        .color(color!(180, 180, 180)),
                    row![
                        button("Choose CSV File").on_press(Message::OpenFileDialog),
                        Space::new().width(Length::Fill),
                        text(self.csv_path.display().to_string()).wrapping(text::Wrapping::WordOrGlyph),
                    ]
                    .spacing(8)
                    .width(Length::Fill)
                    .align_y(Vertical::Center),
                    row![
                        button("Choose Destination")
                            .on_press(Message::OpenExportDialog),
                        Space::new().width(Length::Fill),
                        text(self.export_path.display().to_string()),
                    ]
                    .spacing(8)
                    .width(Length::Fill)
                    .align_y(Vertical::Center),
                    row![
                        text("Filename:").color(color!(210, 210, 210)),
                        text_input("example", &self.filename_out).on_input(Message::FilenameOutChanged),
                        text(".ics"),
                    ]
                    .spacing(8)
                    .width(Length::Fixed(360.00))
                    .align_y(Vertical::Center),
                    button("Convert!").on_press(Message::Convert),
                ]
                .spacing(14)
                .padding(32)
                .align_x(Horizontal::Center)
                .width(Length::Fixed(512.00))
                .into()
            },
            View::ConfirmOverwrite => {
                column![
                    rich_text![
                        span("This will overwrite an existing file at "),
                        span(
                            format!(
                                "{}.ics",
                                self.export_path.join(&self.filename_out).display().to_string()
                            )
                        )
                        .font(Font { style: iced::font::Style::Italic, ..Font::DEFAULT })
                        .color(Color::from_rgb(0.5, 0.5, 1.0)),
                        span(". Proceed?")
                    ]
                        .on_link_click(never),
                    row![
                        button("Cancel")
                            .on_press(Message::ConfirmOverwrite(confirmation_dialog::Message::Cancel)),
                        button("Confirm")
                            .on_press(Message::ConfirmOverwrite(confirmation_dialog::Message::Confirm))
                            .style(button::danger),
                    ]
                    .padding(8)
                    .spacing(14)
                ]
                .padding(32)
                .align_x(Horizontal::Center)
                .width(Length::Fixed(512.00))
                .into()
            },
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FilenameOutChanged(filename) => {
                self.filename_out = filename;
                Task::none()
            }
            Message::FileSelected(path) => {
                if let Some(p) = path {
                    self.csv_path = p;
                };
                Task::none()
            }
            Message::ExportPathSelected(path) => {
                if let Some(p) = path {
                    self.export_path = p;
                }
                Task::none()
            }
            Message::OpenFileDialog => {
                let directory = if !self.csv_path.as_os_str().is_empty() {
                    &self.csv_path
                } else {
                    Path::new("/")
                }
                .to_owned();
                Task::perform(
                    async {
                        AsyncFileDialog::new()
                            .add_filter("csv", &["csv"])
                            .set_directory(directory)
                            .pick_file()
                            .await
                            .map(|f| f.path().to_path_buf())
                    },
                    Message::FileSelected,
                )
            }
            Message::OpenExportDialog => {
                let directory = if !self.export_path.as_os_str().is_empty() {
                    &self.export_path
                } else if !self.csv_path.as_os_str().is_empty() {
                    match &self.csv_path.parent() {
                        Some(p) => p,
                        None => Path::new("/"),
                    }
                } else {
                    Path::new("/")
                }
                .to_owned();
                Task::perform(
                    async {
                        AsyncFileDialog::new()
                            .set_directory(directory)
                            .pick_folder()
                            .await
                            .map(|f| f.path().to_path_buf())
                    },
                    Message::ExportPathSelected,
                )
            }
            Message::Convert => {
                let mut output = self.export_path.join(&self.filename_out);
                output.add_extension("ics");
                println!("Output: {}", output.display());
                if let Ok(exists) = output.try_exists()
                    && exists
                {
                    self.view = View::ConfirmOverwrite;
                } else {
                    self.perform_conversion();
                }
                Task::none()
            }
            Message::ConfirmOverwrite(msg) => match msg {
                confirmation_dialog::Message::Cancel => {
                    self.view = View::Form;
                    Task::none()
                }
                confirmation_dialog::Message::Confirm => {
                    self.perform_conversion();
                    self.view = View::Form;
                    Task::none()
                }
            },
            Message::ErrorChanged(msg) => Task::none(),
        }
    }

    fn perform_conversion(&self) {
        match convert_to_ics(self.get_conversion_options().unwrap()) {
            Ok(file) => {
                Command::new(self.get_open_command())
                    .arg(file)
                    .spawn()
                    .unwrap();
            }
            Err(e) => {
                println!("Error converting CSV: {}", e);
            }
        };
    }

    fn get_open_command(&self) -> String {
        match env::consts::OS {
            "macos" => "open".to_owned(),
            "windows" => "explorer".to_owned(),
            _ => "xdg-open".to_owned(),
        }
    }

    fn get_conversion_options(&self) -> Result<ConvertToIcsOptions<'_>, &str> {
        let Some(csv_path) = self.csv_path.to_str() else {
            // TODO handle error
            println!("No csv_path found");
            return Err("No");
        };

        let Some(export_path) = self.export_path.to_str() else {
            // TODO handle error
            println!("No export_path found");
            return Err("No");
        };

        Ok(ConvertToIcsOptions {
            csv_path: csv_path.to_owned(),
            export_path: export_path.to_owned(),
            filename_out: &self.filename_out,
        })
    }
}

struct ConvertToIcsOptions<'a> {
    pub csv_path: String,
    pub export_path: String,
    pub filename_out: &'a str,
}

fn convert_to_ics(o: ConvertToIcsOptions) -> Result<OsString, Box<dyn Error>> {
    use calendar::{BuildCalendarOptions, build_calendar};

    let opts = BuildCalendarOptions {
        csv_file_path: &o.csv_path,
        filename_out: &o.filename_out,
    };

    let (cal, error_count) = match build_calendar(opts) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    if error_count > 0 {
        println!("Skipped {} row(s) due to parsing errors.", error_count)
    };

    let full_file_out = Path::new(&o.export_path)
        .to_path_buf()
        .join(o.filename_out)
        .with_added_extension("ics");

    println!("Writing to {}", full_file_out.display());

    match fs::exists(&full_file_out) {
        Ok(f) => {
            if f {
                println!("Overwriting {}", full_file_out.display());
                // TODO
                println!("TODO: Ask confirmation before overwriting");
            }
        }
        Err(e) => {
            println!("Error when checking for overwrite: {}", e);
            // TODO
            println!("TODO: Handle malformation errors");
        }
    }

    match cal.save_file(&full_file_out) {
        Ok(_) => {
            if let Some(parent) = full_file_out.parent().to_owned() {
                return Ok(parent.as_os_str().into());
            };
            return Ok(full_file_out.as_os_str().into());
        }
        Err(e) => return Err(Box::new(e)),
    }
}

fn main() -> iced::Result {
    let position = if cfg!(debug_assertions) {
        window::Position::Specific((800.00, 320.00).into())
    } else {
        window::Position::Default
    };

    iced::application(Application::default, Application::update, Application::view)
        .window(window::Settings {
            size: (512.00, 300.00).into(),
            position: position,
            ..Default::default()
        })
        .theme(iced::Theme::CatppuccinMocha)
        .title("CSV to Calendar Converter")
        .run()
}
