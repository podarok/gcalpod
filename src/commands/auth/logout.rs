use std::error::Error;

use crate::profile::Profile;
use crate::util::file;

pub struct LogoutArgs {
    pub all: bool,
    pub purge: bool,
}

pub async fn run(active: &Profile, args: LogoutArgs) -> Result<(), Box<dyn Error>> {
    let profiles = if args.all {
        list_all_profiles()?
    } else {
        vec![active.clone()]
    };

    if profiles.is_empty() {
        println!("gcal: no profiles to log out from.");
        return Ok(());
    }

    for prof in &profiles {
        logout_one(prof, args.purge)?;
    }
    Ok(())
}

fn logout_one(prof: &Profile, purge: bool) -> Result<(), Box<dyn Error>> {
    let store = prof.store_path();
    let secret = prof.secret_path();

    let mut acted = false;

    if store.is_file() {
        std::fs::remove_file(&store)?;
        println!("gcal: removed token at {}", store.display());
        acted = true;
    }

    if purge {
        if secret.is_file() {
            std::fs::remove_file(&secret)?;
            println!("gcal: removed secret at {}", secret.display());
            acted = true;
        }
        if prof.dir.is_dir() {
            // Best-effort: remove dir if it's now empty.
            match std::fs::remove_dir(&prof.dir) {
                Ok(_) => {
                    println!("gcal: removed profile dir {}", prof.dir.display());
                }
                Err(e) if e.kind() == std::io::ErrorKind::DirectoryNotEmpty => {
                    println!(
                        "gcal: profile dir {} not empty (extra files retained); skipping rmdir.",
                        prof.dir.display()
                    );
                }
                Err(e) => return Err(e.into()),
            }
        }
    }

    if !acted {
        println!(
            "gcal: profile '{}' had nothing to remove (no token, no secret).",
            prof.name
        );
    } else {
        println!(
            "gcal: logged out of profile '{}'{}.",
            prof.name,
            if purge { " (purged)" } else { "" }
        );
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
