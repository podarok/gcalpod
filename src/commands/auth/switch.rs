use std::error::Error;

use crate::config::Config;
use crate::profile::Profile;

pub async fn run(target: &str) -> Result<(), Box<dyn Error>> {
    let prof = Profile::new(target)?;

    if !prof.dir.is_dir() {
        return Err(anyhow::anyhow!(
            "Profile '{}' does not exist. Create it via `gcal auth login --profile {}` first.",
            target,
            target
        )
        .into());
    }

    let mut cfg = Config::load_or_default()?;
    let prev = cfg.active_profile.clone();
    cfg.active_profile = Some(target.to_string());
    cfg.save_atomic()?;

    println!(
        "gcal: switched active profile {} -> '{}' (config: {})",
        match &prev {
            Some(p) => format!("'{}'", p),
            None => "<none>".to_string(),
        },
        target,
        Config::path()?.display()
    );
    Ok(())
}
