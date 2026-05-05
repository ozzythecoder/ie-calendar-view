use std::{collections::HashMap, error::Error};

use chrono::prelude::*;
use ics::{Event, components::Property};

use crate::csv::{optional_field, require_field};

#[derive(Debug, Clone)]
pub struct IntellieventRecord {
    job_number: String,
    job_name: String,
    event_start: String,
    deprep: String,
    create_date: Option<String>,
}

impl IntellieventRecord {
    pub fn from_map(map: &HashMap<String, String>) -> Result<Self, Box<dyn Error>> {
        let map: HashMap<String, String> = map
            .iter()
            .map(|(k, v)| (k.clone(), v.replace('\u{200b}', ""))) // clean up zero-width bits
            .collect();

        Ok(Self {
            job_number: require_field(&map, &["Job #"])?,
            job_name: require_field(&map, &["Job Name", "Name"])?,
            event_start: require_field(&map, &["Event Start"])?,
            deprep: require_field(&map, &["De-Prep"])?,
            create_date: optional_field(&map, &["Created", "Create Date"]),
        })
    }
}

const CSV_DATE_FORMAT: &str = "%m/%d/%Y, %r"; // 03/31/2026 1:00 PM
const ICS_DATE_FORMAT: &str = "%Y%m%dT%H%M%S"; // 20260331T130000

pub fn parse_event<'a>(record: &IntellieventRecord) -> Result<Event<'a>, Box<dyn Error>> {

    // unique id
    let uid = (record.job_name.to_owned() + "---" + &record.job_number)
        .replace(&['_', '*', ' ', '.'][..], "-");

    let fallback = Utc::now().format(ICS_DATE_FORMAT).to_string();
    // dtstamp = "created at"
    let dtstamp =
        csv_to_ics_date(record.create_date.as_deref().unwrap_or(&fallback)).unwrap_or(fallback);

    let mut event = Event::new(uid, dtstamp);

    // Event Start as start date
    let dtstart = Property::new("DTSTART", csv_to_ics_date(&record.event_start)?);

    // De-Prep as end date
    let dtend = Property::new("DTEND", csv_to_ics_date(&record.deprep)?);

    event.push(dtstart);
    event.push(dtend);

    event.push(Property::new("SUMMARY", record.job_name.to_owned()));

    // SEQUENCE - allows for overwriting events in Outlook
    event.push(Property::new(
        "SEQUENCE",
        Utc::now().timestamp().to_string(),
    ));
    // LAST-MODIFIED - allows for overwriting events in Apple Calendar
    event.push(Property::new("LAST-MODIFIED", utc_to_ics_date(Utc::now())));

    // Link to IntelliEvent jobs
    // We need the literal '\n' to show up in the string for the line break to be respected
    let description = format!(
        "Job: https://avex.ielightning.net/job/job?id={0}\\n\
        Products: https://avex.ielightning.net/job/jobProducts?id={0}\\n\
        Pullsheet: https://avex.ielightning.net/management/print/preview?printDetailId=32&refId={0}&at=Job",
        &record.job_number
    );
    event.push(Property::new("DESCRIPTION", description));

    Ok(event)
}

fn csv_to_ics_date(csv_date: &str) -> Result<String, chrono::ParseError> {
    match NaiveDateTime::parse_from_str(csv_date, CSV_DATE_FORMAT) {
        Ok(dt) => Ok(dt.format(ICS_DATE_FORMAT).to_string()),
        Err(e) => {
            println!(
                "ERROR: Could not parse valid ICS date from value {}",
                csv_date
            );
            Err(e)
        }
    }
}

fn utc_to_ics_date(utc_date: DateTime<Utc>) -> String {
    utc_date
        .to_rfc3339_opts(SecondsFormat::Secs, true)
        .replace(&['.', '-', ':', '*', ' ', '_', 'Z'][..], "") // 2026-03-31T13:00:00:00 -> 20260331T130000
}
