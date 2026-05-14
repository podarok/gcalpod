use std::collections::HashSet;
use std::error::Error;
use std::io::BufReader;

use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use google_calendar3::api::{Event, EventDateTime};
use google_calendar3::{
    hyper_rustls::HttpsConnector, hyper_util::client::legacy::connect::HttpConnector, CalendarHub,
};

pub struct ImportArgs<'a> {
    pub path: &'a str,
    pub calendar_id: &'a str,
    pub dry_run: bool,
    pub skip_duplicates: bool,
    pub tz: Tz,
}

pub async fn run(
    hub: &CalendarHub<HttpsConnector<HttpConnector>>,
    args: ImportArgs<'_>,
) -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open(args.path)?;
    let reader = BufReader::new(file);
    let parser = ical::IcalParser::new(reader);

    let mut parsed_events: Vec<(Option<String>, Event)> = vec![];
    for cal in parser {
        let cal = cal.map_err(|e| format!("ICS parse failed: {:?}", e))?;
        for vev in cal.events {
            let mut summary = None;
            let mut description = None;
            let mut location = None;
            let mut uid = None;
            let mut transparency = None;
            let mut dtstart: Option<EventDateTime> = None;
            let mut dtend: Option<EventDateTime> = None;
            for prop in vev.properties {
                let val = prop.value.unwrap_or_default();
                match prop.name.as_str() {
                    "SUMMARY" => summary = Some(val),
                    "DESCRIPTION" => description = Some(val),
                    "LOCATION" => location = Some(val),
                    "UID" => uid = Some(val),
                    "TRANSP" => {
                        // RFC 5545 TRANSP: OPAQUE (default, busy) | TRANSPARENT (free).
                        // Google Calendar API expects lowercase.
                        transparency = match val.trim().to_ascii_uppercase().as_str() {
                            "TRANSPARENT" => Some("transparent".to_string()),
                            "OPAQUE" => Some("opaque".to_string()),
                            _ => None,
                        };
                    }
                    "DTSTART" => dtstart = Some(parse_dt_property(&val, args.tz)?),
                    "DTEND" => dtend = Some(parse_dt_property(&val, args.tz)?),
                    _ => {}
                }
            }
            let event = Event {
                summary,
                description,
                location,
                transparency,
                start: dtstart,
                end: dtend,
                ..Default::default()
            };
            parsed_events.push((uid, event));
        }
    }

    println!(
        "gcal: parsed {} event(s) from {}",
        parsed_events.len(),
        args.path
    );
    if parsed_events.is_empty() {
        return Ok(());
    }

    if args.dry_run {
        for (uid, ev) in &parsed_events {
            println!(
                "  - {} | {} | uid={}",
                ev.summary.as_deref().unwrap_or("(no title)"),
                ev.start
                    .as_ref()
                    .and_then(|s| s.date_time)
                    .map(|d| d.to_rfc3339())
                    .or_else(|| ev
                        .start
                        .as_ref()
                        .and_then(|s| s.date.map(|d| d.format("%Y-%m-%d").to_string())))
                    .unwrap_or_else(|| "?".to_string()),
                uid.as_deref().unwrap_or("?"),
            );
        }
        println!("(dry-run; nothing inserted)");
        return Ok(());
    }

    let existing_uids: HashSet<String> = if args.skip_duplicates {
        let (_, listing) = hub
            .events()
            .list(args.calendar_id)
            .max_results(2500)
            .doit()
            .await?;
        listing
            .items
            .unwrap_or_default()
            .into_iter()
            .filter_map(|e| e.i_cal_uid)
            .collect()
    } else {
        HashSet::new()
    };

    let mut inserted = 0usize;
    let mut skipped = 0usize;
    for (uid, ev) in parsed_events {
        if args.skip_duplicates {
            if let Some(u) = &uid {
                if existing_uids.contains(u) {
                    skipped += 1;
                    continue;
                }
            }
        }
        let mut to_insert = ev;
        if let Some(u) = uid {
            to_insert.i_cal_uid = Some(u);
        }
        match hub
            .events()
            .insert(to_insert, args.calendar_id)
            .doit()
            .await
        {
            Ok((_, e)) => {
                inserted += 1;
                if let Some(link) = e.html_link {
                    println!("  inserted: {}", link);
                }
            }
            Err(e) => eprintln!("  insert failed: {}", e),
        }
    }
    println!(
        "gcal: inserted {} event(s); skipped {} duplicate(s).",
        inserted, skipped
    );
    Ok(())
}

fn parse_dt_property(value: &str, tz: Tz) -> Result<EventDateTime, Box<dyn Error>> {
    // ICS supports: 20260504T140000Z (UTC), 20260504T140000 (floating),
    // 20260504 (date-only / all-day).
    let v = value.trim();
    if v.len() == 8 {
        let d = NaiveDate::parse_from_str(v, "%Y%m%d")?;
        return Ok(EventDateTime {
            date: Some(d),
            ..Default::default()
        });
    }
    if let Some(stripped) = v.strip_suffix('Z') {
        let nd = NaiveDateTime::parse_from_str(stripped, "%Y%m%dT%H%M%S")?;
        let dt: DateTime<Utc> = Utc.from_utc_datetime(&nd);
        return Ok(EventDateTime {
            date_time: Some(dt),
            ..Default::default()
        });
    }
    let nd = NaiveDateTime::parse_from_str(v, "%Y%m%dT%H%M%S")
        .map_err(|e| format!("unrecognized DTSTART/DTEND '{}': {}", v, e))?;
    let local = tz
        .from_local_datetime(&nd)
        .single()
        .ok_or_else(|| format!("ambiguous local datetime '{}'", v))?;
    Ok(EventDateTime {
        date_time: Some(local.with_timezone(&Utc)),
        ..Default::default()
    })
}
