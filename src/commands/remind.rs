use std::error::Error;
use std::process::Command;

use chrono::{Duration, Utc};
use chrono_tz::Tz;
use google_calendar3::{
    hyper_rustls::HttpsConnector, hyper_util::client::legacy::connect::HttpConnector, CalendarHub,
};

pub struct RemindArgs<'a> {
    pub mins: i64,
    pub command: Vec<String>,
    pub calendar_id: &'a str,
    pub tz: Tz,
}

pub async fn run(
    hub: &CalendarHub<HttpsConnector<HttpConnector>>,
    args: RemindArgs<'_>,
) -> Result<(), Box<dyn Error>> {
    if args.command.is_empty() {
        return Err(
            "no command provided. Usage: gcal remind <mins> -- <cmd> [args...]".into(),
        );
    }

    let now = Utc::now();
    let window_start = now;
    let window_end = now + Duration::minutes(args.mins);

    let (_, evs) = hub
        .events()
        .list(args.calendar_id)
        .time_min(window_start)
        .time_max(window_end)
        .single_events(true)
        .doit()
        .await?;
    let mut items: Vec<_> = evs
        .items
        .unwrap_or_default()
        .into_iter()
        .filter(|e| e.start.as_ref().and_then(|s| s.date_time).is_some())
        .collect();
    items.sort_by_key(|e| e.start.as_ref().unwrap().date_time.unwrap());

    let target = match items.first() {
        Some(e) => e,
        None => {
            println!("gcal: no events in the next {} minute(s); nothing to remind.", args.mins);
            return Ok(());
        }
    };

    let summary = target.summary.as_deref().unwrap_or("");
    let html_link = target.html_link.as_deref().unwrap_or("");
    let start_dt = target.start.as_ref().and_then(|s| s.date_time).unwrap();
    use chrono::TimeZone;
    let start_local = args.tz.from_utc_datetime(&start_dt.naive_utc()).to_rfc3339();

    let interpolated: Vec<String> = args
        .command
        .iter()
        .map(|tok| {
            tok.replace("{{summary}}", summary)
                .replace("{{start}}", &start_local)
                .replace("{{html_link}}", html_link)
        })
        .collect();

    let (cmd, rest) = interpolated.split_first().unwrap();
    let status = Command::new(cmd).args(rest).status()?;
    if !status.success() {
        eprintln!(
            "gcal: remind command exited with status {} (cmd: {} {:?})",
            status, cmd, rest
        );
    }
    Ok(())
}
