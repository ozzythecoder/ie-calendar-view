use ics::{ICalendar, components::Property};
use std::{collections::HashMap, error::Error};

use crate::calendar::event::IntellieventRecord;

pub mod event;

pub struct BuildCalendarOptions<'a> {
    pub csv_file_path: &'a str,
    pub filename_out: &'a str,
}

impl Default for BuildCalendarOptions<'_> {
    fn default() -> Self {
        BuildCalendarOptions {
            csv_file_path: "./Jobs.csv",
            filename_out: "Jobs.ics",
        }
    }
}

pub fn build_calendar(opts: BuildCalendarOptions) -> Result<(ICalendar, i32), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(opts.csv_file_path)?;

    let mut error_count = 0;

    let mut calendar = ICalendar::new("2.0", opts.filename_out);

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

    println!("Parsing complete.");
    Ok((calendar, error_count))
}
