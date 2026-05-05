use std::env;

pub fn get_open_command() -> String {
    match env::consts::OS {
        "macos" => "open".to_owned(),
        "windows" => "explorer".to_owned(),
        _ => "xdg-open".to_owned(),
    }
}
