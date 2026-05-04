use std::error::Error;

use chrono::{DateTime, Datelike, Timelike, TimeZone, Utc};
use chrono_tz::Tz;
use google_calendar3::api::Event;
use google_calendar3::{
    hyper_rustls::HttpsConnector, hyper_util::client::legacy::connect::HttpConnector, CalendarHub,
};

use crate::util::format::{self, ListEvent, OutputFormat};

pub struct EventsListArgs<'a> {
    pub time_min: DateTime<Utc>,
    pub time_max: DateTime<Utc>,
    pub calendar_id: &'a str,
    pub query: Option<&'a str>,
    pub format: OutputFormat,
    pub tz: Tz,
}

/// Shared between `agenda` and `search`. Renders flat list (table = simple
/// chronological lines) or delegates to `format::render_list` for the
/// machine-readable formats.
pub async fn list(
    hub: &CalendarHub<HttpsConnector<HttpConnector>>,
    args: EventsListArgs<'_>,
) -> Result<(), Box<dyn Error>> {
    let mut req = hub
        .events()
        .list(args.calendar_id)
        .time_min(args.time_min)
        .time_max(args.time_max)
        .single_events(true);
    if let Some(q) = args.query {
        req = req.q(q);
    }
    let (_, evs) = req.doit().await?;
    let raw_events: Vec<Event> = evs.items.unwrap_or_default();

    if args.format == OutputFormat::Table {
        render_flat_table(&raw_events, args.tz);
        return Ok(());
    }

    let list_events: Vec<ListEvent> = raw_events
        .iter()
        .map(|e| ListEvent::from_event(e, args.calendar_id, args.tz))
        .collect();
    format::render_list(args.format, &list_events, &raw_events)?;
    Ok(())
}

fn render_flat_table(events: &[Event], tz: Tz) {
    if events.is_empty() {
        println!("(no events in range)");
        return;
    }
    let mut sorted: Vec<&Event> = events
        .iter()
        .filter(|e| e.start.as_ref().and_then(|s| s.date_time).is_some())
        .collect();
    sorted.sort_by_key(|e| e.start.as_ref().unwrap().date_time.unwrap());
    for ev in &sorted {
        let s = ev.start.as_ref().unwrap().date_time.unwrap();
        let e = ev.end.as_ref().and_then(|x| x.date_time).unwrap_or(s);
        let s_local = tz.from_utc_datetime(&s.naive_utc());
        let e_local = tz.from_utc_datetime(&e.naive_utc());
        let summary = ev.summary.as_deref().unwrap_or("(no title)");
        println!(
            "{:04}-{:02}-{:02}  {:02}:{:02}-{:02}:{:02}  {}",
            s_local.year(),
            s_local.month(),
            s_local.day(),
            s_local.hour(),
            s_local.minute(),
            e_local.hour(),
            e_local.minute(),
            summary,
        );
    }
}
