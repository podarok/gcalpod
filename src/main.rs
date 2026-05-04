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
        .subcommand(Command::new("list").about("Lists all events in Google Calendar"))
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

    // Subcommands that don't need a CalendarHub run before auth().
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

    match matches.subcommand() {
        Some(("list", _)) => {
            let start_of_the_week = get_start_of_the_week();
            let start_of_the_week_utc = start_of_the_week.with_hour(0).unwrap().to_utc();

            let events = hub
                .events()
                .list("primary")
                .time_min(start_of_the_week_utc)
                .time_max(start_of_the_week_utc + Duration::days(7))
                .single_events(true)
                .doit()
                .await;
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
