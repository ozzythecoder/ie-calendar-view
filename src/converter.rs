use crate::calendar::{BuildCalendarOptions, build_calendar};
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone)]
pub struct ConversionDetails {
    pub csv_path: PathBuf,
    pub export_path: PathBuf,
    pub filename_out: String,
}

pub fn duplicate_exists(o: &ConversionDetails) -> (bool, PathBuf) {
    let output = build_output_path(o);
    if let Ok(exists) = output.try_exists() {
        (exists, output)
    } else {
        (false, output)
    }
}

pub fn build_output_path(o: &ConversionDetails) -> PathBuf {
    Path::new(&o.export_path)
        .to_path_buf()
        .join(&o.filename_out)
        .with_added_extension("ics")
}

pub fn convert_to_ics(o: ConversionDetails) -> Result<PathBuf, Box<dyn Error>> {
    let output_path = build_output_path(&o);
    let opts = BuildCalendarOptions {
        csv_file_path: o.csv_path,
        filename_out: PathBuf::from(&o.filename_out),
    };

    let (cal, error_count) = build_calendar(&opts)?;

    if error_count > 0 {
        println!("Skipped {} row(s) due to parsing errors.", error_count)
    };

    println!("Writing to {}", output_path.display());

    if let Ok(e) = output_path.try_exists()
        && e
    {
        println!("Overwriting file at {}", output_path.display())
    }

    cal.save_file(&output_path)?;
    if let Some(parent) = output_path.parent() {
        Ok(parent.into())
    } else {
        Ok(output_path.into())
    }
}
