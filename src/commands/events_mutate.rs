use std::error::Error;
use std::io::{self, Write};

use chrono_tz::Tz;
use google_calendar3::api::{Event, EventDateTime};
use google_calendar3::{
    hyper_rustls::HttpsConnector, hyper_util::client::legacy::connect::HttpConnector, CalendarHub,
};

use crate::util::date::parse_range_input;

pub struct EditArgs<'a> {
    pub event_id: &'a str,
    pub calendar_id: &'a str,
    pub fields: Vec<(String, String)>,
    pub tz: Tz,
}

pub struct DeleteArgs<'a> {
    pub event_id: &'a str,
    pub calendar_id: &'a str,
    pub yes: bool,
}

pub async fn edit(
    hub: &CalendarHub<HttpsConnector<HttpConnector>>,
    args: EditArgs<'_>,
) -> Result<(), Box<dyn Error>> {
    if args.fields.is_empty() {
        return Err(
            "no --field <key=value> provided. Supported keys: summary, description, location, start, end."
                .into(),
        );
    }

    let (_, mut event) = hub
        .events()
        .get(args.calendar_id, args.event_id)
        .doit()
        .await?;

    for (key, value) in &args.fields {
        match key.as_str() {
            "summary" => event.summary = Some(value.clone()),
            "description" => event.description = Some(value.clone()),
            "location" => event.location = Some(value.clone()),
            "start" => {
                let dt = parse_range_input(args.tz, value)
                    .map_err(|e| format!("--field start: {}", e))?;
                event.start = Some(EventDateTime {
                    date_time: Some(dt),
                    ..Default::default()
                });
            }
            "end" => {
                let dt =
                    parse_range_input(args.tz, value).map_err(|e| format!("--field end: {}", e))?;
                event.end = Some(EventDateTime {
                    date_time: Some(dt),
                    ..Default::default()
                });
            }
            other => {
                return Err(format!(
                    "unsupported --field key '{}'. Supported: summary, description, location, start, end.",
                    other
                )
                .into());
            }
        }
    }

    let (_, updated) = hub
        .events()
        .update(event, args.calendar_id, args.event_id)
        .doit()
        .await?;
    println!(
        "gcal: updated event {} ({})",
        args.event_id,
        updated.html_link.unwrap_or_default()
    );
    Ok(())
}

pub async fn delete(
    hub: &CalendarHub<HttpsConnector<HttpConnector>>,
    args: DeleteArgs<'_>,
) -> Result<(), Box<dyn Error>> {
    let (_, event) = hub
        .events()
        .get(args.calendar_id, args.event_id)
        .doit()
        .await?;
    let summary = event.summary.as_deref().unwrap_or("(no title)");
    let start = event
        .start
        .as_ref()
        .and_then(|s| s.date_time)
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_else(|| "(no start)".to_string());
    println!("About to delete event:");
    println!("  id:      {}", args.event_id);
    println!("  summary: {}", summary);
    println!("  start:   {}", start);

    if !args.yes {
        print!("Confirm delete? type 'yes' to proceed: ");
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        if buf.trim() != "yes" {
            println!("aborted.");
            return Ok(());
        }
    }

    let _ = Event::default(); // silence unused-import warnings if Event drops out
    hub.events()
        .delete(args.calendar_id, args.event_id)
        .doit()
        .await?;
    println!("gcal: deleted event {}", args.event_id);
    Ok(())
}
