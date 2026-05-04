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

const FALLBACK_CLIENT_ID: &str =
    "REDACTED_OAUTH_CLIENT_ID";
const FALLBACK_CLIENT_SECRET: &str = "REDACTED_OAUTH_SECRET";

/// Source of the OAuth ApplicationSecret used by `auth()`.
///
/// Resolution order:
/// 1. `GCAL_CLIENT_ID` + `GCAL_CLIENT_SECRET` env vars (in-memory secret).
/// 2. `GCAL_SECRET_FILE` env var pointing to a JSON file.
/// 3. `~/.gcal/secret.json` (legacy default).
/// 4. Built-in fallback OAuth project (shared, rate-limited).
#[derive(Debug)]
enum SecretSource {
    Env,
    EnvFile(PathBuf),
    DefaultFile(PathBuf),
    Fallback,
}

async fn resolve_secret() -> Result<(ApplicationSecret, SecretSource), Box<dyn Error>> {
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

    let default_path: PathBuf = file::get_absolute_path(".gcal/secret.json")?;
    let _ = file::ensure_directory_exists(&default_path);
    if default_path.is_file() {
        let secret = read_google_secret(&default_path).await?;
        return Ok((secret, SecretSource::DefaultFile(default_path)));
    }

    let fallback = build_secret(FALLBACK_CLIENT_ID, FALLBACK_CLIENT_SECRET, None);
    Ok((fallback, SecretSource::Fallback))
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

/// Authenticates the user with Google Calendar API and returns a CalendarHub instance.
///
/// Looks up OAuth credentials via `resolve_secret` (env vars → custom file →
/// `~/.gcal/secret.json` → built-in fallback). Set `GCAL_VERBOSE=1` to print
/// the resolved source on stderr.
pub async fn auth() -> Result<CalendarHub<HttpsConnector<HttpConnector>>, Box<dyn Error>> {
    let (secret, source) = resolve_secret().await?;

    if env::var("GCAL_VERBOSE").ok().as_deref() == Some("1") {
        match &source {
            SecretSource::Env => eprintln!("gcal: OAuth secret from env (GCAL_CLIENT_ID/GCAL_CLIENT_SECRET)"),
            SecretSource::EnvFile(p) => eprintln!("gcal: OAuth secret from GCAL_SECRET_FILE={}", p.display()),
            SecretSource::DefaultFile(p) => eprintln!("gcal: OAuth secret from {}", p.display()),
            SecretSource::Fallback => eprintln!(
                "gcal: using built-in shared OAuth project (rate-limited). \
                 Set GCAL_CLIENT_ID/GCAL_CLIENT_SECRET or place ~/.gcal/secret.json to use your own."
            ),
        }
    } else if matches!(source, SecretSource::Fallback) {
        eprintln!(
            "gcal: warning — using built-in shared OAuth project. \
             Configure your own (see docs/custom_auth.md) to avoid the user cap."
        );
    }

    let auth_builder = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    );

    let store_path = file::get_absolute_path(".gcal/store.json")?;
    let auth = auth_builder
        .persist_tokens_to_disk(&store_path)
        .build()
        .await?;

    let scopes = &[
        "https://www.googleapis.com/auth/calendar",
        "https://www.googleapis.com/auth/calendar.events",
        "https://www.googleapis.com/auth/calendar.readonly",
        "https://www.googleapis.com/auth/calendar.events.readonly",
    ];

    match auth.token(scopes).await {
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
