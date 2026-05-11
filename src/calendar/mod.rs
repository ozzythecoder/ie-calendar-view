pub mod event;

use ics::{ICalendar, components::Property};
use std::{collections::HashMap, error::Error, path::PathBuf};

use crate::calendar::event::IntellieventRecord;

pub struct BuildCalendarOptions {
    pub csv_file_path: PathBuf,
    pub filename_out: PathBuf,
}

pub fn build_calendar<'a>(
    opts: &'a BuildCalendarOptions,
) -> Result<(ICalendar<'a>, i32), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(opts.csv_file_path.clone())?;

    let mut error_count = 0;
    let Some(filename_out) = opts.filename_out.to_str() else {
        return Err(Box::<dyn Error>::from("Malformed filename"));
    };

    let mut calendar = ICalendar::new("2.0", filename_out);

    // enable updating/editing old events
    calendar.push(Property::new("METHOD", "PUBLISH"));
    calendar.push(Property::new("ORGANIZER", "AVEX"));

    for row in reader.deserialize::<HashMap<String, String>>() {
        let Ok(record) = IntellieventRecord::from_map(&row?) else {
            println!("Skipping row due to mapping error");
            error_count += 1;
            continue;
        };

        let event = match event::parse_event(&record) {
            Ok(v) => v,
            Err(e) => {
                println!("Skipping row due to parsing error: {}", e);
                error_count += 1;
                continue;
            }
        };
        calendar.add_event(event);
    }

    println!("Calendar built.");
    Ok((calendar, error_count))
}
