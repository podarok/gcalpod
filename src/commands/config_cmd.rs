use std::error::Error;

use chrono_tz::Tz;

use crate::config::Config;
use crate::util::format::OutputFormat;

pub enum ConfigAction {
    Get { key: String },
    Set { key: String, value: String },
    Unset { key: String },
    List,
    Path,
}

pub async fn run(action: ConfigAction) -> Result<(), Box<dyn Error>> {
    match action {
        ConfigAction::Get { key } => get(&key),
        ConfigAction::Set { key, value } => set(&key, &value),
        ConfigAction::Unset { key } => unset(&key),
        ConfigAction::List => list(),
        ConfigAction::Path => path(),
    }
}

fn read_field(cfg: &Config, key: &str) -> Result<Option<String>, Box<dyn Error>> {
    match key {
        "active_profile" => Ok(cfg.active_profile.clone()),
        "default_calendar" => Ok(cfg.default_calendar.clone()),
        "tz" => Ok(cfg.tz.clone()),
        "default_format" => Ok(cfg.default_format.clone()),
        other => Err(format!(
            "unknown key '{}'. Known: active_profile, default_calendar, tz, default_format",
            other
        )
        .into()),
    }
}

fn write_field(cfg: &mut Config, key: &str, value: Option<String>) -> Result<(), Box<dyn Error>> {
    match key {
        "active_profile" => cfg.active_profile = value,
        "default_calendar" => cfg.default_calendar = value,
        "tz" => cfg.tz = value,
        "default_format" => cfg.default_format = value,
        other => {
            return Err(format!(
                "unknown key '{}'. Known: active_profile, default_calendar, tz, default_format",
                other
            )
            .into())
        }
    }
    Ok(())
}

fn validate(key: &str, value: &str) -> Result<(), Box<dyn Error>> {
    match key {
        "tz" => {
            value
                .parse::<Tz>()
                .map_err(|e| format!("invalid IANA timezone '{}': {}", value, e))?;
        }
        "default_format" => {
            OutputFormat::parse(value).map_err(|e| e.to_string())?;
        }
        _ => {}
    }
    Ok(())
}

fn get(key: &str) -> Result<(), Box<dyn Error>> {
    let cfg = Config::load_or_default()?;
    match read_field(&cfg, key)? {
        Some(v) => println!("{}", v),
        None => {
            // Empty output + non-zero handled by caller; here just exit silently.
            // Match `git config --get` semantics: empty = unset.
        }
    }
    Ok(())
}

fn set(key: &str, value: &str) -> Result<(), Box<dyn Error>> {
    validate(key, value)?;
    let mut cfg = Config::load_or_default()?;
    write_field(&mut cfg, key, Some(value.to_string()))?;
    cfg.save_atomic()?;
    println!("{} = {}", key, value);
    Ok(())
}

fn unset(key: &str) -> Result<(), Box<dyn Error>> {
    let mut cfg = Config::load_or_default()?;
    write_field(&mut cfg, key, None)?;
    cfg.save_atomic()?;
    println!("{} unset", key);
    Ok(())
}

fn list() -> Result<(), Box<dyn Error>> {
    let cfg = Config::load_or_default()?;
    println!(
        "active_profile = {}",
        cfg.active_profile.as_deref().unwrap_or("")
    );
    println!(
        "default_calendar = {}",
        cfg.default_calendar.as_deref().unwrap_or("")
    );
    println!("tz = {}", cfg.tz.as_deref().unwrap_or(""));
    println!(
        "default_format = {}",
        cfg.default_format.as_deref().unwrap_or("")
    );
    Ok(())
}

fn path() -> Result<(), Box<dyn Error>> {
    println!("{}", Config::path()?.display());
    Ok(())
}
