use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Tee directory under XDG-ish path. Falls back to `/tmp/gcal-tee` if
/// `dirs::data_local_dir()` is unavailable.
fn tee_dir() -> PathBuf {
    if let Some(mut p) = dirs::data_local_dir() {
        p.push("gcal");
        p.push("tee");
        return p;
    }
    PathBuf::from("/tmp/gcal-tee")
}

/// Write the full failure context to a log file, then print an RTK-style
/// one-liner with a recovery-metadata path on stderr. Always returns Ok
/// from the caller's perspective (logging is best-effort).
///
/// `command` should be the user-facing subcommand name (e.g. "list",
/// "auth login"). `error` is the full error message; if it spans
/// multiple lines, all of it goes into the log.
pub fn report_error(command: &str, error: &dyn std::fmt::Display) {
    let body = format!("{}\n", error);
    let log_path = match write_log(command, &body) {
        Ok(p) => p,
        Err(_) => {
            // Logging failed — print the full error directly so the user
            // doesn't lose context.
            eprintln!("gcal: {} failed: {}", command, error);
            return;
        }
    };
    eprintln!(
        "gcal: {} failed: {}; see {}",
        command,
        truncate_first_line(&body, 120),
        log_path.display()
    );
}

fn write_log(command: &str, body: &str) -> std::io::Result<PathBuf> {
    let dir = tee_dir();
    fs::create_dir_all(&dir)?;
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let safe_cmd = command.replace([' ', '/', '\\'], "_");
    let path = dir.join(format!("{}_{}.log", ts, safe_cmd));
    let mut f = fs::File::create(&path)?;
    f.write_all(body.as_bytes())?;
    Ok(path)
}

fn truncate_first_line(s: &str, max: usize) -> String {
    let line = s.lines().next().unwrap_or("");
    if line.len() <= max {
        line.to_string()
    } else {
        let mut out = line[..max].to_string();
        out.push('…');
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_short_unchanged() {
        assert_eq!(truncate_first_line("hello", 10), "hello");
    }

    #[test]
    fn truncate_long_clipped() {
        let long = "a".repeat(200);
        let out = truncate_first_line(&long, 10);
        assert!(out.ends_with('…'));
        assert_eq!(out.chars().count(), 11);
    }

    #[test]
    fn truncate_takes_first_line_only() {
        assert_eq!(truncate_first_line("first\nsecond", 100), "first");
    }

    #[test]
    fn write_log_creates_file_with_body() {
        let body = "operation failed\nfull stack here\n";
        let path = write_log("integ_test", body).expect("write_log");
        let read = std::fs::read_to_string(&path).expect("read");
        assert_eq!(read, body);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn write_log_sanitizes_command_name() {
        let path = write_log("auth login", "x").expect("write_log");
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        assert!(name.contains("auth_login"));
        assert!(!name.contains(' '));
        let _ = std::fs::remove_file(&path);
    }
}
