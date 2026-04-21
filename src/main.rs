use iced::{
    Length, Task,
    alignment::{Horizontal, Vertical},
    color,
    widget::{Column, Space, button, column, row, text, text_input},
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

pub mod calendar;

#[derive(Default)]
struct Application {
    csv_path: PathBuf,
    export_path: PathBuf,
    filename_out: String,
}

#[derive(Debug, Clone)]
enum Message {
    OpenFileDialog,
    OpenExportDialog,
    FilenameOutChanged(String),
    FileSelected(Option<PathBuf>),
    ExportPathSelected(Option<PathBuf>),
    Convert,
}

impl Application {
    pub fn view(&self) -> Column<'_, Message> {
        column![
            text("Export a CSV from IntelliEvent, then use this tool to convert it into a calendar file. You can then import this file into a calendar app of your choice.")
                .size(14)
                .color(color!(180, 180, 180))
                ,
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
            button("Convert!").on_press_maybe(self.convert_message()),
        ]
        .spacing(14)
        .padding(32)
        .align_x(Horizontal::Center)
        .width(Length::Fixed(512.00))
        .into()
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
                let Some(csv_path) = self.csv_path.to_str() else {
                    // TODO handle error
                    println!("No csv_path found");
                    return Task::none();
                };

                let Some(export_path) = self.export_path.to_str() else {
                    // TODO handle error
                    println!("No export_path found");
                    return Task::none();
                };

                let opts = ConvertToIcsOptions {
                    csv_path: csv_path.to_owned(),
                    export_path: export_path.to_owned(),
                    filename_out: &self.filename_out,
                };

                match convert_to_ics(opts) {
                    Ok(file) => {
                        Command::new(get_open_command()).arg(file).spawn().unwrap();
                        Task::none()
                    }
                    Err(_) => Task::none(),
                }
            }
        }
    }

    fn convert_message(&self) -> Option<Message> {
        if self.csv_path.as_os_str().is_empty() {
            None
        } else if self.export_path.as_os_str().is_empty() {
            None
        } else if self.filename_out.is_empty() {
            None
        } else {
            Some(Message::Convert)
        }
    }
}

fn get_open_command() -> String {
    match env::consts::OS {
        "macos" => "open".to_owned(),
        "windows" => "explorer".to_owned(),
        _ => "xdg-open".to_owned(),
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
    iced::application(Application::default, Application::update, Application::view)
        .window(window::Settings {
            size: (512.00, 300.00).into(),
            position: window::Position::Specific((800.00, 320.00).into()),
            ..Default::default()
        })
        .theme(iced::Theme::CatppuccinMocha)
        .title("CSV to Calendar Converter")
        .run()
}
