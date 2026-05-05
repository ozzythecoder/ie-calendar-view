use iced::{Color, Font, Length, alignment::Horizontal, never, widget::{button, column, rich_text, row, span}};

pub struct ConfirmOverwrite {
    file_path: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Confirm,
    Cancel,
}

pub enum Action {
    ConfirmOperation,
    CancelOperation,
}

impl ConfirmOverwrite {
    pub(crate) fn new(file_path: &str) -> Self {
        Self {
            file_path: String::from(file_path),
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        column![
            rich_text![
                span("This will overwrite an existing file at "),
                span(format!(
                    "{}.ics",
                    &self.file_path
                ))
                .font(Font {
                    style: iced::font::Style::Italic,
                    ..Font::DEFAULT
                })
                .color(Color::from_rgb(0.5, 0.5, 1.0)),
                span(". Proceed?")
            ]
            .on_link_click(never),
            row![
                button("Cancel").on_press(Message::Cancel),
                button("Confirm")
                    .on_press(Message::Confirm)
                    .style(button::danger),
            ]
            .padding(8)
            .spacing(14)
        ]
        .padding(32)
        .align_x(Horizontal::Center)
        .width(Length::Fixed(512.00))
        .into()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Confirm => Action::ConfirmOperation,
            Message::Cancel => Action::CancelOperation,
        }
    }
}
