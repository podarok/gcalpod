# Changelog

All notable changes to this project follow
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
The project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Fixed

### Removed

## [1.2.0] — 2026-05-14

### Added

- `transparency` (Free/Busy) field support across mutation surfaces (closes #9):
  - `gcal edit <id> --field transparency=opaque|transparent` flips an existing
    event between Busy and Free without a web-UI round-trip.
  - `gcal add <title> <date> --transparency opaque|transparent` sets the flag
    at insert time.
  - `gcal quick <text> --transparency opaque|transparent` applies the flag via
    the same post-create patch path as `--location` / `--description`.
  - `gcal import` parses RFC 5545 `TRANSP:TRANSPARENT` / `TRANSP:OPAQUE` from
    ICS source and forwards to `event.transparency`.
  - `ListEvent` v1 schema gains a `transparency` field; surfaced in `--format
    json`, `tsv`, `csv`. Table renderer unchanged. TSV / CSV column count
    grows from 10 to 11 — consumers that parse by fixed column index keep
    working; consumers that hard-code count must update.

## [1.1.0] — 2026-05-08

### Added

- `gcal quick --location <text>` (`-l`) and `--description <text>` (`-d`) flags:
  set event location/description via post-create patch (single PATCH call when
  both supplied). Mirrors existing `--conference` (`-c`) post-create pattern.
  Fixes the gap where Google quick-add NL parser ignores these fields,
  previously requiring a separate `gcal edit --field key=...` round-trip.

### Fixed

- `--profile` flag accepted before subcommand (closes #4). Dropped
  `args_conflicts_with_subcommands(true)` from root `Command` and the legacy
  top-level positional `<title>`/`<date>` + `--conference` args (covered by
  `gcal quick`). Bare `gcal add` now prints `--help`.
- `gcal auth status` correctly labels secret source (closes #5). Resolution
  chain mirrors `resolve_secret`: env → `GCAL_SECRET_FILE` → per-profile →
  shared (with `shared.flag`) → legacy → `<missing>`. Shared mode renders as
  `<path> (shared)`; plain legacy as `(legacy)`.

## [1.0.0] — 2026-05-04

First stable release. All planned features for v1.0.0 landed:
multi-profile auth (W0), full CLI surface (W1), remind + conky +
man page (W2), detach + rename + Apache 2.0 attribution (W3),
RTK failure-first + verbose (W5), hybrid grid/agenda renderer
with `--style` + `--lineart` (W7), shared OAuth secret across
profiles (W8), token refresh (epic-03).

Working knowledge transferred upstream to
[`template_for_agents/process-knowledge-base/gcalpod-queue/`](https://github.com/ITCare-Company/template_for_agents/tree/main/process-knowledge-base/gcalpod-queue);
local `queue/` removed from this repo as the project graduates from
in-flight planning to stable release.

### License

Relicensed original modifications under [PolyForm Noncommercial 1.0.0](LICENSE)
**plus** [gcalpod Sustainable License Addendum v1 (gSL-v1)](LICENSE-ADDENDUM.md).
Apache 2.0 retained for upstream-derived portions per Apache 2.0 §4.

The addendum adds five permissions on top of PolyForm Noncommercial:
A. Sponsorship Tier Grant ($5/mo+ → automatic commercial use).
B. Solo / Micro Carveout (≤ 2 people, < $20k revenue, ≤ $20k raised).
C. Apache 2.0 Time-Bomb (each release → Apache 2.0 after 4y).
D. Package Distribution Carveout (distros + registries free).
E. Contribution Grant (contributors get commercial rights).

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
