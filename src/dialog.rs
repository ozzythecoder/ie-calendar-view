use std::{path::PathBuf, process::Command};

use iced::Task;
use rfd::AsyncFileDialog;

pub fn pick_directory<M>(
    directory: PathBuf,
    on_select: impl Fn(Option<PathBuf>) -> M + Send + 'static,
) -> Task<M>
where
    M: Send + 'static,
{
    Task::perform(
        async {
            AsyncFileDialog::new()
                .set_directory(directory)
                .pick_folder()
                .await
                .map(|f| f.path().to_path_buf())
        },
        on_select,
    )
}

pub fn pick_csv_file<M>(
    directory: PathBuf,
    on_select: impl Fn(Option<PathBuf>) -> M + Send + 'static,
) -> Task<M>
where
    M: Send + 'static,
{
    Task::perform(
        async {
            AsyncFileDialog::new()
                .add_filter("csv", &["csv"])
                .set_directory(directory)
                .pick_file()
                .await
                .map(|f| f.path().to_path_buf())
        },
        on_select,
    )
}

pub fn open_file(file: PathBuf) {
    Command::new(crate::platform::get_open_command())
        .arg(file)
        .spawn()
        .unwrap();
}