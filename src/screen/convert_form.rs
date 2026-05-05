use iced::{
    Length, Task,
    alignment::{Horizontal, Vertical},
    color,
    widget::{Space, button, column, row, text, text_input},
};
use std::path::{Path, PathBuf};

use crate::{converter::ConversionDetails, dialog};

#[derive(Default)]
pub struct ConvertForm {
    state: FormState,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenFileDialog,
    OpenExportDialog,
    FilenameOutChanged(String),
    FileSelected(Option<PathBuf>),
    ExportPathSelected(Option<PathBuf>),
    Convert,
}

#[derive(Debug, Clone)]
pub enum Action {
    None,
    Convert {
        csv_path: PathBuf,
        export_path: PathBuf,
        filename_out: String,
    },
}

#[derive(Default)]
struct FormState {
    pub csv_path: PathBuf,
    pub export_path: PathBuf,
    pub filename_out: String,
    pub error: Option<String>,
}

impl FormState {
    pub fn is_valid(&self) -> bool {
        self.csv_path.to_str().is_some()
            && self.export_path.to_str().is_some()
            && !self.filename_out.is_empty()
    }

    pub fn set_error(&mut self, e: String) {
        self.error = Some(e);
    }

    pub fn clear_error(&mut self) {
        self.error = None;
    }
}

impl ConvertForm {
    pub fn new(s: ConversionDetails) -> Self {
        Self {
            state: FormState {
                csv_path: s.csv_path,
                export_path: s.export_path,
                filename_out: s.filename_out,
                ..Default::default()
            },
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        column![
            text("Export a CSV from IntelliEvent, then use this tool to convert it into a calendar file. You can then import this file into a calendar app of your choice.")
                .size(14)
                .color(color!(180, 180, 180)),
            row![
                button("Choose CSV File").on_press(Message::OpenFileDialog),
                Space::new().width(Length::Fill),
                text(self.state.csv_path.display().to_string())
                    .wrapping(text::Wrapping::WordOrGlyph),
            ]
            .spacing(8)
            .width(Length::Fill)
            .align_y(Vertical::Center),
            row![
                button("Choose Destination")
                    .on_press(Message::OpenExportDialog),
                Space::new().width(Length::Fill),
                text(self.state.export_path.display().to_string()),
            ]
            .spacing(8)
            .width(Length::Fill)
            .align_y(Vertical::Center),
            row![
                text("Filename:").color(color!(210, 210, 210)),
                text_input("example", &self.state.filename_out).on_input(Message::FilenameOutChanged),
                text(".ics"),
            ]
            .spacing(8)
            .width(Length::Fixed(360.00))
            .align_y(Vertical::Center),
            {
                if self.state.is_valid() {
                    button("Convert!").on_press(Message::Convert)
                } else {
                    button("Convert!")
                }
            },
            text(if let Some(e) = &self.state.error { e.clone() } else { "".to_owned() }).style(text::danger)
        ]
        .spacing(14)
        .padding(32)
        .align_x(Horizontal::Center)
        .width(Length::Fixed(512.00))
        .into()
    }

    pub fn update(&mut self, message: Message) -> (Action, Task<Message>) {
        match message {
            Message::OpenExportDialog => {
                let directory = if !self.state.export_path.as_os_str().is_empty() {
                    &self.state.export_path
                } else if !self.state.csv_path.as_os_str().is_empty() {
                    match &self.state.csv_path.parent() {
                        Some(p) => p,
                        None => Path::new("/"),
                    }
                } else {
                    Path::new("/")
                }
                .to_owned();
                (
                    Action::None,
                    dialog::pick_directory(directory, Message::ExportPathSelected),
                )
            }
            Message::OpenFileDialog => {
                println!("Opening file dialog...");
                let directory = if !self.state.csv_path.as_os_str().is_empty() {
                    &self.state.csv_path
                } else {
                    Path::new("/")
                }
                .to_owned();
                (
                    Action::None,
                    dialog::pick_csv_file(directory, Message::FileSelected),
                )
            }
            Message::ExportPathSelected(p) => {
                if let Some(path) = p {
                    self.state.export_path = path;
                };
                (Action::None, Task::none())
            }
            Message::FileSelected(p) => {
                if let Some(path) = p {
                    self.state.csv_path = path;
                }
                (Action::None, Task::none())
            }
            Message::FilenameOutChanged(f) => {
                self.state.filename_out = f;
                (Action::None, Task::none())
            }
            Message::Convert => (
                Action::Convert {
                    csv_path: self.state.csv_path.to_owned(),
                    export_path: self.state.export_path.to_owned(),
                    filename_out: self.state.filename_out.to_owned(),
                },
                Task::none(),
            ),
        }
    }
}
