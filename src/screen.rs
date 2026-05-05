pub mod convert_form;
pub mod overwrite;

pub enum Screen {
    ConvertForm(convert_form::ConvertForm),
    ConfirmOverwrite(overwrite::ConfirmOverwrite),
}

impl Default for Screen {
    fn default() -> Self {
        Screen::ConvertForm(convert_form::ConvertForm::default())
    }
}
