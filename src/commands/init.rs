use std::error::Error;
use std::io::{self, Write};
use std::path::PathBuf;

use serde::Deserialize;

use crate::commands::auth::login::{self, LoginArgs};
use crate::profile::Profile;

const STEP_PROJECT: &str = "https://console.cloud.google.com/projectcreate";
const STEP_API: &str = "https://console.cloud.google.com/apis/library/calendar-json.googleapis.com";
const STEP_CONSENT: &str = "https://console.cloud.google.com/apis/credentials/consent";
const STEP_CREDS: &str = "https://console.cloud.google.com/apis/credentials";

/// Subset of the Google OAuth client JSON used to validate the file
/// before we move it into place.
#[derive(Debug, Deserialize)]
struct OAuthClientJson {
    installed: Option<InstalledSection>,
    web: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct InstalledSection {
    client_id: String,
    client_secret: String,
}

pub async fn run(profile: &Profile) -> Result<(), Box<dyn Error>> {
    println!();
    println!("== gcal init wizard ==");
    println!("Sets up profile '{}' under {}.", profile.name, profile.dir.display());
    println!();
    println!("You'll create your own Google Cloud OAuth client (~5 minutes,");
    println!("free for personal use). Step-by-step pages:");
    println!();
    println!("  1. Project create: {}", STEP_PROJECT);
    println!("  2. Enable Calendar API: {}", STEP_API);
    println!("  3. OAuth consent screen: {}", STEP_CONSENT);
    println!("  4. Create OAuth client (Desktop app): {}", STEP_CREDS);
    println!();
    println!("Detailed walkthrough: docs/custom_auth.md.");
    println!();

    let _ = prompt(
        "Press <enter> after step 4 (you should have downloaded a JSON file): ",
    )?;

    profile.ensure_dir()?;

    loop {
        let input = prompt(
            "Path to the downloaded OAuth client JSON file (drag-and-drop OK): ",
        )?;
        let trimmed = input.trim().trim_matches(|c| c == '"' || c == '\'').to_string();
        if trimmed.is_empty() {
            println!("Path is empty. Try again or Ctrl-C to abort.");
            continue;
        }
        let src = PathBuf::from(&trimmed);
        if !src.is_file() {
            println!("File not found: {}. Try again.", src.display());
            continue;
        }

        match validate_oauth_json(&src) {
            Ok(()) => {}
            Err(e) => {
                println!("Validation failed: {}. Re-download from Google Console (Desktop app type).", e);
                continue;
            }
        }

        let dst = profile.secret_path();
        std::fs::rename(&src, &dst).or_else(|_| std::fs::copy(&src, &dst).map(|_| ()))?;
        println!("gcal: secret saved to {}", dst.display());
        break;
    }

    println!();
    println!("Running `gcal auth login --profile {}`...", profile.name);
    login::run(
        profile,
        LoginArgs {
            scopes: None,
            no_browser: false,
            reauth: false,
        },
    )
    .await?;

    println!();
    println!("Done. Try: `gcal list` or `gcal calendars list`.");
    Ok(())
}

fn prompt(msg: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", msg);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim_end_matches('\n').to_string())
}

fn validate_oauth_json(path: &PathBuf) -> Result<(), String> {
    let body = std::fs::read_to_string(path).map_err(|e| format!("read failed: {}", e))?;
    let parsed: OAuthClientJson = serde_json::from_str(&body)
        .map_err(|e| format!("not valid JSON or unexpected shape: {}", e))?;
    if parsed.web.is_some() && parsed.installed.is_none() {
        return Err("file is for a Web client. Recreate as 'Desktop app' type".into());
    }
    let installed = parsed
        .installed
        .ok_or_else(|| "missing 'installed' section (Desktop app expected)".to_string())?;
    if installed.client_id.is_empty() || installed.client_secret.is_empty() {
        return Err("client_id or client_secret is empty".into());
    }
    Ok(())
}
