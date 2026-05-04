use std::error::Error;
use std::io::{self, Write};

use comfy_table::{Attribute, Cell, ContentArrangement, Table};
use google_calendar3::{
    hyper_rustls::HttpsConnector, hyper_util::client::legacy::connect::HttpConnector, CalendarHub,
};
use serde::Serialize;

use crate::util::format::OutputFormat;

pub struct ListArgs {
    pub format: OutputFormat,
}

#[derive(Debug, Serialize)]
pub struct CalendarSummary {
    pub id: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub access_role: Option<String>,
    pub primary: bool,
    pub timezone: Option<String>,
    pub selected: bool,
}

const COLUMNS: &[&str] = &["id", "summary", "access_role", "primary", "timezone"];

pub async fn run(
    hub: &CalendarHub<HttpsConnector<HttpConnector>>,
    args: ListArgs,
) -> Result<(), Box<dyn Error>> {
    let (_, list) = hub.calendar_list().list().doit().await?;
    let items = list.items.unwrap_or_default();

    let summaries: Vec<CalendarSummary> = items
        .iter()
        .map(|c| CalendarSummary {
            id: c.id.clone(),
            summary: c.summary.clone(),
            description: c.description.clone(),
            access_role: c.access_role.clone(),
            primary: c.primary.unwrap_or(false),
            timezone: c.time_zone.clone(),
            selected: c.selected.unwrap_or(false),
        })
        .collect();

    match args.format {
        OutputFormat::Json => render_json(&summaries),
        OutputFormat::Tsv => render_tsv(&summaries),
        OutputFormat::Csv => render_csv(&summaries),
        OutputFormat::Raw => render_raw(&items),
        OutputFormat::Table => render_table(&summaries),
        OutputFormat::Conky => Err(
            "calendars list does not support --format conky; use list/agenda/search instead.".into(),
        ),
    }
}

fn cell(s: Option<&str>) -> String {
    s.unwrap_or("").to_string()
}

fn tsv_escape(s: &str) -> String {
    s.replace('\\', r"\\")
        .replace('\t', r"\t")
        .replace('\n', r"\n")
        .replace('\r', r"\r")
}

fn render_json(items: &[CalendarSummary]) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    if io::IsTerminal::is_terminal(&out) {
        serde_json::to_writer_pretty(&mut out, items)?;
    } else {
        serde_json::to_writer(&mut out, items)?;
    }
    writeln!(out)?;
    Ok(())
}

fn render_raw(items: &[google_calendar3::api::CalendarListEntry]) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    serde_json::to_writer_pretty(&mut out, items)?;
    writeln!(out)?;
    Ok(())
}

fn render_tsv(items: &[CalendarSummary]) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    writeln!(out, "{}", COLUMNS.join("\t"))?;
    for c in items {
        let row = [
            cell(c.id.as_deref()),
            cell(c.summary.as_deref()),
            cell(c.access_role.as_deref()),
            c.primary.to_string(),
            cell(c.timezone.as_deref()),
        ];
        let escaped: Vec<String> = row.iter().map(|s| tsv_escape(s)).collect();
        writeln!(out, "{}", escaped.join("\t"))?;
    }
    Ok(())
}

fn render_csv(items: &[CalendarSummary]) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(stdout.lock());
    wtr.write_record(COLUMNS)?;
    for c in items {
        wtr.write_record([
            cell(c.id.as_deref()).as_str(),
            cell(c.summary.as_deref()).as_str(),
            cell(c.access_role.as_deref()).as_str(),
            c.primary.to_string().as_str(),
            cell(c.timezone.as_deref()).as_str(),
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

fn render_table(items: &[CalendarSummary]) -> Result<(), Box<dyn Error>> {
    let mut table = Table::new();
    table
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_header(vec![
            Cell::new("ID").add_attribute(Attribute::Bold),
            Cell::new("Summary").add_attribute(Attribute::Bold),
            Cell::new("Access").add_attribute(Attribute::Bold),
            Cell::new("Primary").add_attribute(Attribute::Bold),
            Cell::new("Timezone").add_attribute(Attribute::Bold),
        ]);

    if items.is_empty() {
        println!("(no calendars accessible)");
        return Ok(());
    }

    for c in items {
        table.add_row(vec![
            cell(c.id.as_deref()),
            cell(c.summary.as_deref()),
            cell(c.access_role.as_deref()),
            if c.primary {
                "✓".to_string()
            } else {
                "".to_string()
            },
            cell(c.timezone.as_deref()),
        ]);
    }
    println!("{table}");
    Ok(())
}
