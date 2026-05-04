use std::error::Error;
use std::io::{self, Write};

use chrono_tz::Tz;
use google_calendar3::api::Event;
use serde::Serialize;

/// Output format for read commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Table,
    Json,
    Tsv,
    Csv,
    Raw,
}

impl OutputFormat {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "table" => Ok(Self::Table),
            "json" => Ok(Self::Json),
            "tsv" => Ok(Self::Tsv),
            "csv" => Ok(Self::Csv),
            "raw" => Ok(Self::Raw),
            other => Err(format!(
                "unknown --format '{}'. Expected: table, json, tsv, csv, raw",
                other
            )),
        }
    }
}

/// v1 stable schema for `gcal list`. Bump on field rename/removal.
#[derive(Debug, Serialize)]
pub struct ListEvent {
    pub id: Option<String>,
    pub calendar_id: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    /// RFC3339 string in user TZ.
    pub start: Option<String>,
    pub end: Option<String>,
    pub all_day: bool,
    pub status: Option<String>,
    pub creator: Option<String>,
    pub attendees_count: usize,
    pub html_link: Option<String>,
    pub updated: Option<String>,
}

impl ListEvent {
    pub fn from_event(ev: &Event, calendar_id: &str, tz: Tz) -> Self {
        let (start_str, all_day_start) = format_event_dt(&ev.start, tz);
        let (end_str, all_day_end) = format_event_dt(&ev.end, tz);
        let attendees_count = ev.attendees.as_ref().map(|v| v.len()).unwrap_or(0);
        Self {
            id: ev.id.clone(),
            calendar_id: calendar_id.to_string(),
            summary: ev.summary.clone(),
            description: ev.description.clone(),
            start: start_str,
            end: end_str,
            all_day: all_day_start || all_day_end,
            status: ev.status.clone(),
            creator: ev.creator.as_ref().and_then(|c| c.email.clone()),
            attendees_count,
            html_link: ev.html_link.clone(),
            updated: ev.updated.map(|t| t.to_rfc3339()),
        }
    }
}

fn format_event_dt(
    edt: &Option<google_calendar3::api::EventDateTime>,
    tz: Tz,
) -> (Option<String>, bool) {
    match edt {
        None => (None, false),
        Some(e) => {
            if let Some(dt) = e.date_time {
                use chrono::TimeZone;
                let local = tz.from_utc_datetime(&dt.naive_utc());
                (Some(local.to_rfc3339()), false)
            } else if let Some(d) = e.date {
                (Some(d.format("%Y-%m-%d").to_string()), true)
            } else {
                (None, false)
            }
        }
    }
}

/// TSV / CSV column header.
const COLUMNS: &[&str] = &[
    "id",
    "calendar_id",
    "summary",
    "start",
    "end",
    "all_day",
    "status",
    "creator",
    "attendees_count",
    "html_link",
];

fn cell(s: Option<&str>) -> String {
    s.unwrap_or("").to_string()
}

fn tsv_escape(s: &str) -> String {
    s.replace('\\', r"\\")
        .replace('\t', r"\t")
        .replace('\n', r"\n")
        .replace('\r', r"\r")
}

pub fn render_list(
    fmt: OutputFormat,
    events: &[ListEvent],
    raw_events: &[Event],
) -> Result<(), Box<dyn Error>> {
    match fmt {
        OutputFormat::Json => render_json(events),
        OutputFormat::Tsv => render_tsv(events),
        OutputFormat::Csv => render_csv(events),
        OutputFormat::Raw => render_raw(raw_events),
        OutputFormat::Table => Err("Table format handled by list arm directly".into()),
    }
}

fn render_json(events: &[ListEvent]) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    if io::IsTerminal::is_terminal(&out) {
        serde_json::to_writer_pretty(&mut out, events)?;
    } else {
        serde_json::to_writer(&mut out, events)?;
    }
    writeln!(out)?;
    Ok(())
}

fn render_raw(events: &[Event]) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    serde_json::to_writer_pretty(&mut out, events)?;
    writeln!(out)?;
    Ok(())
}

fn render_tsv(events: &[ListEvent]) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    writeln!(out, "{}", COLUMNS.join("\t"))?;
    for e in events {
        let row = [
            cell(e.id.as_deref()),
            e.calendar_id.clone(),
            cell(e.summary.as_deref()),
            cell(e.start.as_deref()),
            cell(e.end.as_deref()),
            e.all_day.to_string(),
            cell(e.status.as_deref()),
            cell(e.creator.as_deref()),
            e.attendees_count.to_string(),
            cell(e.html_link.as_deref()),
        ];
        let escaped: Vec<String> = row.iter().map(|s| tsv_escape(s)).collect();
        writeln!(out, "{}", escaped.join("\t"))?;
    }
    Ok(())
}

fn render_csv(events: &[ListEvent]) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(stdout.lock());
    wtr.write_record(COLUMNS)?;
    for e in events {
        wtr.write_record([
            cell(e.id.as_deref()).as_str(),
            e.calendar_id.as_str(),
            cell(e.summary.as_deref()).as_str(),
            cell(e.start.as_deref()).as_str(),
            cell(e.end.as_deref()).as_str(),
            e.all_day.to_string().as_str(),
            cell(e.status.as_deref()).as_str(),
            cell(e.creator.as_deref()).as_str(),
            e.attendees_count.to_string().as_str(),
            cell(e.html_link.as_deref()).as_str(),
        ])?;
    }
    wtr.flush()?;
    Ok(())
}
