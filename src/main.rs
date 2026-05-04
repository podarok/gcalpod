mod commands;
mod config;
mod profile;
mod util;

use std::collections::HashMap;
use std::{collections::hash_map::Entry, fmt::Write};

use chrono::{Datelike, Duration, Month, Timelike, TimeZone};
use chrono_tz::Tz;
use clap::{Arg, ArgAction, Command};
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use google_calendar3::api::{ConferenceData, ConferenceSolutionKey, CreateConferenceRequest};
use google_calendar3::api::{Event, EventDateTime};
use util::calendar::{self, get_default_timezone};
use util::date::{days_in_english, get_date_from_string, get_start_of_the_week};
use uuid::Uuid;

#[tokio::main]

async fn main() {
    let command = Command::new("gcal");
    let matches = command
        .about("Google Calendar - CLI")
        .version("0.0.1")
        .args_conflicts_with_subcommands(true)
        .arg(
            Arg::new("profile")
                .help("OAuth profile name (overrides GCAL_PROFILE env + config.toml active_profile)")
                .long("profile")
                .global(true)
                .required(false),
        )
        .arg(
            Arg::new("title")
                .help("Sets the event title")
                .required(false),
        )
        .arg(Arg::new("date").help("Sets the event date").required(false))
        .arg(
            Arg::new("conference")
                .help("Indicates that this event will be a conference Google Meet")
                .long("conference")
                .short('c')
                .action(ArgAction::SetTrue)
                .required(false)
                .requires("title"),
        )
        .subcommand(
            Command::new("add")
                .about("Adds a new event to Google Calendar")
                .arg(
                    Arg::new("title")
                        .help("Sets the event title")
                        .required(true),
                )
                .arg(Arg::new("date").help("Sets the event date").required(true)),
        )
        .subcommand(
            Command::new("list")
                .about("Lists events in a date range (default: current week)")
                .arg(
                    Arg::new("from")
                        .help("Range start: today, +Nd, +Nw, weekday, YYYY-MM-DD, RFC3339")
                        .long("from")
                        .required(false),
                )
                .arg(
                    Arg::new("to")
                        .help("Range end: today, +Nd, +Nw, weekday, YYYY-MM-DD, RFC3339")
                        .long("to")
                        .required(false),
                )
                .arg(
                    Arg::new("calendar")
                        .help("Calendar id (default: primary)")
                        .long("calendar")
                        .required(false),
                )
                .arg(
                    Arg::new("format")
                        .help("Output format: table | json | tsv | csv | raw")
                        .long("format")
                        .value_parser(["table", "json", "tsv", "csv", "raw"])
                        .default_value("table")
                        .required(false),
                )
                .arg(
                    Arg::new("json")
                        .help("Alias for --format json")
                        .long("json")
                        .action(ArgAction::SetTrue)
                        .required(false)
                        .conflicts_with("format"),
                ),
        )
        .subcommand(
            Command::new("auth")
                .about("Manage OAuth credentials per profile")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("login")
                        .about("Authenticate the active profile (or --profile <name>)")
                        .arg(
                            Arg::new("scopes")
                                .help("Comma-separated OAuth scope override (default: full Calendar read/write)")
                                .long("scopes")
                                .required(false),
                        )
                        .arg(
                            Arg::new("no-browser")
                                .help("Use paste-code flow instead of opening a browser")
                                .long("no-browser")
                                .action(ArgAction::SetTrue)
                                .required(false),
                        )
                        .arg(
                            Arg::new("reauth")
                                .help("Force OAuth flow even if a cached token is valid")
                                .long("reauth")
                                .action(ArgAction::SetTrue)
                                .required(false),
                        ),
                )
                .subcommand(
                    Command::new("status")
                        .about("Show authentication state for the active profile (or all)")
                        .arg(
                            Arg::new("all")
                                .help("Show every profile under ~/.gcal/profiles/")
                                .long("all")
                                .action(ArgAction::SetTrue)
                                .required(false),
                        )
                        .arg(
                            Arg::new("check")
                                .help("Make a live API call to verify the token works")
                                .long("check")
                                .action(ArgAction::SetTrue)
                                .required(false),
                        )
                        .arg(
                            Arg::new("show-token")
                                .help("Print the bearer access token (treat as a credential)")
                                .long("show-token")
                                .action(ArgAction::SetTrue)
                                .required(false),
                        ),
                )
                .subcommand(
                    Command::new("logout")
                        .about("Remove the cached token (and secret with --purge)")
                        .arg(
                            Arg::new("all")
                                .help("Log out every profile under ~/.gcal/profiles/")
                                .long("all")
                                .action(ArgAction::SetTrue)
                                .required(false),
                        )
                        .arg(
                            Arg::new("purge")
                                .help("Also delete secret.json and the profile directory")
                                .long("purge")
                                .action(ArgAction::SetTrue)
                                .required(false),
                        ),
                )
                .subcommand(
                    Command::new("switch")
                        .about("Change the active profile in ~/.gcal/config.toml")
                        .arg(
                            Arg::new("target")
                                .help("Profile name to activate")
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            Command::new("init")
                .about("Interactive setup wizard for a new OAuth profile"),
        )
        .subcommand(
            Command::new("config")
                .about("Read/write ~/.gcal/config.toml")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("get")
                        .about("Print value for <key>")
                        .arg(Arg::new("key").required(true)),
                )
                .subcommand(
                    Command::new("set")
                        .about("Set <key> = <value>")
                        .arg(Arg::new("key").required(true))
                        .arg(Arg::new("value").required(true)),
                )
                .subcommand(
                    Command::new("unset")
                        .about("Remove <key>")
                        .arg(Arg::new("key").required(true)),
                )
                .subcommand(Command::new("list").about("Print all keys"))
                .subcommand(Command::new("path").about("Print absolute path of config.toml")),
        )
        .subcommand(
            Command::new("edit")
                .about("Edit an existing event by id (use --field key=value)")
                .arg(Arg::new("event-id").required(true))
                .arg(Arg::new("calendar").long("calendar").required(false))
                .arg(
                    Arg::new("field")
                        .long("field")
                        .help("key=value (repeatable). Keys: summary, description, location, start, end")
                        .action(ArgAction::Append)
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete an event by id (prompts confirmation unless --yes)")
                .arg(Arg::new("event-id").required(true))
                .arg(Arg::new("calendar").long("calendar").required(false))
                .arg(
                    Arg::new("yes")
                        .long("yes")
                        .short('y')
                        .action(ArgAction::SetTrue)
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("agenda")
                .about("Flat chronological list of events in a date range")
                .arg(Arg::new("from").long("from").required(false))
                .arg(Arg::new("to").long("to").required(false))
                .arg(Arg::new("calendar").long("calendar").required(false))
                .arg(
                    Arg::new("format")
                        .long("format")
                        .value_parser(["table", "json", "tsv", "csv", "raw"])
                        .default_value("table")
                        .required(false),
                )
                .arg(
                    Arg::new("json")
                        .long("json")
                        .action(ArgAction::SetTrue)
                        .required(false)
                        .conflicts_with("format"),
                ),
        )
        .subcommand(
            Command::new("search")
                .about("Full-text search events via Google Calendar API")
                .arg(Arg::new("query").required(true))
                .arg(Arg::new("from").long("from").required(false))
                .arg(Arg::new("to").long("to").required(false))
                .arg(Arg::new("calendar").long("calendar").required(false))
                .arg(
                    Arg::new("format")
                        .long("format")
                        .value_parser(["table", "json", "tsv", "csv", "raw"])
                        .default_value("table")
                        .required(false),
                )
                .arg(
                    Arg::new("json")
                        .long("json")
                        .action(ArgAction::SetTrue)
                        .required(false)
                        .conflicts_with("format"),
                ),
        )
        .subcommand(
            Command::new("calendars")
                .about("List or inspect calendars accessible to the active profile")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("list")
                        .about("List all calendars accessible to the active profile")
                        .arg(
                            Arg::new("format")
                                .help("Output format: table | json | tsv | csv | raw")
                                .long("format")
                                .value_parser(["table", "json", "tsv", "csv", "raw"])
                                .default_value("table")
                                .required(false),
                        )
                        .arg(
                            Arg::new("json")
                                .help("Alias for --format json")
                                .long("json")
                                .action(ArgAction::SetTrue)
                                .required(false)
                                .conflicts_with("format"),
                        ),
                ),
        )
        .get_matches();

    // Resolve active profile: --profile flag > GCAL_PROFILE env > config.toml > "default".
    let cfg = match config::Config::load_or_default() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading ~/.gcal/config.toml - {}", e);
            return;
        }
    };
    let cli_profile = matches.get_one::<String>("profile").map(String::as_str);
    let active_profile = profile::Profile::resolve_active(cli_profile, &cfg);
    let prof = match profile::Profile::new(&active_profile) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error resolving profile '{}' - {}", active_profile, e);
            return;
        }
    };
    if let Err(e) = prof.migrate_legacy_if_needed() {
        eprintln!("Warning: legacy migration failed - {}", e);
    }

    // Subcommands that don't need a pre-built CalendarHub run before auth().
    if let Some(("init", _)) = matches.subcommand() {
        if let Err(e) = commands::init::run(&prof).await {
            eprintln!("Error during init - {}", e);
            std::process::exit(1);
        }
        return;
    }

    if let Some(("config", cfg_m)) = matches.subcommand() {
        let action = match cfg_m.subcommand() {
            Some(("get", m)) => commands::config_cmd::ConfigAction::Get {
                key: m.get_one::<String>("key").unwrap().clone(),
            },
            Some(("set", m)) => commands::config_cmd::ConfigAction::Set {
                key: m.get_one::<String>("key").unwrap().clone(),
                value: m.get_one::<String>("value").unwrap().clone(),
            },
            Some(("unset", m)) => commands::config_cmd::ConfigAction::Unset {
                key: m.get_one::<String>("key").unwrap().clone(),
            },
            Some(("list", _)) => commands::config_cmd::ConfigAction::List,
            Some(("path", _)) => commands::config_cmd::ConfigAction::Path,
            _ => {
                eprintln!("Unknown config subcommand. Run `gcal config --help`.");
                std::process::exit(2);
            }
        };
        if let Err(e) = commands::config_cmd::run(action).await {
            eprintln!("Error in config: {}", e);
            std::process::exit(1);
        }
        return;
    }

    if let Some(("auth", auth_m)) = matches.subcommand() {
        match auth_m.subcommand() {
            Some(("login", login_m)) => {
                let args = commands::auth::login::LoginArgs {
                    scopes: login_m.get_one::<String>("scopes").cloned(),
                    no_browser: login_m.get_flag("no-browser"),
                    reauth: login_m.get_flag("reauth"),
                };
                if let Err(e) = commands::auth::login::run(&prof, args).await {
                    eprintln!("Error during login - {}", e);
                    std::process::exit(1);
                }
                return;
            }
            Some(("status", status_m)) => {
                let args = commands::auth::status::StatusArgs {
                    all: status_m.get_flag("all"),
                    check: status_m.get_flag("check"),
                    show_token: status_m.get_flag("show-token"),
                };
                if let Err(e) = commands::auth::status::run(&prof, args).await {
                    eprintln!("Error during status - {}", e);
                    std::process::exit(1);
                }
                return;
            }
            Some(("logout", logout_m)) => {
                let args = commands::auth::logout::LogoutArgs {
                    all: logout_m.get_flag("all"),
                    purge: logout_m.get_flag("purge"),
                };
                if let Err(e) = commands::auth::logout::run(&prof, args).await {
                    eprintln!("Error during logout - {}", e);
                    std::process::exit(1);
                }
                return;
            }
            Some(("switch", switch_m)) => {
                let target = switch_m.get_one::<String>("target").unwrap();
                if let Err(e) = commands::auth::switch::run(target).await {
                    eprintln!("Error during switch - {}", e);
                    std::process::exit(1);
                }
                return;
            }
            _ => {
                eprintln!("Unknown auth subcommand. Run `gcal auth --help`.");
                std::process::exit(2);
            }
        }
    }

    let hub = match calendar::auth(&prof).await {
        Ok(hub) => hub,
        Err(e) => {
            eprintln!("Error during authentication - {}", e);
            return;
        }
    };

    let tz: Tz = get_default_timezone(&hub).await.unwrap();

    if let Some(("edit", m)) = matches.subcommand() {
        let event_id = m.get_one::<String>("event-id").unwrap();
        let calendar_id: &str = m
            .get_one::<String>("calendar")
            .map(String::as_str)
            .unwrap_or("primary");
        let fields: Vec<(String, String)> = m
            .get_many::<String>("field")
            .map(|vals| {
                vals.filter_map(|kv| kv.split_once('=').map(|(k, v)| (k.trim().to_string(), v.to_string())))
                    .collect()
            })
            .unwrap_or_default();
        if let Err(e) = commands::events_mutate::edit(
            &hub,
            commands::events_mutate::EditArgs {
                event_id,
                calendar_id,
                fields,
                tz,
            },
        )
        .await
        {
            eprintln!("Error editing event: {}", e);
            std::process::exit(1);
        }
        return;
    }

    if let Some(("delete", m)) = matches.subcommand() {
        let event_id = m.get_one::<String>("event-id").unwrap();
        let calendar_id: &str = m
            .get_one::<String>("calendar")
            .map(String::as_str)
            .unwrap_or("primary");
        let yes = m.get_flag("yes");
        if let Err(e) = commands::events_mutate::delete(
            &hub,
            commands::events_mutate::DeleteArgs {
                event_id,
                calendar_id,
                yes,
            },
        )
        .await
        {
            eprintln!("Error deleting event: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // agenda + search use shared events::list helper.
    if let Some((cmd, m)) = matches.subcommand().filter(|(c, _)| *c == "agenda" || *c == "search") {
        let week_start = get_start_of_the_week();
        let default_from_utc = week_start.with_hour(0).unwrap().to_utc();
        let default_to_utc = default_from_utc + Duration::days(7);
        let time_min = match m.get_one::<String>("from") {
            Some(s) => match util::date::parse_range_input(tz, s) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Error parsing --from: {}", e);
                    return;
                }
            },
            None => default_from_utc,
        };
        let time_max = match m.get_one::<String>("to") {
            Some(s) => match util::date::parse_range_input(tz, s) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Error parsing --to: {}", e);
                    return;
                }
            },
            None => default_to_utc,
        };
        if time_max <= time_min {
            eprintln!("Error: --to must be after --from.");
            return;
        }
        let calendar_id: &str = m
            .get_one::<String>("calendar")
            .map(String::as_str)
            .unwrap_or("primary");
        let format_str = if m.get_flag("json") {
            "json"
        } else {
            m.get_one::<String>("format").map(String::as_str).unwrap_or("table")
        };
        let output_format = match util::format::OutputFormat::parse(format_str) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        };
        let query = if cmd == "search" {
            Some(m.get_one::<String>("query").unwrap().as_str())
        } else {
            None
        };
        if let Err(e) = commands::events::list(
            &hub,
            commands::events::EventsListArgs {
                time_min,
                time_max,
                calendar_id,
                query,
                format: output_format,
                tz,
            },
        )
        .await
        {
            eprintln!("Error fetching events: {}", e);
        }
        return;
    }

    // calendars subcommand needs hub (handled before list/add).
    if let Some(("calendars", cal_m)) = matches.subcommand() {
        match cal_m.subcommand() {
            Some(("list", list_m)) => {
                let format_str = if list_m.get_flag("json") {
                    "json"
                } else {
                    list_m
                        .get_one::<String>("format")
                        .map(String::as_str)
                        .unwrap_or("table")
                };
                let format = match util::format::OutputFormat::parse(format_str) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        return;
                    }
                };
                if let Err(e) = commands::calendars::list::run(
                    &hub,
                    commands::calendars::list::ListArgs { format },
                )
                .await
                {
                    eprintln!("Error listing calendars: {}", e);
                }
                return;
            }
            _ => {
                eprintln!("Unknown calendars subcommand. Run `gcal calendars --help`.");
                std::process::exit(2);
            }
        }
    }

    match matches.subcommand() {
        Some(("list", list_m)) => {
            // Resolve --from / --to (defaults: current week, +7d).
            let start_of_the_week = get_start_of_the_week();
            let default_from_utc = start_of_the_week.with_hour(0).unwrap().to_utc();
            let default_to_utc = default_from_utc + Duration::days(7);

            let time_min = match list_m.get_one::<String>("from") {
                Some(s) => match util::date::parse_range_input(tz, s) {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("Error parsing --from: {}", e);
                        return;
                    }
                },
                None => default_from_utc,
            };
            let time_max = match list_m.get_one::<String>("to") {
                Some(s) => match util::date::parse_range_input(tz, s) {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("Error parsing --to: {}", e);
                        return;
                    }
                },
                None => default_to_utc,
            };

            if time_max <= time_min {
                eprintln!("Error: --to must be after --from.");
                return;
            }

            let calendar_id: &str = list_m
                .get_one::<String>("calendar")
                .map(String::as_str)
                .unwrap_or("primary");

            let range_days = (time_max - time_min).num_days();
            let use_flat_list = range_days > 14;
            let _ = default_to_utc;

            // Resolve --format (or --json alias).
            let format_str = if list_m.get_flag("json") {
                "json"
            } else {
                list_m
                    .get_one::<String>("format")
                    .map(String::as_str)
                    .unwrap_or("table")
            };
            let output_format = match util::format::OutputFormat::parse(format_str) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            let events = hub
                .events()
                .list(calendar_id)
                .time_min(time_min)
                .time_max(time_max)
                .single_events(true)
                .doit()
                .await;

            // Non-table formats handled uniformly via util::format::render_list.
            if output_format != util::format::OutputFormat::Table {
                match events {
                    Ok((_, evs)) => {
                        let raw_events: Vec<google_calendar3::api::Event> =
                            evs.items.unwrap_or_default();
                        let list_events: Vec<util::format::ListEvent> = raw_events
                            .iter()
                            .map(|e| util::format::ListEvent::from_event(e, calendar_id, tz))
                            .collect();
                        if let Err(e) = util::format::render_list(
                            output_format,
                            &list_events,
                            &raw_events,
                        ) {
                            eprintln!("Error rendering events: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Error retrieving events: {:?}", e),
                }
                return;
            }

            if use_flat_list {
                match events {
                    Ok((_, evs)) => {
                        if let Some(items) = evs.items {
                            let mut sorted: Vec<_> = items.into_iter().filter(|e| {
                                e.start.as_ref().and_then(|s| s.date_time).is_some()
                            }).collect();
                            sorted.sort_by_key(|e| e.start.as_ref().unwrap().date_time.unwrap());
                            for ev in &sorted {
                                let s = ev.start.as_ref().unwrap().date_time.unwrap();
                                let e = ev
                                    .end
                                    .as_ref()
                                    .and_then(|x| x.date_time)
                                    .unwrap_or(s);
                                let s_local = tz.from_utc_datetime(&s.naive_utc());
                                let e_local = tz.from_utc_datetime(&e.naive_utc());
                                let summary = ev.summary.as_deref().unwrap_or("(no title)");
                                println!(
                                    "{}  {:02}:{:02}-{:02}:{:02}  {}",
                                    s_local.format("%Y-%m-%d"),
                                    s_local.hour(),
                                    s_local.minute(),
                                    e_local.hour(),
                                    e_local.minute(),
                                    summary
                                );
                            }
                            if sorted.is_empty() {
                                println!("(no events in range)");
                            }
                        } else {
                            println!("(no events in range)");
                        }
                    }
                    Err(e) => println!("Error retrieving events: {:?}", e),
                }
                return;
            }
            match events {
                Ok((_, events)) => {
                    let mut table: Table = Table::new();
                    let mut event_dates: HashMap<_, Vec<_>> = HashMap::new();

                    if let Some(items) = events.items {
                        for event in items {
                            if event.start.as_ref().is_none()
                                || event.start.as_ref().unwrap().date_time.is_none()
                            {
                                continue;
                            }
                            let event_start =
                                event.start.as_ref().unwrap().date_time.unwrap().date_naive();
                            match event_dates.entry(event_start) {
                                Entry::Vacant(e) => {
                                    e.insert(vec![event]);
                                }
                                Entry::Occupied(mut e) => {
                                    e.get_mut().push(event);
                                }
                            }
                        }
                    }

                    let mut row_before_12: Vec<String> = vec![];
                    let mut row_after_12: Vec<String> = vec![];
                    let mut header: Vec<Cell> = vec![];
                    for (i, v) in days_in_english().iter().enumerate() {
                        let i = i as i64;
                        let next_date = start_of_the_week + Duration::days(i);
                        let header_value = format!(
                            "{} - {} {:?}",
                            v,
                            next_date.day(),
                            Month::try_from(u8::try_from(next_date.month()).unwrap())
                                .ok()
                                .unwrap()
                        );
                        if i < 5 {
                            header.push(
                                Cell::new(header_value)
                                    .fg(Color::DarkGreen)
                                    .add_attribute(Attribute::Bold),
                            );
                        } else {
                            header.push(Cell::new(header_value).fg(Color::DarkBlue));
                        }

                        let mut row_value_before_12: String = "".to_string();
                        let mut row_value_after_12: String = "".to_string();
                        if let Some(next_events) = event_dates.get_mut(&next_date.date_naive()) {
                            next_events.sort_by(|a, b| {
                                    a.start.as_ref()
                                        .unwrap()
                                        .date_time
                                        .unwrap()
                                        .cmp(&b.start.as_ref().unwrap().date_time.unwrap())
                                });
                            for next_event in next_events {
                                let event_start = next_event.start.as_ref().unwrap().date_time.unwrap();
                                let event_end = next_event.end.as_ref().unwrap().date_time.unwrap();
                                let summary = next_event.summary.as_ref().unwrap().to_string();
                                let formatted_event = format!(
                                    "{:02}:{:02} - {:02}:{:02}: {}\n\n",
                                    tz.from_utc_datetime(&event_start.naive_local()).hour(),
                                    event_start.minute(),
                                    tz.from_utc_datetime(&event_end.naive_local()).hour(),
                                    event_end.minute(),
                                    summary
                                );

                                if event_start.hour() < 12 {
                                    write!(row_value_before_12, "{}", formatted_event).unwrap();
                                } else {
                                    write!(row_value_after_12, "{}", formatted_event).unwrap();
                                }
                            }
                        }
                        row_before_12.push(row_value_before_12);
                        row_after_12.push(row_value_after_12);
                    }

                    table
                        .set_header(header)
                        .add_row(row_before_12)
                        .add_row(row_after_12)
                        .set_content_arrangement(ContentArrangement::DynamicFullWidth);

                    println!("{table}");
                }
                Err(e) => println!("Error retrieving events: {:?}", e),
            }
        }
        Some(("add", _)) | _ => {
            let title = matches.get_one::<String>("title");
            let date = matches.get_one::<String>("date");
            let conference = matches.get_one::<bool>("conference");
            if title.is_none() {
                return;
            }

            if date.is_none() {
                let result = hub
                    .events()
                    .quick_add("primary", title.as_ref().unwrap())
                    .doit()
                    .await;

                match result {
                    Ok((_, event)) => {
                        println!("Event created: {:?}", event.html_link.unwrap().to_string())
                    }
                    Err(e) => {
                        eprintln!("Error creating event: {:?}", e);
                    }
                }
            } else {
                let event_date_with_timezone = get_date_from_string(tz, date.unwrap());
                let mut event = Event {
                    summary: Some(title.unwrap().to_string()),
                    start: Some(EventDateTime {
                        date_time: Some(event_date_with_timezone),
                        ..Default::default()
                    }),
                    end: Some(EventDateTime {
                        date_time: Some(event_date_with_timezone + Duration::hours(1)),
                        ..Default::default()
                    }),
                    ..Default::default()
                };
                if conference.map_or(false, |&c| c) {
                    event.conference_data = Some(ConferenceData {
                        create_request: Some(CreateConferenceRequest {
                            request_id: Some(Uuid::new_v4().to_string()),
                            conference_solution_key: Some(ConferenceSolutionKey {
                                type_: Some("hangoutsMeet".to_string()),
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                }
                    
                let result = hub.events().insert(event, "primary").conference_data_version(1).doit().await;

                match result {
                    Ok((_, event)) => {
                        println!("Event created: {:?}", event.html_link.unwrap().to_string());
                    }
                    Err(e) => {
                        eprintln!("Error creating event: {:?}", e);
                    }
                }
            }
        }
    }
}
