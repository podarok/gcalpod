# Changelog

All notable changes to this project follow
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
The project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] — 2026-05-04

First stable release after the W3 detach + rename. Scope corresponds
to queue waves W0 (MVP) + W1 (full CLI surface) + W2-P0 (remind) +
W5-P2 step-1 (failure-first + recovery tee).

### Added

- **Multi-profile auth** under `~/.gcal/profiles/<name>/`:
  - `gcal auth login [--profile] [--scopes <csv>] [--no-browser] [--reauth]`
  - `gcal auth status [--profile|--all] [--check] [--show-token]`
  - `gcal auth logout [--profile|--all] [--purge]`
  - `gcal auth switch <profile>`
  - `gcal init` — interactive Google Cloud Console wizard.
- **Configuration**: `gcal config get/set/unset/list/path` for
  `~/.gcal/config.toml`. Validated keys: `active_profile`,
  `default_calendar`, `tz`, `default_format`.
- **Event range queries**:
  - `gcal list [--from] [--to] [--calendar] [--format json|tsv|csv|raw|table]`
    with natural-language inputs (`today`, `+7d`, `+2w`, weekday
    names, RFC3339, `YYYY-MM-DD`).
  - `gcal list --json` ergonomic alias for `--format json`.
  - Flat-list renderer for ranges > 14 days.
- **Agenda + search**:
  - `gcal agenda [--from --to --calendar --format]`
  - `gcal search <query> [--from --to --calendar --format]`
- **Event mutation**:
  - `gcal edit <event-id> [--field key=value]...` (keys: summary,
    description, location, start, end).
  - `gcal delete <event-id> [-y|--yes]` with confirmation gate.
- **Quick-add + conference**:
  - `gcal quick <text> [--conference]` — natural-language event
    create with optional Google Meet attachment via post-create patch.
  - `gcal "<text>" [<time>] [--conference]` legacy alias preserved.
- **Calendars**: `gcal calendars list [--format]` — id / summary /
  access role / primary / timezone.
- **ICS import**: `gcal import <path> [--calendar] [--dry-run]
  [--skip-duplicates]` — bulk insert from RFC 5545 files.
- **Reminders**: `gcal remind <mins> [--calendar] -- <command>...`
  with `{{summary}}` / `{{start}}` / `{{html_link}}` template
  interpolation.
- **OAuth credential resolution**:
  - `GCAL_CLIENT_ID` + `GCAL_CLIENT_SECRET` env vars (optional `GCAL_PROJECT_ID`).
  - `GCAL_SECRET_FILE=<path>` env var.
  - `~/.gcal/profiles/<active>/secret.json`.
  - Legacy `~/.gcal/secret.json` fallback (auto-migrated to
    `profiles/default/` on first default-profile run).
  - `GCAL_VERBOSE=1` prints which source was used.
- **Output formats** (read commands): `table` (human, default),
  `json` (tty pretty / pipe compact), `tsv`, `csv` (RFC 4180), `raw`
  (full upstream `Vec<Event>` JSON unchanged).
- **Failure-first reporting**: every operational command writes the
  full error body to `~/Library/Application Support/gcal/tee/<unix>_<cmd>.log`
  (or XDG `data_local_dir/gcal/tee` on Linux), prints a one-line
  summary with `; see <log path>` recovery metadata.
- **Sponsorship + attribution**: `.github/FUNDING.yml`, `NOTICE.md`,
  Apache 2.0 `LICENSE` retained from upstream.

### Changed

- Detached from `rust-dd/google-calendar-cli` GitHub fork (W3) —
  repo is now standalone `podarok/gcalpod`.
- Removed hardcoded shared-OAuth fallback secret from source
  (security fix: GitHub secret-scanning alert resolved). History
  rewritten via `git-filter-repo` + force-pushed.
- `gcal list` defaults preserved (current Monday-anchored week)
  for back-compat.

### Removed

- Built-in shared OAuth project. Every user must configure their
  own Google Cloud OAuth client (env vars / file / `~/.gcal/secret.json`).

### Pending (post-1.0.0)

- `--help` text trim + `-u/--ultra-compact` global flag (W5-P2 step-2/3).
- Conky / Tera template formatters (W2-P1).
- Man page via `clap_mangen` (W2-P2).
- crates.io publish (owner gate).

### Acknowledgements

Builds on top of [`rust-dd/google-calendar-cli`](https://github.com/rust-dd/google-calendar-cli)
(Apache 2.0). Substantial modifications listed in [`NOTICE.md`](NOTICE.md).
