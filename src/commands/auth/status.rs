use std::error::Error;

use serde::Deserialize;

use crate::profile::Profile;
use crate::util::calendar;
use crate::util::file;

pub struct StatusArgs {
    pub all: bool,
    pub check: bool,
    pub show_token: bool,
}

/// Subset of yup_oauth2's persisted token entry. We only need
/// presence of `access_token` + `refresh_token` for offline status.
#[derive(Debug, Deserialize)]
struct StoreToken {
    access_token: Option<String>,
    refresh_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StoreEntry {
    scopes: Vec<String>,
    token: StoreToken,
}

pub async fn run(active: &Profile, args: StatusArgs) -> Result<(), Box<dyn Error>> {
    let profiles = if args.all {
        list_all_profiles()?
    } else {
        vec![active.clone()]
    };

    if profiles.is_empty() {
        println!("gcal: no profiles found under ~/.gcal/profiles/");
        return Ok(());
    }

    let mut first = true;
    for prof in &profiles {
        if !first {
            println!();
        }
        first = false;
        report_profile(prof, active, &args).await?;
    }
    Ok(())
}

fn list_all_profiles() -> Result<Vec<Profile>, Box<dyn Error>> {
    let dir = file::get_absolute_path(".gcal/profiles")?;
    if !dir.is_dir() {
        return Ok(vec![]);
    }
    let mut out = vec![];
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        out.push(Profile::new(&name)?);
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

async fn report_profile(
    prof: &Profile,
    active: &Profile,
    args: &StatusArgs,
) -> Result<(), Box<dyn Error>> {
    let active_marker = if prof.name == active.name {
        " (active)"
    } else {
        ""
    };
    println!("Profile: {}{}", prof.name, active_marker);

    let secret = prof.secret_path();
    let store = prof.store_path();

    let secret_label =
        if std::env::var("GCAL_CLIENT_ID").is_ok() && std::env::var("GCAL_CLIENT_SECRET").is_ok() {
            "env (GCAL_CLIENT_ID/GCAL_CLIENT_SECRET)".to_string()
        } else if let Ok(env_file) = std::env::var("GCAL_SECRET_FILE") {
            format!("env file: {}", env_file)
        } else if secret.is_file() {
            secret.display().to_string()
        } else {
            let shared = Profile::shared_secret_path()?;
            let flag = Profile::shared_flag_path()?;
            if shared.is_file() && flag.is_file() {
                format!("{} (shared)", shared.display())
            } else if shared.is_file() {
                format!("{} (legacy)", shared.display())
            } else {
                "<missing>".into()
            }
        };
    println!("  Secret:  {}", secret_label);

    if !store.is_file() {
        println!("  Token:   <not authenticated>");
        println!("  State:   ✗ run `gcal auth login --profile {}`", prof.name);
        return Ok(());
    }

    let body = std::fs::read_to_string(&store)?;
    let entries: Vec<StoreEntry> = serde_json::from_str(&body)?;
    let entry = match entries.first() {
        Some(e) => e,
        None => {
            println!("  Token:   <empty store>");
            println!("  State:   ✗ re-run `gcal auth login`");
            return Ok(());
        }
    };

    let has_access = entry
        .token
        .access_token
        .as_deref()
        .map(|s| !s.is_empty())
        .unwrap_or(false);
    let has_refresh = entry
        .token
        .refresh_token
        .as_deref()
        .map(|s| !s.is_empty())
        .unwrap_or(false);

    println!("  Scopes:  {}", entry.scopes.join(", "));
    println!(
        "  Token:   access={} refresh={} (path: {})",
        if has_access { "yes" } else { "no" },
        if has_refresh { "yes" } else { "no" },
        store.display()
    );

    if args.show_token {
        if let Some(t) = &entry.token.access_token {
            println!("  Bearer:  {}", t);
            eprintln!(
                "gcal: warning — access token printed. Treat as a credential. \
                 Token rotates ~hourly; rerun on expiry."
            );
        }
    }

    if args.check {
        match calendar::auth(prof).await {
            Ok(hub) => match hub.calendar_list().list().max_results(1).doit().await {
                Ok(_) => println!("  State:   ✓ ready (live API check passed)"),
                Err(e) => println!("  State:   ✗ live check failed: {}", e),
            },
            Err(e) => println!("  State:   ✗ build auth failed: {}", e),
        }
    } else {
        let state = if has_access && has_refresh {
            "✓ ready"
        } else {
            "✗ incomplete"
        };
        println!("  State:   {} (offline check)", state);
    }

    Ok(())
}
