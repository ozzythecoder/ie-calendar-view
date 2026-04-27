use std::error::Error;

use chrono::{ParseError, prelude::*};
use ics::{Event, components::Property};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntellieventRecord {
    #[serde(rename = "Job #")]
    job_number: String,
    #[serde(rename = "Job Name")]
    job_name: String,
    #[serde(rename = "Prep")]
    prep: String,
    #[serde(rename = "Event Start")]
    event_start: String,
    #[serde(rename = "EventEnd/Strike")]
    event_end: String,
    #[serde(rename = "De-Prep")]
    deprep: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "Create Date")]
    create_date: String,
    #[serde(rename = "PO #")]
    po_number: String,
    #[serde(rename = "Project #")]
    project_number: String,
}

impl IntellieventRecord {
    pub fn as_array_mut(&mut self) -> [&mut String; 10] {
        [
            &mut self.job_number,
            &mut self.job_name,
            &mut self.prep,
            &mut self.event_start,
            &mut self.event_end,
            &mut self.deprep,
            &mut self.status,
            &mut self.create_date,
            &mut self.po_number,
            &mut self.project_number,
        ]
    }
}

const CSV_DATE_FORMAT: &str = "%m/%d/%Y, %r"; // 03/31/2026 1:00 PM
const ICS_DATE_FORMAT: &str = "%Y%m%dT%H%M%S"; // 20260331T130000

pub fn parse_event<'a>(record_in: IntellieventRecord) -> Result<Event<'a>, Box<dyn Error>> {
    let record = cleanup_zero_width_bits(record_in.clone());

    // unique id
    let uid = (record.job_name.to_owned() + "---" + &record.job_number)
        .replace(&['_', '*', ' ', '.'][..], "-");
    // dtstamp = "created at"
    let dtstamp = csv_to_ics_date(&record.create_date)
        .unwrap_or(Utc::now().format(ICS_DATE_FORMAT).to_string());

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

    // Link to IntelliEvent job
    let description = format!(
        r"Job: https://avex.ielightning.net/job/job?id={}\nProducts: https://avex.ielightning.net/job/jobProducts?id={}\nPullsheet: https://avex.ielightning.net/management/print/preview?printDetailId=32&refId={}&at=Job",
        &record.job_number,
        &record.job_number,
        &record.job_number
    );
    event.push(Property::new("DESCRIPTION", description));

    Ok(event)
}

fn csv_to_ics_date(csv_date: &str) -> Result<String, ParseError> {
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

fn cleanup_zero_width_bits(mut record: IntellieventRecord) -> IntellieventRecord {
    for value in record.as_array_mut() {
        if value == "\u{200b}" {
            *value = String::from("");
        }
    }
    record
}
