use crate::{
    converter::{self, ConversionDetails, duplicate_exists},
    dialog,
    screen::{Screen, convert_form, overwrite},
};
use iced::{Element, Task};

#[derive(Default)]
pub struct Application {
    screen: Screen,
    conversion_details: Option<ConversionDetails>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ConvertForm(convert_form::Message),
    ConfirmOverwrite(overwrite::Message),
}

impl Application {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.screen {
            Screen::ConvertForm(s) => s.view().map(Message::ConvertForm),
            Screen::ConfirmOverwrite(s) => s.view().map(Message::ConfirmOverwrite),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ConvertForm(msg) => {
                if let Screen::ConvertForm(form) = &mut self.screen {
                    let (action, task) = form.update(msg);

                    match action {
                        convert_form::Action::None => {}
                        convert_form::Action::Convert {
                            csv_path,
                            export_path,
                            filename_out,
                        } => {
                            let opts = ConversionDetails {
                                csv_path: csv_path,
                                export_path: export_path,
                                filename_out: filename_out,
                            };
                            self.set_conversion_options(&opts);
                            let (exists, output_path) = duplicate_exists(&opts);
                            if exists {
                                let new_screen =
                                    overwrite::ConfirmOverwrite::new(output_path.to_str().unwrap());
                                self.screen = Screen::ConfirmOverwrite(new_screen);
                            } else {
                                self.perform_convert();
                            }
                        }
                    };
                    
                    task.map(Message::ConvertForm)
                } else {
                    Task::none()
                }
            }
            Message::ConfirmOverwrite(action) => match action {
                overwrite::Message::Cancel => {
                    let Some(opts) = &self.conversion_details else {
                        return Task::none();
                    };
                    self.screen = Screen::ConvertForm(convert_form::ConvertForm::new(opts.clone()));
                    Task::none()
                }
                overwrite::Message::Confirm => {
                    self.perform_convert();
                    Task::none()
                }
            },
        }
    }

    fn set_conversion_options(&mut self, opts: &ConversionDetails) {
        self.conversion_details = Some(opts.to_owned());
    }

    fn perform_convert(&mut self) {
        let Some(opts) = &self.conversion_details else {
            println!("ERROR: attempted to convert without initializing options.");
            return;
        };
        match converter::convert_to_ics(opts.clone()) {
            Ok(file) => dialog::open_file(file),
            Err(e) => {
                println!("ERROR: Failure converting ICS file: {}", e);
            }
        }
        self.screen = Screen::ConvertForm(convert_form::ConvertForm::new(opts.clone()));
    }
}
