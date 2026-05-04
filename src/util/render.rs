use chrono::{DateTime, Datelike, Duration, Local, Timelike, TimeZone, Utc};
use chrono_tz::Tz;
use comfy_table::presets::{UTF8_BORDERS_ONLY, UTF8_FULL, ASCII_FULL};
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use google_calendar3::api::Event;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutStyle {
    Auto,
    Grid,
    Agenda,
}

impl LayoutStyle {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "grid" => Ok(Self::Grid),
            "agenda" => Ok(Self::Agenda),
            other => Err(format!("unknown --style '{}'. Try: auto, grid, agenda.", other)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineArt {
    Unicode,
    Fancy,
    Ascii,
}

impl LineArt {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "unicode" => Ok(Self::Unicode),
            "fancy" => Ok(Self::Fancy),
            "ascii" => Ok(Self::Ascii),
            other => Err(format!("unknown --lineart '{}'. Try: unicode, fancy, ascii.", other)),
        }
    }

    fn preset(self) -> &'static str {
        match self {
            Self::Unicode => UTF8_BORDERS_ONLY,
            Self::Fancy => UTF8_FULL,
            Self::Ascii => ASCII_FULL,
        }
    }
}

/// Top-level dispatcher for `gcal list` table rendering.
pub fn render_list_smart(
    events: &[Event],
    range_start: DateTime<Utc>,
    range_end: DateTime<Utc>,
    tz: Tz,
    style: LayoutStyle,
    lineart: LineArt,
) {
    let cols = terminal_cols();
    let range_days = (range_end - range_start).num_days();

    let resolved = match style {
        LayoutStyle::Grid => LayoutStyle::Grid,
        LayoutStyle::Agenda => LayoutStyle::Agenda,
        LayoutStyle::Auto => {
            if range_days <= 7 && cols >= 100 {
                LayoutStyle::Grid
            } else {
                LayoutStyle::Agenda
            }
        }
    };

    match resolved {
        LayoutStyle::Grid => render_week_grid(events, range_start, tz, lineart, cols),
        LayoutStyle::Agenda => render_agenda_grouped(events, range_start, range_end, tz, cols),
        LayoutStyle::Auto => unreachable!(),
    }
}

fn terminal_cols() -> u16 {
    terminal_size::terminal_size()
        .map(|(w, _)| w.0)
        .unwrap_or(120)
}

fn truncate_to_width(s: &str, max: usize) -> String {
    if max == 0 {
        return String::new();
    }
    if UnicodeWidthStr::width(s) <= max {
        return s.to_string();
    }
    let mut out = String::new();
    let mut w = 0usize;
    for ch in s.chars() {
        let cw = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
        if w + cw + 1 > max {
            break;
        }
        out.push(ch);
        w += cw;
    }
    out.push('…');
    out
}

fn day_of_events(ev: &Event, tz: Tz) -> Option<chrono::NaiveDate> {
    if let Some(start) = &ev.start {
        if let Some(dt) = start.date_time {
            return Some(tz.from_utc_datetime(&dt.naive_utc()).date_naive());
        }
        if let Some(d) = start.date {
            return Some(d);
        }
    }
    None
}

fn is_all_day(ev: &Event) -> bool {
    ev.start
        .as_ref()
        .map(|s| s.date.is_some() && s.date_time.is_none())
        .unwrap_or(false)
}

fn is_today(date: chrono::NaiveDate, tz: Tz) -> bool {
    Local::now().with_timezone(&tz).date_naive() == date
}

fn render_agenda_grouped(
    events: &[Event],
    range_start: DateTime<Utc>,
    range_end: DateTime<Utc>,
    tz: Tz,
    cols: u16,
) {
    let max_summary = (cols as usize).saturating_sub(20).max(20);

    // Group by date; preserve a stable sorted iteration of all days in range.
    let mut sorted: Vec<&Event> = events.iter().collect();
    sorted.sort_by_key(|e| {
        e.start
            .as_ref()
            .and_then(|s| s.date_time.map(|d| d.timestamp()))
            .or_else(|| e.start.as_ref().and_then(|s| s.date.map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp())))
            .unwrap_or(0)
    });

    let start_date = tz.from_utc_datetime(&range_start.naive_utc()).date_naive();
    let end_date = tz.from_utc_datetime(&range_end.naive_utc()).date_naive();

    let mut day = start_date;
    let mut total_rendered = 0usize;
    let mut first_day = true;

    while day < end_date {
        let day_events: Vec<&Event> = sorted
            .iter()
            .filter(|e| day_of_events(e, tz) == Some(day))
            .copied()
            .collect();
        if !first_day {
            println!();
        }
        first_day = false;
        let header_text = format!(
            "{} {} {} {}",
            day.format("%a"),
            day.day(),
            day.format("%b"),
            day.year()
        );
        let underline_len = (cols as usize).saturating_sub(header_text.len()).min(60).max(8);
        let underline = "─".repeat(underline_len);
        let today_marker = if is_today(day, tz) { " ← today" } else { "" };
        println!("{}{} {}", header_text, today_marker, underline);

        if day_events.is_empty() {
            println!("  (none)");
        } else {
            // All-day first.
            let mut all_day_seen = false;
            for ev in day_events.iter().filter(|e| is_all_day(e)) {
                let summary = ev.summary.as_deref().unwrap_or("(no title)");
                println!(
                    "  [ALL DAY]   {}",
                    truncate_to_width(summary, max_summary)
                );
                all_day_seen = true;
                total_rendered += 1;
            }
            if all_day_seen && day_events.iter().any(|e| !is_all_day(e)) {
                println!();
            }
            for ev in day_events.iter().filter(|e| !is_all_day(e)) {
                let s = ev.start.as_ref().and_then(|s| s.date_time);
                let e = ev.end.as_ref().and_then(|x| x.date_time);
                let summary = ev.summary.as_deref().unwrap_or("(no title)");
                if let Some(s) = s {
                    let s_local = tz.from_utc_datetime(&s.naive_utc());
                    let e_local = e.map(|t| tz.from_utc_datetime(&t.naive_utc())).unwrap_or(s_local);
                    println!(
                        "  {:02}:{:02}-{:02}:{:02}  {}",
                        s_local.hour(),
                        s_local.minute(),
                        e_local.hour(),
                        e_local.minute(),
                        truncate_to_width(summary, max_summary)
                    );
                    total_rendered += 1;
                }
            }
        }
        day = day + Duration::days(1);
    }

    if total_rendered == 0 {
        println!();
        println!("(no events in range)");
    }
}

fn render_week_grid(
    events: &[Event],
    range_start: DateTime<Utc>,
    tz: Tz,
    lineart: LineArt,
    cols: u16,
) {
    let preset = lineart.preset();
    let mut table = Table::new();
    table
        .load_preset(preset)
        .set_content_arrangement(ContentArrangement::DynamicFullWidth);
    if cols >= 80 {
        table.set_width(cols);
    }

    // Establish 7 days starting from range_start.
    let start_local = tz.from_utc_datetime(&range_start.naive_utc()).date_naive();

    let mut header: Vec<Cell> = Vec::with_capacity(7);
    let mut all_day_row: Vec<String> = Vec::with_capacity(7);
    let mut am_row: Vec<String> = Vec::with_capacity(7);
    let mut pm_row: Vec<String> = Vec::with_capacity(7);

    let cell_width = ((cols as usize).saturating_sub(20) / 7).max(12);

    for offset in 0..7 {
        let day = start_local + Duration::days(offset);
        let header_text = format!("{}\n{} {}", day.format("%a"), day.day(), day.format("%b"));
        let mut cell = Cell::new(&header_text);
        cell = if is_today(day, tz) {
            cell.fg(Color::Yellow).add_attribute(Attribute::Bold)
        } else if offset >= 5 {
            cell.fg(Color::DarkBlue)
        } else {
            cell.fg(Color::DarkGreen).add_attribute(Attribute::Bold)
        };
        header.push(cell);

        let day_events: Vec<&Event> = events
            .iter()
            .filter(|e| day_of_events(e, tz) == Some(day))
            .collect();

        let all_day_text: Vec<String> = day_events
            .iter()
            .filter(|e| is_all_day(e))
            .map(|e| {
                truncate_to_width(
                    e.summary.as_deref().unwrap_or("(no title)"),
                    cell_width,
                )
            })
            .collect();
        all_day_row.push(if all_day_text.is_empty() {
            "·".to_string()
        } else {
            all_day_text.join("\n")
        });

        let mut am: Vec<String> = vec![];
        let mut pm: Vec<String> = vec![];
        let mut timed: Vec<&Event> = day_events
            .iter()
            .filter(|e| !is_all_day(e))
            .copied()
            .collect();
        timed.sort_by_key(|e| e.start.as_ref().unwrap().date_time.unwrap());
        for ev in timed {
            let s = ev.start.as_ref().unwrap().date_time.unwrap();
            let e = ev.end.as_ref().and_then(|x| x.date_time).unwrap_or(s);
            let s_local = tz.from_utc_datetime(&s.naive_utc());
            let e_local = tz.from_utc_datetime(&e.naive_utc());
            let line = format!(
                "{:02}:{:02}-{:02}:{:02} {}",
                s_local.hour(),
                s_local.minute(),
                e_local.hour(),
                e_local.minute(),
                truncate_to_width(
                    ev.summary.as_deref().unwrap_or("(no title)"),
                    cell_width.saturating_sub(12),
                )
            );
            if s_local.hour() < 12 {
                am.push(line);
            } else {
                pm.push(line);
            }
        }
        am_row.push(if am.is_empty() { "·".to_string() } else { am.join("\n") });
        pm_row.push(if pm.is_empty() { "·".to_string() } else { pm.join("\n") });
    }

    table.set_header(header);
    table.add_row(all_day_row);
    table.add_row(am_row);
    table.add_row(pm_row);
    println!("{table}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_style_parse() {
        assert_eq!(LayoutStyle::parse("auto").unwrap(), LayoutStyle::Auto);
        assert_eq!(LayoutStyle::parse("GRID").unwrap(), LayoutStyle::Grid);
        assert_eq!(LayoutStyle::parse("agenda").unwrap(), LayoutStyle::Agenda);
        assert!(LayoutStyle::parse("yaml").is_err());
    }

    #[test]
    fn lineart_parse() {
        assert_eq!(LineArt::parse("unicode").unwrap(), LineArt::Unicode);
        assert_eq!(LineArt::parse("Fancy").unwrap(), LineArt::Fancy);
        assert_eq!(LineArt::parse("ascii").unwrap(), LineArt::Ascii);
        assert!(LineArt::parse("emoji").is_err());
    }

    #[test]
    fn truncate_short_passes_through() {
        assert_eq!(truncate_to_width("hello", 20), "hello");
    }

    #[test]
    fn truncate_long_clips_with_ellipsis() {
        let out = truncate_to_width("abcdefghij", 5);
        assert!(out.ends_with('…'));
        assert!(UnicodeWidthStr::width(out.as_str()) <= 5);
    }

    #[test]
    fn truncate_zero_yields_empty() {
        assert_eq!(truncate_to_width("hello", 0), "");
    }

    #[test]
    fn truncate_unicode_safe() {
        // Cyrillic / wide chars get correct width.
        let out = truncate_to_width("привіт world", 6);
        assert!(UnicodeWidthStr::width(out.as_str()) <= 6);
    }
}
