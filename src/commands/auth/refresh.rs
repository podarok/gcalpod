use std::error::Error;

use crate::profile::Profile;
use crate::util::calendar::{self, AuthOptions, DEFAULT_SCOPES};

/// `gcal auth refresh [--profile]` — force a token refresh by
/// running the OAuth flow's token() call. yup_oauth2 will use the
/// existing refresh_token and write the new access_token to
/// store.json. If the refresh_token is revoked, surfaces an
/// actionable error pointing the user to `gcal auth login --reauth`.
pub async fn run(profile: &Profile) -> Result<(), Box<dyn Error>> {
    if !profile.store_path().is_file() {
        return Err(anyhow::anyhow!(
            "profile '{}' has no cached token. Run `gcal auth login --profile {}` first.",
            profile.name,
            profile.name
        )
        .into());
    }

    let opts = AuthOptions::default();
    calendar::run_login_flow(profile, DEFAULT_SCOPES, &opts)
        .await
        .map_err(|e| {
            anyhow::anyhow!(
                "token refresh failed: {}. Try `gcal auth login --profile {} --reauth`.",
                e,
                profile.name
            )
        })?;

    println!(
        "gcal: refreshed token for profile '{}' ({})",
        profile.name,
        profile.store_path().display()
    );
    Ok(())
}
