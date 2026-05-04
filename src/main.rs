mod commands;
mod config;
mod profile;
mod util;

use chrono::{Duration, Timelike};
use chrono_tz::Tz;
use clap::{Arg, ArgAction, Command};
use google_calendar3::api::{ConferenceData, ConferenceSolutionKey, CreateConferenceRequest};
use google_calendar3::api::{Event, EventDateTime};
use util::calendar::{self, get_default_timezone};
use util::date::{get_date_from_string, get_start_of_the_week};
use uuid::Uuid;

fn build_cli() -> Command {
    Command::new("gcal")
        .about("Google Calendar - CLI")
        .version(env!("CARGO_PKG_VERSION"))
        .args_conflicts_with_subcommands(true)
        .arg(
            Arg::new("gen-man")
                .help("Print man page (clap_mangen) to stdout and exit")
                .long("gen-man")
                .action(ArgAction::SetTrue)
                .global(false)
                .required(false),
        )
        .arg(
            Arg::new("profile")
                .help("OAuth profile name (overrides GCAL_PROFILE env + config.toml active_profile)")
                .long("profile")
                .global(true)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .help("Verbose output: extra context + hints (for newcomers / init agents)")
                .long("verbose")
                .short('v')
                .action(ArgAction::SetTrue)
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
                        .value_parser(["table", "json", "tsv", "csv", "raw", "conky"])
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
                )
                .arg(
                    Arg::new("style")
                        .help("Table layout: auto | grid | agenda")
                        .long("style")
                        .value_parser(["auto", "grid", "agenda"])
                        .default_value("auto")
                        .required(false),
                )
                .arg(
                    Arg::new("lineart")
                        .help("Box drawing: unicode | fancy | ascii")
                        .long("lineart")
                        .value_parser(["unicode", "fancy", "ascii"])
                        .default_value("unicode")
                        .required(false),
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
                .about("Interactive setup wizard for a new OAuth profile")
                .arg(
                    Arg::new("shared")
                        .help("Save the OAuth client at ~/.gcal/secret.json (reused by every profile)")
                        .long("shared")
                        .action(ArgAction::SetTrue)
                        .required(false),
                ),
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
            Command::new("remind")
                .about("Run <command> if next event starts within <mins>")
                .trailing_var_arg(true)
                .arg(Arg::new("mins").required(true).value_parser(clap::value_parser!(i64)))
                .arg(Arg::new("calendar").long("calendar").required(false))
                .arg(
                    Arg::new("command")
                        .num_args(1..)
                        .required(true)
                        .help("Command + args. {{summary}} {{start}} {{html_link}} get interpolated."),
                ),
        )
        .subcommand(
            Command::new("quick")
                .about("Natural-language event creation (Google quick-add)")
                .arg(Arg::new("text").required(true))
                .arg(Arg::new("calendar").long("calendar").required(false))
                .arg(
                    Arg::new("conference")
                        .help("Attach a Google Meet conference (post-create patch)")
                        .long("conference")
                        .short('c')
                        .action(ArgAction::SetTrue)
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("import")
                .about("Bulk-insert events from an ICS / VCAL file")
                .arg(Arg::new("path").required(true))
                .arg(Arg::new("calendar").long("calendar").required(false))
                .arg(
                    Arg::new("dry-run")
                        .long("dry-run")
                        .action(ArgAction::SetTrue)
                        .required(false),
                )
                .arg(
                    Arg::new("skip-duplicates")
                        .long("skip-duplicates")
                        .action(ArgAction::SetTrue)
                        .required(false),
                ),
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
                        .value_parser(["table", "json", "tsv", "csv", "raw", "conky"])
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
                        .value_parser(["table", "json", "tsv", "csv", "raw", "conky"])
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
                                .value_parser(["table", "json", "tsv", "csv", "raw", "conky"])
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
}

#[tokio::main]
async fn main() {
    let cli = build_cli();
    let matches = cli.clone().get_matches();

    // --gen-man: render man page and exit before any auth.
    if matches.get_flag("gen-man") {
        let cmd = build_cli();
        let man = clap_mangen::Man::new(cmd);
        let mut buf: Vec<u8> = vec![];
        if let Err(e) = man.render(&mut buf) {
            eprintln!("gcal: failed to render man page: {}", e);
            std::process::exit(1);
        }
        use std::io::Write;
        let _ = std::io::stdout().write_all(&buf);
        return;
    }

    // Surface --verbose as GCAL_VERBOSE=1 so existing verbose paths
    // (calendar::auth, recovery hooks) trigger consistently.
    if matches.get_flag("verbose") {
        std::env::set_var("GCAL_VERBOSE", "1");
    }
    let verbose = matches.get_flag("verbose");

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
    if let Some(("init", init_m)) = matches.subcommand() {
        let shared = init_m.get_flag("shared");
        if let Err(e) = commands::init::run(&prof, shared).await {
            util::recovery::report_error("init", &e);
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
            util::recovery::report_error("config", &e);
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
                    util::recovery::report_error("auth login", &e);
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
                    util::recovery::report_error("auth status", &e);
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
                    util::recovery::report_error("auth logout", &e);
                    std::process::exit(1);
                }
                return;
            }
            Some(("switch", switch_m)) => {
                let target = switch_m.get_one::<String>("target").unwrap();
                if let Err(e) = commands::auth::switch::run(target).await {
                    util::recovery::report_error("auth switch", &e);
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
            util::recovery::report_error("authentication", &e);
            std::process::exit(1);
        }
    };

    let tz: Tz = get_default_timezone(&hub).await.unwrap();

    if let Some(("remind", m)) = matches.subcommand() {
        let mins = *m.get_one::<i64>("mins").unwrap();
        let calendar_id: &str = m
            .get_one::<String>("calendar")
            .map(String::as_str)
            .unwrap_or("primary");
        let command: Vec<String> = m.get_many::<String>("command").unwrap().cloned().collect();
        if let Err(e) = commands::remind::run(
            &hub,
            commands::remind::RemindArgs {
                mins,
                command,
                calendar_id,
                tz,
            },
        )
        .await
        {
            util::recovery::report_error("remind", &e);
            std::process::exit(1);
        }
        return;
    }

    if let Some(("quick", m)) = matches.subcommand() {
        let text = m.get_one::<String>("text").unwrap();
        let calendar_id: &str = m
            .get_one::<String>("calendar")
            .map(String::as_str)
            .unwrap_or("primary");
        let with_conf = m.get_flag("conference");
        match hub.events().quick_add(calendar_id, text).doit().await {
            Ok((_, event)) => {
                let event_id = event.id.clone();
                println!("Event created: {:?}", event.html_link.unwrap_or_default());
                if with_conf {
                    if let Some(eid) = event_id {
                        let mut patch_event = Event::default();
                        patch_event.conference_data = Some(ConferenceData {
                            create_request: Some(CreateConferenceRequest {
                                request_id: Some(Uuid::new_v4().to_string()),
                                conference_solution_key: Some(ConferenceSolutionKey {
                                    type_: Some("hangoutsMeet".to_string()),
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        });
                        match hub
                            .events()
                            .patch(patch_event, calendar_id, &eid)
                            .conference_data_version(1)
                            .doit()
                            .await
                        {
                            Ok((_, _)) => println!("Conference attached."),
                            Err(e) => eprintln!("Failed to attach conference: {:?}", e),
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error creating event: {:?}", e),
        }
        return;
    }

    if let Some(("import", m)) = matches.subcommand() {
        let path = m.get_one::<String>("path").unwrap();
        let calendar_id: &str = m
            .get_one::<String>("calendar")
            .map(String::as_str)
            .unwrap_or("primary");
        let dry_run = m.get_flag("dry-run");
        let skip_duplicates = m.get_flag("skip-duplicates");
        if let Err(e) = commands::import::run(
            &hub,
            commands::import::ImportArgs {
                path,
                calendar_id,
                dry_run,
                skip_duplicates,
                tz,
            },
        )
        .await
        {
            util::recovery::report_error("import", &e);
            std::process::exit(1);
        }
        return;
    }

    if let Some(("edit", m)) = matches.subcommand() {
        let event_id = m.get_one::<String>("event-id").unwrap();
        let calendar_id: &str = m
            .get_one::<String>("calendar")
            .map(String::as_str)
            .unwrap_or("primary");
        let fields: Vec<(String, String)> = m
            .get_many::<String>("field")
            .map(|vals| {
                vals.filter_map(|kv| {
                    kv.split_once('=')
                        .map(|(k, v)| (k.trim().to_string(), v.to_string()))
                })
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
            util::recovery::report_error("edit", &e);
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
            util::recovery::report_error("delete", &e);
            std::process::exit(1);
        }
        return;
    }

    // agenda + search use shared events::list helper.
    if let Some((cmd, m)) = matches
        .subcommand()
        .filter(|(c, _)| *c == "agenda" || *c == "search")
    {
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
            m.get_one::<String>("format")
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

            let _range_days = (time_max - time_min).num_days();
            let _ = default_to_utc;

            // --style + --lineart for table output (W7-P2).
            let style_str = list_m
                .get_one::<String>("style")
                .map(String::as_str)
                .unwrap_or("auto");
            let lineart_str = list_m
                .get_one::<String>("lineart")
                .map(String::as_str)
                .unwrap_or("unicode");
            let layout_style = match util::render::LayoutStyle::parse(style_str) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            let lineart = match util::render::LineArt::parse(lineart_str) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

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
                        if let Err(e) =
                            util::format::render_list(output_format, &list_events, &raw_events)
                        {
                            eprintln!("Error rendering events: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Error retrieving events: {:?}", e),
                }
                return;
            }

            // Hybrid renderer: grid for short range on wide tty, agenda otherwise.
            if verbose {
                eprintln!(
                    "gcal: list profile='{}' calendar='{}' range={} -> {} style={:?} lineart={:?}",
                    prof.name,
                    calendar_id,
                    time_min.to_rfc3339(),
                    time_max.to_rfc3339(),
                    layout_style,
                    lineart
                );
            }
            match events {
                Ok((_, evs)) => {
                    let raw_events: Vec<google_calendar3::api::Event> =
                        evs.items.unwrap_or_default();
                    util::render::render_list_smart(
                        &raw_events,
                        time_min,
                        time_max,
                        tz,
                        layout_style,
                        lineart,
                    );
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

                let result = hub
                    .events()
                    .insert(event, "primary")
                    .conference_data_version(1)
                    .doit()
                    .await;

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
