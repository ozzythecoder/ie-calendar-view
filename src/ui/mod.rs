pub mod confirmation_dialog {
    use iced::widget::{button, column, row, text};

    pub struct Confirmation {
        prompt: String,
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

    impl Confirmation {
        pub(crate) fn new(prompt: &str) -> Self {
            Self {
                prompt: String::from(prompt),
            }
        }

        pub fn view(&self) -> iced::Element<'_, Message> {
            column![
                text(&self.prompt),
                row![
                    button("Cancel").on_press(Message::Cancel),
                    button("Confirm").on_press(Message::Confirm),
                ]
            ]
            .into()
        }

        pub fn update(&mut self, message: Message) -> Action {
            match message {
                Message::Confirm => Action::ConfirmOperation,
                Message::Cancel => Action::CancelOperation,
            }
        }
    }
}
