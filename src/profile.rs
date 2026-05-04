use std::{env, error::Error, path::PathBuf};

use anyhow::{Context, Result};

use crate::config::Config;
use crate::util::file;

/// Per-profile credential + token directory under `~/.gcal/profiles/<name>/`.
#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub dir: PathBuf,
}

impl Profile {
    pub fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        let mut dir = file::get_absolute_path(".gcal/profiles")?;
        dir.push(name);
        Ok(Self {
            name: name.to_string(),
            dir,
        })
    }

    pub fn ensure_dir(&self) -> Result<(), Box<dyn Error>> {
        std::fs::create_dir_all(&self.dir)
            .with_context(|| format!("Failed to create profile dir {}", self.dir.display()))?;
        Ok(())
    }

    pub fn secret_path(&self) -> PathBuf {
        self.dir.join("secret.json")
    }

    pub fn store_path(&self) -> PathBuf {
        self.dir.join("store.json")
    }

    /// Resolve the active profile name.
    ///
    /// Order: `cli_flag` > `GCAL_PROFILE` env > `config.active_profile` > `"default"`.
    pub fn resolve_active(cli_flag: Option<&str>, config: &Config) -> String {
        if let Some(p) = cli_flag {
            return p.to_string();
        }
        if let Ok(p) = env::var("GCAL_PROFILE") {
            if !p.is_empty() {
                return p;
            }
        }
        if let Some(p) = &config.active_profile {
            if !p.is_empty() {
                return p.clone();
            }
        }
        "default".to_string()
    }

    /// Migrate legacy `~/.gcal/secret.json` + `~/.gcal/store.json` into
    /// this profile's dir if (a) the profile is `default`, (b) profile dir
    /// has no existing secret/store, and (c) legacy files exist.
    ///
    /// Move (not copy) — single source of truth. Logs one stderr line on success.
    pub fn migrate_legacy_if_needed(&self) -> Result<(), Box<dyn Error>> {
        if self.name != "default" {
            return Ok(());
        }
        self.ensure_dir()?;

        let legacy_secret = file::get_absolute_path(".gcal/secret.json")?;
        let legacy_store = file::get_absolute_path(".gcal/store.json")?;

        let target_secret = self.secret_path();
        let target_store = self.store_path();

        let mut moved = false;
        if legacy_secret.is_file() && !target_secret.exists() {
            std::fs::rename(&legacy_secret, &target_secret).with_context(|| {
                format!(
                    "Failed to move {} -> {}",
                    legacy_secret.display(),
                    target_secret.display()
                )
            })?;
            moved = true;
        }
        if legacy_store.is_file() && !target_store.exists() {
            std::fs::rename(&legacy_store, &target_store).with_context(|| {
                format!(
                    "Failed to move {} -> {}",
                    legacy_store.display(),
                    target_store.display()
                )
            })?;
            moved = true;
        }
        if moved {
            eprintln!(
                "gcal: migrated legacy ~/.gcal/{{secret,store}}.json -> {}",
                self.dir.display()
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn resolve_active_flag_wins() {
        let cfg = Config {
            active_profile: Some("config_p".into()),
            ..Default::default()
        };
        std::env::set_var("GCAL_PROFILE", "env_p");
        assert_eq!(Profile::resolve_active(Some("flag_p"), &cfg), "flag_p");
        std::env::remove_var("GCAL_PROFILE");
    }

    #[test]
    fn resolve_active_env_beats_config() {
        let cfg = Config {
            active_profile: Some("config_p".into()),
            ..Default::default()
        };
        std::env::set_var("GCAL_PROFILE", "env_p");
        assert_eq!(Profile::resolve_active(None, &cfg), "env_p");
        std::env::remove_var("GCAL_PROFILE");
    }

    #[test]
    fn resolve_active_config_beats_default() {
        let cfg = Config {
            active_profile: Some("config_p".into()),
            ..Default::default()
        };
        std::env::remove_var("GCAL_PROFILE");
        assert_eq!(Profile::resolve_active(None, &cfg), "config_p");
    }

    #[test]
    fn resolve_active_falls_back_to_default() {
        let cfg = Config::default();
        std::env::remove_var("GCAL_PROFILE");
        assert_eq!(Profile::resolve_active(None, &cfg), "default");
    }
}
