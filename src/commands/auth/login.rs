use std::error::Error;

use crate::profile::Profile;
use crate::util::calendar::{self, AuthOptions, DEFAULT_SCOPES};

/// Inputs to `gcal auth login`.
pub struct LoginArgs {
    /// Comma-separated scope override. `None` → `DEFAULT_SCOPES`.
    pub scopes: Option<String>,
    /// Use paste-code flow instead of browser HTTPRedirect.
    pub no_browser: bool,
    /// Force re-flow even if cached token still valid.
    pub reauth: bool,
}

pub async fn run(profile: &Profile, args: LoginArgs) -> Result<(), Box<dyn Error>> {
    profile.ensure_dir()?;

    if args.reauth {
        let store = profile.store_path();
        if store.exists() {
            std::fs::remove_file(&store)?;
            eprintln!("gcal: removed cached token at {}", store.display());
        }
    }

    let scopes_owned: Vec<String> = match &args.scopes {
        Some(csv) => csv.split(',').map(|s| s.trim().to_string()).collect(),
        None => DEFAULT_SCOPES.iter().map(|s| s.to_string()).collect(),
    };
    let scopes_ref: Vec<&str> = scopes_owned.iter().map(String::as_str).collect();

    let opts = AuthOptions {
        no_browser: args.no_browser,
    };

    calendar::run_login_flow(profile, &scopes_ref, &opts).await?;

    println!(
        "gcal: logged in as profile '{}'. Token cached at {}.",
        profile.name,
        profile.store_path().display()
    );
    Ok(())
}
