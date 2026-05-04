use std::{env, error::Error, path::{Path, PathBuf}};

use anyhow::{Context, Result};
use chrono_tz::Tz;
use google_calendar3::{
    hyper_rustls::{self, HttpsConnector},
    hyper_util::{self, client::legacy::connect::HttpConnector},
    yup_oauth2::{self, ApplicationSecret},
    CalendarHub,
};

use super::file;
use crate::profile::Profile;

/// Source of the OAuth ApplicationSecret used by `auth()`.
///
/// Resolution order:
/// 1. `GCAL_CLIENT_ID` + `GCAL_CLIENT_SECRET` env vars (in-memory secret).
/// 2. `GCAL_SECRET_FILE` env var pointing to a JSON file.
/// 3. `~/.gcal/profiles/<active>/secret.json` (per-profile).
/// 4. `~/.gcal/secret.json` (legacy fallback for un-migrated installs).
///
/// If none are configured, `auth()` returns an error directing the
/// user to `docs/custom_auth.md`. There is no built-in fallback —
/// every user must configure their own Google Cloud OAuth client.
#[derive(Debug)]
enum SecretSource {
    Env,
    EnvFile(PathBuf),
    ProfileFile(PathBuf),
    LegacyFile(PathBuf),
}

async fn resolve_secret(
    profile: &Profile,
) -> Result<(ApplicationSecret, SecretSource), Box<dyn Error>> {
    if let (Ok(id), Ok(sec)) = (env::var("GCAL_CLIENT_ID"), env::var("GCAL_CLIENT_SECRET")) {
        let secret = build_secret(&id, &sec, env::var("GCAL_PROJECT_ID").ok());
        return Ok((secret, SecretSource::Env));
    }

    if let Ok(custom) = env::var("GCAL_SECRET_FILE") {
        let path = PathBuf::from(&custom);
        let secret = read_google_secret(&path).await.with_context(|| {
            format!("GCAL_SECRET_FILE={} could not be read", path.display())
        })?;
        return Ok((secret, SecretSource::EnvFile(path)));
    }

    let profile_secret = profile.secret_path();
    if profile_secret.is_file() {
        let secret = read_google_secret(&profile_secret).await?;
        return Ok((secret, SecretSource::ProfileFile(profile_secret)));
    }

    let legacy_path: PathBuf = file::get_absolute_path(".gcal/secret.json")?;
    if legacy_path.is_file() {
        let secret = read_google_secret(&legacy_path).await?;
        return Ok((secret, SecretSource::LegacyFile(legacy_path)));
    }

    Err(anyhow::anyhow!(
        "No OAuth credentials configured for profile '{}'. Set \
         GCAL_CLIENT_ID + GCAL_CLIENT_SECRET, GCAL_SECRET_FILE=<path>, \
         or place your OAuth client JSON at {}. See docs/custom_auth.md \
         for step-by-step Google Cloud Console setup.",
        profile.name,
        profile_secret.display()
    )
    .into())
}

fn build_secret(client_id: &str, client_secret: &str, project_id: Option<String>) -> ApplicationSecret {
    ApplicationSecret {
        client_id: client_id.to_string(),
        client_secret: client_secret.to_string(),
        auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
        token_uri: "https://accounts.google.com/o/oauth2/token".to_string(),
        redirect_uris: vec!["urn:ietf:wg:oauth:2.0:oob".to_string()],
        auth_provider_x509_cert_url: Some(
            "https://www.googleapis.com/oauth2/v1/certs".to_string(),
        ),
        project_id,
        client_email: None,
        client_x509_cert_url: None,
    }
}

/// Default OAuth scopes — full Calendar read + write.
pub const DEFAULT_SCOPES: &[&str] = &[
    "https://www.googleapis.com/auth/calendar",
    "https://www.googleapis.com/auth/calendar.events",
    "https://www.googleapis.com/auth/calendar.readonly",
    "https://www.googleapis.com/auth/calendar.events.readonly",
];

/// Options for the OAuth installed flow used by `auth_with_options`.
#[derive(Debug, Default, Clone)]
pub struct AuthOptions {
    /// If `true`, use `InstalledFlowReturnMethod::Interactive` (paste code).
    /// Else use `HTTPRedirect` (default — opens browser to localhost).
    pub no_browser: bool,
}

/// Build an authenticator for the given profile.
///
/// Resolves the secret, ensures the profile dir, returns a built
/// `Authenticator` configured to persist tokens at the profile's
/// `store.json`. Does NOT yet acquire a token (caller chooses
/// scopes via `auth.token(scopes)`).
async fn build_authenticator(
    profile: &Profile,
    opts: &AuthOptions,
) -> Result<
    yup_oauth2::authenticator::Authenticator<HttpsConnector<HttpConnector>>,
    Box<dyn Error>,
> {
    let (secret, source) = resolve_secret(profile).await?;

    if env::var("GCAL_VERBOSE").ok().as_deref() == Some("1") {
        eprintln!("gcal: profile '{}'", profile.name);
        match &source {
            SecretSource::Env => eprintln!("gcal: OAuth secret from env (GCAL_CLIENT_ID/GCAL_CLIENT_SECRET)"),
            SecretSource::EnvFile(p) => eprintln!("gcal: OAuth secret from GCAL_SECRET_FILE={}", p.display()),
            SecretSource::ProfileFile(p) => eprintln!("gcal: OAuth secret from {}", p.display()),
            SecretSource::LegacyFile(p) => eprintln!("gcal: OAuth secret from legacy {} (run any cmd as profile 'default' to migrate)", p.display()),
        }
    }

    let return_method = if opts.no_browser {
        yup_oauth2::InstalledFlowReturnMethod::Interactive
    } else {
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect
    };

    profile.ensure_dir()?;
    let store_path = profile.store_path();
    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(secret, return_method)
        .persist_tokens_to_disk(&store_path)
        .build()
        .await?;
    Ok(auth)
}

/// Run the OAuth installed flow for `profile` with explicit `scopes`.
///
/// Used by `gcal auth login` — does NOT build a `CalendarHub`. Returns
/// once a valid token is cached at `profile.store_path()`.
pub async fn run_login_flow(
    profile: &Profile,
    scopes: &[&str],
    opts: &AuthOptions,
) -> Result<(), Box<dyn Error>> {
    let auth = build_authenticator(profile, opts).await?;
    auth.token(scopes)
        .await
        .map_err(|e| anyhow::anyhow!("OAuth flow failed: {}", e))?;
    Ok(())
}

/// Authenticates the user with Google Calendar API and returns a CalendarHub instance.
///
/// Looks up OAuth credentials via `resolve_secret` for the given `profile`.
/// Persists the token to `~/.gcal/profiles/<profile>/store.json`. Errors if
/// no credentials configured. Set `GCAL_VERBOSE=1` to print the resolved
/// source + profile on stderr.
pub async fn auth(
    profile: &Profile,
) -> Result<CalendarHub<HttpsConnector<HttpConnector>>, Box<dyn Error>> {
    let auth = build_authenticator(profile, &AuthOptions::default()).await?;

    match auth.token(DEFAULT_SCOPES).await {
        Ok(_) => {}
        Err(e) => println!("Authentication error: {:?}", e),
    }
    let client = hyper_util::client::legacy::Client::builder(
        hyper_util::rt::TokioExecutor::new()
    )
    .build(
        hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_or_http()
            .enable_http1()
            .build()
    );

    let hub = CalendarHub::new(client, auth);
    Ok(hub)
}

pub async fn get_default_timezone(hub: &CalendarHub<HttpsConnector<HttpConnector>>) -> Result<Tz> {
    let result = hub.settings().list().doit().await;
    let settings = result.unwrap().1.items.unwrap_or_default();

    let timezone_setting = settings
        .iter()
        .find(|setting| setting.id == Some("timezone".to_string()))
        .ok_or("Timezone setting not found");

    let timezone = timezone_setting.unwrap();
    let tz: Tz = timezone.value.as_ref().unwrap().parse().unwrap();
    Ok(tz)
}

/// Reads the Google application secret from the specified path.
///
/// This function reads and parses the Google application secret JSON file into an ApplicationSecret structure.
///
/// ## Arguments
///
/// * `path` - A reference to the Path of the Google application secret JSON file.
///
/// ## Returns
///
/// * `Result<ApplicationSecret, anyhow::Error>` - A result containing the ApplicationSecret or an error if the file cannot be read.
///
/// ## Errors
///
/// This function will return an error if:
/// - The file cannot be read.
/// - The contents of the file cannot be parsed into an ApplicationSecret.
async fn read_google_secret(path: &Path) -> Result<ApplicationSecret> {
    let secret = yup_oauth2::read_application_secret(path)
        .await
        .with_context(|| {
            format!(
                "Failed to read the Google application secret file from path {:?}.",
                path
            )
        })?;
    Ok(secret)
}
