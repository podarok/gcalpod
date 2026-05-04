use std::{error::Error, path::PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::util::file;

/// `~/.gcal/config.toml` — global, profile-agnostic settings.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub active_profile: Option<String>,
    pub default_calendar: Option<String>,
    pub tz: Option<String>,
    pub default_format: Option<String>,
}

impl Config {
    pub fn path() -> Result<PathBuf, Box<dyn Error>> {
        file::get_absolute_path(".gcal/config.toml")
    }

    pub fn load_or_default() -> Result<Self, Box<dyn Error>> {
        let p = Self::path()?;
        if !p.is_file() {
            return Ok(Self::default());
        }
        let raw = std::fs::read_to_string(&p)
            .with_context(|| format!("Failed to read {}", p.display()))?;
        let cfg = toml::from_str::<Self>(&raw)
            .with_context(|| format!("Failed to parse {} as TOML", p.display()))?;
        Ok(cfg)
    }

    /// Atomic write: serialize to `<path>.tmp` then rename.
    pub fn save_atomic(&self) -> Result<(), Box<dyn Error>> {
        let p = Self::path()?;
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let body = toml::to_string_pretty(self).context("Failed to serialize config to TOML")?;
        let tmp = p.with_extension("toml.tmp");
        std::fs::write(&tmp, body).with_context(|| format!("Failed to write {}", tmp.display()))?;
        std::fs::rename(&tmp, &p)
            .with_context(|| format!("Failed to rename {} -> {}", tmp.display(), p.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_serialize() {
        let cfg = Config {
            active_profile: Some("work".into()),
            default_calendar: Some("primary".into()),
            tz: Some("Europe/Kyiv".into()),
            default_format: Some("table".into()),
        };
        let s = toml::to_string_pretty(&cfg).unwrap();
        let back: Config = toml::from_str(&s).unwrap();
        assert_eq!(back.active_profile.as_deref(), Some("work"));
        assert_eq!(back.default_calendar.as_deref(), Some("primary"));
        assert_eq!(back.tz.as_deref(), Some("Europe/Kyiv"));
        assert_eq!(back.default_format.as_deref(), Some("table"));
    }

    #[test]
    fn empty_config_serializes() {
        let cfg = Config::default();
        let s = toml::to_string_pretty(&cfg).unwrap();
        let back: Config = toml::from_str(&s).unwrap();
        assert!(back.active_profile.is_none());
    }
}
